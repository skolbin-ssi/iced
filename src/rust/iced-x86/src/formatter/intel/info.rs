// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

use crate::formatter::intel::enums::*;
use crate::formatter::intel::fmt_utils::show_segment_prefix;
use crate::formatter::intel::get_mnemonic_cc;
use crate::formatter::intel::mem_size_tbl::MEM_SIZE_TBL;
use crate::formatter::intel::regs::*;
use crate::formatter::FormatterString;
use crate::iced_constants::IcedConstants;
use crate::*;
use alloc::string::String;
use alloc::vec::Vec;
use core::{mem, u32};
use static_assertions::const_assert_eq;

#[derive(Debug)]
pub(super) struct InstrOpInfo<'a> {
	pub(super) mnemonic: &'a FormatterString,
	pub(super) flags: u16, // InstrOpInfoFlags
	pub(super) op_count: u8,
	op_kinds: [InstrOpKind; IcedConstants::MAX_OP_COUNT],
	op_registers: [u8; IcedConstants::MAX_OP_COUNT],
	op_indexes: [i8; IcedConstants::MAX_OP_COUNT],
}

impl<'a> InstrOpInfo<'a> {
	#[allow(dead_code)]
	pub(super) const TEST_REGISTER_BITS: u32 = IcedConstants::REGISTER_BITS;

	pub(super) fn op_register(&self, operand: u32) -> usize {
		self.op_registers[operand as usize] as usize
	}

	pub(super) fn op_kind(&self, operand: u32) -> InstrOpKind {
		if let Some(op_kind) = self.op_kinds.get(operand as usize) {
			*op_kind
		} else {
			debug_assert!(
				self.op_kinds[0] == InstrOpKind::DeclareByte
					|| self.op_kinds[0] == InstrOpKind::DeclareWord
					|| self.op_kinds[0] == InstrOpKind::DeclareDword
					|| self.op_kinds[0] == InstrOpKind::DeclareQword
			);
			self.op_kinds[0]
		}
	}

	pub(super) fn instruction_index(&self, operand: u32) -> Option<u32> {
		let instruction_operand = if let Some(operand) = self.op_indexes.get(operand as usize) {
			*operand
		} else {
			debug_assert!(
				self.op_kinds[0] == InstrOpKind::DeclareByte
					|| self.op_kinds[0] == InstrOpKind::DeclareWord
					|| self.op_kinds[0] == InstrOpKind::DeclareDword
					|| self.op_kinds[0] == InstrOpKind::DeclareQword
			);
			-1
		};
		if instruction_operand < 0 {
			None
		} else {
			Some(instruction_operand as u32)
		}
	}

	#[cfg(feature = "instr_info")]
	pub(super) fn op_access(&self, operand: u32) -> Option<OpAccess> {
		let instruction_operand = match operand {
			0 => self.op_indexes[0],
			1 => self.op_indexes[1],
			2 => self.op_indexes[2],
			3 => self.op_indexes[3],
			4 => self.op_indexes[4],
			_ => {
				debug_assert!(
					self.op_kinds[0] == InstrOpKind::DeclareByte
						|| self.op_kinds[0] == InstrOpKind::DeclareWord
						|| self.op_kinds[0] == InstrOpKind::DeclareDword
						|| self.op_kinds[0] == InstrOpKind::DeclareQword
				);
				self.op_indexes[0]
			}
		};
		if instruction_operand < OP_ACCESS_INVALID {
			Some(unsafe { mem::transmute(-instruction_operand - 2) })
		} else {
			None
		}
	}

	pub(super) fn operand_index(&self, instruction_operand: u32) -> Option<u32> {
		let index: i32 = if instruction_operand == self.op_indexes[0] as u32 {
			0
		} else if instruction_operand == self.op_indexes[1] as u32 {
			1
		} else if instruction_operand == self.op_indexes[2] as u32 {
			2
		} else if instruction_operand == self.op_indexes[3] as u32 {
			3
		} else if instruction_operand == self.op_indexes[4] as u32 {
			4
		} else {
			-1
		};
		if (index as u32) < self.op_count as u32 {
			Some(index as u32)
		} else {
			None
		}
	}

	#[inline]
	fn default(mnemonic: &'a FormatterString) -> Self {
		Self {
			mnemonic,
			flags: 0,
			op_count: 0,
			op_kinds: [InstrOpKind::default(); IcedConstants::MAX_OP_COUNT],
			op_registers: [0; IcedConstants::MAX_OP_COUNT],
			op_indexes: [0; IcedConstants::MAX_OP_COUNT],
		}
	}

