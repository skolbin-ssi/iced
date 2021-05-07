// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

// These funcs should be in Instruction but since racer (used by RLS) shows
// all functions even if they're private, they've been moved here.
//	https://github.com/racer-rust/racer/issues/165
// If this 5 year old issue is ever fixed, move these funcs back and remove
// pub(crate) from Instruction's fields.

use crate::iced_constants::IcedConstants;
#[cfg(feature = "encoder")]
use crate::iced_error::IcedError;
use crate::*;
#[cfg(feature = "encoder")]
use core::{i16, i32, i8, u8};
use core::{u16, u32};
#[cfg(any(feature = "encoder", feature = "fast_fmt"))]
use static_assertions::const_assert_eq;

#[cfg(feature = "decoder")]
#[inline]
pub(crate) fn internal_set_code_size(this: &mut Instruction, new_value: CodeSize) {
	this.op_kind_flags |= (new_value as u32) << OpKindFlags::CODE_SIZE_SHIFT;
}

#[cfg(any(feature = "decoder", feature = "encoder"))]
#[inline]
pub(crate) fn internal_set_code(this: &mut Instruction, new_value: Code) {
	this.code_flags |= new_value as u32;
}

#[cfg(feature = "decoder")]
#[inline]
pub(crate) fn internal_set_code_u32(this: &mut Instruction, new_value: u32) {
	this.code_flags |= new_value;
}

#[cfg(feature = "decoder")]
#[inline]
pub(crate) fn internal_set_len(this: &mut Instruction, new_value: u32) {
	this.code_flags |= new_value << CodeFlags::INSTR_LENGTH_SHIFT;
}

#[cfg(feature = "encoder")]
#[must_use]
#[inline]
pub(crate) fn internal_has_repe_prefix_has_xrelease_prefix(this: &Instruction) -> bool {
	(this.code_flags & (CodeFlags::REPE_PREFIX | CodeFlags::XRELEASE_PREFIX)) != 0
}

#[cfg(feature = "encoder")]
#[must_use]
#[inline]
pub(crate) fn internal_has_repne_prefix_has_xacquire_prefix(this: &Instruction) -> bool {
	(this.code_flags & (CodeFlags::REPNE_PREFIX | CodeFlags::XACQUIRE_PREFIX)) != 0
}

#[cfg(feature = "instr_info")]
#[must_use]
#[inline]
pub(crate) fn internal_has_repe_or_repne_prefix(this: &Instruction) -> bool {
	(this.code_flags & (CodeFlags::REPE_PREFIX | CodeFlags::REPNE_PREFIX)) != 0
}

#[cfg(feature = "decoder")]
#[inline]
pub(crate) fn internal_set_has_xacquire_prefix(this: &mut Instruction) {
	this.code_flags |= CodeFlags::XACQUIRE_PREFIX
}

#[cfg(feature = "decoder")]
#[inline]
pub(crate) fn internal_set_has_xrelease_prefix(this: &mut Instruction) {
	this.code_flags |= CodeFlags::XRELEASE_PREFIX
}

#[cfg(any(feature = "decoder", feature = "encoder"))]
#[inline]
pub(crate) fn internal_set_has_repe_prefix(this: &mut Instruction) {
	this.code_flags = (this.code_flags & !CodeFlags::REPNE_PREFIX) | CodeFlags::REPE_PREFIX
}

#[cfg(any(feature = "gas", feature = "intel", feature = "masm", feature = "nasm", feature = "fast_fmt"))]
#[inline]
pub(crate) fn internal_has_any_of_xacquire_xrelease_lock_rep_repne_prefix(this: &Instruction) -> u32 {
	this.code_flags
		& (CodeFlags::XACQUIRE_PREFIX | CodeFlags::XRELEASE_PREFIX | CodeFlags::LOCK_PREFIX | CodeFlags::REPE_PREFIX | CodeFlags::REPNE_PREFIX)
}

#[cfg(any(feature = "gas", feature = "intel", feature = "masm", feature = "nasm", feature = "fast_fmt"))]
#[inline]
pub(crate) fn internal_has_op_mask_or_zeroing_masking(this: &Instruction) -> bool {
	(this.code_flags & ((CodeFlags::OP_MASK_MASK << CodeFlags::OP_MASK_SHIFT) | CodeFlags::ZEROING_MASKING)) != 0
}

#[cfg(feature = "decoder")]
#[inline]
pub(crate) fn internal_clear_has_repe_repne_prefix(this: &mut Instruction) {
	this.code_flags &= !(CodeFlags::REPE_PREFIX | CodeFlags::REPNE_PREFIX)
}

#[cfg(feature = "decoder")]
#[inline]
pub(crate) fn internal_clear_has_repe_prefix(this: &mut Instruction) {
	this.code_flags &= !CodeFlags::REPE_PREFIX
}

