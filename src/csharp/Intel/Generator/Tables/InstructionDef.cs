// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

using System;
using Generator.InstructionInfo;
using Generator.Enums;
using Generator.Enums.InstructionInfo;
using Generator.Enums.Encoder;
using System.Diagnostics;
using Generator.Formatters;
using Generator.Enums.Formatter;

namespace Generator.Tables {
	[Flags]
	enum InstructionDefFlags1 : uint {
		None					= 0,
		/// <summary>
		/// Available in 16-bit mode
		/// </summary>
		Bit16					= 0x00000001,
		/// <summary>
		/// Available in 32-bit mode
		/// </summary>
		Bit32					= 0x00000002,
		/// <summary>
		/// Available in 64-bit mode
		/// </summary>
		Bit64					= 0x00000004,
		/// <summary>
		/// Can execute when CPL=0
		/// </summary>
		Cpl0					= 0x00000008,
		/// <summary>
		/// Can execute when CPL=1
		/// </summary>
		Cpl1					= 0x00000010,
		/// <summary>
		/// Can execute when CPL=2
		/// </summary>
		Cpl2					= 0x00000020,
		/// <summary>
		/// Can execute when CPL=3
		/// </summary>
		Cpl3					= 0x00000040,
		/// <summary>
		/// It reads/writes too many registers
		/// </summary>
		SaveRestore				= 0x00000080,
		/// <summary>
		/// It's an instruction that implicitly uses the stack register, eg. <c>CALL</c>, <c>POP</c>, etc
		/// </summary>
		StackInstruction		= 0x00000100,
		/// <summary>
		/// The instruction doesn't read the segment register if it uses a memory operand
		/// </summary>
		IgnoresSegment			= 0x00000200,
		/// <summary>
		/// The opmask register is read and written (instead of just read). This also implies that it can't be <c>K0</c>.
		/// </summary>
		OpMaskReadWrite			= 0x00000400,
		/// <summary>
		/// <c>W</c> bit is ignored in 16/32-bit mode (but not 64-bit mode)
		/// </summary>
		WIG32					= 0x00000800,
		/// <summary>
		/// <c>LOCK</c> prefix can be used
		/// </summary>
		Lock					= 0x00001000,
		/// <summary>
		/// <c>XACQUIRE</c> prefix can be used
		/// </summary>
		Xacquire				= 0x00002000,
		/// <summary>
		/// <c>XRELEASE</c> prefix can be used
		/// </summary>
		Xrelease				= 0x00004000,
		/// <summary>
		/// <c>REP</c>/<c>REPE</c> prefixes can be used
		/// </summary>
		Rep						= 0x00008000,
		/// <summary>
		/// <c>REPNE</c> prefix can be used
		/// </summary>
		Repne					= 0x00010000,
		/// <summary>
		/// <c>BND</c> prefix can be used
		/// </summary>
		Bnd						= 0x00020000,
		/// <summary>
		/// <c>HINT-TAKEN</c> and <c>HINT-NOT-TAKEN</c> prefixes can be used
		/// </summary>
		HintTaken				= 0x00040000,
		/// <summary>
		/// <c>NOTRACK</c> prefix can be used
		/// </summary>
		Notrack					= 0x00080000,
		/// <summary>
		/// It's an <c>FWAIT</c> + <c>FNxxx</c> instruction
		/// </summary>
		Fwait					= 0x00100000,
		/// <summary>
		/// It's one of: <c>INVALID</c>, <c>db</c>, <c>dw</c>, <c>dd</c>, <c>dq</c>
		/// </summary>
		NoInstruction			= 0x00200000,
		/// <summary>
		/// Broadcasting is supported (<c>EVEX.b</c> bit)
		/// </summary>
		Broadcast				= 0x00400000,
		/// <summary>
		/// Rounding control is supported
		/// </summary>
		RoundingControl			= 0x00800000,
		/// <summary>
		/// Suppress-all-exceptions is supported
		/// </summary>
		SuppressAllExceptions	= 0x01000000,
		/// <summary>
		/// Opmask register is supported
		/// </summary>
		OpMaskRegister			= 0x02000000,
		/// <summary>
		/// Zeroing masking is supported
		/// </summary>
		ZeroingMasking			= 0x04000000,
		/// <summary>
		/// Opmask can't be <c>K0</c>
		/// </summary>
		RequireOpMaskRegister	= 0x08000000,
		/// <summary>
		/// The mod bits are ignored and it's assumed <c>modrm[7:6] == 11b</c>
		/// </summary>
		IgnoresModBits			= 0x10000000,
		/// <summary>
		/// The index reg's reg-num (vsib op) (if any) and register ops' reg-nums must be unique, eg. <c>MNEMONIC XMM1,YMM1,[RAX+ZMM1*2]</c>
		/// is invalid. Registers = <c>XMM</c>/<c>YMM</c>/<c>ZMM</c>/<c>TMM</c>.
		/// </summary>
		RequiresUniqueRegNums	= 0x20000000,
		/// <summary>
		/// <c>66</c> prefix is not allowed (it will #UD)
		/// </summary>
		No66					= 0x40000000,
		/// <summary>
		/// <c>F2</c>/<c>F3</c> prefixes aren't allowed
		/// </summary>
		NFx						= 0x80000000,
	}

