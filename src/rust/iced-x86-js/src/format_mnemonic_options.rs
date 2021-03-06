// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

use wasm_bindgen::prelude::*;

// GENERATOR-BEGIN: Enum
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Format mnemonic options
#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum FormatMnemonicOptions {
	/// No option is set
	None = 0x0000_0000,
	/// Don't add any prefixes
	NoPrefixes = 0x0000_0001,
	/// Don't add the mnemonic
	NoMnemonic = 0x0000_0002,
}
// GENERATOR-END: Enum
