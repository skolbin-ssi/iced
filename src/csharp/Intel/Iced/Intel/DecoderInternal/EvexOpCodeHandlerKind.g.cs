// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

// ⚠️This file was generated by GENERATOR!🦹‍♂️

#nullable enable

#if DECODER && !NO_EVEX
namespace Iced.Intel.DecoderInternal {
	enum EvexOpCodeHandlerKind : byte {
		Invalid,
		Invalid2,
		Dup,
		HandlerReference,
		ArrayReference,
		RM,
		Group,
		W,
		MandatoryPrefix2,
		VectorLength,
		VectorLength_er,
		Ed_V_Ib,
		Ev_VX,
		Ev_VX_Ib,
		Gv_W_er,
		GvM_VX_Ib,
		HkWIb_3,
		HkWIb_3b,
		HWIb,
		KkHW_3,
		KkHW_3b,
		KkHWIb_sae_3,
		KkHWIb_sae_3b,
		KkHWIb_3,
		KkHWIb_3b,
		KkWIb_3,
		KkWIb_3b,
		KP1HW,
		KR,
		MV,
		V_H_Ev_er,
		V_H_Ev_Ib,
		VHM,
		VHW_3,
		VHW_4,
		VHWIb,
		VK,
		Vk_VSIB,
		VkEv_REXW_2,
		VkEv_REXW_3,
		VkHM,
		VkHW_3,
		VkHW_3b,
		VkHW_5,
		VkHW_er_4,
		VkHW_er_4b,
		VkHWIb_3,
		VkHWIb_3b,
		VkHWIb_5,
		VkHWIb_er_4,
		VkHWIb_er_4b,
		VkM,
		VkW_3,
		VkW_3b,
		VkW_4,
		VkW_4b,
		VkW_er_4,
		VkW_er_5,
		VkW_er_6,
		VkWIb_3,
		VkWIb_3b,
		VkWIb_er,
		VM,
		VSIB_k1,
		VSIB_k1_VX,
		VW,
		VW_er,
		VX_Ev,
		WkHV,
		WkV_3,
		WkV_4a,
		WkV_4b,
		WkVIb,
		WkVIb_er,
		WV,
	}
}
#endif