	fn new(mnemonic: &'a FormatterString, instruction: &Instruction, flags: u32) -> Self {
		let mut res = InstrOpInfo::default(mnemonic);

		const_assert_eq!(IcedConstants::MAX_OP_COUNT, 5);
		res.flags = flags as u16;
		res.op_kinds[0] = unsafe { mem::transmute(instruction.op0_kind() as u8) };
		res.op_kinds[1] = unsafe { mem::transmute(instruction.op1_kind() as u8) };
		res.op_kinds[2] = unsafe { mem::transmute(instruction.op2_kind() as u8) };
		res.op_kinds[3] = unsafe { mem::transmute(instruction.op3_kind() as u8) };
		res.op_kinds[4] = unsafe { mem::transmute(instruction.op4_kind() as u8) };
		const_assert_eq!(InstrOpInfo::TEST_REGISTER_BITS, 8);
		res.op_registers[0] = instruction.op0_register() as u8;
		const_assert_eq!(InstrOpInfo::TEST_REGISTER_BITS, 8);
		res.op_registers[1] = instruction.op1_register() as u8;
		const_assert_eq!(InstrOpInfo::TEST_REGISTER_BITS, 8);
		res.op_registers[2] = instruction.op2_register() as u8;
		const_assert_eq!(InstrOpInfo::TEST_REGISTER_BITS, 8);
		res.op_registers[3] = instruction.op3_register() as u8;
		const_assert_eq!(InstrOpInfo::TEST_REGISTER_BITS, 8);
		res.op_registers[4] = instruction.op4_register() as u8;
		let op_count = instruction.op_count();
		res.op_count = op_count as u8;
		match op_count {
			0 => {
				res.op_indexes[0] = OP_ACCESS_INVALID;
				res.op_indexes[1] = OP_ACCESS_INVALID;
				res.op_indexes[2] = OP_ACCESS_INVALID;
				res.op_indexes[3] = OP_ACCESS_INVALID;
				res.op_indexes[4] = OP_ACCESS_INVALID;
			}

			1 => {
				res.op_indexes[1] = OP_ACCESS_INVALID;
				res.op_indexes[2] = OP_ACCESS_INVALID;
				res.op_indexes[3] = OP_ACCESS_INVALID;
				res.op_indexes[4] = OP_ACCESS_INVALID;
			}

			2 => {
				res.op_indexes[1] = 1;
				res.op_indexes[2] = OP_ACCESS_INVALID;
				res.op_indexes[3] = OP_ACCESS_INVALID;
				res.op_indexes[4] = OP_ACCESS_INVALID;
			}

			3 => {
				res.op_indexes[1] = 1;
				res.op_indexes[2] = 2;
				res.op_indexes[3] = OP_ACCESS_INVALID;
				res.op_indexes[4] = OP_ACCESS_INVALID;
			}

			4 => {
				res.op_indexes[1] = 1;
				res.op_indexes[2] = 2;
				res.op_indexes[3] = 3;
				res.op_indexes[4] = OP_ACCESS_INVALID;
			}

			5 => {
				res.op_indexes[1] = 1;
				res.op_indexes[2] = 2;
				res.op_indexes[3] = 3;
				res.op_indexes[4] = 4;
			}

			_ => unreachable!(),
		}

		res
	}
}

const OP_ACCESS_INVALID: i8 = -1;
struct InstrInfoConstants;
#[cfg(feature = "instr_info")]
impl InstrInfoConstants {
	pub const OP_ACCESS_NONE: i8 = -(OpAccess::None as i8 + 2);
	pub const OP_ACCESS_READ: i8 = -(OpAccess::Read as i8 + 2);
	pub const OP_ACCESS_WRITE: i8 = -(OpAccess::Write as i8 + 2);
	pub const OP_ACCESS_READ_WRITE: i8 = -(OpAccess::ReadWrite as i8 + 2);
}
#[cfg(not(feature = "instr_info"))]
impl InstrInfoConstants {
	pub const OP_ACCESS_NONE: i8 = OP_ACCESS_INVALID;
	pub const OP_ACCESS_READ: i8 = OP_ACCESS_INVALID;
	pub const OP_ACCESS_WRITE: i8 = OP_ACCESS_INVALID;
	pub const OP_ACCESS_READ_WRITE: i8 = OP_ACCESS_INVALID;
}

pub(super) trait InstrInfo {
	fn op_info<'a>(&'a self, options: &FormatterOptions, instruction: &Instruction) -> InstrOpInfo<'a>;
}

fn get_bitness(code_size: CodeSize) -> u32 {
	match code_size {
		CodeSize::Code16 => 16,
		CodeSize::Code32 => 32,
		CodeSize::Code64 => 64,
		_ => 0,
	}
}

pub(super) struct SimpleInstrInfo {
	mnemonic: FormatterString,
	flags: u32,
}

impl SimpleInstrInfo {
	pub(super) fn with_mnemonic(mnemonic: String) -> Self {
		Self { mnemonic: FormatterString::new(mnemonic), flags: InstrOpInfoFlags::NONE }
	}
	pub(super) fn new(mnemonic: String, flags: u32) -> Self {
		Self { mnemonic: FormatterString::new(mnemonic), flags }
	}
}

impl InstrInfo for SimpleInstrInfo {
	fn op_info<'a>(&'a self, _options: &FormatterOptions, instruction: &Instruction) -> InstrOpInfo<'a> {
		InstrOpInfo::new(&self.mnemonic, instruction, self.flags)
	}
}

#[allow(non_camel_case_types)]
pub(super) struct SimpleInstrInfo_cc {
	mnemonics: Vec<FormatterString>,
	cc_index: u32,
}

impl SimpleInstrInfo_cc {
	pub(super) fn new(cc_index: u32, mnemonics: Vec<String>) -> Self {
		Self { mnemonics: FormatterString::with_strings(mnemonics), cc_index }
	}
}

impl InstrInfo for SimpleInstrInfo_cc {
	fn op_info<'a>(&'a self, options: &FormatterOptions, instruction: &Instruction) -> InstrOpInfo<'a> {
		const FLAGS: u32 = InstrOpInfoFlags::NONE;
		let mnemonic = get_mnemonic_cc(options, self.cc_index, &self.mnemonics);
		InstrOpInfo::new(mnemonic, instruction, FLAGS)
	}
}

#[allow(non_camel_case_types)]
pub(super) struct SimpleInstrInfo_memsize {
	mnemonic: FormatterString,
	bitness: u32,
}

impl SimpleInstrInfo_memsize {
	pub(super) fn new(bitness: u32, mnemonic: String) -> Self {
		Self { mnemonic: FormatterString::new(mnemonic), bitness }
	}
}

impl InstrInfo for SimpleInstrInfo_memsize {
	fn op_info<'a>(&'a self, _options: &FormatterOptions, instruction: &Instruction) -> InstrOpInfo<'a> {
		let instr_bitness = get_bitness(instruction.code_size());
		let flags = if instr_bitness == 0 || (instr_bitness & self.bitness) != 0 {
			InstrOpInfoFlags::MEM_SIZE_NOTHING
		} else {
			InstrOpInfoFlags::SHOW_NO_MEM_SIZE_FORCE_SIZE | InstrOpInfoFlags::SHOW_MIN_MEM_SIZE_FORCE_SIZE
		};
		InstrOpInfo::new(&self.mnemonic, instruction, flags)
	}
}