	[Flags]
	enum InstructionDefFlags2 : uint {
		None					= 0,
		/// <summary>
		/// Decoded by iced's 16-bit Intel decoder
		/// </summary>
		IntelDecoder16			= 0x00000001,
		/// <summary>
		/// Decoded by iced's 32-bit Intel decoder
		/// </summary>
		IntelDecoder32			= 0x00000002,
		/// <summary>
		/// Decoded by iced's 64-bit Intel decoder
		/// </summary>
		IntelDecoder64			= 0x00000004,
		/// <summary>
		/// Decoded by iced's 16-bit AMD decoder
		/// </summary>
		AmdDecoder16			= 0x00000008,
		/// <summary>
		/// Decoded by iced's 32-bit AMD decoder
		/// </summary>
		AmdDecoder32			= 0x00000010,
		/// <summary>
		/// Decoded by iced's 64-bit AMD decoder
		/// </summary>
		AmdDecoder64			= 0x00000020,
		/// <summary>
		/// Supported in real mode
		/// </summary>
		RealMode				= 0x00000040,
		/// <summary>
		/// Supported in protected mode
		/// </summary>
		ProtectedMode			= 0x00000080,
		/// <summary>
		/// Supported in virtual 8086 mode
		/// </summary>
		Virtual8086Mode			= 0x00000100,
		/// <summary>
		/// Supported in compatibility mode
		/// </summary>
		CompatibilityMode		= 0x00000200,
		/// <summary>
		/// Supported in 64-bit mode
		/// </summary>
		LongMode				= 0x00000400,
		/// <summary>
		/// Can be used outside SMM
		/// </summary>
		UseOutsideSmm			= 0x00000800,
		/// <summary>
		/// Can be used in SMM
		/// </summary>
		UseInSmm				= 0x00001000,
		/// <summary>
		/// Can be used outside an enclave (SGX)
		/// </summary>
		UseOutsideEnclaveSgx	= 0x00002000,
		/// <summary>
		/// Can be used inside an enclave (SGX1)
		/// </summary>
		UseInEnclaveSgx1		= 0x00004000,
		/// <summary>
		/// Can be used inside an enclave (SGX2)
		/// </summary>
		UseInEnclaveSgx2		= 0x00008000,
		/// <summary>
		/// Can be used outside VMX operation
		/// </summary>
		UseOutsideVmxOp			= 0x00010000,
		/// <summary>
		/// Can be used in VMX root operation
		/// </summary>
		UseInVmxRootOp			= 0x00020000,
		/// <summary>
		/// Can be used in VMX non-root operation
		/// </summary>
		UseInVmxNonRootOp		= 0x00040000,
		/// <summary>
		/// Can be used outside SEAM
		/// </summary>
		UseOutsideSeam			= 0x00080000,
		/// <summary>
		/// Can be used in SEAM
		/// </summary>
		UseInSeam				= 0x00100000,
		/// <summary>
		/// #UD is generated in TDX non-root operation
		/// </summary>
		TdxNonRootGenUd			= 0x00200000,
		/// <summary>
		/// #VE is generated in TDX non-root operation
		/// </summary>
		TdxNonRootGenVe			= 0x00400000,
		/// <summary>
		/// An exception (eg. #GP(0), #VE) may be generated in TDX non-root operation
		/// </summary>
		TdxNonRootMayGenEx		= 0x00800000,
		/// <summary>
		/// (Intel VMX) Causes a VM exit in VMX non-root operation
		/// </summary>
		IntelVmExit				= 0x01000000,
		/// <summary>
		/// (Intel VMX) May cause a VM exit in VMX non-root operation
		/// </summary>
		IntelMayVmExit			= 0x02000000,
		/// <summary>
		/// (Intel VMX) Causes an SMM VM exit in VMX root operation (if dual-monitor treatment is activated)
		/// </summary>
		IntelSmmVmExit			= 0x04000000,
		/// <summary>
		/// (AMD SVM) Causes a #VMEXIT in guest mode
		/// </summary>
		AmdVmExit				= 0x08000000,
		/// <summary>
		/// (AMD SVM) May cause a #VMEXIT in guest mode
		/// </summary>
		AmdMayVmExit			= 0x10000000,
		/// <summary>
		/// Causes a TSX abort inside a TSX transaction
		/// </summary>
		TsxAbort				= 0x20000000,
		/// <summary>
		/// Causes a TSX abort inside a TSX transaction depending on the implementation
		/// </summary>
		TsxImplAbort			= 0x40000000,
		/// <summary>
		/// May cause a TSX abort inside a TSX transaction depending on some condition
		/// </summary>
		TsxMayAbort				= 0x80000000,
	}

