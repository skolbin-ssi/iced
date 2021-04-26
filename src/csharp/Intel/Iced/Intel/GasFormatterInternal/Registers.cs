// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

#if GAS
using System.Diagnostics;
using Iced.Intel.FormatterInternal;

namespace Iced.Intel.GasFormatterInternal {
	static class Registers {
		public const int Register_ST = IcedConstants.RegisterEnumCount + 0;
		public const int ExtraRegisters = 1;
		public static readonly FormatterString[] AllRegistersNaked = RegistersTable.GetRegisters();
		public static readonly FormatterString[] AllRegisters = GetRegistersWithPrefix();
		static FormatterString[] GetRegistersWithPrefix() {
			var registers = AllRegistersNaked;
			Debug2.Assert(registers is not null);
			var result = new FormatterString[registers.Length];
			for (int i = 0; i < registers.Length; i++)
				result[i] = new FormatterString("%" + registers[i].Get(false));
			return result;
		}
	}
}
#endif