#[cfg(any(feature = "decoder", feature = "encoder"))]
#[inline]
pub(crate) fn internal_set_has_repne_prefix(this: &mut Instruction) {
	this.code_flags = (this.code_flags & !CodeFlags::REPE_PREFIX) | CodeFlags::REPNE_PREFIX
}

#[cfg(feature = "decoder")]
#[inline]
pub(crate) fn internal_clear_has_repne_prefix(this: &mut Instruction) {
	this.code_flags &= !CodeFlags::REPNE_PREFIX
}

#[cfg(feature = "decoder")]
#[inline]
pub(crate) fn internal_set_has_lock_prefix(this: &mut Instruction) {
	this.code_flags |= CodeFlags::LOCK_PREFIX
}

#[cfg(feature = "decoder")]
#[inline]
pub(crate) fn internal_clear_has_lock_prefix(this: &mut Instruction) {
	this.code_flags &= !CodeFlags::LOCK_PREFIX
}

#[cfg(any(feature = "decoder", feature = "encoder"))]
#[inline]
pub(crate) fn internal_set_op0_kind(this: &mut Instruction, new_value: OpKind) {
	this.op_kind_flags |= new_value as u32;
}

#[cfg(feature = "instr_info")]
#[must_use]
#[inline]
pub(crate) fn internal_op0_is_not_reg_or_op1_is_not_reg(this: &Instruction) -> bool {
	(this.op_kind_flags & (OpKindFlags::OP_KIND_MASK | (OpKindFlags::OP_KIND_MASK << OpKindFlags::OP1_KIND_SHIFT))) != 0
}

#[cfg(any(feature = "decoder", feature = "encoder"))]
#[inline]
pub(crate) fn internal_set_op1_kind(this: &mut Instruction, new_value: OpKind) {
	this.op_kind_flags |= (new_value as u32) << OpKindFlags::OP1_KIND_SHIFT;
}

#[cfg(any(feature = "decoder", feature = "encoder"))]
#[inline]
pub(crate) fn internal_set_op2_kind(this: &mut Instruction, new_value: OpKind) {
	this.op_kind_flags |= (new_value as u32) << OpKindFlags::OP2_KIND_SHIFT;
}

#[cfg(any(feature = "decoder", feature = "encoder"))]
#[inline]
pub(crate) fn internal_set_op3_kind(this: &mut Instruction, new_value: OpKind) {
	this.op_kind_flags |= (new_value as u32) << OpKindFlags::OP3_KIND_SHIFT;
}

#[cfg(feature = "decoder")]
#[inline]
pub(crate) fn internal_set_memory_displ_size(this: &mut Instruction, new_value: u32) {
	debug_assert!(new_value <= 4);
	this.memory_flags |= (new_value << MemoryFlags::DISPL_SIZE_SHIFT) as u16;
}

#[cfg(feature = "decoder")]
#[cfg(not(feature = "no_evex"))]
#[inline]
pub(crate) fn internal_set_is_broadcast(this: &mut Instruction) {
	this.memory_flags |= MemoryFlags::BROADCAST as u16;
}

#[must_use]
#[inline]
pub(crate) fn internal_get_memory_index_scale(this: &Instruction) -> u32 {
	(this.memory_flags & (MemoryFlags::SCALE_MASK as u16)) as u32
}

#[cfg(feature = "decoder")]
#[inline]
pub(crate) fn internal_set_memory_index_scale(this: &mut Instruction, new_value: u32) {
	this.memory_flags |= new_value as u16;
}

#[cfg(any(feature = "decoder", feature = "encoder"))]
#[inline]
pub(crate) fn internal_set_immediate8(this: &mut Instruction, new_value: u32) {
	this.immediate = new_value;
}

#[cfg(any(feature = "decoder", feature = "encoder"))]
#[inline]
pub(crate) fn internal_set_immediate8_2nd(this: &mut Instruction, new_value: u32) {
	this.mem_displ = new_value;
}

#[cfg(any(feature = "decoder", feature = "encoder"))]
#[inline]
pub(crate) fn internal_set_immediate16(this: &mut Instruction, new_value: u32) {
	this.immediate = new_value;
}

#[cfg(feature = "decoder")]
#[inline]
pub(crate) fn internal_set_immediate64_lo(this: &mut Instruction, new_value: u32) {
	this.immediate = new_value;
}

#[cfg(feature = "decoder")]
#[inline]
pub(crate) fn internal_set_immediate64_hi(this: &mut Instruction, new_value: u32) {
	this.mem_displ = new_value;
}

#[cfg(any(feature = "decoder", feature = "encoder"))]
#[inline]
pub(crate) fn internal_set_memory_displacement64_lo(this: &mut Instruction, new_value: u32) {
	this.mem_displ = new_value;
}

