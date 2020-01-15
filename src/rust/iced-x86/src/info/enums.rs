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

use super::super::*;
use core::fmt;

// GENERATOR-BEGIN: OpAccesses
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
pub(crate) static OP_ACCESS_1: [OpAccess; 7] = [
	OpAccess::None,
	OpAccess::CondRead,
	OpAccess::NoMemAccess,
	OpAccess::Read,
	OpAccess::Read,
	OpAccess::ReadWrite,
	OpAccess::Write,
];
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
pub(crate) static OP_ACCESS_2: [OpAccess; 3] = [
	OpAccess::None,
	OpAccess::Read,
	OpAccess::ReadWrite,
];
// GENERATOR-END: OpAccesses

// GENERATOR-BEGIN: InstrInfoConstants
// ⚠️This was generated by GENERATOR!🦹‍♂️
pub(crate) struct InstrInfoConstants;
impl InstrInfoConstants {
	pub(crate) const OP_INFO0_COUNT: usize = 10;
	pub(crate) const OP_INFO1_COUNT: usize = 7;
	pub(crate) const OP_INFO2_COUNT: usize = 3;
	pub(crate) const OP_INFO3_COUNT: usize = 2;
	pub(crate) const OP_INFO4_COUNT: usize = 2;
	pub(crate) const RFLAGS_INFO_COUNT: usize = 54;
	pub(crate) const DEFAULT_USED_REGISTER_COLL_CAPACITY: usize = 10;
	pub(crate) const DEFAULT_USED_MEMORY_COLL_CAPACITY: usize = 8;
}
// GENERATOR-END: InstrInfoConstants

// GENERATOR-BEGIN: InfoFlags1
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[cfg(feature = "instr_info")]
pub(crate) struct InfoFlags1;
#[cfg(feature = "instr_info")]
impl InfoFlags1 {
	pub(crate) const OP_INFO0_SHIFT: u32 = 0x0000_0000;
	pub(crate) const OP_INFO0_MASK: u32 = 0x0000_000F;
	pub(crate) const OP_INFO1_SHIFT: u32 = 0x0000_0004;
	pub(crate) const OP_INFO1_MASK: u32 = 0x0000_0007;
	pub(crate) const OP_INFO2_SHIFT: u32 = 0x0000_0007;
	pub(crate) const OP_INFO2_MASK: u32 = 0x0000_0003;
	pub(crate) const OP_INFO3_SHIFT: u32 = 0x0000_0009;
	pub(crate) const OP_INFO3_MASK: u32 = 0x0000_0001;
	pub(crate) const OP_INFO4_SHIFT: u32 = 0x0000_000A;
	pub(crate) const OP_INFO4_MASK: u32 = 0x0000_0001;
	pub(crate) const RFLAGS_INFO_SHIFT: u32 = 0x0000_000E;
	pub(crate) const RFLAGS_INFO_MASK: u32 = 0x0000_003F;
	pub(crate) const CODE_INFO_SHIFT: u32 = 0x0000_0014;
	pub(crate) const CODE_INFO_MASK: u32 = 0x0000_007F;
	pub(crate) const SAVE_RESTORE: u32 = 0x0800_0000;
	pub(crate) const STACK_INSTRUCTION: u32 = 0x1000_0000;
	pub(crate) const PROTECTED_MODE: u32 = 0x2000_0000;
	pub(crate) const PRIVILEGED: u32 = 0x4000_0000;
	pub(crate) const NO_SEGMENT_READ: u32 = 0x8000_0000;
}
// GENERATOR-END: InfoFlags1

// GENERATOR-BEGIN: InfoFlags2
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[cfg(feature = "instr_info")]
pub(crate) struct InfoFlags2;
#[cfg(feature = "instr_info")]
impl InfoFlags2 {
	pub(crate) const ENCODING_SHIFT: u32 = 0x0000_0000;
	pub(crate) const ENCODING_MASK: u32 = 0x0000_0007;
	pub(crate) const AVX2_CHECK: u32 = 0x0004_0000;
	pub(crate) const OP_MASK_REG_READ_WRITE: u32 = 0x0008_0000;
	pub(crate) const FLOW_CONTROL_SHIFT: u32 = 0x0000_0014;
	pub(crate) const FLOW_CONTROL_MASK: u32 = 0x0000_000F;
	pub(crate) const CPUID_FEATURE_INTERNAL_SHIFT: u32 = 0x0000_0018;
	pub(crate) const CPUID_FEATURE_INTERNAL_MASK: u32 = 0x0000_00FF;
}
// GENERATOR-END: InfoFlags2

