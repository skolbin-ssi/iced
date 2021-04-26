// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

namespace Generator.Constants {
	[TypeGen(TypeGenOrders.NoDeps)]
	sealed class DecoderTestParserConstantsType {
		DecoderTestParserConstantsType(GenTypes genTypes) {
			var type = new ConstantsType(TypeIds.DecoderTestParserConstants, ConstantsTypeFlags.None, null, GetConstants());
			genTypes.Add(type);
		}

		static Constant[] GetConstants() =>
			new Constant[] {
				new Constant(ConstantKind.String, "DecoderError", "err", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "Broadcast", "bcst", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "Xacquire", "xacquire", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "Xrelease", "xrelease", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "Rep", "rep", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "Repe", "repe", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "Repne", "repne", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "Lock", "lock", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "ZeroingMasking", "zmsk", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "SuppressAllExceptions", "sae", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "Vsib32", "vsib32", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "Vsib64", "vsib64", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "RoundToNearest", "rc-rn", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "RoundDown", "rc-rd", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "RoundUp", "rc-ru", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "RoundTowardZero", "rc-rz", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "Op0Kind", "op0", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "Op1Kind", "op1", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "Op2Kind", "op2", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "Op3Kind", "op3", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "Op4Kind", "op4", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "EncodedHexBytes", "enc", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "Code", "code", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "DecoderOptions", "decopt"),
				new Constant(ConstantKind.String, "SegmentPrefix_ES", "es:", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "SegmentPrefix_CS", "cs:", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "SegmentPrefix_SS", "ss:", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "SegmentPrefix_DS", "ds:", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "SegmentPrefix_FS", "fs:", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "SegmentPrefix_GS", "gs:", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "OpMask_k1", "k1", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "OpMask_k2", "k2", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "OpMask_k3", "k3", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "OpMask_k4", "k4", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "OpMask_k5", "k5", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "OpMask_k6", "k6", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "OpMask_k7", "k7", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "ConstantOffsets", "co", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "OpKind_Register", "r", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "OpKind_NearBranch16", "nb16", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "OpKind_NearBranch32", "nb32", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "OpKind_NearBranch64", "nb64", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "OpKind_FarBranch16", "fb16", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "OpKind_FarBranch32", "fb32", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "OpKind_Immediate8", "i8", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "OpKind_Immediate16", "i16", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "OpKind_Immediate32", "i32", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "OpKind_Immediate64", "i64", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "OpKind_Immediate8to16", "i8to16", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "OpKind_Immediate8to32", "i8to32", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "OpKind_Immediate8to64", "i8to64", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "OpKind_Immediate32to64", "i32to64", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "OpKind_Immediate8_2nd", "i8_2nd", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "OpKind_MemorySegSI", "segsi", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "OpKind_MemorySegESI", "segesi", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "OpKind_MemorySegRSI", "segrsi", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "OpKind_MemorySegDI", "segdi", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "OpKind_MemorySegEDI", "segedi", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "OpKind_MemorySegRDI", "segrdi", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "OpKind_MemoryESDI", "esdi", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "OpKind_MemoryESEDI", "esedi", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "OpKind_MemoryESRDI", "esrdi", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "OpKind_Memory", "m", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "DecoderTestOptions_NoEncode", "noencode", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "DecoderTestOptions_NoOptDisableTest", "no_opt_disable_test", ConstantsTypeFlags.None),
				new Constant(ConstantKind.String, "IP", "ip", ConstantsTypeFlags.None),
			};
	}
}
