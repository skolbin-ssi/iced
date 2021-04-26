// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

// Keep this file in sync with pseudo_ops_fast.rs

use crate::formatter::enums_shared::PseudoOpsKind;
use crate::formatter::FormatterString;
use alloc::string::String;
use alloc::vec::Vec;
use lazy_static::lazy_static;

pub(super) fn get_pseudo_ops(kind: PseudoOpsKind) -> &'static Vec<FormatterString> {
	let pseudo_ops = &*PSEUDO_OPS;
	match kind {
		PseudoOpsKind::cmpps => &pseudo_ops.cmpps,
		PseudoOpsKind::vcmpps => &pseudo_ops.vcmpps,
		PseudoOpsKind::cmppd => &pseudo_ops.cmppd,
		PseudoOpsKind::vcmppd => &pseudo_ops.vcmppd,
		PseudoOpsKind::cmpss => &pseudo_ops.cmpss,
		PseudoOpsKind::vcmpss => &pseudo_ops.vcmpss,
		PseudoOpsKind::cmpsd => &pseudo_ops.cmpsd,
		PseudoOpsKind::vcmpsd => &pseudo_ops.vcmpsd,
		PseudoOpsKind::pclmulqdq => &pseudo_ops.pclmulqdq,
		PseudoOpsKind::vpclmulqdq => &pseudo_ops.vpclmulqdq,
		PseudoOpsKind::vpcomb => &pseudo_ops.vpcomb,
		PseudoOpsKind::vpcomw => &pseudo_ops.vpcomw,
		PseudoOpsKind::vpcomd => &pseudo_ops.vpcomd,
		PseudoOpsKind::vpcomq => &pseudo_ops.vpcomq,
		PseudoOpsKind::vpcomub => &pseudo_ops.vpcomub,
		PseudoOpsKind::vpcomuw => &pseudo_ops.vpcomuw,
		PseudoOpsKind::vpcomud => &pseudo_ops.vpcomud,
		PseudoOpsKind::vpcomuq => &pseudo_ops.vpcomuq,
	}
}

struct PseudoOps {
	cmpps: Vec<FormatterString>,
	vcmpps: Vec<FormatterString>,
	cmppd: Vec<FormatterString>,
	vcmppd: Vec<FormatterString>,
	cmpss: Vec<FormatterString>,
	vcmpss: Vec<FormatterString>,
	cmpsd: Vec<FormatterString>,
	vcmpsd: Vec<FormatterString>,
	pclmulqdq: Vec<FormatterString>,
	vpclmulqdq: Vec<FormatterString>,
	vpcomb: Vec<FormatterString>,
	vpcomw: Vec<FormatterString>,
	vpcomd: Vec<FormatterString>,
	vpcomq: Vec<FormatterString>,
	vpcomub: Vec<FormatterString>,
	vpcomuw: Vec<FormatterString>,
	vpcomud: Vec<FormatterString>,
	vpcomuq: Vec<FormatterString>,
}

lazy_static! {
	static ref PSEUDO_OPS: PseudoOps = {
		const CAP: usize = 14;
		let mut sb = String::with_capacity(CAP);
		#[rustfmt::skip]
		let cc: [&'static str; 32] = [
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
		];
		let cmpps = create(&mut sb, &cc, 8, "cmp", "ps");
		let vcmpps = create(&mut sb, &cc, 32, "vcmp", "ps");
		let cmppd = create(&mut sb, &cc, 8, "cmp", "pd");
		let vcmppd = create(&mut sb, &cc, 32, "vcmp", "pd");
		let cmpss = create(&mut sb, &cc, 8, "cmp", "ss");
		let vcmpss = create(&mut sb, &cc, 32, "vcmp", "ss");
		let cmpsd = create(&mut sb, &cc, 8, "cmp", "sd");
		let vcmpsd = create(&mut sb, &cc, 32, "vcmp", "sd");

		#[rustfmt::skip]
		let xopcc: [&'static str; 8] = [
			"lt",
			"le",
			"gt",
			"ge",
			"eq",
			"neq",
			"false",
			"true",
		];
		let vpcomb = create(&mut sb, &xopcc, 8, "vpcom", "b");
		let vpcomw = create(&mut sb, &xopcc, 8, "vpcom", "w");
		let vpcomd = create(&mut sb, &xopcc, 8, "vpcom", "d");
		let vpcomq = create(&mut sb, &xopcc, 8, "vpcom", "q");
		let vpcomub = create(&mut sb, &xopcc, 8, "vpcom", "ub");
		let vpcomuw = create(&mut sb, &xopcc, 8, "vpcom", "uw");
		let vpcomud = create(&mut sb, &xopcc, 8, "vpcom", "ud");
		let vpcomuq = create(&mut sb, &xopcc, 8, "vpcom", "uq");

		#[rustfmt::skip]
		let pclmulqdq = vec![
			FormatterString::new_str("pclmullqlqdq"),
			FormatterString::new_str("pclmulhqlqdq"),
			FormatterString::new_str("pclmullqhqdq"),
			FormatterString::new_str("pclmulhqhqdq"),
		];
		#[rustfmt::skip]
		let vpclmulqdq = vec![
			FormatterString::new_str("vpclmullqlqdq"),
			FormatterString::new_str("vpclmulhqlqdq"),
			FormatterString::new_str("vpclmullqhqdq"),
			FormatterString::new_str("vpclmulhqhqdq"),
		];

		debug_assert_eq!(sb.capacity(), CAP);
		PseudoOps {
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
	};
}

fn create(sb: &mut String, cc: &[&str], size: usize, prefix: &str, suffix: &str) -> Vec<FormatterString> {
	let mut strings = Vec::with_capacity(size);
	for &cc_s in cc {
		if strings.len() == size {
			break;
		}
		sb.clear();
		sb.push_str(prefix);
		sb.push_str(cc_s);
		sb.push_str(suffix);
		strings.push(FormatterString::new(sb.clone()));
	}
	strings
}