	[Flags]
	enum InstructionDefFlags3 : uint {
		None					= 0,
		/// <summary>
		/// Default operand size is 64 in 64-bit mode
		/// </summary>
		DefaultOpSize64			= 0x00000001,
		/// <summary>
		/// The operand size is always 64 in 64-bit mode
		/// </summary>
		ForceOpSize64			= 0x00000002,
		/// <summary>
		/// Intel decoder forces 64-bit operand size
		/// </summary>
		IntelForceOpSize64		= 0x00000004,
		/// <summary>
		/// The instruction accesses the I/O address space (eg. <c>IN</c>, <c>OUT</c>, <c>INS</c>, <c>OUTS</c>)
		/// </summary>
		InputOutput				= 0x00000008,
		/// <summary>
		/// It's one of the many nop instructions (does not include FPU nop instructions, eg. <c>FNOP</c>)
		/// </summary>
		Nop						= 0x00000010,
		/// <summary>
		/// It's one of the many reserved nop instructions (eg. <c>0F0D</c>, <c>0F18-0F1F</c>)
		/// </summary>
		ReservedNop				= 0x00000020,
		/// <summary>
		/// The rounding control is ignored (#UD is not generated)
		/// </summary>
		IgnoresRoundingControl	= 0x00000040,
		/// <summary>
		/// It's a serializing instruction (Intel CPUs)
		/// </summary>
		SerializingIntel		= 0x00000080,
		/// <summary>
		/// It's a serializing instruction (AMD CPUs)
		/// </summary>
		SerializingAmd			= 0x00000100,
		/// <summary>
		/// The instruction requires either CPL=0 or CPL<=3 depending on some CPU option (eg. <c>CR4.TSD</c>, <c>CR4.PCE</c>, <c>CR4.UMIP</c>)
		/// </summary>
		MayRequireCpl0			= 0x00000200,
		/// <summary>
		/// (AMD) The <c>LOCK</c> prefix can be used as an extra register bit (bit 3) to access registers 8-15 without a <c>REX</c> prefix (eg. in 32-bit mode)
		/// </summary>
		AmdLockRegBit			= 0x00000400,
		/// <summary>
		/// It's a tracked <c>JMP</c>/<c>CALL</c> indirect instruction (CET)
		/// </summary>
		CetTracked				= 0x00000800,
		/// <summary>
		/// Non-temporal hint memory access (eg. <c>MOVNTDQ</c>)
		/// </summary>
		NonTemporal				= 0x00001000,
		/// <summary>
		/// It's a no-wait FPU instruction, eg. <c>FNINIT</c>
		/// </summary>
		FpuNoWait				= 0x00002000,
		/// <summary>
		/// It's a privileged instruction (all CPL=0 instructions (except <c>VMCALL</c>) and IOPL instructions <c>IN</c>, <c>INS</c>, <c>OUT</c>, <c>OUTS</c>, <c>CLI</c>, <c>STI</c>)
		/// </summary>
		Privileged				= 0x00004000,
		/// <summary>
		/// This instruction always uses zeroing masking
		/// </summary>
		ImpliedZeroingMasking	= 0x00008000,//TODO: Add to OpCodeInfo
		/// <summary>
		/// The opmask register is an element selector and not a write mask
		/// </summary>
		OpMaskIsElementSelector	= 0x00010000,//TODO: Add to OpCodeInfo
		/// <summary>
		/// This is a prefetch instruction (it can't cause a memory fault)
		/// </summary>
		Prefetch				= 0x00020000,//TODO: Add to OpCodeInfo
		/// <summary>
		/// The index register is ignored when calculating the effective address (eg. <c>BNDLDX</c>, <c>BNDSTX</c>)
		/// </summary>
		IgnoresIndex			= 0x00040000,//TODO: Add to OpCodeInfo
		/// <summary>
		/// The index register (if present) is the tile stride indicator
		/// </summary>
		TileStrideIndex			= 0x00080000,//TODO: Add to OpCodeInfo
		/// <summary>
		/// FPU <c>TOP</c> is written (eg. push or pop instruction, <c>FLDENV</c>, etc)
		/// </summary>
		WritesFpuTop			= 0x00100000,
		/// <summary>
		/// Set if it's a conditional write to FPU <c>TOP</c> bits
		/// </summary>
		IsFpuCondWriteTop		= 0x00200000,
	}

