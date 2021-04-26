// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

#if GAS || INTEL || MASM || NASM
using System;
using System.Diagnostics;
using System.Runtime.CompilerServices;

namespace Iced.Intel.FormatterInternal {
	// GENERATOR-BEGIN: FormatterFlowControl
	// ⚠️This was generated by GENERATOR!🦹‍♂️
	enum FormatterFlowControl {
		AlwaysShortBranch,
		ShortBranch,
		NearBranch,
		NearCall,
		FarBranch,
		FarCall,
		Xbegin,
	}
	// GENERATOR-END: FormatterFlowControl

	static partial class FormatterUtils {
		static readonly string[] spaceStrings = CreateStrings(' ', 20);
		static readonly string[] tabStrings = CreateStrings('\t', 6);

		static string[] CreateStrings(char c, int max) {
			var strings = new string[max];
			for (int i = 0; i < strings.Length; i++)
				strings[i] = new string(c, i + 1);
			return strings;
		}

		public static void AddTabs(FormatterOutput output, int column, int firstOperandCharIndex, int tabSize) {
#if DEBUG
			for (int i = 0; i < spaceStrings.Length; i++)
				Debug.Assert(spaceStrings[i].Length == i + 1);
			for (int i = 0; i < tabStrings.Length; i++)
				Debug.Assert(tabStrings[i].Length == i + 1);
#endif
			const int max_firstOperandCharIndex = 256;
			if (firstOperandCharIndex < 0)
				firstOperandCharIndex = 0;
			else if (firstOperandCharIndex > max_firstOperandCharIndex)
				firstOperandCharIndex = max_firstOperandCharIndex;

			if (tabSize <= 0) {
				int charsLeft = firstOperandCharIndex - column;
				if (charsLeft <= 0)
					charsLeft = 1;
				AddStrings(output, spaceStrings, charsLeft);
			}
			else {
				int endCol = firstOperandCharIndex;
				if (endCol <= column)
					endCol = column + 1;
				int endColRoundedDown = endCol / tabSize * tabSize;
				bool addedTabs = endColRoundedDown > column;
				if (addedTabs) {
					int tabs = (endColRoundedDown - (column / tabSize * tabSize)) / tabSize;
					AddStrings(output, tabStrings, tabs);
					column = endColRoundedDown;
				}
				int spaces = firstOperandCharIndex - column;
				if (spaces > 0)
					AddStrings(output, spaceStrings, spaces);
				else if (!addedTabs)
					AddStrings(output, spaceStrings, 1);
			}
		}

		static void AddStrings(FormatterOutput output, string[] strings, int count) {
			while (count > 0) {
				int n = count;
				if (n >= strings.Length)
					n = strings.Length;
				output.Write(strings[n - 1], FormatterTextKind.Text);
				count -= n;
			}
		}

		public static bool IsCall(FormatterFlowControl kind) => kind == FormatterFlowControl.NearCall || kind == FormatterFlowControl.FarCall;