// GENERATOR-BEGIN: OpInfo0
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg(feature = "instr_info")]
#[allow(non_camel_case_types)]
pub(crate) enum OpInfo0 {
	None,
	CondWrite,
	CondWrite32_ReadWrite64,
	NoMemAccess,
	Read,
	ReadCondWrite,
	ReadWrite,
	Write,
	WriteForce,
	WriteMem_ReadWriteReg,
}
#[cfg(feature = "instr_info")]
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_OP_INFO0: [&str; 10] = [
	"None",
	"CondWrite",
	"CondWrite32_ReadWrite64",
	"NoMemAccess",
	"Read",
	"ReadCondWrite",
	"ReadWrite",
	"Write",
	"WriteForce",
	"WriteMem_ReadWriteReg",
];
#[cfg(feature = "instr_info")]
impl fmt::Debug for OpInfo0 {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_OP_INFO0[*self as usize])?;
		Ok(())
	}
}
#[cfg(feature = "instr_info")]
impl Default for OpInfo0 {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		OpInfo0::None
	}
}
// GENERATOR-END: OpInfo0

// GENERATOR-BEGIN: OpInfo1
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg(feature = "instr_info")]
#[allow(non_camel_case_types)]
pub(crate) enum OpInfo1 {
	None,
	CondRead,
	NoMemAccess,
	Read,
	ReadP3,
	ReadWrite,
	Write,
}
#[cfg(feature = "instr_info")]
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_OP_INFO1: [&str; 7] = [
	"None",
	"CondRead",
	"NoMemAccess",
	"Read",
	"ReadP3",
	"ReadWrite",
	"Write",
];
#[cfg(feature = "instr_info")]
impl fmt::Debug for OpInfo1 {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_OP_INFO1[*self as usize])?;
		Ok(())
	}
}
#[cfg(feature = "instr_info")]
impl Default for OpInfo1 {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		OpInfo1::None
	}
}
// GENERATOR-END: OpInfo1

// GENERATOR-BEGIN: OpInfo2
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg(feature = "instr_info")]
#[allow(non_camel_case_types)]
pub(crate) enum OpInfo2 {
	None,
	Read,
	ReadWrite,
}
#[cfg(feature = "instr_info")]
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_OP_INFO2: [&str; 3] = [
	"None",
	"Read",
	"ReadWrite",
];
#[cfg(feature = "instr_info")]
impl fmt::Debug for OpInfo2 {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_OP_INFO2[*self as usize])?;
		Ok(())
	}
}
#[cfg(feature = "instr_info")]
impl Default for OpInfo2 {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		OpInfo2::None
	}
}
// GENERATOR-END: OpInfo2

// GENERATOR-BEGIN: OpInfo3
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg(feature = "instr_info")]
#[allow(non_camel_case_types)]
pub(crate) enum OpInfo3 {
	None,
	Read,
}
#[cfg(feature = "instr_info")]
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_OP_INFO3: [&str; 2] = [
	"None",
	"Read",
];
#[cfg(feature = "instr_info")]
impl fmt::Debug for OpInfo3 {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_OP_INFO3[*self as usize])?;
		Ok(())
	}
}
#[cfg(feature = "instr_info")]
impl Default for OpInfo3 {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		OpInfo3::None
	}
}
// GENERATOR-END: OpInfo3

// GENERATOR-BEGIN: OpInfo4
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg(feature = "instr_info")]
#[allow(non_camel_case_types)]
pub(crate) enum OpInfo4 {
	None,
	Read,
}
#[cfg(feature = "instr_info")]
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_OP_INFO4: [&str; 2] = [
	"None",
	"Read",
];
#[cfg(feature = "instr_info")]
impl fmt::Debug for OpInfo4 {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_OP_INFO4[*self as usize])?;
		Ok(())
	}
}
#[cfg(feature = "instr_info")]
impl Default for OpInfo4 {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		OpInfo4::None
	}
}
// GENERATOR-END: OpInfo4

