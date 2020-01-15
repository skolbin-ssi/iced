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
	pub(super) keyword: &'static FormatterString,
	pub(super) bcst_to: &'static FormatterString,
	pub(super) size: u32,
}

enum Size {
	S0,
	S1,
	S2,
	S4,
	S6,
	S8,
	S10,
	S14,
	S16,
	S28,
	S32,
	S64,
	S94,
	S108,
	S512,
}

#[allow(non_camel_case_types)]
enum MemoryKeywords {
	None,
	byte,
	dword,
	far,
	fpuenv14,
	fpuenv28,
	fpustate108,
	fpustate94,
	oword,
	qword,
	tword,
	word,
	yword,
	zword,
}

#[allow(non_camel_case_types)]
enum BroadcastToKind {
	b1to2,
	b1to4,
	b1to8,
	b1to16,
}

const SIZE_KIND_SHIFT: u32 = 4;
const MEMORY_KEYWORDS_MASK: u8 = 0xF;

// GENERATOR-BEGIN: BcstTo
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static BCST_TO_DATA: [BroadcastToKind; 36] = [
	BroadcastToKind::b1to2,
	BroadcastToKind::b1to2,
	BroadcastToKind::b1to2,
	BroadcastToKind::b1to4,
	BroadcastToKind::b1to4,
	BroadcastToKind::b1to2,
	BroadcastToKind::b1to2,
	BroadcastToKind::b1to2,
	BroadcastToKind::b1to4,
	BroadcastToKind::b1to2,
	BroadcastToKind::b1to8,
	BroadcastToKind::b1to8,
	BroadcastToKind::b1to4,
	BroadcastToKind::b1to4,
	BroadcastToKind::b1to4,
	BroadcastToKind::b1to8,
	BroadcastToKind::b1to4,
	BroadcastToKind::b1to16,
	BroadcastToKind::b1to16,
	BroadcastToKind::b1to8,
	BroadcastToKind::b1to8,
	BroadcastToKind::b1to8,
	BroadcastToKind::b1to16,
	BroadcastToKind::b1to8,
	BroadcastToKind::b1to4,
	BroadcastToKind::b1to8,
	BroadcastToKind::b1to16,
	BroadcastToKind::b1to2,
	BroadcastToKind::b1to4,
	BroadcastToKind::b1to8,
	BroadcastToKind::b1to2,
	BroadcastToKind::b1to4,
	BroadcastToKind::b1to8,
	BroadcastToKind::b1to4,
	BroadcastToKind::b1to8,
	BroadcastToKind::b1to16,
];
// GENERATOR-END: BcstTo

