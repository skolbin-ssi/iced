// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

#if MASM
using System.Collections.Generic;
using Iced.Intel;
using Iced.UnitTests.Intel.DecoderTests;
using Xunit;

namespace Iced.UnitTests.Intel.FormatterTests.Masm {
	public sealed class FormatterTest32 : FormatterTest {
		[Theory]
		[MemberData(nameof(Format_Data_MemAlways))]
		void Format_MemAlways(int index, InstructionInfo info, string formattedString) => FormatBase(index, info, formattedString, FormatterFactory.Create_MemAlways());
		public static IEnumerable<object[]> Format_Data_MemAlways => FormatterTestCases.GetFormatData(32, "Masm", "MemAlways");

		[Theory]
		[MemberData(nameof(Format_Data_MemDefault))]
		void Format_MemDefault(int index, InstructionInfo info, string formattedString) => FormatBase(index, info, formattedString, FormatterFactory.Create_MemDefault());
		public static IEnumerable<object[]> Format_Data_MemDefault => FormatterTestCases.GetFormatData(32, "Masm", "MemDefault");

		[Theory]
		[MemberData(nameof(Format_Data_MemMinimum))]
		void Format_MemMinimum(int index, InstructionInfo info, string formattedString) => FormatBase(index, info, formattedString, FormatterFactory.Create_MemMinimum());
		public static IEnumerable<object[]> Format_Data_MemMinimum => FormatterTestCases.GetFormatData(32, "Masm", "MemMinimum");

#if ENCODER
		[Theory]
		[MemberData(nameof(Format_Data_NonDec_MemAlways))]
		void Format_NonDec_MemAlways(int index, Instruction info, string formattedString) => FormatBase(index, info, formattedString, FormatterFactory.Create_MemAlways());
		public static IEnumerable<object[]> Format_Data_NonDec_MemAlways => FormatterTestCases.GetFormatData(32, NonDecodedInstructions.Infos32, "Masm", "NonDec_MemAlways");

		[Theory]
		[MemberData(nameof(Format_Data_NonDec_MemDefault))]
		void Format_NonDec_MemDefault(int index, Instruction info, string formattedString) => FormatBase(index, info, formattedString, FormatterFactory.Create_MemDefault());
		public static IEnumerable<object[]> Format_Data_NonDec_MemDefault => FormatterTestCases.GetFormatData(32, NonDecodedInstructions.Infos32, "Masm", "NonDec_MemDefault");

		[Theory]
		[MemberData(nameof(Format_Data_NonDec_MemMinimum))]
		void Format_NonDec_MemMinimum(int index, Instruction info, string formattedString) => FormatBase(index, info, formattedString, FormatterFactory.Create_MemMinimum());
		public static IEnumerable<object[]> Format_Data_NonDec_MemMinimum => FormatterTestCases.GetFormatData(32, NonDecodedInstructions.Infos32, "Masm", "NonDec_MemMinimum");
#endif

		[Theory]
		[MemberData(nameof(Format_Data_Misc))]
		void Format_Misc(int index, InstructionInfo info, string formattedString) => FormatBase(index, info, formattedString, FormatterFactory.Create());
		public static IEnumerable<object[]> Format_Data_Misc => FormatterTestCases.GetFormatData(32, "Masm", "Misc", isMisc: true);
	}
}
#endif