// GENERATOR-BEGIN: CodeInfo
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg(feature = "instr_info")]
#[allow(non_camel_case_types)]
pub(crate) enum CodeInfo {
	None,
	Cdq,
	Cdqe,
	Clzero,
	Cmps,
	Cmpxchg,
	Cmpxchg8b,
	Cpuid,
	Cqo,
	Cwd,
	Cwde,
	Div,
	Encls,
	Enter,
	Ins,
	Invlpga,
	Iret,
	Jrcxz,
	KP1,
	Lahf,
	Lds,
	Leave,
	Llwpcb,
	Loadall386,
	Lods,
	Loop,
	Maskmovq,
	Monitor,
	Montmul,
	Movdir64b,
	Movs,
	Mul,
	Mulx,
	Mwait,
	Mwaitx,
	Outs,
	PcmpXstrY,
	Pconfig,
	Pop_2,
	Pop_2_2,
	Pop_4,
	Pop_4_4,
	Pop_8,
	Pop_8_8,
	Pop_Ev,
	Popa,
	Push_2,
	Push_2_2,
	Push_4,
	Push_4_4,
	Push_8,
	Push_8_8,
	Pusha,
	R_AL_W_AH,
	R_AL_W_AX,
	R_CR0,
	R_EAX_ECX_EDX,
	R_EAX_EDX,
	R_ECX_W_EAX_EDX,
	R_ST0,
	R_ST0_R_ST1,
	R_ST0_RW_ST1,
	R_ST0_ST1,
	R_XMM0,
	Read_Reg8_Op0,
	Read_Reg8_Op1,
	Read_Reg8_Op2,
	Read_Reg16_Op0,
	Read_Reg16_Op1,
	Read_Reg16_Op2,
	RW_AL,
	RW_AX,
	RW_CR0,
	RW_ST0,
	RW_ST0_R_ST1,
	Salc,
	Scas,
	Shift_Ib_MASK1FMOD9,
	Shift_Ib_MASK1FMOD11,
	Shift_Ib_MASK1F,
	Shift_Ib_MASK3F,
	Clear_rflags,
	Clear_reg_regmem,
	Clear_reg_reg_regmem,
	Stos,
	Syscall,
	Umonitor,
	Vmfunc,
	Vmload,
	Vzeroall,
	W_EAX_ECX_EDX,
	W_EAX_EDX,
	W_ST0,
	Xbts,
	Xcrypt,
	Xsha,
	Xstore,
}
#[cfg(feature = "instr_info")]
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_CODE_INFO: [&str; 97] = [
	"None",
	"Cdq",
	"Cdqe",
	"Clzero",
	"Cmps",
	"Cmpxchg",
	"Cmpxchg8b",
	"Cpuid",
	"Cqo",
	"Cwd",
	"Cwde",
	"Div",
	"Encls",
	"Enter",
	"Ins",
	"Invlpga",
	"Iret",
	"Jrcxz",
	"KP1",
	"Lahf",
	"Lds",
	"Leave",
	"Llwpcb",
	"Loadall386",
	"Lods",
	"Loop",
	"Maskmovq",
	"Monitor",
	"Montmul",
	"Movdir64b",
	"Movs",
	"Mul",
	"Mulx",
	"Mwait",
	"Mwaitx",
	"Outs",
	"PcmpXstrY",
	"Pconfig",
	"Pop_2",
	"Pop_2_2",
	"Pop_4",
	"Pop_4_4",
	"Pop_8",
	"Pop_8_8",
	"Pop_Ev",
	"Popa",
	"Push_2",
	"Push_2_2",
	"Push_4",
	"Push_4_4",
	"Push_8",
	"Push_8_8",
	"Pusha",
	"R_AL_W_AH",
	"R_AL_W_AX",
	"R_CR0",
	"R_EAX_ECX_EDX",
	"R_EAX_EDX",
	"R_ECX_W_EAX_EDX",
	"R_ST0",
	"R_ST0_R_ST1",
	"R_ST0_RW_ST1",
	"R_ST0_ST1",
	"R_XMM0",
	"Read_Reg8_Op0",
	"Read_Reg8_Op1",
	"Read_Reg8_Op2",
	"Read_Reg16_Op0",
	"Read_Reg16_Op1",
	"Read_Reg16_Op2",
	"RW_AL",
	"RW_AX",
	"RW_CR0",
	"RW_ST0",
	"RW_ST0_R_ST1",
	"Salc",
	"Scas",
	"Shift_Ib_MASK1FMOD9",
	"Shift_Ib_MASK1FMOD11",
	"Shift_Ib_MASK1F",
	"Shift_Ib_MASK3F",
	"Clear_rflags",
	"Clear_reg_regmem",
	"Clear_reg_reg_regmem",
	"Stos",
	"Syscall",
	"Umonitor",
	"Vmfunc",
	"Vmload",
	"Vzeroall",
	"W_EAX_ECX_EDX",
	"W_EAX_EDX",
	"W_ST0",
	"Xbts",
	"Xcrypt",
	"Xsha",
	"Xstore",
];
#[cfg(feature = "instr_info")]
impl fmt::Debug for CodeInfo {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_CODE_INFO[*self as usize])?;
		Ok(())
	}
}
#[cfg(feature = "instr_info")]
impl Default for CodeInfo {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		CodeInfo::None
	}
}
// GENERATOR-END: CodeInfo