#[cfg(feature = "decoder")]
#[inline]
pub(crate) fn internal_set_memory_displacement64_hi(this: &mut Instruction, new_value: u32) {
	this.mem_displ_hi = new_value;
}

#[cfg(feature = "decoder")]
#[inline]
pub(crate) fn internal_set_near_branch16(this: &mut Instruction, new_value: u32) {
	this.immediate = new_value;
}

#[cfg(feature = "decoder")]
#[inline]
pub(crate) fn internal_set_far_branch16(this: &mut Instruction, new_value: u32) {
	this.immediate = new_value;
}

#[cfg(feature = "decoder")]
#[inline]
pub(crate) fn internal_set_far_branch_selector(this: &mut Instruction, new_value: u32) {
	this.mem_displ = new_value;
}

#[cfg(any(feature = "decoder", feature = "encoder"))]
#[inline]
pub(crate) fn internal_set_memory_base(this: &mut Instruction, new_value: Register) {
	this.mem_base_reg = new_value as u8;
}

#[cfg(feature = "decoder")]
#[inline]
pub(crate) fn internal_set_memory_base_u32(this: &mut Instruction, new_value: u32) {
	this.mem_base_reg = new_value as u8;
}

#[cfg(any(feature = "decoder", feature = "encoder"))]
#[inline]
pub(crate) fn internal_set_memory_index(this: &mut Instruction, new_value: Register) {
	this.mem_index_reg = new_value as u8;
}

#[cfg(feature = "decoder")]
#[inline]
pub(crate) fn internal_set_memory_index_u32(this: &mut Instruction, new_value: u32) {
	this.mem_index_reg = new_value as u8;
}

#[cfg(feature = "fast_fmt")]
#[inline]
pub(crate) fn internal_segment_prefix_raw(this: &Instruction) -> u32 {
	(((this.memory_flags as u32) >> MemoryFlags::SEGMENT_PREFIX_SHIFT) & MemoryFlags::SEGMENT_PREFIX_MASK).wrapping_sub(1)
}

#[cfg(feature = "fast_fmt")]
#[inline]
pub(crate) fn internal_op_register(this: &Instruction, operand: u32) -> Register {
	use core::mem;

	const_assert_eq!(IcedConstants::MAX_OP_COUNT, 5);
	if let Some(&reg) = this.regs.get(operand as usize) {
		unsafe { mem::transmute(reg) }
	} else {
		debug_assert_eq!(operand, 4);
		this.op4_register()
	}
}

#[cfg(any(feature = "decoder", feature = "encoder"))]
#[inline]
pub(crate) fn internal_set_op0_register(this: &mut Instruction, new_value: Register) {
	this.regs[0] = new_value as u8;
}

#[cfg(feature = "decoder")]
#[inline]
pub(crate) fn internal_set_op0_register_u32(this: &mut Instruction, new_value: u32) {
	this.regs[0] = new_value as u8;
}

#[cfg(any(feature = "decoder", feature = "encoder"))]
#[inline]
pub(crate) fn internal_set_op1_register(this: &mut Instruction, new_value: Register) {
	this.regs[1] = new_value as u8;
}

#[cfg(feature = "decoder")]
#[inline]
pub(crate) fn internal_set_op1_register_u32(this: &mut Instruction, new_value: u32) {
	this.regs[1] = new_value as u8;
}

#[cfg(any(feature = "decoder", feature = "encoder"))]
#[inline]
pub(crate) fn internal_set_op2_register(this: &mut Instruction, new_value: Register) {
	this.regs[2] = new_value as u8;
}

#[cfg(feature = "decoder")]
#[inline]
pub(crate) fn internal_set_op2_register_u32(this: &mut Instruction, new_value: u32) {
	this.regs[2] = new_value as u8;
}

#[allow(dead_code)]
#[cfg(feature = "encoder")]
#[inline]
pub(crate) fn internal_set_op3_register(this: &mut Instruction, new_value: Register) {
	this.regs[3] = new_value as u8;
}

#[cfg(feature = "decoder")]
#[cfg(any(not(feature = "no_vex"), not(feature = "no_xop")))]
#[inline]
pub(crate) fn internal_set_op3_register_u32(this: &mut Instruction, new_value: u32) {
	this.regs[3] = new_value as u8;
}

#[cfg(feature = "encoder")]
#[cfg(not(feature = "no_evex"))]
#[must_use]
#[inline]
pub(crate) fn internal_op_mask(this: &Instruction) -> u32 {
	(this.code_flags >> CodeFlags::OP_MASK_SHIFT) & CodeFlags::OP_MASK_MASK
}

#[cfg(feature = "decoder")]
#[cfg(not(feature = "no_evex"))]
#[inline]
pub(crate) fn internal_set_op_mask(this: &mut Instruction, new_value: u32) {
	this.code_flags |= new_value << CodeFlags::OP_MASK_SHIFT
}