	enum VmxMode {
		/// <summary>
		/// No #UD is generated. This instruction can execute outside VMX operation.
		/// </summary>
		None,
		/// <summary>
		/// #UD if not in VMX operation
		/// </summary>
		VmxOp,
		/// <summary>
		/// #UD if not in VMX root operation
		/// </summary>
		VmxRootOp,
		/// <summary>
		/// #UD if not in VMX non-root operation
		/// </summary>
		VmxNonRootOp,
	}

	[Enum("InstrStrFmtOption")]
	enum InstrStrFmtOption {
		/// <summary>
		/// No special code is needed to format the instruction string
		/// </summary>
		None,
		/// <summary>
		/// Used if the opmask is `{k1}` even if the first operand is also a `k` reg, eg. `xxx k2 {k1}, xmm3`
		/// or
		/// Don't print the GPR suffix (a, b, etc), eg. `xxx r32, r32` instead of `xxx r32a, r32b`
		/// </summary>
		OpMaskIsK1_or_NoGprSuffix,
		/// <summary>
		/// Increment the vector index which causes the first vector register number to be `2` instead of `1`, eg. `xxx eax, xmm2`
		/// </summary>
		IncVecIndex,
		/// <summary>
		/// Don't print the vector index (usually set if it's an MMX instruction), eg. `xxx mm, mm/m64`
		/// </summary>
		NoVecIndex,
		/// <summary>
		/// The first operand should use index `2` and the next operand index `1`, eg. `xxx xmm2, xmm1`
		/// </summary>
		SwapVecIndex12,
		/// <summary>
		/// Don't print the first operand
		/// </summary>
		SkipOp0,
	}