// GENERATOR-BEGIN: RflagsInfo
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg(feature = "instr_info")]
#[allow(non_camel_case_types)]
pub(crate) enum RflagsInfo {
	None,
	C_AC,
	C_c,
	C_cos_S_pz_U_a,
	C_d,
	C_i,
	R_a_W_ac_U_opsz,
	R_ac_W_acpsz_U_o,
	R_acopszid,
	R_acopszidAC,
	R_acpsz,
	R_c,
	R_c_W_acopsz,
	R_c_W_c,
	R_c_W_co,
	R_cz,
	R_d,
	R_d_W_acopsz,
	R_o,
	R_o_W_o,
	R_os,
	R_osz,
	R_p,
	R_s,
	R_z,
	S_AC,
	S_c,
	S_d,
	S_i,
	U_acopsz,
	W_acopsz,
	W_acopszid,
	W_acopszidAC,
	W_acpsz,
	W_aopsz,
	W_c,
	W_c_C_aopsz,
	W_c_U_aops,
	W_co,
	W_co_U_apsz,
	W_copsz_U_a,
	W_cosz_C_ap,
	W_cpz_C_aos,
	W_cs_C_oz_U_ap,
	W_csz_C_o_U_ap,
	W_cz_C_aops,
	W_cz_U_aops,
	W_psz_C_co_U_a,
	W_psz_U_aco,
	W_sz_C_co_U_ap,
	W_z,
	W_z_C_acops,
	W_z_C_co_U_aps,
	W_z_U_acops,
}
#[cfg(feature = "instr_info")]
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_RFLAGS_INFO: [&str; 54] = [
	"None",
	"C_AC",
	"C_c",
	"C_cos_S_pz_U_a",
	"C_d",
	"C_i",
	"R_a_W_ac_U_opsz",
	"R_ac_W_acpsz_U_o",
	"R_acopszid",
	"R_acopszidAC",
	"R_acpsz",
	"R_c",
	"R_c_W_acopsz",
	"R_c_W_c",
	"R_c_W_co",
	"R_cz",
	"R_d",
	"R_d_W_acopsz",
	"R_o",
	"R_o_W_o",
	"R_os",
	"R_osz",
	"R_p",
	"R_s",
	"R_z",
	"S_AC",
	"S_c",
	"S_d",
	"S_i",
	"U_acopsz",
	"W_acopsz",
	"W_acopszid",
	"W_acopszidAC",
	"W_acpsz",
	"W_aopsz",
	"W_c",
	"W_c_C_aopsz",
	"W_c_U_aops",
	"W_co",
	"W_co_U_apsz",
	"W_copsz_U_a",
	"W_cosz_C_ap",
	"W_cpz_C_aos",
	"W_cs_C_oz_U_ap",
	"W_csz_C_o_U_ap",
	"W_cz_C_aops",
	"W_cz_U_aops",
	"W_psz_C_co_U_a",
	"W_psz_U_aco",
	"W_sz_C_co_U_ap",
	"W_z",
	"W_z_C_acops",
	"W_z_C_co_U_aps",
	"W_z_U_acops",
];
#[cfg(feature = "instr_info")]
impl fmt::Debug for RflagsInfo {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_RFLAGS_INFO[*self as usize])?;
		Ok(())
	}
}
#[cfg(feature = "instr_info")]
impl Default for RflagsInfo {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		RflagsInfo::None
	}
}
// GENERATOR-END: RflagsInfo