#[cfg(feature = "decoder")]
#[cfg(not(feature = "no_evex"))]
#[inline]
pub(crate) fn internal_set_zeroing_masking(this: &mut Instruction) {
	this.code_flags |= CodeFlags::ZEROING_MASKING;
}

#[cfg(feature = "decoder")]
#[cfg(not(feature = "no_evex"))]
#[inline]
pub(crate) fn internal_set_rounding_control(this: &mut Instruction, new_value: u32) {
	this.code_flags |= new_value << CodeFlags::ROUNDING_CONTROL_SHIFT;
}

#[cfg(any(feature = "intel", feature = "masm", feature = "fast_fmt"))]
#[inline]
pub(crate) fn internal_has_rounding_control_or_sae(this: &Instruction) -> bool {
	(this.code_flags & ((CodeFlags::ROUNDING_CONTROL_MASK << CodeFlags::ROUNDING_CONTROL_SHIFT) | CodeFlags::SUPPRESS_ALL_EXCEPTIONS)) != 0
}

#[cfg(feature = "encoder")]
#[inline]
pub(crate) fn internal_set_declare_data_len(this: &mut Instruction, new_value: u32) {
	this.op_kind_flags |= (new_value - 1) << OpKindFlags::DATA_LENGTH_SHIFT;
}

#[cfg(feature = "decoder")]
#[cfg(not(feature = "no_evex"))]
#[inline]
pub(crate) fn internal_set_suppress_all_exceptions(this: &mut Instruction) {
	this.code_flags |= CodeFlags::SUPPRESS_ALL_EXCEPTIONS;
}

// GENERATOR-BEGIN: RegToAddrSize
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[rustfmt::skip]
static REG_TO_ADDR_SIZE: [u8; IcedConstants::REGISTER_ENUM_COUNT] = [
	0, // None
	0, // AL
	0, // CL
	0, // DL
	0, // BL
	0, // AH
	0, // CH
	0, // DH
	0, // BH
	0, // SPL
	0, // BPL
	0, // SIL
	0, // DIL
	0, // R8L
	0, // R9L
	0, // R10L
	0, // R11L
	0, // R12L
	0, // R13L
	0, // R14L
	0, // R15L
	2, // AX
	2, // CX
	2, // DX
	2, // BX
	2, // SP
	2, // BP
	2, // SI
	2, // DI
	2, // R8W
	2, // R9W
	2, // R10W
	2, // R11W
	2, // R12W
	2, // R13W
	2, // R14W
	2, // R15W
	4, // EAX
	4, // ECX
	4, // EDX
	4, // EBX
	4, // ESP
	4, // EBP
	4, // ESI
	4, // EDI
	4, // R8D
	4, // R9D
	4, // R10D
	4, // R11D
	4, // R12D
	4, // R13D
	4, // R14D
	4, // R15D
	8, // RAX
	8, // RCX
	8, // RDX
	8, // RBX
	8, // RSP
	8, // RBP
	8, // RSI
	8, // RDI
	8, // R8
	8, // R9
	8, // R10
	8, // R11
	8, // R12
	8, // R13
	8, // R14
	8, // R15
	4, // EIP
	8, // RIP
	0, // ES
	0, // CS
	0, // SS
	0, // DS
	0, // FS
	0, // GS
	0, // XMM0
	0, // XMM1
	0, // XMM2
	0, // XMM3
	0, // XMM4
	0, // XMM5
	0, // XMM6
	0, // XMM7
	0, // XMM8
	0, // XMM9
	0, // XMM10
	0, // XMM11
	0, // XMM12
	0, // XMM13
	0, // XMM14
	0, // XMM15
	0, // XMM16
	0, // XMM17
	0, // XMM18
	0, // XMM19
	0, // XMM20
	0, // XMM21
	0, // XMM22
	0, // XMM23
	0, // XMM24
	0, // XMM25
	0, // XMM26
	0, // XMM27
	0, // XMM28
	0, // XMM29
	0, // XMM30
	0, // XMM31
	0, // YMM0
	0, // YMM1
	0, // YMM2
	0, // YMM3
	0, // YMM4
	0, // YMM5
	0, // YMM6
	0, // YMM7
	0, // YMM8
	0, // YMM9
	0, // YMM10
	0, // YMM11
	0, // YMM12
	0, // YMM13
	0, // YMM14
	0, // YMM15
	0, // YMM16
	0, // YMM17
	0, // YMM18
	0, // YMM19
	0, // YMM20
	0, // YMM21
	0, // YMM22
	0, // YMM23
	0, // YMM24
	0, // YMM25
	0, // YMM26
	0, // YMM27
	0, // YMM28
	0, // YMM29
	0, // YMM30
	0, // YMM31
	0, // ZMM0
	0, // ZMM1
	0, // ZMM2
	0, // ZMM3
	0, // ZMM4
	0, // ZMM5
	0, // ZMM6
	0, // ZMM7
	0, // ZMM8
	0, // ZMM9
	0, // ZMM10
	0, // ZMM11
	0, // ZMM12
	0, // ZMM13
	0, // ZMM14
	0, // ZMM15
	0, // ZMM16
	0, // ZMM17
	0, // ZMM18
	0, // ZMM19
	0, // ZMM20
	0, // ZMM21
	0, // ZMM22
	0, // ZMM23
	0, // ZMM24
	0, // ZMM25
	0, // ZMM26
	0, // ZMM27
	0, // ZMM28
	0, // ZMM29
	0, // ZMM30
	0, // ZMM31
	0, // K0
	0, // K1
	0, // K2
	0, // K3
	0, // K4
	0, // K5
	0, // K6
	0, // K7
	0, // BND0
	0, // BND1
	0, // BND2
	0, // BND3
	0, // CR0
	0, // CR1
	0, // CR2
	0, // CR3
	0, // CR4
	0, // CR5
	0, // CR6
	0, // CR7
	0, // CR8
	0, // CR9
	0, // CR10
	0, // CR11
	0, // CR12
	0, // CR13
	0, // CR14
	0, // CR15
	0, // DR0
	0, // DR1
	0, // DR2
	0, // DR3
	0, // DR4
	0, // DR5
	0, // DR6
	0, // DR7
	0, // DR8
	0, // DR9
	0, // DR10
	0, // DR11
	0, // DR12
	0, // DR13
	0, // DR14
	0, // DR15
	0, // ST0
	0, // ST1
	0, // ST2
	0, // ST3
	0, // ST4
	0, // ST5
	0, // ST6
	0, // ST7
	0, // MM0
	0, // MM1
	0, // MM2
	0, // MM3
	0, // MM4
	0, // MM5
	0, // MM6
	0, // MM7
	0, // TR0
	0, // TR1
	0, // TR2
	0, // TR3
	0, // TR4
	0, // TR5
	0, // TR6
	0, // TR7
	0, // TMM0
	0, // TMM1
	0, // TMM2
	0, // TMM3
	0, // TMM4
	0, // TMM5
	0, // TMM6
	0, // TMM7
];
// GENERATOR-END: RegToAddrSize

