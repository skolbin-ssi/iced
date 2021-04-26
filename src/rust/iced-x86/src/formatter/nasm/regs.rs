// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

use crate::formatter::regs_tbl::{MAX_STRING_LENGTH, REGS_TBL};
use crate::formatter::FormatterString;
use crate::Register;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::Write;
use lazy_static::lazy_static;

#[allow(dead_code)]
pub(super) struct Registers;
impl Registers {
	#[allow(dead_code)]
	pub(super) const EXTRA_REGISTERS: u32 = 0;
}

lazy_static! {
	pub(super) static ref ALL_REGISTERS: Vec<FormatterString> = {
		let mut v: Vec<_> = (&*REGS_TBL).to_vec();
		let mut s = String::with_capacity(MAX_STRING_LENGTH);
		for i in 0..8usize {
			write!(s, "st{}", i).unwrap();
			v[Register::ST0 as usize + i] = FormatterString::new(s.clone());
			s.clear();
		}
		v
	};
}
