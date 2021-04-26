// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

using System;
using Generator.Enums;

namespace Generator {
	static class RustConstants {
		public const string AttributeNoRustFmt = "#[rustfmt::skip]";
		public const string AttributeCopyClone = "#[derive(Copy, Clone)]";
		public const string AttributeCopyEq = "#[derive(Copy, Clone, Eq, PartialEq)]";
		public const string AttributeCopyEqOrdHash = "#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]";
		public const string AttributeAllowNonCamelCaseTypes = "#[allow(non_camel_case_types)]";
		public const string AttributeMustUse = "#[must_use]";
		public const string AttributeNonExhaustive = "#[cfg_attr(not(feature = \"exhaustive_enums\"), non_exhaustive)]";
		public const string AttributeInline = "#[inline]";
		public const string AttributeAllowMissingDocs = "#[allow(missing_docs)]";
		public const string AttributeAllowMissingCopyImplementations = "#[allow(missing_copy_implementations)]";
		public const string AttributeAllowMissingDebugImplementations = "#[allow(missing_debug_implementations)]";
		public const string AttributeAllowMissingInlineInPublicItems = "#[allow(clippy::missing_inline_in_public_items)]";
		public const string AttributeAllowTrivialCasts = "#[allow(trivial_casts)]";
		public const string AttributeAllowDeadCode = "#[allow(dead_code)]";
		public const string AttributeWasmBindgen = "#[wasm_bindgen]";
		public const string AttributeWasmBindgenJsName = "#[wasm_bindgen(js_name = \"{0}\")]";
		public const string AttributeAllowNonSnakeCase = "#[allow(non_snake_case)]";
		public const string AttributeAllowUnwrapUsed = "#[allow(clippy::unwrap_used)]";

		public const string FeaturePrefix = "#[cfg(";
		public const string FeatureInstrInfo = "#[cfg(feature = \"instr_info\")]";
		public const string FeatureEncoder = "#[cfg(feature = \"encoder\")]";
		public const string FeatureOpCodeInfo = "#[cfg(all(feature = \"encoder\", feature = \"op_code_info\"))]";
		public const string Vex = "not(feature = \"no_vex\")";
		public const string Evex = "not(feature = \"no_evex\")";
		public const string Xop = "not(feature = \"no_xop\")";
		public const string FeatureVex = "#[cfg(not(feature = \"no_vex\"))]";
		public const string FeatureXop = "#[cfg(not(feature = \"no_xop\"))]";
		public const string FeatureVexOrXop = "#[cfg(any(not(feature = \"no_vex\"), not(feature = \"no_xop\")))]";
		public const string FeatureVexOrXopOrEvex = "#[cfg(any(not(feature = \"no_vex\"), not(feature = \"no_xop\"), not(feature = \"no_evex\")))]";
		public const string FeatureEvex = "#[cfg(not(feature = \"no_evex\"))]";
		public const string FeatureD3now = "#[cfg(not(feature = \"no_d3now\"))]";
		public const string FeatureEncodingOne = "#[cfg({0})]";
		public const string FeatureEncodingMany = "#[cfg(any({0}))]";
		public const string FeatureDecoder = "#[cfg(feature = \"decoder\")]";
		public const string FeatureDecoderOrEncoder = "#[cfg(any(feature = \"decoder\", feature = \"encoder\"))]";
		public const string FeatureDecoderOrEncoderOrInstrInfo = "#[cfg(any(feature = \"decoder\", feature = \"encoder\", feature = \"instr_info\"))]";
		public const string FeatureBigInt = "#[cfg(feature = \"bigint\")]";
		public const string FeatureNotBigInt = "#[cfg(not(feature = \"bigint\"))]";
		public const string FeatureGasIntelNasm = "#[cfg(any(feature = \"gas\", feature = \"intel\", feature = \"nasm\"))]";

		public static string? GetFeature(EncodingKind encoding) =>
			encoding switch {
				EncodingKind.Legacy => null,
				EncodingKind.VEX => FeatureVex,
				EncodingKind.EVEX => FeatureEvex,
				EncodingKind.XOP => FeatureXop,
				EncodingKind.D3NOW => FeatureD3now,
				_ => throw new InvalidOperationException(),
			};
	}
}