#[allow(non_camel_case_types)]
pub(super) struct SimpleInstrInfo_StringIg1 {
	mnemonic: FormatterString,
}

impl SimpleInstrInfo_StringIg1 {
	pub(super) fn new(mnemonic: String) -> Self {
		Self { mnemonic: FormatterString::new(mnemonic) }
	}
}

impl InstrInfo for SimpleInstrInfo_StringIg1 {
	fn op_info<'a>(&'a self, _options: &FormatterOptions, instruction: &Instruction) -> InstrOpInfo<'a> {
		let mut info = InstrOpInfo::default(&self.mnemonic);
		info.op_count = 1;
		info.op_kinds[0] = unsafe { mem::transmute(instruction.op0_kind() as u8) };
		info
	}
}

#[allow(non_camel_case_types)]
pub(super) struct SimpleInstrInfo_StringIg0 {
	mnemonic: FormatterString,
}

impl SimpleInstrInfo_StringIg0 {
	pub(super) fn new(mnemonic: String) -> Self {
		Self { mnemonic: FormatterString::new(mnemonic) }
	}
}

impl InstrInfo for SimpleInstrInfo_StringIg0 {
	fn op_info<'a>(&'a self, _options: &FormatterOptions, instruction: &Instruction) -> InstrOpInfo<'a> {
		let mut info = InstrOpInfo::default(&self.mnemonic);
		info.op_count = 1;
		info.op_kinds[0] = unsafe { mem::transmute(instruction.op1_kind() as u8) };
		info.op_indexes[0] = info.op_indexes[1];
		info
	}
}

#[allow(non_camel_case_types)]
pub(super) struct SimpleInstrInfo_nop {
	mnemonic: FormatterString,
	bitness: u32,
	register: Register,
	str_xchg: FormatterString,
}

impl SimpleInstrInfo_nop {
	pub(super) fn new(bitness: u32, mnemonic: String, register: Register) -> Self {
		Self { mnemonic: FormatterString::new(mnemonic), bitness, register, str_xchg: FormatterString::new_str("xchg") }
	}
}

impl InstrInfo for SimpleInstrInfo_nop {
	fn op_info<'a>(&'a self, _options: &FormatterOptions, instruction: &Instruction) -> InstrOpInfo<'a> {
		let instr_bitness = get_bitness(instruction.code_size());
		if instr_bitness == 0 || (instr_bitness & self.bitness) != 0 {
			InstrOpInfo::new(&self.mnemonic, instruction, InstrOpInfoFlags::NONE)
		} else {
			let mut info = InstrOpInfo::default(&self.str_xchg);
			info.op_count = 2;
			const_assert_eq!(InstrOpKind::Register as u32, 0);
			//info.op_kinds[0] = InstrOpKind::Register;
			//info.op_kinds[1] = InstrOpKind::Register;
			const_assert_eq!(InstrOpInfo::TEST_REGISTER_BITS, 8);
			info.op_registers[0] = self.register as u8;
			const_assert_eq!(InstrOpInfo::TEST_REGISTER_BITS, 8);
			info.op_registers[1] = self.register as u8;
			info.op_indexes[0] = InstrInfoConstants::OP_ACCESS_NONE;
			info.op_indexes[1] = InstrInfoConstants::OP_ACCESS_NONE;
			info
		}
	}
}

#[allow(non_camel_case_types)]
pub(super) struct SimpleInstrInfo_ST1 {
	mnemonic: FormatterString,
	flags: u32,
	op0_access: i8,
}

impl SimpleInstrInfo_ST1 {
	pub(super) fn with_mnemonic(mnemonic: String, flags: u32) -> Self {
		SimpleInstrInfo_ST1::new(mnemonic, flags, false)
	}
	pub(super) fn new(mnemonic: String, flags: u32, is_load: bool) -> Self {
		Self {
			mnemonic: FormatterString::new(mnemonic),
			flags,
			op0_access: if is_load { InstrInfoConstants::OP_ACCESS_WRITE } else { InstrInfoConstants::OP_ACCESS_READ_WRITE },
		}
	}
}

impl InstrInfo for SimpleInstrInfo_ST1 {
	fn op_info<'a>(&'a self, _options: &FormatterOptions, instruction: &Instruction) -> InstrOpInfo<'a> {
		let mut info = InstrOpInfo::new(&self.mnemonic, instruction, self.flags);
		debug_assert_eq!(instruction.op_count(), 1);
		info.op_count = 2;
		info.op_kinds[1] = info.op_kinds[0];
		info.op_registers[1] = info.op_registers[0];
		info.op_kinds[0] = InstrOpKind::Register;
		const_assert_eq!(InstrOpInfo::TEST_REGISTER_BITS, 8);
		info.op_registers[0] = Registers::REGISTER_ST as u8;
		info.op_indexes[1] = info.op_indexes[0];
		info.op_indexes[0] = self.op0_access;
		info
	}
}

#[allow(non_camel_case_types)]
pub(super) struct SimpleInstrInfo_ST2 {
	mnemonic: FormatterString,
	flags: u32,
}

impl SimpleInstrInfo_ST2 {
	pub(super) fn new(mnemonic: String, flags: u32) -> Self {
		Self { mnemonic: FormatterString::new(mnemonic), flags }
	}
}

impl InstrInfo for SimpleInstrInfo_ST2 {
	fn op_info<'a>(&'a self, _options: &FormatterOptions, instruction: &Instruction) -> InstrOpInfo<'a> {
		let mut info = InstrOpInfo::new(&self.mnemonic, instruction, self.flags);
		debug_assert_eq!(instruction.op_count(), 1);
		info.op_count = 2;
		info.op_kinds[1] = InstrOpKind::Register;
		const_assert_eq!(InstrOpInfo::TEST_REGISTER_BITS, 8);
		info.op_registers[1] = Registers::REGISTER_ST as u8;
		info.op_indexes[1] = InstrInfoConstants::OP_ACCESS_READ;
		info
	}
}

