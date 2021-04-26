// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

namespace Generator.Enums.Formatter {
	[Enum("PseudoOpsKind")]
	enum PseudoOpsKind {
		cmpps,
		vcmpps,
		cmppd,
		vcmppd,
		cmpss,
		vcmpss,
		cmpsd,
		vcmpsd,
		pclmulqdq,
		vpclmulqdq,
		vpcomb,
		vpcomw,
		vpcomd,
		vpcomq,
		vpcomub,
		vpcomuw,
		vpcomud,
		vpcomuq,
	}
}