#[must_use]
pub(crate) fn get_address_size_in_bytes(base_reg: Register, index_reg: Register, displ_size: u32, code_size: CodeSize) -> u32 {
	let size = REG_TO_ADDR_SIZE[base_reg as usize] | REG_TO_ADDR_SIZE[index_reg as usize];
	if size != 0 {
		size as u32
	} else if displ_size >= 2 {
		displ_size
	} else {
		match code_size {
			CodeSize::Code64 => 8,
			CodeSize::Code32 => 4,
			CodeSize::Code16 => 2,
			_ => 8,
		}
	}
}

#[cfg(feature = "encoder")]
pub(crate) fn initialize_signed_immediate(instruction: &mut Instruction, operand: usize, immediate: i64) -> Result<(), IcedError> {
	let op_kind = get_immediate_op_kind(instruction.code(), operand)?;
	instruction.try_set_op_kind(operand as u32, op_kind)?;

	match op_kind {
		OpKind::Immediate8 => {
			// All i8 and all u8 values can be used
			if i8::MIN as i64 <= immediate && immediate <= u8::MAX as i64 {
				internal_set_immediate8(instruction, immediate as u8 as u32);
				return Ok(());
			}
		}

		OpKind::Immediate8_2nd => {
			// All i8 and all u8 values can be used
			if i8::MIN as i64 <= immediate && immediate <= u8::MAX as i64 {
				internal_set_immediate8_2nd(instruction, immediate as u8 as u32);
				return Ok(());
			}
		}

		OpKind::Immediate8to16 => {
			if i8::MIN as i64 <= immediate && immediate <= i8::MAX as i64 {
				internal_set_immediate8(instruction, immediate as u8 as u32);
				return Ok(());
			}
		}

		OpKind::Immediate8to32 => {
			if i8::MIN as i64 <= immediate && immediate <= i8::MAX as i64 {
				internal_set_immediate8(instruction, immediate as u8 as u32);
				return Ok(());
			}
		}

		OpKind::Immediate8to64 => {
			if i8::MIN as i64 <= immediate && immediate <= i8::MAX as i64 {
				internal_set_immediate8(instruction, immediate as u8 as u32);
				return Ok(());
			}
		}

		OpKind::Immediate16 => {
			// All short and all ushort values can be used
			if i16::MIN as i64 <= immediate && immediate <= u16::MAX as i64 {
				internal_set_immediate16(instruction, immediate as u16 as u32);
				return Ok(());
			}
		}

		OpKind::Immediate32 => {
			// All int and all uint values can be used
			if i32::MIN as i64 <= immediate && immediate <= u32::MAX as i64 {
				instruction.set_immediate32(immediate as u32);
				return Ok(());
			}
		}

		OpKind::Immediate32to64 => {
			if i32::MIN as i64 <= immediate && immediate <= i32::MAX as i64 {
				instruction.set_immediate32(immediate as u32);
				return Ok(());
			}
		}

		OpKind::Immediate64 => {
			instruction.set_immediate64(immediate as u64);
			return Ok(());
		}

		_ => return Err(IcedError::new("Not an immediate operand")),
	}

	Err(IcedError::new("Invalid signed immediate"))
}

