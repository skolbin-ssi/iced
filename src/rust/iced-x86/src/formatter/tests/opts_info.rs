/*
Copyright (C) 2018-2019 de4dot@gmail.com

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use super::super::super::*;
use super::enums::OptionsProps;

pub(super) enum OptionValue {
	Boolean(bool),
	Int32(i32),
	UInt64(u64),
	String(String),
	MemorySizeOptions(MemorySizeOptions),
	NumberBase(NumberBase),
}

impl OptionValue {
	fn to_bool(&self) -> bool {
		if let &OptionValue::Boolean(value) = self {
			value
		} else {
			panic!()
		}
	}

	fn to_i32_as_u32(&self) -> u32 {
		if let &OptionValue::Int32(value) = self {
			if value <= 0 {
				0
			} else {
				value as u32
			}
		} else {
			panic!()
		}
	}

	fn to_u64(&self) -> u64 {
		if let &OptionValue::UInt64(value) = self {
			value
		} else {
			panic!()
		}
	}

	fn to_str(&self) -> String {
		if let &OptionValue::String(ref value) = self {
			value.clone()
		} else {
			panic!()
		}
	}

	fn to_memory_size_options(&self) -> MemorySizeOptions {
		if let &OptionValue::MemorySizeOptions(value) = self {
			value
		} else {
			panic!()
		}
	}

	fn to_number_base(&self) -> NumberBase {
		if let &OptionValue::NumberBase(value) = self {
			value
		} else {
			panic!()
		}
	}
}

pub(super) struct OptionsInstructionInfo {
	pub(super) bitness: u32,
	pub(super) hex_bytes: String,
	pub(super) code: Code,
	pub(super) vec: Vec<(OptionsProps, OptionValue)>,
}

impl OptionsInstructionInfo {
	pub(super) fn initialize_options(&self, options: &mut FormatterOptions) {
		for info in self.vec.iter() {
			match info.0 {
				OptionsProps::AddLeadingZeroToHexNumbers => options.set_add_leading_zero_to_hex_numbers(info.1.to_bool()),
				OptionsProps::AlwaysShowScale => options.set_always_show_scale(info.1.to_bool()),
				OptionsProps::AlwaysShowSegmentRegister => options.set_always_show_segment_register(info.1.to_bool()),
				OptionsProps::BinaryDigitGroupSize => options.set_binary_digit_group_size(info.1.to_i32_as_u32()),
				OptionsProps::BinaryPrefix => options.set_binary_prefix(info.1.to_str()),
				OptionsProps::BinarySuffix => options.set_binary_suffix(info.1.to_str()),
				OptionsProps::BranchLeadingZeroes => options.set_branch_leading_zeroes(info.1.to_bool()),
				OptionsProps::DecimalDigitGroupSize => options.set_decimal_digit_group_size(info.1.to_i32_as_u32()),
				OptionsProps::DecimalPrefix => options.set_decimal_prefix(info.1.to_str()),
				OptionsProps::DecimalSuffix => options.set_decimal_suffix(info.1.to_str()),
				OptionsProps::DigitSeparator => options.set_digit_separator(info.1.to_str()),
				OptionsProps::DisplacementLeadingZeroes => options.set_displacement_leading_zeroes(info.1.to_bool()),
				OptionsProps::FirstOperandCharIndex => options.set_first_operand_char_index(info.1.to_i32_as_u32()),
				OptionsProps::GasNakedRegisters => {
					if cfg!(feature = "gas") {
						options.set_gas_naked_registers(info.1.to_bool())
					} else {
						panic!()
					}
				}
				OptionsProps::GasShowMnemonicSizeSuffix => {
					if cfg!(feature = "gas") {
						options.set_gas_show_mnemonic_size_suffix(info.1.to_bool())
					} else {
						panic!()
					}
				}
				OptionsProps::GasSpaceAfterMemoryOperandComma => {
					if cfg!(feature = "gas") {
						options.set_gas_space_after_memory_operand_comma(info.1.to_bool())
					} else {
						panic!()
					}
				}
				OptionsProps::HexDigitGroupSize => options.set_hex_digit_group_size(info.1.to_i32_as_u32()),
				OptionsProps::HexPrefix => options.set_hex_prefix(info.1.to_str()),
				OptionsProps::HexSuffix => options.set_hex_suffix(info.1.to_str()),
				OptionsProps::LeadingZeroes => options.set_leading_zeroes(info.1.to_bool()),
				OptionsProps::MasmAddDsPrefix32 => {
					if cfg!(feature = "masm") {
						options.set_masm_add_ds_prefix32(info.1.to_bool())
					} else {
						panic!()
					}
				}
				OptionsProps::MemorySizeOptions => options.set_memory_size_options(info.1.to_memory_size_options()),
				OptionsProps::NasmShowSignExtendedImmediateSize => {
					if cfg!(feature = "nasm") {
						options.set_nasm_show_sign_extended_immediate_size(info.1.to_bool())
					} else {
						panic!()
					}
				}
				OptionsProps::NumberBase => options.set_number_base(info.1.to_number_base()),
				OptionsProps::OctalDigitGroupSize => options.set_octal_digit_group_size(info.1.to_i32_as_u32()),
				OptionsProps::OctalPrefix => options.set_octal_prefix(info.1.to_str()),
				OptionsProps::OctalSuffix => options.set_octal_suffix(info.1.to_str()),
				OptionsProps::PreferST0 => options.set_prefer_st0(info.1.to_bool()),
				OptionsProps::RipRelativeAddresses => options.set_rip_relative_addresses(info.1.to_bool()),
				OptionsProps::ScaleBeforeIndex => options.set_scale_before_index(info.1.to_bool()),
				OptionsProps::ShowBranchSize => options.set_show_branch_size(info.1.to_bool()),
				OptionsProps::ShowZeroDisplacements => options.set_show_zero_displacements(info.1.to_bool()),
				OptionsProps::SignedImmediateOperands => options.set_signed_immediate_operands(info.1.to_bool()),
				OptionsProps::SignedMemoryDisplacements => options.set_signed_memory_displacements(info.1.to_bool()),
				OptionsProps::SmallHexNumbersInDecimal => options.set_small_hex_numbers_in_decimal(info.1.to_bool()),
				OptionsProps::SpaceAfterMemoryBracket => options.set_space_after_memory_bracket(info.1.to_bool()),
				OptionsProps::SpaceAfterOperandSeparator => options.set_space_after_operand_separator(info.1.to_bool()),
				OptionsProps::SpaceBetweenMemoryAddOperators => options.set_space_between_memory_add_operators(info.1.to_bool()),
				OptionsProps::SpaceBetweenMemoryMulOperators => options.set_space_between_memory_mul_operators(info.1.to_bool()),
				OptionsProps::TabSize => options.set_tab_size(info.1.to_i32_as_u32()),
				OptionsProps::UpperCaseAll => options.set_upper_case_all(info.1.to_bool()),
				OptionsProps::UpperCaseDecorators => options.set_upper_case_decorators(info.1.to_bool()),
				OptionsProps::UpperCaseHex => options.set_upper_case_hex(info.1.to_bool()),
				OptionsProps::UpperCaseKeywords => options.set_upper_case_keywords(info.1.to_bool()),
				OptionsProps::UpperCaseMnemonics => options.set_upper_case_mnemonics(info.1.to_bool()),
				OptionsProps::UpperCasePrefixes => options.set_upper_case_prefixes(info.1.to_bool()),
				OptionsProps::UpperCaseRegisters => options.set_upper_case_registers(info.1.to_bool()),
				OptionsProps::UsePseudoOps => options.set_use_pseudo_ops(info.1.to_bool()),
				_ => {}
			}
		}
	}

	pub(super) fn initialize_decoder(&self, decoder: &mut Decoder) {
		for info in self.vec.iter() {
			if let OptionsProps::IP = info.0 {
				decoder.set_ip(info.1.to_u64());
			}
		}
	}
}