		public static FormatterFlowControl GetFlowControl(in Instruction instruction) {
			switch (instruction.Code) {
			// GENERATOR-BEGIN: FormatterFlowControlSwitch
			// ⚠️This was generated by GENERATOR!🦹‍♂️
			case Code.Jo_rel8_16:
			case Code.Jo_rel8_32:
			case Code.Jo_rel8_64:
			case Code.Jno_rel8_16:
			case Code.Jno_rel8_32:
			case Code.Jno_rel8_64:
			case Code.Jb_rel8_16:
			case Code.Jb_rel8_32:
			case Code.Jb_rel8_64:
			case Code.Jae_rel8_16:
			case Code.Jae_rel8_32:
			case Code.Jae_rel8_64:
			case Code.Je_rel8_16:
			case Code.Je_rel8_32:
			case Code.Je_rel8_64:
			case Code.Jne_rel8_16:
			case Code.Jne_rel8_32:
			case Code.Jne_rel8_64:
			case Code.Jbe_rel8_16:
			case Code.Jbe_rel8_32:
			case Code.Jbe_rel8_64:
			case Code.Ja_rel8_16:
			case Code.Ja_rel8_32:
			case Code.Ja_rel8_64:
			case Code.Js_rel8_16:
			case Code.Js_rel8_32:
			case Code.Js_rel8_64:
			case Code.Jns_rel8_16:
			case Code.Jns_rel8_32:
			case Code.Jns_rel8_64:
			case Code.Jp_rel8_16:
			case Code.Jp_rel8_32:
			case Code.Jp_rel8_64:
			case Code.Jnp_rel8_16:
			case Code.Jnp_rel8_32:
			case Code.Jnp_rel8_64:
			case Code.Jl_rel8_16:
			case Code.Jl_rel8_32:
			case Code.Jl_rel8_64:
			case Code.Jge_rel8_16:
			case Code.Jge_rel8_32:
			case Code.Jge_rel8_64:
			case Code.Jle_rel8_16:
			case Code.Jle_rel8_32:
			case Code.Jle_rel8_64:
			case Code.Jg_rel8_16:
			case Code.Jg_rel8_32:
			case Code.Jg_rel8_64:
			case Code.Jmp_rel8_16:
			case Code.Jmp_rel8_32:
			case Code.Jmp_rel8_64:
				return FormatterFlowControl.ShortBranch;
			case Code.Loopne_rel8_16_CX:
			case Code.Loopne_rel8_32_CX:
			case Code.Loopne_rel8_16_ECX:
			case Code.Loopne_rel8_32_ECX:
			case Code.Loopne_rel8_64_ECX:
			case Code.Loopne_rel8_16_RCX:
			case Code.Loopne_rel8_64_RCX:
			case Code.Loope_rel8_16_CX:
			case Code.Loope_rel8_32_CX:
			case Code.Loope_rel8_16_ECX:
			case Code.Loope_rel8_32_ECX:
			case Code.Loope_rel8_64_ECX:
			case Code.Loope_rel8_16_RCX:
			case Code.Loope_rel8_64_RCX:
			case Code.Loop_rel8_16_CX:
			case Code.Loop_rel8_32_CX:
			case Code.Loop_rel8_16_ECX:
			case Code.Loop_rel8_32_ECX:
			case Code.Loop_rel8_64_ECX:
			case Code.Loop_rel8_16_RCX:
			case Code.Loop_rel8_64_RCX:
			case Code.Jcxz_rel8_16:
			case Code.Jcxz_rel8_32:
			case Code.Jecxz_rel8_16:
			case Code.Jecxz_rel8_32:
			case Code.Jecxz_rel8_64:
			case Code.Jrcxz_rel8_16:
			case Code.Jrcxz_rel8_64:
				return FormatterFlowControl.AlwaysShortBranch;
			case Code.Call_rel16:
			case Code.Call_rel32_32:
			case Code.Call_rel32_64:
				return FormatterFlowControl.NearCall;
			case Code.Jmp_rel16:
			case Code.Jmp_rel32_32:
			case Code.Jmp_rel32_64:
			case Code.Jo_rel16:
			case Code.Jo_rel32_32:
			case Code.Jo_rel32_64:
			case Code.Jno_rel16:
			case Code.Jno_rel32_32:
			case Code.Jno_rel32_64:
			case Code.Jb_rel16:
			case Code.Jb_rel32_32:
			case Code.Jb_rel32_64:
			case Code.Jae_rel16:
			case Code.Jae_rel32_32:
			case Code.Jae_rel32_64:
			case Code.Je_rel16:
			case Code.Je_rel32_32:
			case Code.Je_rel32_64:
			case Code.Jne_rel16:
			case Code.Jne_rel32_32:
			case Code.Jne_rel32_64:
			case Code.Jbe_rel16:
			case Code.Jbe_rel32_32:
			case Code.Jbe_rel32_64:
			case Code.Ja_rel16:
			case Code.Ja_rel32_32:
			case Code.Ja_rel32_64:
			case Code.Js_rel16:
			case Code.Js_rel32_32:
			case Code.Js_rel32_64:
			case Code.Jns_rel16:
			case Code.Jns_rel32_32:
			case Code.Jns_rel32_64:
			case Code.Jp_rel16:
			case Code.Jp_rel32_32:
			case Code.Jp_rel32_64:
			case Code.Jnp_rel16:
			case Code.Jnp_rel32_32:
			case Code.Jnp_rel32_64:
			case Code.Jl_rel16:
			case Code.Jl_rel32_32:
			case Code.Jl_rel32_64:
			case Code.Jge_rel16:
			case Code.Jge_rel32_32:
			case Code.Jge_rel32_64:
			case Code.Jle_rel16:
			case Code.Jle_rel32_32:
			case Code.Jle_rel32_64:
			case Code.Jg_rel16:
			case Code.Jg_rel32_32:
			case Code.Jg_rel32_64:
			case Code.Jmpe_disp16:
			case Code.Jmpe_disp32:
				return FormatterFlowControl.NearBranch;
			case Code.Call_ptr1616:
			case Code.Call_ptr1632:
				return FormatterFlowControl.FarCall;
			case Code.Jmp_ptr1616:
			case Code.Jmp_ptr1632:
				return FormatterFlowControl.FarBranch;
			case Code.Xbegin_rel16:
			case Code.Xbegin_rel32:
				return FormatterFlowControl.Xbegin;
			// GENERATOR-END: FormatterFlowControlSwitch

			default:
				throw new InvalidOperationException();
			}
		}

		public static bool IsRepeOrRepneInstruction(Code code) {
			switch (code) {
			case Code.Cmpsb_m8_m8:
			case Code.Cmpsw_m16_m16:
			case Code.Cmpsd_m32_m32:
			case Code.Cmpsq_m64_m64:
			case Code.Scasb_AL_m8:
			case Code.Scasw_AX_m16:
			case Code.Scasd_EAX_m32:
			case Code.Scasq_RAX_m64:
				return true;

			default:
				return false;
			}
		}