#[allow(non_camel_case_types)]
pub(super) struct SimpleInstrInfo_maskmovq {
	mnemonic: FormatterString,
}

impl SimpleInstrInfo_maskmovq {
	pub(super) fn new(mnemonic: String) -> Self {
		Self { mnemonic: FormatterString::new(mnemonic) }
	}
}

impl InstrInfo for SimpleInstrInfo_maskmovq {
	fn op_info<'a>(&'a self, options: &FormatterOptions, instruction: &Instruction) -> InstrOpInfo<'a> {
		debug_assert_eq!(instruction.op_count(), 3);

		let op_kind = instruction.op0_kind();
		let short_form_op_kind = match instruction.code_size() {
			CodeSize::Unknown => op_kind,
			CodeSize::Code16 => OpKind::MemorySegDI,
			CodeSize::Code32 => OpKind::MemorySegEDI,
			CodeSize::Code64 => OpKind::MemorySegRDI,
		};
		let mut flags = InstrOpInfoFlags::IGNORE_SEGMENT_PREFIX;
		if op_kind != short_form_op_kind {
			if op_kind == OpKind::MemorySegDI {
				flags |= InstrOpInfoFlags::ADDR_SIZE16;
			} else if op_kind == OpKind::MemorySegEDI {
				flags |= InstrOpInfoFlags::ADDR_SIZE32;
			} else if op_kind == OpKind::MemorySegRDI {
				flags |= InstrOpInfoFlags::ADDR_SIZE64;
			}
		}
		let mut info = InstrOpInfo::default(&self.mnemonic);
		info.flags = flags as u16;
		info.op_count = 2;
		info.op_kinds[0] = unsafe { mem::transmute(instruction.op1_kind() as u8) };
		info.op_indexes[0] = 1;
		const_assert_eq!(InstrOpInfo::TEST_REGISTER_BITS, 8);
		info.op_registers[0] = instruction.op1_register() as u8;
		info.op_kinds[1] = unsafe { mem::transmute(instruction.op2_kind() as u8) };
		info.op_indexes[1] = 2;
		const_assert_eq!(InstrOpInfo::TEST_REGISTER_BITS, 8);
		info.op_registers[1] = instruction.op2_register() as u8;
		let seg_reg = instruction.segment_prefix();
		if seg_reg != Register::None && show_segment_prefix(Register::None, instruction, options) {
			info.op_count = 3;
			info.op_kinds[2] = InstrOpKind::Register;
			const_assert_eq!(InstrOpInfo::TEST_REGISTER_BITS, 8);
			info.op_registers[2] = seg_reg as u8;
			info.op_indexes[2] = InstrInfoConstants::OP_ACCESS_READ;
		}
		info
	}
}

#[allow(non_camel_case_types)]
pub(super) struct SimpleInstrInfo_os {
	mnemonic: FormatterString,
	bitness: u32,
	flags: u32,
}

impl SimpleInstrInfo_os {
	pub(super) fn with_mnemonic(bitness: u32, mnemonic: String) -> Self {
		Self { mnemonic: FormatterString::new(mnemonic), bitness, flags: InstrOpInfoFlags::NONE }
	}
	pub(super) fn new(bitness: u32, mnemonic: String, flags: u32) -> Self {
		Self { mnemonic: FormatterString::new(mnemonic), bitness, flags }
	}
}

impl InstrInfo for SimpleInstrInfo_os {
	fn op_info<'a>(&'a self, _options: &FormatterOptions, instruction: &Instruction) -> InstrOpInfo<'a> {
		let mut flags = self.flags;
		let instr_bitness = get_bitness(instruction.code_size());
		if instr_bitness != 0 && instr_bitness != self.bitness {
			if self.bitness == 16 {
				flags |= InstrOpInfoFlags::OP_SIZE16;
			} else if self.bitness == 32 {
				if instr_bitness != 64 {
					flags |= InstrOpInfoFlags::OP_SIZE32;
				}
			} else {
				flags |= InstrOpInfoFlags::OP_SIZE64;
			}
		}
		InstrOpInfo::new(&self.mnemonic, instruction, flags)
	}
}

#[allow(non_camel_case_types)]
pub(super) struct SimpleInstrInfo_os_bnd {
	mnemonic: FormatterString,
	bitness: u32,
}

impl SimpleInstrInfo_os_bnd {
	pub(super) fn new(bitness: u32, mnemonic: String) -> Self {
		Self { mnemonic: FormatterString::new(mnemonic), bitness }
	}
}

impl InstrInfo for SimpleInstrInfo_os_bnd {
	fn op_info<'a>(&'a self, _options: &FormatterOptions, instruction: &Instruction) -> InstrOpInfo<'a> {
		let mut flags = InstrOpInfoFlags::NONE;
		if instruction.has_repne_prefix() {
			flags |= InstrOpInfoFlags::BND_PREFIX;
		}
		let instr_bitness = get_bitness(instruction.code_size());
		if instr_bitness != 0 && instr_bitness != self.bitness {
			if self.bitness == 16 {
				flags |= InstrOpInfoFlags::OP_SIZE16;
			} else if self.bitness == 32 {
				flags |= InstrOpInfoFlags::OP_SIZE32;
			} else {
				flags |= InstrOpInfoFlags::OP_SIZE64;
			}
		}
		InstrOpInfo::new(&self.mnemonic, instruction, flags)
	}
}

#[allow(non_camel_case_types)]
pub(super) struct SimpleInstrInfo_as {
	mnemonic: FormatterString,
	bitness: u32,
}

impl SimpleInstrInfo_as {
	pub(super) fn new(bitness: u32, mnemonic: String) -> Self {
		Self { mnemonic: FormatterString::new(mnemonic), bitness }
	}
}

