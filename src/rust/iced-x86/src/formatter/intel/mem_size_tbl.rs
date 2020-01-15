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

use super::super::super::iced_constants::IcedConstants;
use super::super::fmt_consts::*;
use super::FormatterString;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use core::mem;

pub(super) struct Info {
	pub(super) bcst_to: &'static FormatterString,
	pub(super) keywords: &'static [&'static FormatterString],
}

#[allow(non_camel_case_types)]
enum MemoryKeywords {
	None,
	byte_ptr,
	dword_ptr,
	fpuenv14_ptr,
	fpuenv28_ptr,
	fpustate108_ptr,
	fpustate94_ptr,
	fword_ptr,
	qword_ptr,
	tbyte_ptr,
	word_ptr,
	xmmword_ptr,
	ymmword_ptr,
	zmmword_ptr,
}

#[allow(non_camel_case_types)]
enum BroadcastToKind {
	None,
	b1to2,
	b1to4,
	b1to8,
	b1to16,
}

const BROADCAST_TO_KIND_SHIFT: u32 = 5;
const MEMORY_KEYWORDS_MASK: u8 = 0x1F;

// GENERATOR-BEGIN: MemorySizes
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static MEM_SIZE_TBL_DATA: [u8; 136] = [
	((MemoryKeywords::None as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::byte_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::word_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::dword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::qword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::qword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::xmmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::ymmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::zmmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::byte_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::word_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::dword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::qword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::xmmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::ymmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::zmmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::dword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::fword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::tbyte_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::word_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::dword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::qword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::dword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::qword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::qword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::xmmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::fword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::fword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::word_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::dword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::qword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::tbyte_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::xmmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::word_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::fpuenv14_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::fpuenv28_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::fpustate94_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::fpustate108_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::None as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::None as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::None as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::None as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::tbyte_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::word_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::word_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::dword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::dword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::dword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::dword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::dword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::qword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::qword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::qword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::qword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::qword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::qword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::qword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::qword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::xmmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::xmmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::xmmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::xmmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::xmmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::xmmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::xmmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::xmmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::xmmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::xmmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::xmmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::xmmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::xmmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::ymmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::ymmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::ymmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::ymmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::ymmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::ymmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::ymmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::ymmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::ymmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::ymmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::ymmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::ymmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::ymmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::ymmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::ymmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::ymmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::zmmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::zmmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::zmmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::zmmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::zmmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::zmmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::zmmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::zmmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::zmmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::zmmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::zmmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::zmmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::zmmword_ptr as u8) | ((BroadcastToKind::None as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::dword_ptr as u8) | ((BroadcastToKind::b1to2 as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::dword_ptr as u8) | ((BroadcastToKind::b1to2 as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::dword_ptr as u8) | ((BroadcastToKind::b1to2 as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::dword_ptr as u8) | ((BroadcastToKind::b1to4 as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::dword_ptr as u8) | ((BroadcastToKind::b1to4 as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::qword_ptr as u8) | ((BroadcastToKind::b1to2 as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::qword_ptr as u8) | ((BroadcastToKind::b1to2 as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::qword_ptr as u8) | ((BroadcastToKind::b1to2 as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::dword_ptr as u8) | ((BroadcastToKind::b1to4 as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::qword_ptr as u8) | ((BroadcastToKind::b1to2 as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::dword_ptr as u8) | ((BroadcastToKind::b1to8 as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::dword_ptr as u8) | ((BroadcastToKind::b1to8 as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::qword_ptr as u8) | ((BroadcastToKind::b1to4 as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::qword_ptr as u8) | ((BroadcastToKind::b1to4 as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::qword_ptr as u8) | ((BroadcastToKind::b1to4 as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::dword_ptr as u8) | ((BroadcastToKind::b1to8 as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::qword_ptr as u8) | ((BroadcastToKind::b1to4 as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::dword_ptr as u8) | ((BroadcastToKind::b1to16 as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::dword_ptr as u8) | ((BroadcastToKind::b1to16 as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::qword_ptr as u8) | ((BroadcastToKind::b1to8 as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::qword_ptr as u8) | ((BroadcastToKind::b1to8 as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::qword_ptr as u8) | ((BroadcastToKind::b1to8 as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::dword_ptr as u8) | ((BroadcastToKind::b1to16 as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::qword_ptr as u8) | ((BroadcastToKind::b1to8 as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::dword_ptr as u8) | ((BroadcastToKind::b1to4 as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::dword_ptr as u8) | ((BroadcastToKind::b1to8 as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::dword_ptr as u8) | ((BroadcastToKind::b1to16 as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::qword_ptr as u8) | ((BroadcastToKind::b1to2 as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::qword_ptr as u8) | ((BroadcastToKind::b1to4 as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::qword_ptr as u8) | ((BroadcastToKind::b1to8 as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::qword_ptr as u8) | ((BroadcastToKind::b1to2 as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::qword_ptr as u8) | ((BroadcastToKind::b1to4 as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::qword_ptr as u8) | ((BroadcastToKind::b1to8 as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::dword_ptr as u8) | ((BroadcastToKind::b1to4 as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::dword_ptr as u8) | ((BroadcastToKind::b1to8 as u8) << BROADCAST_TO_KIND_SHIFT)),
	((MemoryKeywords::dword_ptr as u8) | ((BroadcastToKind::b1to16 as u8) << BROADCAST_TO_KIND_SHIFT)),
];
// GENERATOR-END: MemorySizes

lazy_static! {
	pub(super) static ref MEM_SIZE_TBL: Vec<Info> = {
		let mut v = Vec::with_capacity(IcedConstants::NUMBER_OF_MEMORY_SIZES);
		let c = &*FORMATTER_CONSTANTS;
		let ac = &*ARRAY_CONSTS;
		for &d in MEM_SIZE_TBL_DATA.iter() {
			let mem_keywords: MemoryKeywords = unsafe { mem::transmute(d & MEMORY_KEYWORDS_MASK) };
			let keywords: &'static [&'static FormatterString] = match mem_keywords {
				MemoryKeywords::None => &ac.nothing,
				MemoryKeywords::byte_ptr => &ac.byte_ptr,
				MemoryKeywords::dword_ptr => &ac.dword_ptr,
				MemoryKeywords::fpuenv14_ptr => &ac.fpuenv14_ptr,
				MemoryKeywords::fpuenv28_ptr => &ac.fpuenv28_ptr,
				MemoryKeywords::fpustate108_ptr => &ac.fpustate108_ptr,
				MemoryKeywords::fpustate94_ptr => &ac.fpustate94_ptr,
				MemoryKeywords::fword_ptr => &ac.fword_ptr,
				MemoryKeywords::qword_ptr => &ac.qword_ptr,
				MemoryKeywords::tbyte_ptr => &ac.tbyte_ptr,
				MemoryKeywords::word_ptr => &ac.word_ptr,
				MemoryKeywords::xmmword_ptr => &ac.xmmword_ptr,
				MemoryKeywords::ymmword_ptr => &ac.ymmword_ptr,
				MemoryKeywords::zmmword_ptr => &ac.zmmword_ptr,
			};
			let bcst_kind: BroadcastToKind = unsafe { mem::transmute(d >> BROADCAST_TO_KIND_SHIFT) };
			let bcst_to = match bcst_kind {
				BroadcastToKind::None => &c.empty,
				BroadcastToKind::b1to2 => &c.b1to2,
				BroadcastToKind::b1to4 => &c.b1to4,
				BroadcastToKind::b1to8 => &c.b1to8,
				BroadcastToKind::b1to16 => &c.b1to16,
			};
			v.push(Info { keywords, bcst_to });
		}
		v
	};
}