		static bool IsRepRepeRepneInstruction(Code code) {
			switch (code) {
			case Code.Insb_m8_DX:
			case Code.Insw_m16_DX:
			case Code.Insd_m32_DX:
			case Code.Outsb_DX_m8:
			case Code.Outsw_DX_m16:
			case Code.Outsd_DX_m32:
			case Code.Movsb_m8_m8:
			case Code.Movsw_m16_m16:
			case Code.Movsd_m32_m32:
			case Code.Movsq_m64_m64:
			case Code.Cmpsb_m8_m8:
			case Code.Cmpsw_m16_m16:
			case Code.Cmpsd_m32_m32:
			case Code.Cmpsq_m64_m64:
			case Code.Stosb_m8_AL:
			case Code.Stosw_m16_AX:
			case Code.Stosd_m32_EAX:
			case Code.Stosq_m64_RAX:
			case Code.Lodsb_AL_m8:
			case Code.Lodsw_AX_m16:
			case Code.Lodsd_EAX_m32:
			case Code.Lodsq_RAX_m64:
			case Code.Scasb_AL_m8:
			case Code.Scasw_AX_m16:
			case Code.Scasd_EAX_m32:
			case Code.Scasq_RAX_m64:
			case Code.Montmul_16:
			case Code.Montmul_32:
			case Code.Montmul_64:
			case Code.Xsha1_16:
			case Code.Xsha1_32:
			case Code.Xsha1_64:
			case Code.Xsha256_16:
			case Code.Xsha256_32:
			case Code.Xsha256_64:
			case Code.Xstore_16:
			case Code.Xstore_32:
			case Code.Xstore_64:
			case Code.Xcryptecb_16:
			case Code.Xcryptecb_32:
			case Code.Xcryptecb_64:
			case Code.Xcryptcbc_16:
			case Code.Xcryptcbc_32:
			case Code.Xcryptcbc_64:
			case Code.Xcryptctr_16:
			case Code.Xcryptctr_32:
			case Code.Xcryptctr_64:
			case Code.Xcryptcfb_16:
			case Code.Xcryptcfb_32:
			case Code.Xcryptcfb_64:
			case Code.Xcryptofb_16:
			case Code.Xcryptofb_32:
			case Code.Xcryptofb_64:
			case Code.Ccs_hash_16:
			case Code.Ccs_hash_32:
			case Code.Ccs_hash_64:
			case Code.Ccs_encrypt_16:
			case Code.Ccs_encrypt_32:
			case Code.Ccs_encrypt_64:
				return true;

			default:
				return false;
			}
		}

		public static bool ShowRepOrRepePrefix(Code code, FormatterOptions options) =>
			ShowRepOrRepePrefix(code, options.ShowUselessPrefixes);

		public static bool ShowRepnePrefix(Code code, FormatterOptions options) =>
			ShowRepnePrefix(code, options.ShowUselessPrefixes);

		[MethodImpl(MethodImplOptions.AggressiveInlining)]
		public static PrefixKind GetSegmentRegisterPrefixKind(Register register) {
			Debug.Assert(register == Register.ES || register == Register.CS || register == Register.SS ||
						register == Register.DS || register == Register.FS || register == Register.GS);
			Static.Assert(PrefixKind.ES + 1 == PrefixKind.CS ? 0 : -1);
			Static.Assert(PrefixKind.ES + 2 == PrefixKind.SS ? 0 : -1);
			Static.Assert(PrefixKind.ES + 3 == PrefixKind.DS ? 0 : -1);
			Static.Assert(PrefixKind.ES + 4 == PrefixKind.FS ? 0 : -1);
			Static.Assert(PrefixKind.ES + 5 == PrefixKind.GS ? 0 : -1);
			return (register - Register.ES) + PrefixKind.ES;
		}

		static bool IsCode64(CodeSize codeSize) =>
			codeSize == CodeSize.Code64 || codeSize == CodeSize.Unknown;

		public static bool ShowIndexScale(in Instruction instruction, FormatterOptions options) =>
			options.ShowUselessPrefixes || !instruction.Code.IgnoresIndex();

		public static bool ShowSegmentPrefix(Register defaultSegReg, in Instruction instruction, FormatterOptions options) =>
			ShowSegmentPrefix(defaultSegReg, instruction, options.ShowUselessPrefixes);

		static Register GetDefaultSegmentRegister(in Instruction instruction) {
			var baseReg = instruction.MemoryBase;
			if (baseReg == Register.BP || baseReg == Register.EBP || baseReg == Register.ESP || baseReg == Register.RBP || baseReg == Register.RSP)
				return Register.SS;
			return Register.DS;
		}

		public static bool CanShowRoundingControl(in Instruction instruction, FormatterOptions options) {
			switch (instruction.Code) {
#if !NO_EVEX
			case Code.EVEX_Vcvtsi2sd_xmm_xmm_rm32_er:
			case Code.EVEX_Vcvtusi2sd_xmm_xmm_rm32_er:
			case Code.EVEX_Vcvtdq2pd_zmm_k1z_ymmm256b32_er:
			case Code.EVEX_Vcvtudq2pd_zmm_k1z_ymmm256b32_er:
				return options.ShowUselessPrefixes;
#endif
			default:
				return true;
			}
		}
	}
}
#endif
