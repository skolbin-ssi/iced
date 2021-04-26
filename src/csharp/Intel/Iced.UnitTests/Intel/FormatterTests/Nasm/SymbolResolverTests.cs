// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

#if NASM
using System.Collections.Generic;
using Xunit;

namespace Iced.UnitTests.Intel.FormatterTests.Nasm {
	public sealed class SymbolResolverTests : FormatterTests.SymbolResolverTests {
		[Theory]
		[MemberData(nameof(Format_Data))]
		void Format(int index, SymbolResolverTestCase info, string formattedString) => FormatBase(index, info, formattedString, FormatterFactory.Create_Resolver(new TestSymbolResolver(info)));
		public static IEnumerable<object[]> Format_Data => SymbolResolverTestUtils.GetFormatData("Nasm", "SymbolResolverTests");
	}
}
#endif