impl InstrInfo for SimpleInstrInfo_as {
	fn op_info<'a>(&'a self, _options: &FormatterOptions, instruction: &Instruction) -> InstrOpInfo<'a> {
		let mut flags = InstrOpInfoFlags::NONE;
		let instr_bitness = get_bitness(instruction.code_size());
		if instr_bitness != 0 && instr_bitness != self.bitness {
			if self.bitness == 16 {
				flags |= InstrOpInfoFlags::ADDR_SIZE16;
			} else if self.bitness == 32 {
				flags |= InstrOpInfoFlags::ADDR_SIZE32;
			} else {
				flags |= InstrOpInfoFlags::ADDR_SIZE64;
			}
		}
		InstrOpInfo::new(&self.mnemonic, instruction, flags)
	}
}

#[allow(non_camel_case_types)]
pub(super) struct SimpleInstrInfo_os_jcc {
	mnemonics: Vec<FormatterString>,
	bitness: u32,
	cc_index: u32,
	flags: u32,
}

impl SimpleInstrInfo_os_jcc {
	pub(super) fn with_mnemonic(bitness: u32, cc_index: u32, mnemonics: Vec<String>) -> Self {
		Self { mnemonics: FormatterString::with_strings(mnemonics), bitness, cc_index, flags: InstrOpInfoFlags::NONE }
	}
	pub(super) fn new(bitness: u32, cc_index: u32, mnemonics: Vec<String>, flags: u32) -> Self {
		Self { mnemonics: FormatterString::with_strings(mnemonics), bitness, cc_index, flags }
	}
}

impl InstrInfo for SimpleInstrInfo_os_jcc {
	fn op_info<'a>(&'a self, options: &FormatterOptions, instruction: &Instruction) -> InstrOpInfo<'a> {
		let mut flags = self.flags;
		let instr_bitness = get_bitness(instruction.code_size());
		if instr_bitness != 0 && instr_bitness != self.bitness {
			if self.bitness == 16 {
				flags |= InstrOpInfoFlags::OP_SIZE16;
			} else if self.bitness == 32 {
				flags |= InstrOpInfoFlags::OP_SIZE32;
			} else {
				flags |= InstrOpInfoFlags::OP_SIZE64;
			}
		}
		let prefix_seg = instruction.segment_prefix();
		if prefix_seg == Register::CS {
			flags |= InstrOpInfoFlags::IGNORE_SEGMENT_PREFIX | InstrOpInfoFlags::JCC_NOT_TAKEN;
		} else if prefix_seg == Register::DS {
			flags |= InstrOpInfoFlags::IGNORE_SEGMENT_PREFIX | InstrOpInfoFlags::JCC_TAKEN;
		}
		if instruction.has_repne_prefix() {
			flags |= InstrOpInfoFlags::BND_PREFIX;
		}
		let mnemonic = get_mnemonic_cc(options, self.cc_index, &self.mnemonics);
		InstrOpInfo::new(mnemonic, instruction, flags)
	}
}

#[allow(non_camel_case_types)]
pub(super) struct SimpleInstrInfo_os_loop {
	mnemonics: Vec<FormatterString>,
	bitness: u32,
	cc_index: u32,
	register: Register,
}

impl SimpleInstrInfo_os_loop {
	pub(super) fn new(bitness: u32, cc_index: u32, register: Register, mnemonics: Vec<String>) -> Self {
		Self { mnemonics: FormatterString::with_strings(mnemonics), bitness, cc_index, register }
	}
}

impl InstrInfo for SimpleInstrInfo_os_loop {
	fn op_info<'a>(&'a self, options: &FormatterOptions, instruction: &Instruction) -> InstrOpInfo<'a> {
		let mut flags = InstrOpInfoFlags::NONE;
		let instr_bitness = get_bitness(instruction.code_size());
		let expected_reg = match instr_bitness {
			0 => self.register,
			16 => Register::CX,
			32 => Register::ECX,
			64 => Register::RCX,
			_ => unreachable!(),
		};
		if instr_bitness != 0 && instr_bitness != self.bitness {
			if self.bitness == 16 {
				flags |= InstrOpInfoFlags::OP_SIZE16;
			} else if self.bitness == 32 {
				flags |= InstrOpInfoFlags::OP_SIZE32;
			} else {
				flags |= InstrOpInfoFlags::OP_SIZE64;
			}
		}
		if expected_reg != self.register {
			if self.register == Register::CX {
				flags |= InstrOpInfoFlags::ADDR_SIZE16;
			} else if self.register == Register::ECX {
				flags |= InstrOpInfoFlags::ADDR_SIZE32;
			} else {
				flags |= InstrOpInfoFlags::ADDR_SIZE64;
			}
		}
		let mnemonic = if self.cc_index == u32::MAX { &self.mnemonics[0] } else { get_mnemonic_cc(options, self.cc_index, &self.mnemonics) };
		InstrOpInfo::new(mnemonic, instruction, flags)
	}
}

#[allow(non_camel_case_types)]
pub(super) struct SimpleInstrInfo_movabs {
	mnemonic: FormatterString,
}

impl SimpleInstrInfo_movabs {
	pub(super) fn new(mnemonic: String) -> Self {
		Self { mnemonic: FormatterString::new(mnemonic) }
	}
}

impl InstrInfo for SimpleInstrInfo_movabs {
	fn op_info<'a>(&'a self, _options: &FormatterOptions, instruction: &Instruction) -> InstrOpInfo<'a> {
		let mut flags = InstrOpInfoFlags::NONE;
		let mut instr_bitness = get_bitness(instruction.code_size());
		let mem_size = match instruction.memory_displ_size() {
			2 => 16,
			4 => 32,
			_ => 64,
		};
		if instr_bitness == 0 {
			instr_bitness = mem_size;
		}
		if instr_bitness != mem_size {
			if mem_size == 16 {
				flags |= InstrOpInfoFlags::ADDR_SIZE16;
			} else if mem_size == 32 {
				flags |= InstrOpInfoFlags::ADDR_SIZE32;
			} else {
				flags |= InstrOpInfoFlags::ADDR_SIZE64;
			}
		}
		InstrOpInfo::new(&self.mnemonic, instruction, flags)
	}
}

