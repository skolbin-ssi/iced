#
# Copyright (C) 2018-2019 de4dot@gmail.com
#
# Permission is hereby granted, free of charge, to any person obtaining
# a copy of this software and associated documentation files (the
# "Software"), to deal in the Software without restriction, including
# without limitation the rights to use, copy, modify, merge, publish,
# distribute, sublicense, and/or sell copies of the Software, and to
# permit persons to whom the Software is furnished to do so, subject to
# the following conditions:
#
# The above copyright notice and this permission notice shall be
# included in all copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
# EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
# MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
# IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
# CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
# TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
# SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
#

# ⚠️This file was generated by GENERATOR!🦹‍♂️

# pylint: disable=invalid-name
# pylint: disable=line-too-long
# pylint: disable=redefined-builtin
# pylint: disable=too-many-lines

"""
A register
"""

from typing import List

NONE: int = 0
"""
<no docs>
"""
AL: int = 1
"""
<no docs>
"""
CL: int = 2
"""
<no docs>
"""
DL: int = 3
"""
<no docs>
"""
BL: int = 4
"""
<no docs>
"""
AH: int = 5
"""
<no docs>
"""
CH: int = 6
"""
<no docs>
"""
DH: int = 7
"""
<no docs>
"""
BH: int = 8
"""
<no docs>
"""
SPL: int = 9
"""
<no docs>
"""
BPL: int = 10
"""
<no docs>
"""
SIL: int = 11
"""
<no docs>
"""
DIL: int = 12
"""
<no docs>
"""
R8L: int = 13
"""
<no docs>
"""
R9L: int = 14
"""
<no docs>
"""
R10L: int = 15
"""
<no docs>
"""
R11L: int = 16
"""
<no docs>
"""
R12L: int = 17
"""
<no docs>
"""
R13L: int = 18
"""
<no docs>
"""
R14L: int = 19
"""
<no docs>
"""
R15L: int = 20
"""
<no docs>
"""
AX: int = 21
"""
<no docs>
"""
CX: int = 22
"""
<no docs>
"""
DX: int = 23
"""
<no docs>
"""
BX: int = 24
"""
<no docs>
"""
SP: int = 25
"""
<no docs>
"""
BP: int = 26
"""
<no docs>
"""
SI: int = 27
"""
<no docs>
"""
DI: int = 28
"""
<no docs>
"""
R8W: int = 29
"""
<no docs>
"""
R9W: int = 30
"""
<no docs>
"""
R10W: int = 31
"""
<no docs>
"""
R11W: int = 32
"""
<no docs>
"""
R12W: int = 33
"""
<no docs>
"""
R13W: int = 34
"""
<no docs>
"""
R14W: int = 35
"""
<no docs>
"""
R15W: int = 36
"""
<no docs>
"""
EAX: int = 37
"""
<no docs>
"""
ECX: int = 38
"""
<no docs>
"""
EDX: int = 39
"""
<no docs>
"""
EBX: int = 40
"""
<no docs>
"""
ESP: int = 41
"""
<no docs>
"""
EBP: int = 42
"""
<no docs>
"""
ESI: int = 43
"""
<no docs>
"""
EDI: int = 44
"""
<no docs>
"""
R8D: int = 45
"""
<no docs>
"""
R9D: int = 46
"""
<no docs>
"""
R10D: int = 47
"""
<no docs>
"""
R11D: int = 48
"""
<no docs>
"""
R12D: int = 49
"""
<no docs>
"""
R13D: int = 50
"""
<no docs>
"""
R14D: int = 51
"""
<no docs>
"""
R15D: int = 52
"""
<no docs>
"""
RAX: int = 53
"""
<no docs>
"""
RCX: int = 54
"""
<no docs>
"""
RDX: int = 55
"""
<no docs>
"""
RBX: int = 56
"""
<no docs>
"""
RSP: int = 57
"""
<no docs>
"""
RBP: int = 58
"""
<no docs>
"""
RSI: int = 59
"""
<no docs>
"""
RDI: int = 60
"""
<no docs>
"""
R8: int = 61
"""
<no docs>
"""
R9: int = 62
"""
<no docs>
"""
R10: int = 63
"""
<no docs>
"""
R11: int = 64
"""
<no docs>
"""
R12: int = 65
"""
<no docs>
"""
R13: int = 66
"""
<no docs>
"""
R14: int = 67
"""
<no docs>
"""
R15: int = 68
"""
<no docs>
"""
EIP: int = 69
"""
<no docs>
"""
RIP: int = 70
"""
<no docs>
"""
ES: int = 71
"""
<no docs>
"""
CS: int = 72
"""
<no docs>
"""
SS: int = 73
"""
<no docs>
"""
DS: int = 74
"""
<no docs>
"""
FS: int = 75
"""
<no docs>
"""
GS: int = 76
"""
<no docs>
"""
XMM0: int = 77
"""
<no docs>
"""
XMM1: int = 78
"""
<no docs>
"""
XMM2: int = 79
"""
<no docs>
"""
XMM3: int = 80
"""
<no docs>
"""
XMM4: int = 81
"""
<no docs>
"""
XMM5: int = 82
"""
<no docs>
"""
XMM6: int = 83
"""
<no docs>
"""
XMM7: int = 84
"""
<no docs>
"""
XMM8: int = 85
"""
<no docs>
"""
XMM9: int = 86
"""
<no docs>
"""
XMM10: int = 87
"""
<no docs>
"""
XMM11: int = 88
"""
<no docs>
"""
XMM12: int = 89
"""
<no docs>
"""
XMM13: int = 90
"""
<no docs>
"""
XMM14: int = 91
"""
<no docs>
"""
XMM15: int = 92
"""
<no docs>
"""
XMM16: int = 93
"""
<no docs>
"""
XMM17: int = 94
"""
<no docs>
"""
XMM18: int = 95
"""
<no docs>
"""
XMM19: int = 96
"""
<no docs>
"""
XMM20: int = 97
"""
<no docs>
"""
XMM21: int = 98
"""
<no docs>
"""
XMM22: int = 99
"""
<no docs>
"""
XMM23: int = 100
"""
<no docs>
"""
XMM24: int = 101
"""
<no docs>
"""
XMM25: int = 102
"""
<no docs>
"""
XMM26: int = 103
"""
<no docs>
"""
XMM27: int = 104
"""
<no docs>
"""
XMM28: int = 105
"""
<no docs>
"""
XMM29: int = 106
"""
<no docs>
"""
XMM30: int = 107
"""
<no docs>
"""
XMM31: int = 108
"""
<no docs>
"""
YMM0: int = 109
"""
<no docs>
"""
YMM1: int = 110
"""
<no docs>
"""
YMM2: int = 111
"""
<no docs>
"""
YMM3: int = 112
"""
<no docs>
"""
YMM4: int = 113
"""
<no docs>
"""
YMM5: int = 114
"""
<no docs>
"""
YMM6: int = 115
"""
<no docs>
"""
YMM7: int = 116
"""
<no docs>
"""
YMM8: int = 117
"""
<no docs>
"""
YMM9: int = 118
"""
<no docs>
"""
YMM10: int = 119
"""
<no docs>
"""
YMM11: int = 120
"""
<no docs>
"""
YMM12: int = 121
"""
<no docs>
"""
YMM13: int = 122
"""
<no docs>
"""
YMM14: int = 123
"""
<no docs>
"""
YMM15: int = 124
"""
<no docs>
"""
YMM16: int = 125
"""
<no docs>
"""
YMM17: int = 126
"""
<no docs>
"""
YMM18: int = 127
"""
<no docs>
"""
YMM19: int = 128
"""
<no docs>
"""
YMM20: int = 129
"""
<no docs>
"""
YMM21: int = 130
"""
<no docs>
"""
YMM22: int = 131
"""
<no docs>
"""
YMM23: int = 132
"""
<no docs>
"""
YMM24: int = 133
"""
<no docs>
"""
YMM25: int = 134
"""
<no docs>
"""
YMM26: int = 135
"""
<no docs>
"""
YMM27: int = 136
"""
<no docs>
"""
YMM28: int = 137
"""
<no docs>
"""
YMM29: int = 138
"""
<no docs>
"""
YMM30: int = 139
"""
<no docs>
"""
YMM31: int = 140
"""
<no docs>
"""
ZMM0: int = 141
"""
<no docs>
"""
ZMM1: int = 142
"""
<no docs>
"""
ZMM2: int = 143
"""
<no docs>
"""
ZMM3: int = 144
"""
<no docs>
"""
ZMM4: int = 145
"""
<no docs>
"""
ZMM5: int = 146
"""
<no docs>
"""
ZMM6: int = 147
"""
<no docs>
"""
ZMM7: int = 148
"""
<no docs>
"""
ZMM8: int = 149
"""
<no docs>
"""
ZMM9: int = 150
"""
<no docs>
"""
ZMM10: int = 151
"""
<no docs>
"""
ZMM11: int = 152
"""
<no docs>
"""
ZMM12: int = 153
"""
<no docs>
"""
ZMM13: int = 154
"""
<no docs>
"""
ZMM14: int = 155
"""
<no docs>
"""
ZMM15: int = 156
"""
<no docs>
"""
ZMM16: int = 157
"""
<no docs>
"""
ZMM17: int = 158
"""
<no docs>
"""
ZMM18: int = 159
"""
<no docs>
"""
ZMM19: int = 160
"""
<no docs>
"""
ZMM20: int = 161
"""
<no docs>
"""
ZMM21: int = 162
"""
<no docs>
"""
ZMM22: int = 163
"""
<no docs>
"""
ZMM23: int = 164
"""
<no docs>
"""
ZMM24: int = 165
"""
<no docs>
"""
ZMM25: int = 166
"""
<no docs>
"""
ZMM26: int = 167
"""
<no docs>
"""
ZMM27: int = 168
"""
<no docs>
"""
ZMM28: int = 169
"""
<no docs>
"""
ZMM29: int = 170
"""
<no docs>
"""
ZMM30: int = 171
"""
<no docs>
"""
ZMM31: int = 172
"""
<no docs>
"""
K0: int = 173
"""
<no docs>
"""
K1: int = 174
"""
<no docs>
"""
K2: int = 175
"""
<no docs>
"""
K3: int = 176
"""
<no docs>
"""
K4: int = 177
"""
<no docs>
"""
K5: int = 178
"""
<no docs>
"""
K6: int = 179
"""
<no docs>
"""
K7: int = 180
"""
<no docs>
"""
BND0: int = 181
"""
<no docs>
"""
BND1: int = 182
"""
<no docs>
"""
BND2: int = 183
"""
<no docs>
"""
BND3: int = 184
"""
<no docs>
"""
CR0: int = 185
"""
<no docs>
"""
CR1: int = 186
"""
<no docs>
"""
CR2: int = 187
"""
<no docs>
"""
CR3: int = 188
"""
<no docs>
"""
CR4: int = 189
"""
<no docs>
"""
CR5: int = 190
"""
<no docs>
"""
CR6: int = 191
"""
<no docs>
"""
CR7: int = 192
"""
<no docs>
"""
CR8: int = 193
"""
<no docs>
"""
CR9: int = 194
"""
<no docs>
"""
CR10: int = 195
"""
<no docs>
"""
CR11: int = 196
"""
<no docs>
"""
CR12: int = 197
"""
<no docs>
"""
CR13: int = 198
"""
<no docs>
"""
CR14: int = 199
"""
<no docs>
"""
CR15: int = 200
"""
<no docs>
"""
DR0: int = 201
"""
<no docs>
"""
DR1: int = 202
"""
<no docs>
"""
DR2: int = 203
"""
<no docs>
"""
DR3: int = 204
"""
<no docs>
"""
DR4: int = 205
"""
<no docs>
"""
DR5: int = 206
"""
<no docs>
"""
DR6: int = 207
"""
<no docs>
"""
DR7: int = 208
"""
<no docs>
"""
DR8: int = 209
"""
<no docs>
"""
DR9: int = 210
"""
<no docs>
"""
DR10: int = 211
"""
<no docs>
"""
DR11: int = 212
"""
<no docs>
"""
DR12: int = 213
"""
<no docs>
"""
DR13: int = 214
"""
<no docs>
"""
DR14: int = 215
"""
<no docs>
"""
DR15: int = 216
"""
<no docs>
"""
ST0: int = 217
"""
<no docs>
"""
ST1: int = 218
"""
<no docs>
"""
ST2: int = 219
"""
<no docs>
"""
ST3: int = 220
"""
<no docs>
"""
ST4: int = 221
"""
<no docs>
"""
ST5: int = 222
"""
<no docs>
"""
ST6: int = 223
"""
<no docs>
"""
ST7: int = 224
"""
<no docs>
"""
MM0: int = 225
"""
<no docs>
"""
MM1: int = 226
"""
<no docs>
"""
MM2: int = 227
"""
<no docs>
"""
MM3: int = 228
"""
<no docs>
"""
MM4: int = 229
"""
<no docs>
"""
MM5: int = 230
"""
<no docs>
"""
MM6: int = 231
"""
<no docs>
"""
MM7: int = 232
"""
<no docs>
"""
TR0: int = 233
"""
<no docs>
"""
TR1: int = 234
"""
<no docs>
"""
TR2: int = 235
"""
<no docs>
"""
TR3: int = 236
"""
<no docs>
"""
TR4: int = 237
"""
<no docs>
"""
TR5: int = 238
"""
<no docs>
"""
TR6: int = 239
"""
<no docs>
"""
TR7: int = 240
"""
<no docs>
"""
TMM0: int = 241
"""
<no docs>
"""
TMM1: int = 242
"""
<no docs>
"""
TMM2: int = 243
"""
<no docs>
"""
TMM3: int = 244
"""
<no docs>
"""
TMM4: int = 245
"""
<no docs>
"""
TMM5: int = 246
"""
<no docs>
"""
TMM6: int = 247
"""
<no docs>
"""
TMM7: int = 248
"""
<no docs>
"""

__all__: List[str] = []
