// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

namespace Generator.Constants.Encoder {
	static class OpCodeInfoConstants {
		public const string Encoding_Legacy = "legacy";
		public const string Encoding_VEX = "VEX";
		public const string Encoding_EVEX = "EVEX";
		public const string Encoding_XOP = "XOP";
		public const string Encoding_3DNOW = "3DNow!";
		public const string MandatoryPrefix_None = "";
		public const string MandatoryPrefix_NP = "NP";
		public const string MandatoryPrefix_66 = "66";
		public const string MandatoryPrefix_F3 = "F3";
		public const string MandatoryPrefix_F2 = "F2";
		public const string Table_Legacy = "legacy";
		public const string Table_0F = "0F";
		public const string Table_0F38 = "0F38";
		public const string Table_0F3A = "0F3A";
		public const string Table_XOP8 = "X8";
		public const string Table_XOP9 = "X9";
		public const string Table_XOPA = "XA";
	}

	static class OpCodeInfoKeywords {
		public const string NoInstruction = "no-instr";
		public const string Bit16 = "16";
		public const string Bit32 = "32";
		public const string Bit64 = "64";
		public const string Fwait = "fwait";
		public const string OperandSize16 = "o16";
		public const string OperandSize32 = "o32";
		public const string OperandSize64 = "o64";
		public const string AddressSize16 = "a16";
		public const string AddressSize32 = "a32";
		public const string AddressSize64 = "a64";
		public const string LIG = "LIG";
		public const string L0 = "L0";
		public const string L1 = "L1";
		public const string L128 = "L128";
		public const string L256 = "L256";
		public const string L512 = "L512";
		public const string WIG = "WIG";
		public const string WIG32 = "WIG32";
		public const string W0 = "W0";
		public const string W1 = "W1";
		public const string Broadcast = "b";
		public const string RoundingControl = "er";
		public const string SuppressAllExceptions = "sae";
		public const string OpMaskRegister = "k";
		public const string RequireOpMaskRegister = "knz";
		public const string ZeroingMasking = "z";
		public const string Lock = "lock";
		public const string Xacquire = "xacquire";
		public const string Xrelease = "xrelease";
		public const string Rep = "rep";
		public const string Repe = "repe";
		public const string Repne = "repne";
		public const string Bnd = "bnd";
		public const string HintTaken = "ht";
		public const string Notrack = "notrack";
		public const string IgnoresRoundingControl = "ignore-er";
		public const string AmdLockRegBit = "amd-lock-reg-bit";
		public const string DefaultOpSize64 = "do64";
		public const string ForceOpSize64 = "fo64";
		public const string IntelForceOpSize64 = "intel-fo64";
		public const string Cpl0 = "cpl0";
		public const string Cpl1 = "cpl1";
		public const string Cpl2 = "cpl2";
		public const string Cpl3 = "cpl3";
		public const string InputOutput = "io";
		public const string Nop = "nop";
		public const string ReservedNop = "res-nop";
		public const string SerializingIntel = "intel-serialize";
		public const string SerializingAmd = "amd-serialize";
		public const string MayRequireCpl0 = "may-require-cpl0";
		public const string CetTracked = "cet-tracked";
		public const string NonTemporal = "nt";
		public const string FpuNoWait = "nowait";
		public const string IgnoresModBits = "ignores-mod";
		public const string No66 = "no66";
		public const string NFx = "nfx";
		public const string RequiresUniqueRegNums = "unique-reg-nums";
		public const string Privileged = "priv";
		public const string SaveRestore = "save-restore";
		public const string StackInstruction = "stack";
		public const string IgnoresSegment = "ignore-seg";
		public const string OpMaskReadWrite = "krw";
		public const string RealMode = "rm";
		public const string ProtectedMode = "pm";
		public const string Virtual8086Mode = "v86";
		public const string CompatibilityMode = "cm";
		public const string LongMode = "lm";
		public const string UseOutsideSmm = "outside-smm";
		public const string UseInSmm = "in-smm";
		public const string UseOutsideEnclaveSgx = "outside-sgx";
		public const string UseInEnclaveSgx1 = "in-sgx1";
		public const string UseInEnclaveSgx2 = "in-sgx2";
		public const string UseOutsideVmxOp = "outside-vmx-op";
		public const string UseInVmxRootOp = "in-vmx-root-op";
		public const string UseInVmxNonRootOp = "in-vmx-non-root-op";
		public const string UseOutsideSeam = "outside-seam";
		public const string UseInSeam = "in-seam";
		public const string TdxNonRootGenUd = "tdx-non-root-gen-ud";
		public const string TdxNonRootGenVe = "td-non-root-gen-ve";
		public const string TdxNonRootMayGenEx = "tdx-non-root-may-gen-ex";
		public const string IntelVmExit = "intel-vm-exit";
		public const string IntelMayVmExit = "intel-may-vm-exit";
		public const string IntelSmmVmExit = "intel-smm-vm-exit";
		public const string AmdVmExit = "amd-vm-exit";
		public const string AmdMayVmExit = "amd-may-vm-exit";
		public const string TsxAbort = "tsx-abort";
		public const string TsxImplAbort = "tsx-impl-abort";
		public const string TsxMayAbort = "tsx-may-abort";
		public const string IntelDecoder16 = "intel16";
		public const string IntelDecoder32 = "intel32";
		public const string IntelDecoder64 = "intel64";
		public const string AmdDecoder16 = "amd16";
		public const string AmdDecoder32 = "amd32";
		public const string AmdDecoder64 = "amd64";
	}

	static class OpCodeInfoKeywordKeys {
		public const string GroupIndex = "g";
		public const string RmGroupIndex = "rmg";
		public const string OpCodeOperandKind = "op";
		public const string TupleType = "tt";
		public const string DecoderOption = "dec-opt";
	}
}