#[allow(non_camel_case_types)]
pub(super) struct SimpleInstrInfo_opmask_op {
	mnemonic: FormatterString,
}

impl SimpleInstrInfo_opmask_op {
	pub(super) fn new(mnemonic: String) -> Self {
		Self { mnemonic: FormatterString::new(mnemonic) }
	}
}

impl InstrInfo for SimpleInstrInfo_opmask_op {
	fn op_info<'a>(&'a self, _options: &FormatterOptions, instruction: &Instruction) -> InstrOpInfo<'a> {
		debug_assert!(instruction.op_count() <= 2);
		let mut info = InstrOpInfo::new(&self.mnemonic, instruction, InstrOpInfoFlags::NONE);
		let kreg = instruction.op_mask();
		if kreg != Register::None {
			info.op_count += 1;
			info.op_kinds[2] = info.op_kinds[1];
			info.op_registers[2] = info.op_registers[1];
			info.op_indexes[2] = 1;
			info.op_kinds[1] = InstrOpKind::Register;
			const_assert_eq!(InstrOpInfo::TEST_REGISTER_BITS, 8);
			info.op_registers[1] = kreg as u8;
			info.op_indexes[1] = InstrInfoConstants::OP_ACCESS_READ;
			info.flags |= InstrOpInfoFlags::IGNORE_OP_MASK as u16;
		}
		info
	}
}

#[allow(non_camel_case_types)]
pub(super) struct SimpleInstrInfo_bnd {
	mnemonic: FormatterString,
	flags: u32,
}

impl SimpleInstrInfo_bnd {
	pub(super) fn new(mnemonic: String, flags: u32) -> Self {
		Self { mnemonic: FormatterString::new(mnemonic), flags }
	}
}

impl InstrInfo for SimpleInstrInfo_bnd {
	fn op_info<'a>(&'a self, _options: &FormatterOptions, instruction: &Instruction) -> InstrOpInfo<'a> {
		let mut flags = self.flags;
		if instruction.has_repne_prefix() {
			flags |= InstrOpInfoFlags::BND_PREFIX;
		}
		InstrOpInfo::new(&self.mnemonic, instruction, flags)
	}
}

#[allow(non_camel_case_types)]
pub(super) struct SimpleInstrInfo_ST_STi {
	mnemonic: FormatterString,
	pseudo_op: bool,
}

impl SimpleInstrInfo_ST_STi {
	pub(super) fn new(mnemonic: String, pseudo_op: bool) -> Self {
		Self { mnemonic: FormatterString::new(mnemonic), pseudo_op }
	}
}

impl InstrInfo for SimpleInstrInfo_ST_STi {
	fn op_info<'a>(&'a self, options: &FormatterOptions, instruction: &Instruction) -> InstrOpInfo<'a> {
		const FLAGS: u32 = 0;
		let mut info;
		if self.pseudo_op && options.use_pseudo_ops() && (instruction.op0_register() == Register::ST1 || instruction.op1_register() == Register::ST1)
		{
			info = InstrOpInfo::default(&self.mnemonic);
		} else {
			info = InstrOpInfo::new(&self.mnemonic, instruction, FLAGS);
			debug_assert_eq!(info.op_registers[0], Register::ST0 as u8);
			const_assert_eq!(InstrOpInfo::TEST_REGISTER_BITS, 8);
			info.op_registers[0] = Registers::REGISTER_ST as u8;
		}
		info
	}
}

#[allow(non_camel_case_types)]
pub(super) struct SimpleInstrInfo_STi_ST {
	mnemonic: FormatterString,
	pseudo_op: bool,
}

impl SimpleInstrInfo_STi_ST {
	pub(super) fn new(mnemonic: String, pseudo_op: bool) -> Self {
		Self { mnemonic: FormatterString::new(mnemonic), pseudo_op }
	}
}

impl InstrInfo for SimpleInstrInfo_STi_ST {
	fn op_info<'a>(&'a self, options: &FormatterOptions, instruction: &Instruction) -> InstrOpInfo<'a> {
		const FLAGS: u32 = 0;
		let mut info;
		if self.pseudo_op && options.use_pseudo_ops() && (instruction.op0_register() == Register::ST1 || instruction.op1_register() == Register::ST1)
		{
			info = InstrOpInfo::default(&self.mnemonic);
		} else {
			info = InstrOpInfo::new(&self.mnemonic, instruction, FLAGS);
			debug_assert_eq!(info.op_registers[1], Register::ST0 as u8);
			const_assert_eq!(InstrOpInfo::TEST_REGISTER_BITS, 8);
			info.op_registers[1] = Registers::REGISTER_ST as u8;
		}
		info
	}
}

#[allow(non_camel_case_types)]
pub(super) struct SimpleInstrInfo_pops {
	mnemonic: FormatterString,
	pseudo_ops: &'static [FormatterString],
}

impl SimpleInstrInfo_pops {
	pub(super) fn new(mnemonic: String, pseudo_ops: &'static [FormatterString]) -> Self {
		Self { mnemonic: FormatterString::new(mnemonic), pseudo_ops }
	}

	fn remove_last_op(info: &mut InstrOpInfo<'_>) {
		match info.op_count {
			4 => info.op_indexes[3] = OP_ACCESS_INVALID,
			3 => info.op_indexes[2] = OP_ACCESS_INVALID,
			_ => unreachable!(),
		}
		info.op_count -= 1;
	}
}

impl InstrInfo for SimpleInstrInfo_pops {
	fn op_info<'a>(&'a self, options: &FormatterOptions, instruction: &Instruction) -> InstrOpInfo<'a> {
		let mut info = InstrOpInfo::new(&self.mnemonic, instruction, InstrOpInfoFlags::NONE);
		let imm = instruction.immediate8() as usize;
		if options.use_pseudo_ops() && imm < self.pseudo_ops.len() {
			info.mnemonic = &self.pseudo_ops[imm];
			SimpleInstrInfo_pops::remove_last_op(&mut info);
		}
		info
	}
}