#[cfg(feature = "encoder")]
pub(crate) fn initialize_unsigned_immediate(instruction: &mut Instruction, operand: usize, immediate: u64) -> Result<(), IcedError> {
	let op_kind = get_immediate_op_kind(instruction.code(), operand)?;
	instruction.try_set_op_kind(operand as u32, op_kind)?;

	match op_kind {
		OpKind::Immediate8 => {
			if immediate <= u8::MAX as u64 {
				internal_set_immediate8(instruction, immediate as u8 as u32);
				return Ok(());
			}
		}

		OpKind::Immediate8_2nd => {
			if immediate <= u8::MAX as u64 {
				internal_set_immediate8_2nd(instruction, immediate as u8 as u32);
				return Ok(());
			}
		}

		OpKind::Immediate8to16 => {
			if immediate <= i8::MAX as u64 || (0xFF80 <= immediate && immediate <= 0xFFFF) {
				internal_set_immediate8(instruction, immediate as u8 as u32);
				return Ok(());
			}
		}

		OpKind::Immediate8to32 => {
			if immediate <= i8::MAX as u64 || (0xFFFF_FF80 <= immediate && immediate <= 0xFFFF_FFFF) {
				internal_set_immediate8(instruction, immediate as u8 as u32);
				return Ok(());
			}
		}

		OpKind::Immediate8to64 => {
			// Allow 00..7F and FFFF_FFFF_FFFF_FF80..FFFF_FFFF_FFFF_FFFF
			if immediate.wrapping_add(0x80) <= u8::MAX as u64 {
				internal_set_immediate8(instruction, immediate as u8 as u32);
				return Ok(());
			}
		}

		OpKind::Immediate16 => {
			if immediate <= u16::MAX as u64 {
				internal_set_immediate16(instruction, immediate as u16 as u32);
				return Ok(());
			}
		}

		OpKind::Immediate32 => {
			if immediate <= u32::MAX as u64 {
				instruction.set_immediate32(immediate as u32);
				return Ok(());
			}
		}

		OpKind::Immediate32to64 => {
			// Allow 0..7FFF_FFFF and FFFF_FFFF_8000_0000..FFFF_FFFF_FFFF_FFFF
			if immediate.wrapping_add(0x8000_0000) <= u32::MAX as u64 {
				instruction.set_immediate32(immediate as u32);
				return Ok(());
			}
		}

		OpKind::Immediate64 => {
			instruction.set_immediate64(immediate);
			return Ok(());
		}

		_ => return Err(IcedError::new("Not an immediate operand")),
	}

	Err(IcedError::new("Invalid unsigned immediate"))
}

#[cfg(feature = "encoder")]
fn get_immediate_op_kind(code: Code, operand: usize) -> Result<OpKind, IcedError> {
	let operands = &crate::encoder::handlers_table::HANDLERS_TABLE[code as usize].operands;
	if let Some(op) = operands.get(operand) {
		match op.immediate_op_kind() {
			Some(op_kind) => {
				if op_kind == OpKind::Immediate8 && operand > 0 && operand + 1 == operands.len() {
					if let Some(op_prev) = operands.get(operand.wrapping_sub(1)) {
						match op_prev.immediate_op_kind() {
							Some(OpKind::Immediate8) | Some(OpKind::Immediate16) => Ok(OpKind::Immediate8_2nd),
							_ => Ok(op_kind),
						}
					} else {
						Ok(op_kind)
					}
				} else {
					Ok(op_kind)
				}
			}
			None => {
				if cfg!(debug_assertions) {
					Err(IcedError::with_string(format!("{:?}'s op{} isn't an immediate operand", code, operand)))
				} else {
					Err(IcedError::with_string(format!("Code value {}'s op{} isn't an immediate operand", code as u32, operand)))
				}
			}
		}
	} else {
		if cfg!(debug_assertions) {
			Err(IcedError::with_string(format!("{:?} doesn't have at least {} operands", code, operand + 1)))
		} else {
			Err(IcedError::with_string(format!("Code value {} doesn't have at least {} operands", code as u32, operand + 1)))
		}
	}
}