// GENERATOR-BEGIN: CpuidFeatureInternal
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg(feature = "instr_info")]
#[allow(non_camel_case_types)]
pub(crate) enum CpuidFeatureInternal {
	ADX,
	AES,
	AES_and_AVX,
	AVX,
	AVX2,
	AVX512BW,
	AVX512CD,
	AVX512DQ,
	AVX512ER,
	AVX512F,
	AVX512F_and_AVX512_VP2INTERSECT,
	AVX512F_and_GFNI,
	AVX512F_and_VAES,
	AVX512F_and_VPCLMULQDQ,
	AVX512PF,
	AVX512VL_and_AVX512BW,
	AVX512VL_and_AVX512CD,
	AVX512VL_and_AVX512DQ,
	AVX512VL_and_AVX512F,
	AVX512VL_and_AVX512_BF16,
	AVX512VL_and_AVX512_BITALG,
	AVX512VL_and_AVX512_IFMA,
	AVX512VL_and_AVX512_VBMI,
	AVX512VL_and_AVX512_VBMI2,
	AVX512VL_and_AVX512_VNNI,
	AVX512VL_and_AVX512_VP2INTERSECT,
	AVX512VL_and_AVX512_VPOPCNTDQ,
	AVX512VL_and_GFNI,
	AVX512VL_and_VAES,
	AVX512VL_and_VPCLMULQDQ,
	AVX512_4FMAPS,
	AVX512_4VNNIW,
	AVX512_BITALG,
	AVX512_IFMA,
	AVX512_VBMI,
	AVX512_VBMI2,
	AVX512_VNNI,
	AVX512_VPOPCNTDQ,
	AVX_and_GFNI,
	BMI1,
	BMI2,
	CET_IBT,
	CET_SS,
	CL1INVMB,
	CLDEMOTE,
	CLFLUSHOPT,
	CLFSH,
	CLWB,
	CLZERO,
	CMOV,
	CMPXCHG16B,
	CPUID,
	CX8,
	D3NOW,
	D3NOWEXT,
	ENCLV,
	ENQCMD,
	F16C,
	FMA,
	FMA4,
	FPU,
	FPU287,
	FPU287XL_ONLY,
	FPU387,
	FPU387SL_ONLY,
	FPU_and_CMOV,
	FPU_and_SSE3,
	FSGSBASE,
	FXSR,
	GEODE,
	GFNI,
	HLE_or_RTM,
	IA64,
	INTEL186,
	INTEL286,
	INTEL286_ONLY,
	INTEL386,
	INTEL386_486_ONLY,
	INTEL386_A0_ONLY,
	INTEL386_ONLY,
	INTEL486,
	INTEL486_A_ONLY,
	INTEL8086,
	INTEL8086_ONLY,
	INVEPT,
	INVPCID,
	INVVPID,
	LWP,
	LZCNT,
	MCOMMIT,
	MMX,
	MONITOR,
	MONITORX,
	MOVBE,
	MOVDIR64B,
	MOVDIRI,
	MPX,
	MSR,
	MULTIBYTENOP,
	PADLOCK_ACE,
	PADLOCK_PHE,
	PADLOCK_PMM,
	PADLOCK_RNG,
	PAUSE,
	PCLMULQDQ,
	PCLMULQDQ_and_AVX,
	PCOMMIT,
	PCONFIG,
	PKU,
	POPCNT,
	PREFETCHW,
	PREFETCHWT1,
	PTWRITE,
	RDPID,
	RDPMC,
	RDPRU,
	RDRAND,
	RDSEED,
	RDTSCP,
	RTM,
	SEP,
	SGX1,
	SHA,
	SKINIT_or_SVML,
	SMAP,
	SMX,
	SSE,
	SSE2,
	SSE3,
	SSE4A,
	SSE4_1,
	SSE4_2,
	SSSE3,
	SVM,
	SYSCALL,
	TBM,
	TSC,
	VAES,
	VMX,
	VPCLMULQDQ,
	WAITPKG,
	WBNOINVD,
	X64,
	XOP,
	XSAVE,
	XSAVEC,
	XSAVEOPT,
	XSAVES,
}
#[cfg(feature = "instr_info")]
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_CPUID_FEATURE_INTERNAL: [&str; 148] = [
	"ADX",
	"AES",
	"AES_and_AVX",
	"AVX",
	"AVX2",
	"AVX512BW",
	"AVX512CD",
	"AVX512DQ",
	"AVX512ER",
	"AVX512F",
	"AVX512F_and_AVX512_VP2INTERSECT",
	"AVX512F_and_GFNI",
	"AVX512F_and_VAES",
	"AVX512F_and_VPCLMULQDQ",
	"AVX512PF",
	"AVX512VL_and_AVX512BW",
	"AVX512VL_and_AVX512CD",
	"AVX512VL_and_AVX512DQ",
	"AVX512VL_and_AVX512F",
	"AVX512VL_and_AVX512_BF16",
	"AVX512VL_and_AVX512_BITALG",
	"AVX512VL_and_AVX512_IFMA",
	"AVX512VL_and_AVX512_VBMI",
	"AVX512VL_and_AVX512_VBMI2",
	"AVX512VL_and_AVX512_VNNI",
	"AVX512VL_and_AVX512_VP2INTERSECT",
	"AVX512VL_and_AVX512_VPOPCNTDQ",
	"AVX512VL_and_GFNI",
	"AVX512VL_and_VAES",
	"AVX512VL_and_VPCLMULQDQ",
	"AVX512_4FMAPS",
	"AVX512_4VNNIW",
	"AVX512_BITALG",
	"AVX512_IFMA",
	"AVX512_VBMI",
	"AVX512_VBMI2",
	"AVX512_VNNI",
	"AVX512_VPOPCNTDQ",
	"AVX_and_GFNI",
	"BMI1",
	"BMI2",
	"CET_IBT",
	"CET_SS",
	"CL1INVMB",
	"CLDEMOTE",
	"CLFLUSHOPT",
	"CLFSH",
	"CLWB",
	"CLZERO",
	"CMOV",
	"CMPXCHG16B",
	"CPUID",
	"CX8",
	"D3NOW",
	"D3NOWEXT",
	"ENCLV",
	"ENQCMD",
	"F16C",
	"FMA",
	"FMA4",
	"FPU",
	"FPU287",
	"FPU287XL_ONLY",
	"FPU387",
	"FPU387SL_ONLY",
	"FPU_and_CMOV",
	"FPU_and_SSE3",
	"FSGSBASE",
	"FXSR",
	"GEODE",
	"GFNI",
	"HLE_or_RTM",
	"IA64",
	"INTEL186",
	"INTEL286",
	"INTEL286_ONLY",
	"INTEL386",
	"INTEL386_486_ONLY",
	"INTEL386_A0_ONLY",
	"INTEL386_ONLY",
	"INTEL486",
	"INTEL486_A_ONLY",
	"INTEL8086",
	"INTEL8086_ONLY",
	"INVEPT",
	"INVPCID",
	"INVVPID",
	"LWP",
	"LZCNT",
	"MCOMMIT",
	"MMX",
	"MONITOR",
	"MONITORX",
	"MOVBE",
	"MOVDIR64B",
	"MOVDIRI",
	"MPX",
	"MSR",
	"MULTIBYTENOP",
	"PADLOCK_ACE",
	"PADLOCK_PHE",
	"PADLOCK_PMM",
	"PADLOCK_RNG",
	"PAUSE",
	"PCLMULQDQ",
	"PCLMULQDQ_and_AVX",
	"PCOMMIT",
	"PCONFIG",
	"PKU",
	"POPCNT",
	"PREFETCHW",
	"PREFETCHWT1",
	"PTWRITE",
	"RDPID",
	"RDPMC",
	"RDPRU",
	"RDRAND",
	"RDSEED",
	"RDTSCP",
	"RTM",
	"SEP",
	"SGX1",
	"SHA",
	"SKINIT_or_SVML",
	"SMAP",
	"SMX",
	"SSE",
	"SSE2",
	"SSE3",
	"SSE4A",
	"SSE4_1",
	"SSE4_2",
	"SSSE3",
	"SVM",
	"SYSCALL",
	"TBM",
	"TSC",
	"VAES",
	"VMX",
	"VPCLMULQDQ",
	"WAITPKG",
	"WBNOINVD",
	"X64",
	"XOP",
	"XSAVE",
	"XSAVEC",
	"XSAVEOPT",
	"XSAVES",
];
#[cfg(feature = "instr_info")]
impl fmt::Debug for CpuidFeatureInternal {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_CPUID_FEATURE_INTERNAL[*self as usize])?;
		Ok(())
	}
}
#[cfg(feature = "instr_info")]
impl Default for CpuidFeatureInternal {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		CpuidFeatureInternal::ADX
	}
}
// GENERATOR-END: CpuidFeatureInternal