#[allow(non_camel_case_types)]
pub(super) struct SimpleInstrInfo_pclmulqdq {
	mnemonic: FormatterString,
	pseudo_ops: &'static [FormatterString],
}

impl SimpleInstrInfo_pclmulqdq {
	pub(super) fn new(mnemonic: String, pseudo_ops: &'static [FormatterString]) -> Self {
		Self { mnemonic: FormatterString::new(mnemonic), pseudo_ops }
	}
}

impl InstrInfo for SimpleInstrInfo_pclmulqdq {
	fn op_info<'a>(&'a self, options: &FormatterOptions, instruction: &Instruction) -> InstrOpInfo<'a> {
		let mut info = InstrOpInfo::new(&self.mnemonic, instruction, InstrOpInfoFlags::NONE);
		if options.use_pseudo_ops() {
			let index: isize = match instruction.immediate8() {
				0 => 0,
				1 => 1,
				0x10 => 2,
				0x11 => 3,
				_ => -1,
			};
			if index >= 0 {
				info.mnemonic = &self.pseudo_ops[index as usize];
				SimpleInstrInfo_pops::remove_last_op(&mut info);
			}
		}
		info
	}
}

#[allow(non_camel_case_types)]
pub(super) struct SimpleInstrInfo_imul {
	mnemonic: FormatterString,
}

impl SimpleInstrInfo_imul {
	pub(super) fn new(mnemonic: String) -> Self {
		Self { mnemonic: FormatterString::new(mnemonic) }
	}
}

impl InstrInfo for SimpleInstrInfo_imul {
	fn op_info<'a>(&'a self, options: &FormatterOptions, instruction: &Instruction) -> InstrOpInfo<'a> {
		let mut info = InstrOpInfo::new(&self.mnemonic, instruction, InstrOpInfoFlags::NONE);
		debug_assert_eq!(info.op_count, 3);
		if options.use_pseudo_ops()
			&& info.op_kinds[0] == InstrOpKind::Register
			&& info.op_kinds[1] == InstrOpKind::Register
			&& info.op_registers[0] == info.op_registers[1]
		{
			info.op_count -= 1;
			info.op_indexes[0] = InstrInfoConstants::OP_ACCESS_READ_WRITE;
			info.op_kinds[1] = info.op_kinds[2];
			info.op_indexes[1] = 2;
			info.op_indexes[2] = OP_ACCESS_INVALID;
		}
		info
	}
}

#[allow(non_camel_case_types)]
pub(super) struct SimpleInstrInfo_Reg16 {
	mnemonic: FormatterString,
}

impl SimpleInstrInfo_Reg16 {
	pub(super) fn new(mnemonic: String) -> Self {
		Self { mnemonic: FormatterString::new(mnemonic) }
	}
}

impl InstrInfo for SimpleInstrInfo_Reg16 {
	fn op_info<'a>(&'a self, _options: &FormatterOptions, instruction: &Instruction) -> InstrOpInfo<'a> {
		const FLAGS: u32 = InstrOpInfoFlags::NONE;
		let mut info = InstrOpInfo::new(&self.mnemonic, instruction, FLAGS);
		if Register::EAX as u8 <= info.op_registers[0] && info.op_registers[0] <= Register::R15 as u8 {
			const_assert_eq!(InstrOpInfo::TEST_REGISTER_BITS, 8);
			info.op_registers[0] = (info.op_registers[0].wrapping_sub(Register::EAX as u8) & 0xF).wrapping_add(Register::AX as u8);
		}
		if Register::EAX as u8 <= info.op_registers[1] && info.op_registers[1] <= Register::R15 as u8 {
			const_assert_eq!(InstrOpInfo::TEST_REGISTER_BITS, 8);
			info.op_registers[1] = (info.op_registers[1].wrapping_sub(Register::EAX as u8) & 0xF).wrapping_add(Register::AX as u8);
		}
		if Register::EAX as u8 <= info.op_registers[2] && info.op_registers[2] <= Register::R15 as u8 {
			const_assert_eq!(InstrOpInfo::TEST_REGISTER_BITS, 8);
			info.op_registers[2] = (info.op_registers[2].wrapping_sub(Register::EAX as u8) & 0xF).wrapping_add(Register::AX as u8);
		}
		info
	}
}

#[allow(non_camel_case_types)]
pub(super) struct SimpleInstrInfo_Reg32 {
	mnemonic: FormatterString,
}

impl SimpleInstrInfo_Reg32 {
	pub(super) fn new(mnemonic: String) -> Self {
		Self { mnemonic: FormatterString::new(mnemonic) }
	}
}

impl InstrInfo for SimpleInstrInfo_Reg32 {
	fn op_info<'a>(&'a self, _options: &FormatterOptions, instruction: &Instruction) -> InstrOpInfo<'a> {
		const FLAGS: u32 = InstrOpInfoFlags::NONE;
		let mut info = InstrOpInfo::new(&self.mnemonic, instruction, FLAGS);
		if Register::RAX as u8 <= info.op_registers[0] && info.op_registers[0] <= Register::R15 as u8 {
			const_assert_eq!(InstrOpInfo::TEST_REGISTER_BITS, 8);
			info.op_registers[0] = info.op_registers[0].wrapping_sub(Register::RAX as u8).wrapping_add(Register::EAX as u8);
		}
		if Register::RAX as u8 <= info.op_registers[1] && info.op_registers[1] <= Register::R15 as u8 {
			const_assert_eq!(InstrOpInfo::TEST_REGISTER_BITS, 8);
			info.op_registers[1] = info.op_registers[1].wrapping_sub(Register::RAX as u8).wrapping_add(Register::EAX as u8);
		}
		if Register::RAX as u8 <= info.op_registers[2] && info.op_registers[2] <= Register::R15 as u8 {
			const_assert_eq!(InstrOpInfo::TEST_REGISTER_BITS, 8);
			info.op_registers[2] = info.op_registers[2].wrapping_sub(Register::RAX as u8).wrapping_add(Register::EAX as u8);
		}
		info
	}
}