	[Flags]
	enum InstructionStringFlags : uint {
		None					= 0,
		/// <summary>
		/// The modrm info is part of the string, eg. `!(11):000:bbb`
		/// </summary>
		ModRegRmString			= 0x00000001,
	}

	enum BranchKind {
		None,
		JccShort,
		JccNear,
		JmpShort,
		JmpNear,
		JmpFar,
		JmpNearIndirect,
		JmpFarIndirect,
		CallNear,
		CallFar,
		CallNearIndirect,
		CallFarIndirect,
		JmpeNear,
		JmpeNearIndirect,
		Loop,
		Jrcxz,
		Xbegin,
	}

	readonly struct InstrStrImpliedOp {
		public readonly bool IsUpper;
		public readonly string Operand;
		public InstrStrImpliedOp(string operand) {
			Operand = operand.ToUpperInvariant();
			IsUpper = Operand == operand;
		}
	}

	enum StackInfoKind {
		None,
		Increment,
		Enter,
		Iret,
		PopImm16,
	}

	readonly struct StackInfo : IEquatable<StackInfo>, IComparable<StackInfo> {
		public readonly StackInfoKind Kind;
		public readonly int Value;
		public StackInfo(StackInfoKind kind, int value) {
			Kind = kind;
			Value = value;
		}
		public override bool Equals(object? obj) => obj is StackInfo info && Equals(info);
		public bool Equals(StackInfo other) => Kind == other.Kind && Value == other.Value;
		public override int GetHashCode() => HashCode.Combine(Kind, Value);
		public int CompareTo(StackInfo other) {
			int c = Kind.CompareTo(other.Kind);
			if (c != 0) return c;
			return Value.CompareTo(other.Value);
		}
	}

	[DebuggerDisplay("{OpCodeString,nq} | {InstructionString,nq}")]
	sealed class InstructionDef {
		public readonly string OpCodeString;
		public readonly string InstructionString;
		public int OpCount => OpKindDefs.Length;
		public readonly EnumValue Code;
		public readonly EnumValue Mnemonic;
		public readonly EnumValue Memory;
		public readonly EnumValue MemoryBroadcast;
		public readonly EnumValue DecoderOption;
		public readonly EnumValue EncodingValue;
		public EncodingKind Encoding => (EncodingKind)EncodingValue.Value;
		public readonly InstructionDefFlags1 Flags1;
		public readonly InstructionDefFlags2 Flags2;
		public readonly InstructionDefFlags3 Flags3;
		public readonly InstrStrFmtOption InstrStrFmtOption;
		public readonly InstructionStringFlags InstrStrFlags;
		public readonly InstrStrImpliedOp[] InstrStrImpliedOps;

		public readonly CodeSize OperandSize;
		public readonly CodeSize AddressSize;
		public readonly MandatoryPrefix MandatoryPrefix;
		public readonly OpCodeTableKind Table;
		public readonly OpCodeL LBit;
		public readonly OpCodeW WBit;
		public readonly uint OpCode;
		public readonly int OpCodeLength;
		public readonly int GroupIndex;
		public readonly int RmGroupIndex;
		public readonly TupleType TupleType;
		public readonly OpCodeOperandKindDef[] OpKindDefs;

		public ImpliedAccessesDef ImpliedAccessDef => impliedAccessDef ?? throw new InvalidOperationException();
		ImpliedAccessesDef? impliedAccessDef;
		public readonly PseudoOpsKind? PseudoOp;
		public readonly EnumValue ControlFlow;
		public readonly ConditionCode ConditionCode;
		public readonly BranchKind BranchKind;//TODO: Add to OpCodeInfo
		public readonly StackInfo StackInfo;
		public readonly int FpuStackIncrement;
		public readonly RflagsBits RflagsRead;
		public readonly RflagsBits RflagsUndefined;
		public readonly RflagsBits RflagsWritten;
		public readonly RflagsBits RflagsCleared;
		public readonly RflagsBits RflagsSet;
		public EnumValue? RflagsInfo { get; internal set; }
		public readonly EnumValue[] Cpuid;
		public EnumValue? CpuidInternal { get; internal set; }
		public readonly OpInfo[] OpInfo;
		public readonly EnumValue[] OpInfoEnum;

