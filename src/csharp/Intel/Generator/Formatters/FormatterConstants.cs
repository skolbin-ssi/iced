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

using System;

using Generator.Enums.Formatter;
namespace Generator.Formatters {
	static class FormatterConstants {
		public static string[] GetPseudoOps(PseudoOpsKind kind) =>
			kind switch {
				PseudoOpsKind.cmpps => cmpps_pseudo_ops,
				PseudoOpsKind.vcmpps => vcmpps_pseudo_ops,
				PseudoOpsKind.cmppd => cmppd_pseudo_ops,
				PseudoOpsKind.vcmppd => vcmppd_pseudo_ops,
				PseudoOpsKind.cmpss => cmpss_pseudo_ops,
				PseudoOpsKind.vcmpss => vcmpss_pseudo_ops,
				PseudoOpsKind.cmpsd => cmpsd_pseudo_ops,
				PseudoOpsKind.vcmpsd => vcmpsd_pseudo_ops,
				PseudoOpsKind.pclmulqdq => pclmulqdq_pseudo_ops,
				PseudoOpsKind.vpclmulqdq => vpclmulqdq_pseudo_ops,
				PseudoOpsKind.vpcomb => vpcomb_pseudo_ops,
				PseudoOpsKind.vpcomw => vpcomw_pseudo_ops,
				PseudoOpsKind.vpcomd => vpcomd_pseudo_ops,
				PseudoOpsKind.vpcomq => vpcomq_pseudo_ops,
				PseudoOpsKind.vpcomub => vpcomub_pseudo_ops,
				PseudoOpsKind.vpcomuw => vpcomuw_pseudo_ops,
				PseudoOpsKind.vpcomud => vpcomud_pseudo_ops,
				PseudoOpsKind.vpcomuq => vpcomuq_pseudo_ops,
				_ => throw new ArgumentOutOfRangeException(nameof(kind)),
			};

		static FormatterConstants() {
			var cc = new string[32] {
				"eq",
				"lt",
				"le",
				"unord",
				"neq",
				"nlt",
				"nle",
				"ord",
				"eq_uq",
				"nge",
				"ngt",
				"false",
				"neq_oq",
				"ge",
				"gt",
				"true",
				"eq_os",
				"lt_oq",
				"le_oq",
				"unord_s",
				"neq_us",
				"nlt_uq",
				"nle_uq",
				"ord_s",
				"eq_us",
				"nge_uq",
				"ngt_uq",
				"false_os",
				"neq_os",
				"ge_oq",
				"gt_oq",
				"true_us",
			};
			cmpps_pseudo_ops = Create(cc, 8, "cmp", "ps");
			vcmpps_pseudo_ops = Create(cc, 32, "vcmp", "ps");
			cmppd_pseudo_ops = Create(cc, 8, "cmp", "pd");
			vcmppd_pseudo_ops = Create(cc, 32, "vcmp", "pd");
			cmpss_pseudo_ops = Create(cc, 8, "cmp", "ss");
			vcmpss_pseudo_ops = Create(cc, 32, "vcmp", "ss");
			cmpsd_pseudo_ops = Create(cc, 8, "cmp", "sd");
			vcmpsd_pseudo_ops = Create(cc, 32, "vcmp", "sd");

			var xopcc = new string[8] {
				"lt",
				"le",
				"gt",
				"ge",
				"eq",
				"neq",
				"false",
				"true",
			};
			vpcomb_pseudo_ops = Create(xopcc, 8, "vpcom", "b");
			vpcomw_pseudo_ops = Create(xopcc, 8, "vpcom", "w");
			vpcomd_pseudo_ops = Create(xopcc, 8, "vpcom", "d");
			vpcomq_pseudo_ops = Create(xopcc, 8, "vpcom", "q");
			vpcomub_pseudo_ops = Create(xopcc, 8, "vpcom", "ub");
			vpcomuw_pseudo_ops = Create(xopcc, 8, "vpcom", "uw");
			vpcomud_pseudo_ops = Create(xopcc, 8, "vpcom", "ud");
			vpcomuq_pseudo_ops = Create(xopcc, 8, "vpcom", "uq");
		}

		static string[] Create(string[] cc, int size, string prefix, string suffix) {
			var strings = new string[size];
			for (int i = 0; i < strings.Length; i++)
				strings[i] = prefix + cc[i] + suffix;
			return strings;
		}

		static readonly string[] cmpps_pseudo_ops;
		static readonly string[] vcmpps_pseudo_ops;
		static readonly string[] cmppd_pseudo_ops;
		static readonly string[] vcmppd_pseudo_ops;
		static readonly string[] cmpss_pseudo_ops;
		static readonly string[] vcmpss_pseudo_ops;
		static readonly string[] cmpsd_pseudo_ops;
		static readonly string[] vcmpsd_pseudo_ops;

		static readonly string[] pclmulqdq_pseudo_ops = new string[4] {
			"pclmullqlqdq",
			"pclmulhqlqdq",
			"pclmullqhqdq",
			"pclmulhqhqdq",
		};

		static readonly string[] vpclmulqdq_pseudo_ops = new string[4] {
			"vpclmullqlqdq",
			"vpclmulhqlqdq",
			"vpclmullqhqdq",
			"vpclmulhqhqdq",
		};

		static readonly string[] vpcomb_pseudo_ops;
		static readonly string[] vpcomw_pseudo_ops;
		static readonly string[] vpcomd_pseudo_ops;
		static readonly string[] vpcomq_pseudo_ops;
		static readonly string[] vpcomub_pseudo_ops;
		static readonly string[] vpcomuw_pseudo_ops;
		static readonly string[] vpcomud_pseudo_ops;
		static readonly string[] vpcomuq_pseudo_ops;
	}
}