#[cfg(feature = "encoder")]
pub(crate) fn get_near_branch_op_kind(code: Code, operand: usize) -> Result<OpKind, IcedError> {
	let operands = &crate::encoder::handlers_table::HANDLERS_TABLE[code as usize].operands;
	if let Some(op) = operands.get(operand) {
		match op.near_branch_op_kind() {
			Some(op_kind) => Ok(op_kind),
			None => {
				if cfg!(debug_assertions) {
					Err(IcedError::with_string(format!("{:?}'s op{} isn't a near branch operand", code, operand)))
				} else {
					Err(IcedError::with_string(format!("Code value {}'s op{} isn't a near branch operand", code as u32, operand)))
				}
			}
		}
	} else {
		if cfg!(debug_assertions) {
			Err(IcedError::with_string(format!("{:?} doesn't have at least {} operands", code, operand + 1)))
		} else {
			Err(IcedError::with_string(format!("Code value {} doesn't have at least {} operands", code as u32, operand + 1)))
		}
	}
}

#[cfg(feature = "encoder")]
pub(crate) fn get_far_branch_op_kind(code: Code, operand: usize) -> Result<OpKind, IcedError> {
	let operands = &crate::encoder::handlers_table::HANDLERS_TABLE[code as usize].operands;
	if let Some(op) = operands.get(operand) {
		match op.far_branch_op_kind() {
			Some(op_kind) => Ok(op_kind),
			None => {
				if cfg!(debug_assertions) {
					Err(IcedError::with_string(format!("{:?}'s op{} isn't a far branch operand", code, operand)))
				} else {
					Err(IcedError::with_string(format!("Code value {}'s op{} isn't a far branch operand", code as u32, operand)))
				}
			}
		}
	} else {
		if cfg!(debug_assertions) {
			Err(IcedError::with_string(format!("{:?} doesn't have at least {} operands", code, operand + 1)))
		} else {
			Err(IcedError::with_string(format!("Code value {} doesn't have at least {} operands", code as u32, operand + 1)))
		}
	}
}

#[cfg(feature = "encoder")]
pub(crate) fn with_string_reg_segrsi(
	code: Code, address_size: u32, register: Register, segment_prefix: Register, rep_prefix: RepPrefixKind,
) -> Result<Instruction, IcedError> {
	let mut instruction = Instruction::default();
	internal_set_code(&mut instruction, code);

	match rep_prefix {
		RepPrefixKind::None => {}
		RepPrefixKind::Repe => internal_set_has_repe_prefix(&mut instruction),
		RepPrefixKind::Repne => internal_set_has_repne_prefix(&mut instruction),
	}

	const_assert_eq!(OpKind::Register as u32, 0);
	//internal_set_op0_kind(&mut instruction, OpKind::Register);
	internal_set_op0_register(&mut instruction, register);

	match address_size {
		64 => internal_set_op1_kind(&mut instruction, OpKind::MemorySegRSI),
		32 => internal_set_op1_kind(&mut instruction, OpKind::MemorySegESI),
		16 => internal_set_op1_kind(&mut instruction, OpKind::MemorySegSI),
		_ => return Err(IcedError::new("Invalid address size")),
	}

	instruction.set_segment_prefix(segment_prefix);

	debug_assert_eq!(instruction.op_count(), 2);
	Ok(instruction)
}

#[cfg(feature = "encoder")]
pub(crate) fn with_string_reg_esrdi(code: Code, address_size: u32, register: Register, rep_prefix: RepPrefixKind) -> Result<Instruction, IcedError> {
	let mut instruction = Instruction::default();
	internal_set_code(&mut instruction, code);

	match rep_prefix {
		RepPrefixKind::None => {}
		RepPrefixKind::Repe => internal_set_has_repe_prefix(&mut instruction),
		RepPrefixKind::Repne => internal_set_has_repne_prefix(&mut instruction),
	}

	const_assert_eq!(OpKind::Register as u32, 0);
	//internal_set_op0_kind(&mut instruction, OpKind::Register);
	internal_set_op0_register(&mut instruction, register);

	match address_size {
		64 => internal_set_op1_kind(&mut instruction, OpKind::MemoryESRDI),
		32 => internal_set_op1_kind(&mut instruction, OpKind::MemoryESEDI),
		16 => internal_set_op1_kind(&mut instruction, OpKind::MemoryESDI),
		_ => return Err(IcedError::new("Invalid address size")),
	}

	debug_assert_eq!(instruction.op_count(), 2);
	Ok(instruction)
}

