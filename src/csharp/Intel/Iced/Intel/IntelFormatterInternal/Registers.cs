// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

#if INTEL
using Iced.Intel.FormatterInternal;

namespace Iced.Intel.IntelFormatterInternal {
	static class Registers {
		public const int Register_ST = IcedConstants.RegisterEnumCount + 0;
		public const int ExtraRegisters = 1;
		public static readonly FormatterString[] AllRegisters = RegistersTable.GetRegisters();
	}
}
#endif