#[allow(non_camel_case_types)]
pub(super) struct SimpleInstrInfo_reg {
	mnemonic: FormatterString,
	register: Register,
}

impl SimpleInstrInfo_reg {
	pub(super) fn new(mnemonic: String, register: Register) -> Self {
		Self { mnemonic: FormatterString::new(mnemonic), register }
	}
}

impl InstrInfo for SimpleInstrInfo_reg {
	fn op_info<'a>(&'a self, _options: &FormatterOptions, instruction: &Instruction) -> InstrOpInfo<'a> {
		let mut info = InstrOpInfo::new(&self.mnemonic, instruction, InstrOpInfoFlags::NONE);
		debug_assert_eq!(instruction.op_count(), 0);
		info.op_count = 1;
		info.op_kinds[0] = InstrOpKind::Register;
		const_assert_eq!(InstrOpInfo::TEST_REGISTER_BITS, 8);
		info.op_registers[0] = self.register as u8;
		if instruction.code() == Code::Skinit {
			info.op_indexes[0] = InstrInfoConstants::OP_ACCESS_READ_WRITE;
		} else {
			info.op_indexes[0] = InstrInfoConstants::OP_ACCESS_READ;
		}
		info
	}
}

#[allow(non_camel_case_types)]
pub(super) struct SimpleInstrInfo_invlpga {
	mnemonic: FormatterString,
	bitness: u32,
}

impl SimpleInstrInfo_invlpga {
	pub(super) fn new(bitness: u32, mnemonic: String) -> Self {
		Self { mnemonic: FormatterString::new(mnemonic), bitness }
	}
}

impl InstrInfo for SimpleInstrInfo_invlpga {
	fn op_info<'a>(&'a self, _options: &FormatterOptions, _instruction: &Instruction) -> InstrOpInfo<'a> {
		let mut info = InstrOpInfo::default(&self.mnemonic);
		info.op_count = 2;
		info.op_kinds[0] = InstrOpKind::Register;
		info.op_kinds[1] = InstrOpKind::Register;
		const_assert_eq!(InstrOpInfo::TEST_REGISTER_BITS, 8);
		info.op_registers[1] = Register::ECX as u8;
		info.op_indexes[0] = InstrInfoConstants::OP_ACCESS_READ;
		info.op_indexes[1] = InstrInfoConstants::OP_ACCESS_READ;

		match self.bitness {
			16 => {
				const_assert_eq!(InstrOpInfo::TEST_REGISTER_BITS, 8);
				info.op_registers[0] = Register::AX as u8;
			}

			32 => {
				const_assert_eq!(InstrOpInfo::TEST_REGISTER_BITS, 8);
				info.op_registers[0] = Register::EAX as u8;
			}

			64 => {
				const_assert_eq!(InstrOpInfo::TEST_REGISTER_BITS, 8);
				info.op_registers[0] = Register::RAX as u8;
			}

			_ => unreachable!(),
		}
		info
	}
}

#[allow(non_camel_case_types)]
pub(super) struct SimpleInstrInfo_DeclareData {
	mnemonic: FormatterString,
	op_kind: InstrOpKind,
}

impl SimpleInstrInfo_DeclareData {
	pub(super) fn new(code: Code, mnemonic: String) -> Self {
		Self {
			mnemonic: FormatterString::new(mnemonic),
			op_kind: match code {
				Code::DeclareByte => InstrOpKind::DeclareByte,
				Code::DeclareWord => InstrOpKind::DeclareWord,
				Code::DeclareDword => InstrOpKind::DeclareDword,
				Code::DeclareQword => InstrOpKind::DeclareQword,
				_ => unreachable!(),
			},
		}
	}
}

impl InstrInfo for SimpleInstrInfo_DeclareData {
	fn op_info<'a>(&'a self, _options: &FormatterOptions, instruction: &Instruction) -> InstrOpInfo<'a> {
		let mut info = InstrOpInfo::new(&self.mnemonic, instruction, InstrOpInfoFlags::MNEMONIC_IS_DIRECTIVE);
		info.op_count = instruction.declare_data_len() as u8;
		info.op_kinds[0] = self.op_kind;
		info.op_kinds[1] = self.op_kind;
		info.op_kinds[2] = self.op_kind;
		info.op_kinds[3] = self.op_kind;
		info.op_kinds[4] = self.op_kind;
		info.op_indexes[0] = InstrInfoConstants::OP_ACCESS_READ;
		info.op_indexes[1] = InstrInfoConstants::OP_ACCESS_READ;
		info.op_indexes[2] = InstrInfoConstants::OP_ACCESS_READ;
		info.op_indexes[3] = InstrInfoConstants::OP_ACCESS_READ;
		info.op_indexes[4] = InstrInfoConstants::OP_ACCESS_READ;
		info
	}
}

#[allow(non_camel_case_types)]
pub(super) struct SimpleInstrInfo_bcst {
	mnemonic: FormatterString,
	flags_no_broadcast: u32,
}

impl SimpleInstrInfo_bcst {
	pub(super) fn new(mnemonic: String, flags_no_broadcast: u32) -> Self {
		Self { mnemonic: FormatterString::new(mnemonic), flags_no_broadcast }
	}
}

impl InstrInfo for SimpleInstrInfo_bcst {
	fn op_info<'a>(&'a self, _options: &FormatterOptions, instruction: &Instruction) -> InstrOpInfo<'a> {
		let bcst_to = (&*MEM_SIZE_TBL)[instruction.memory_size() as usize].bcst_to;
		let flags = if !bcst_to.is_default() { InstrOpInfoFlags::NONE } else { self.flags_no_broadcast };
		InstrOpInfo::new(&self.mnemonic, instruction, flags)
	}
}