// GENERATOR-BEGIN: MemorySizes
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static MEM_SIZE_TBL_DATA: [u8; 136] = [
	((MemoryKeywords::None as u8) | ((Size::S0 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::byte as u8) | ((Size::S1 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::word as u8) | ((Size::S2 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::dword as u8) | ((Size::S4 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::qword as u8) | ((Size::S8 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::qword as u8) | ((Size::S8 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::oword as u8) | ((Size::S16 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::yword as u8) | ((Size::S32 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::zword as u8) | ((Size::S64 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::byte as u8) | ((Size::S1 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::word as u8) | ((Size::S2 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::dword as u8) | ((Size::S4 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::qword as u8) | ((Size::S8 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::oword as u8) | ((Size::S16 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::yword as u8) | ((Size::S32 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::zword as u8) | ((Size::S64 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::far as u8) | ((Size::S4 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::far as u8) | ((Size::S6 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::far as u8) | ((Size::S10 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::word as u8) | ((Size::S2 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::dword as u8) | ((Size::S4 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::qword as u8) | ((Size::S8 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::None as u8) | ((Size::S4 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::None as u8) | ((Size::S8 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::qword as u8) | ((Size::S8 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::oword as u8) | ((Size::S16 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::None as u8) | ((Size::S6 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::None as u8) | ((Size::S10 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::word as u8) | ((Size::S2 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::dword as u8) | ((Size::S4 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::qword as u8) | ((Size::S8 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::tword as u8) | ((Size::S10 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::oword as u8) | ((Size::S16 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::word as u8) | ((Size::S2 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::fpuenv14 as u8) | ((Size::S14 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::fpuenv28 as u8) | ((Size::S28 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::fpustate94 as u8) | ((Size::S94 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::fpustate108 as u8) | ((Size::S108 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::None as u8) | ((Size::S512 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::None as u8) | ((Size::S512 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::None as u8) | ((Size::S0 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::None as u8) | ((Size::S0 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::tword as u8) | ((Size::S10 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::word as u8) | ((Size::S2 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::word as u8) | ((Size::S2 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::dword as u8) | ((Size::S4 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::dword as u8) | ((Size::S4 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::dword as u8) | ((Size::S4 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::dword as u8) | ((Size::S4 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::dword as u8) | ((Size::S4 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::qword as u8) | ((Size::S8 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::qword as u8) | ((Size::S8 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::qword as u8) | ((Size::S8 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::qword as u8) | ((Size::S8 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::qword as u8) | ((Size::S8 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::qword as u8) | ((Size::S8 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::qword as u8) | ((Size::S8 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::qword as u8) | ((Size::S8 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::oword as u8) | ((Size::S16 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::oword as u8) | ((Size::S16 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::oword as u8) | ((Size::S16 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::oword as u8) | ((Size::S16 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::oword as u8) | ((Size::S16 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::oword as u8) | ((Size::S16 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::oword as u8) | ((Size::S16 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::oword as u8) | ((Size::S16 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::oword as u8) | ((Size::S16 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::oword as u8) | ((Size::S16 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::oword as u8) | ((Size::S16 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::oword as u8) | ((Size::S16 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::oword as u8) | ((Size::S16 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::yword as u8) | ((Size::S32 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::yword as u8) | ((Size::S32 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::yword as u8) | ((Size::S32 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::yword as u8) | ((Size::S32 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::yword as u8) | ((Size::S32 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::yword as u8) | ((Size::S32 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::yword as u8) | ((Size::S32 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::yword as u8) | ((Size::S32 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::yword as u8) | ((Size::S32 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::yword as u8) | ((Size::S32 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::yword as u8) | ((Size::S32 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::yword as u8) | ((Size::S32 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::yword as u8) | ((Size::S32 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::yword as u8) | ((Size::S32 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::yword as u8) | ((Size::S32 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::yword as u8) | ((Size::S32 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::zword as u8) | ((Size::S64 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::zword as u8) | ((Size::S64 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::zword as u8) | ((Size::S64 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::zword as u8) | ((Size::S64 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::zword as u8) | ((Size::S64 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::zword as u8) | ((Size::S64 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::zword as u8) | ((Size::S64 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::zword as u8) | ((Size::S64 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::zword as u8) | ((Size::S64 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::zword as u8) | ((Size::S64 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::zword as u8) | ((Size::S64 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::zword as u8) | ((Size::S64 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::zword as u8) | ((Size::S64 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::dword as u8) | ((Size::S4 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::dword as u8) | ((Size::S4 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::dword as u8) | ((Size::S4 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::dword as u8) | ((Size::S4 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::dword as u8) | ((Size::S4 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::qword as u8) | ((Size::S8 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::qword as u8) | ((Size::S8 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::qword as u8) | ((Size::S8 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::dword as u8) | ((Size::S4 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::qword as u8) | ((Size::S8 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::dword as u8) | ((Size::S4 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::dword as u8) | ((Size::S4 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::qword as u8) | ((Size::S8 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::qword as u8) | ((Size::S8 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::qword as u8) | ((Size::S8 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::dword as u8) | ((Size::S4 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::qword as u8) | ((Size::S8 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::dword as u8) | ((Size::S4 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::dword as u8) | ((Size::S4 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::qword as u8) | ((Size::S8 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::qword as u8) | ((Size::S8 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::qword as u8) | ((Size::S8 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::dword as u8) | ((Size::S4 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::qword as u8) | ((Size::S8 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::dword as u8) | ((Size::S4 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::dword as u8) | ((Size::S4 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::dword as u8) | ((Size::S4 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::qword as u8) | ((Size::S8 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::qword as u8) | ((Size::S8 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::qword as u8) | ((Size::S8 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::qword as u8) | ((Size::S8 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::qword as u8) | ((Size::S8 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::qword as u8) | ((Size::S8 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::dword as u8) | ((Size::S4 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::dword as u8) | ((Size::S4 as u8) << SIZE_KIND_SHIFT)),
	((MemoryKeywords::dword as u8) | ((Size::S4 as u8) << SIZE_KIND_SHIFT)),
];
// GENERATOR-END: MemorySizes

#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static SIZES: [u16; 15] = [
	0,
	1,
	2,
	4,
	6,
	8,
	10,
	14,
	16,
	28,
	32,
	64,
	94,
	108,
	512,
];

lazy_static! {
	pub(super) static ref MEM_SIZE_TBL: Vec<Info> = {
		let mut v = Vec::with_capacity(IcedConstants::NUMBER_OF_MEMORY_SIZES);
		let c = &*FORMATTER_CONSTANTS;
		for (i, &d) in MEM_SIZE_TBL_DATA.iter().enumerate() {
			let mem_keywords: MemoryKeywords = unsafe { mem::transmute(d & MEMORY_KEYWORDS_MASK) };
			let keyword = match mem_keywords {
				MemoryKeywords::None => &c.empty,
				MemoryKeywords::byte => &c.byte,
				MemoryKeywords::dword => &c.dword,
				MemoryKeywords::far => &c.far,
				MemoryKeywords::fpuenv14 => &c.fpuenv14,
				MemoryKeywords::fpuenv28 => &c.fpuenv28,
				MemoryKeywords::fpustate108 => &c.fpustate108,
				MemoryKeywords::fpustate94 => &c.fpustate94,
				MemoryKeywords::oword => &c.oword,
				MemoryKeywords::qword => &c.qword,
				MemoryKeywords::tword => &c.tword,
				MemoryKeywords::word => &c.word,
				MemoryKeywords::yword => &c.yword,
				MemoryKeywords::zword => &c.zword,
			};
			let bcst_to = if i < IcedConstants::FIRST_BROADCAST_MEMORY_SIZE as usize {
				&c.empty
			} else {
				match BCST_TO_DATA[i - IcedConstants::FIRST_BROADCAST_MEMORY_SIZE as usize] {
					BroadcastToKind::b1to2 => &c.b1to2,
					BroadcastToKind::b1to4 => &c.b1to4,
					BroadcastToKind::b1to8 => &c.b1to8,
					BroadcastToKind::b1to16 => &c.b1to16,
				}
			};

			let size = SIZES[(d as usize) >> SIZE_KIND_SHIFT] as u32;
			v.push(Info { keyword, bcst_to, size });
		}
		v
	};
}