		public readonly FastFmtInstructionDef Fast;
		public readonly FmtInstructionDef Gas;
		public readonly FmtInstructionDef Intel;
		public readonly FmtInstructionDef Masm;
		public readonly FmtInstructionDef Nasm;

		public InstructionDef(EnumValue code, string opCodeString, string instructionString, EnumValue mnemonic,
			EnumValue mem, EnumValue bcst, EnumValue decoderOption, InstructionDefFlags1 flags1, InstructionDefFlags2 flags2,
			InstructionDefFlags3 flags3, InstrStrFmtOption instrStrFmtOption, InstructionStringFlags instrStrFlags,
			InstrStrImpliedOp[] instrStrImpliedOps,
			MandatoryPrefix mandatoryPrefix, OpCodeTableKind table, OpCodeL lBit, OpCodeW wBit, uint opCode, int opCodeLength,
			int groupIndex, int rmGroupIndex, CodeSize operandSize, CodeSize addressSize, TupleType tupleType, OpCodeOperandKindDef[] opKinds,
			PseudoOpsKind? pseudoOp, EnumValue encoding, EnumValue flowControl, ConditionCode conditionCode,
			BranchKind branchKind, StackInfo stackInfo, int fpuStackIncrement,
			RflagsBits read, RflagsBits undefined, RflagsBits written, RflagsBits cleared, RflagsBits set,
			EnumValue[] cpuid, OpInfo[] opInfo,
			FastFmtInstructionDef fast, FmtInstructionDef gas, FmtInstructionDef intel, FmtInstructionDef masm, FmtInstructionDef nasm) {
			Code = code;
			OpCodeString = opCodeString;
			InstructionString = instructionString;
			Mnemonic = mnemonic;
			Memory = mem;
			MemoryBroadcast = bcst;
			DecoderOption = decoderOption;
			EncodingValue = encoding;
			Flags1 = flags1;
			Flags2 = flags2;
			Flags3 = flags3;
			InstrStrFmtOption = instrStrFmtOption;
			InstrStrFlags = instrStrFlags;
			InstrStrImpliedOps = instrStrImpliedOps;

			MandatoryPrefix = mandatoryPrefix;
			Table = table;
			LBit = lBit;
			WBit = wBit;
			OpCode = opCode;
			OpCodeLength = opCodeLength;
			GroupIndex = groupIndex;
			RmGroupIndex = rmGroupIndex;
			TupleType = tupleType;
			OperandSize = operandSize;
			AddressSize = addressSize;
			OpKindDefs = opKinds;

			PseudoOp = pseudoOp;
			ControlFlow = flowControl;
			ConditionCode = conditionCode;
			BranchKind = branchKind;
			StackInfo = stackInfo;
			FpuStackIncrement = fpuStackIncrement;
			RflagsRead = read;
			RflagsUndefined = undefined;
			RflagsWritten = written;
			RflagsCleared = cleared;
			RflagsSet = set;
			RflagsInfo = null;
			Cpuid = cpuid;
			CpuidInternal = null;
			OpInfo = opInfo;
			OpInfoEnum = new EnumValue[opInfo.Length];

			Fast = fast;
			Gas = gas;
			Intel = intel;
			Masm = masm;
			Nasm = nasm;
		}

		internal void SetImpliedAccess(ImpliedAccessesDef value) {
			if (impliedAccessDef is not null)
				throw new InvalidOperationException();
			impliedAccessDef = value;
		}
	}
}