#[cfg(feature = "encoder")]
pub(crate) fn with_string_esrdi_reg(code: Code, address_size: u32, register: Register, rep_prefix: RepPrefixKind) -> Result<Instruction, IcedError> {
	let mut instruction = Instruction::default();
	internal_set_code(&mut instruction, code);

	match rep_prefix {
		RepPrefixKind::None => {}
		RepPrefixKind::Repe => internal_set_has_repe_prefix(&mut instruction),
		RepPrefixKind::Repne => internal_set_has_repne_prefix(&mut instruction),
	}

	match address_size {
		64 => internal_set_op0_kind(&mut instruction, OpKind::MemoryESRDI),
		32 => internal_set_op0_kind(&mut instruction, OpKind::MemoryESEDI),
		16 => internal_set_op0_kind(&mut instruction, OpKind::MemoryESDI),
		_ => return Err(IcedError::new("Invalid address size")),
	}

	const_assert_eq!(OpKind::Register as u32, 0);
	//internal_set_op1_kind(&mut instruction, OpKind::Register);
	internal_set_op1_register(&mut instruction, register);

	debug_assert_eq!(instruction.op_count(), 2);
	Ok(instruction)
}

#[cfg(feature = "encoder")]
pub(crate) fn with_string_segrsi_esrdi(
	code: Code, address_size: u32, segment_prefix: Register, rep_prefix: RepPrefixKind,
) -> Result<Instruction, IcedError> {
	let mut instruction = Instruction::default();
	internal_set_code(&mut instruction, code);

	match rep_prefix {
		RepPrefixKind::None => {}
		RepPrefixKind::Repe => internal_set_has_repe_prefix(&mut instruction),
		RepPrefixKind::Repne => internal_set_has_repne_prefix(&mut instruction),
	}

	match address_size {
		64 => {
			internal_set_op0_kind(&mut instruction, OpKind::MemorySegRSI);
			internal_set_op1_kind(&mut instruction, OpKind::MemoryESRDI);
		}
		32 => {
			internal_set_op0_kind(&mut instruction, OpKind::MemorySegESI);
			internal_set_op1_kind(&mut instruction, OpKind::MemoryESEDI);
		}
		16 => {
			internal_set_op0_kind(&mut instruction, OpKind::MemorySegSI);
			internal_set_op1_kind(&mut instruction, OpKind::MemoryESDI);
		}
		_ => return Err(IcedError::new("Invalid address size")),
	}

	instruction.set_segment_prefix(segment_prefix);

	debug_assert_eq!(instruction.op_count(), 2);
	Ok(instruction)
}

#[cfg(feature = "encoder")]
pub(crate) fn with_string_esrdi_segrsi(
	code: Code, address_size: u32, segment_prefix: Register, rep_prefix: RepPrefixKind,
) -> Result<Instruction, IcedError> {
	let mut instruction = Instruction::default();
	internal_set_code(&mut instruction, code);

	match rep_prefix {
		RepPrefixKind::None => {}
		RepPrefixKind::Repe => internal_set_has_repe_prefix(&mut instruction),
		RepPrefixKind::Repne => internal_set_has_repne_prefix(&mut instruction),
	}

	match address_size {
		64 => {
			internal_set_op0_kind(&mut instruction, OpKind::MemoryESRDI);
			internal_set_op1_kind(&mut instruction, OpKind::MemorySegRSI);
		}
		32 => {
			internal_set_op0_kind(&mut instruction, OpKind::MemoryESEDI);
			internal_set_op1_kind(&mut instruction, OpKind::MemorySegESI);
		}
		16 => {
			internal_set_op0_kind(&mut instruction, OpKind::MemoryESDI);
			internal_set_op1_kind(&mut instruction, OpKind::MemorySegSI);
		}
		_ => return Err(IcedError::new("Invalid address size")),
	}

	instruction.set_segment_prefix(segment_prefix);

	debug_assert_eq!(instruction.op_count(), 2);
	Ok(instruction)
}

#[cfg(feature = "encoder")]
pub(crate) fn with_maskmov(
	code: Code, address_size: u32, register1: Register, register2: Register, segment_prefix: Register,
) -> Result<Instruction, IcedError> {
	let mut instruction = Instruction::default();
	internal_set_code(&mut instruction, code);

	match address_size {
		64 => internal_set_op0_kind(&mut instruction, OpKind::MemorySegRDI),
		32 => internal_set_op0_kind(&mut instruction, OpKind::MemorySegEDI),
		16 => internal_set_op0_kind(&mut instruction, OpKind::MemorySegDI),
		_ => return Err(IcedError::new("Invalid address size")),
	}

	const_assert_eq!(OpKind::Register as u32, 0);
	//internal_set_op1_kind(&mut instruction, OpKind::Register);
	internal_set_op1_register(&mut instruction, register1);

	const_assert_eq!(OpKind::Register as u32, 0);
	//internal_set_op2_kind(&mut instruction, OpKind::Register);
	internal_set_op2_register(&mut instruction, register2);

	instruction.set_segment_prefix(segment_prefix);

	debug_assert_eq!(instruction.op_count(), 3);
	Ok(instruction)
}
