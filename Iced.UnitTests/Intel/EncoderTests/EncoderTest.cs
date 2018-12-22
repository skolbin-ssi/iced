﻿/*
    Copyright (C) 2018 de4dot@gmail.com

    This file is part of Iced.

    Iced is free software: you can redistribute it and/or modify
    it under the terms of the GNU Lesser General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Iced is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License
    along with Iced.  If not, see <https://www.gnu.org/licenses/>.
*/

#if !NO_ENCODER
using System;
using System.Collections.Generic;
using Iced.Intel;
using Iced.UnitTests.Intel.DecoderTests;
using Xunit;

namespace Iced.UnitTests.Intel.EncoderTests {
	public abstract class EncoderTest {
		protected void EncodeBase(int codeSize, Code code, string hexBytes, DecoderOptions options) {
			var origBytes = HexUtils.ToByteArray(hexBytes);
			var decoder = CreateDecoder(codeSize, origBytes, options);
			var origRip = decoder.InstructionPointer;
			var origInstr = decoder.Decode();
			var origConstantOffsets = decoder.GetConstantOffsets(ref origInstr);
			Assert.Equal(code, origInstr.Code);
			Assert.Equal(origBytes.Length, origInstr.ByteLength);
			Assert.True(origInstr.ByteLength <= Iced.Intel.DecoderConstants.MaxInstructionLength);
			Assert.Equal((ushort)origRip, origInstr.IP16);
			Assert.Equal((uint)origRip, origInstr.IP32);
			Assert.Equal(origRip, origInstr.IP64);
			var afterRip = decoder.InstructionPointer;
			Assert.Equal((ushort)afterRip, origInstr.NextIP16);
			Assert.Equal((uint)afterRip, origInstr.NextIP32);
			Assert.Equal(afterRip, origInstr.NextIP64);

			var writer = new CodeWriterImpl();
			var encoder = decoder.CreateEncoder(writer);
			Assert.Equal(codeSize, encoder.Bitness);
			var origInstrCopy = origInstr;
			bool result = encoder.TryEncode(ref origInstr, origRip, out uint encodedInstrLen, out string errorMessage);
			Assert.True(errorMessage == null, "Unexpected ErrorMessage: " + errorMessage);
			Assert.True(result, "Error, result from Encoder.TryEncode must be true");
			var encodedConstantOffsets = encoder.GetConstantOffsets();
			FixConstantOffsets(ref encodedConstantOffsets, origInstr.ByteLength, (int)encodedInstrLen);
			Assert.True(Equals(ref origConstantOffsets, ref encodedConstantOffsets));
			var encodedBytes = writer.ToArray();
			Assert.Equal(encodedBytes.Length, (int)encodedInstrLen);
			Assert.True(Instruction.TEST_BitByBitEquals(origInstr, origInstrCopy), "Instruction are differing: " + Instruction.TEST_DumpDiff(origInstr, origInstrCopy));

			// We don't compare the generated bytes, bit by bit, for many reasons. Eg. prefixes
			// can be put in any order. Unused prefixes. [xxxx] can be encoded w/ and w/o a sib
			// byte in 32-bit mode. The decoder-tests test ignored bits by setting them to 1, but
			// the encoder always writes 0.
			// The instruction is decoded again and then it's compared against the old one to
			// make sure it matches exactly, bit by bit.

			var newInstr = CreateDecoder(codeSize, encodedBytes, options).Decode();
			Assert.Equal(code, newInstr.Code);
			Assert.Equal(encodedBytes.Length, newInstr.ByteLength);
			newInstr.ByteLength = origInstr.ByteLength;
			newInstr.NextIP64 = origInstr.NextIP64;
			Assert.True(Instruction.TEST_BitByBitEquals(origInstr, newInstr), "Instruction are differing: " + Instruction.TEST_DumpDiff(origInstr, newInstr));
			// Some tests use useless or extra prefixes, so we can't verify the exact length
			Assert.True(encodedBytes.Length <= origBytes.Length, "Unexpected encoded prefixes: " + HexUtils.ToString(encodedBytes));
		}

		static void FixConstantOffsets(ref ConstantOffsets ca, int origInstrLen, int newInstrLen) {
			byte diff = (byte)(origInstrLen - newInstrLen);
			if (ca.HasDisplacement)
				ca.DisplacementOffset += diff;
			if (ca.HasImmediate)
				ca.ImmediateOffset += diff;
			if (ca.HasImmediate2)
				ca.ImmediateOffset2 += diff;
		}

		static bool Equals(ref ConstantOffsets a, ref ConstantOffsets b) =>
			a.DisplacementOffset == b.DisplacementOffset &&
			a.ImmediateOffset == b.ImmediateOffset &&
			a.ImmediateOffset2 == b.ImmediateOffset2 &&
			a.DisplacementSize == b.DisplacementSize &&
			a.ImmediateSize == b.ImmediateSize &&
			a.ImmediateSize2 == b.ImmediateSize2;

		protected void NonDecodeEncodeBase(int codeSize, ref Instruction instr, string hexBytes, ulong rip) {
			var expectedBytes = HexUtils.ToByteArray(hexBytes);
			var writer = new CodeWriterImpl();
			var encoder = Encoder.Create(codeSize, writer);
			Assert.Equal(codeSize, encoder.Bitness);
			var origInstrCopy = instr;
			bool result = encoder.TryEncode(ref instr, rip, out uint encodedInstrLen, out string errorMessage);
			Assert.True(errorMessage == null, "Unexpected ErrorMessage: " + errorMessage);
			Assert.True(result, "Error, result from Encoder.TryEncode must be true");
			var encodedBytes = writer.ToArray();
			Assert.Equal(encodedBytes.Length, (int)encodedInstrLen);
			Assert.True(Instruction.TEST_BitByBitEquals(instr, origInstrCopy), "Instruction are differing: " + Instruction.TEST_DumpDiff(instr, origInstrCopy));
#pragma warning disable xUnit2006 // Do not use invalid string equality check
			// Show the full string without ellipses by using Equal<string>() instead of Equal()
			Assert.Equal<string>(HexUtils.ToString(expectedBytes), HexUtils.ToString(encodedBytes));
#pragma warning restore xUnit2006 // Do not use invalid string equality check
		}

		protected void EncodeInvalidBase(int codeSize, Code code, string hexBytes, DecoderOptions options, int invalidCodeSize) {
			var origBytes = HexUtils.ToByteArray(hexBytes);
			var decoder = CreateDecoder(codeSize, origBytes, options);
			var origRip = decoder.InstructionPointer;
			var origInstr = decoder.Decode();
			Assert.Equal(code, origInstr.Code);
			Assert.Equal(origBytes.Length, origInstr.ByteLength);
			Assert.True(origInstr.ByteLength <= Iced.Intel.DecoderConstants.MaxInstructionLength);
			Assert.Equal((ushort)origRip, origInstr.IP16);
			Assert.Equal((uint)origRip, origInstr.IP32);
			Assert.Equal(origRip, origInstr.IP64);
			var afterRip = decoder.InstructionPointer;
			Assert.Equal((ushort)afterRip, origInstr.NextIP16);
			Assert.Equal((uint)afterRip, origInstr.NextIP32);
			Assert.Equal(afterRip, origInstr.NextIP64);

			var writer = new CodeWriterImpl();
			var encoder = CreateEncoder(invalidCodeSize, writer);
			var origInstrCopy = origInstr;
			bool result = encoder.TryEncode(ref origInstr, origRip, out uint encodedInstrLen, out string errorMessage);
			Assert.Equal(invalidCodeSize == 64 ? Encoder.ERROR_ONLY_1632_BIT_MODE : Encoder.ERROR_ONLY_64_BIT_MODE, errorMessage);
			Assert.False(result);
			Assert.True(Instruction.TEST_BitByBitEquals(origInstr, origInstrCopy), "Instruction are differing: " + Instruction.TEST_DumpDiff(origInstr, origInstrCopy));
		}

		Encoder CreateEncoder(int codeSize, CodeWriter writer) {
			var encoder = Encoder.Create(codeSize, writer);
			Assert.Equal(codeSize, encoder.Bitness);
			return encoder;
		}

		Decoder CreateDecoder(int codeSize, byte[] hexBytes, DecoderOptions options) {
			Decoder decoder;
			var codeReader = new ByteArrayCodeReader(hexBytes);
			switch (codeSize) {
			case 16:
				decoder = Decoder.Create16(codeReader, options);
				decoder.InstructionPointer = DecoderConstants.DEFAULT_IP16;
				break;

			case 32:
				decoder = Decoder.Create32(codeReader, options);
				decoder.InstructionPointer = DecoderConstants.DEFAULT_IP32;
				break;

			case 64:
				decoder = Decoder.Create64(codeReader, options);
				decoder.InstructionPointer = DecoderConstants.DEFAULT_IP64;
				break;

			default:
				throw new ArgumentOutOfRangeException(nameof(codeSize));
			}

			Assert.Equal(codeSize, decoder.Bitness);
			return decoder;
		}

		protected static IEnumerable<object[]> GetEncodeData(int codeSize) {
			foreach (var info in DecoderTestUtils.GetDecoderTests(needHexBytes: true, includeOtherTests: true)) {
				if (codeSize != info.Bitness)
					continue;
				yield return new object[] { info.Bitness, info.Code, info.HexBytes, info.Options };
			}
		}

		protected static IEnumerable<object[]> GetNonDecodedEncodeData(int codeSize) {
			foreach (var info in NonDecodedInstructions.GetTests()) {
				if (codeSize != info.bitness)
					continue;
				ulong rip = 0;
				yield return new object[] { info.bitness, info.instruction, info.hexBytes, rip };
			}
		}
	}
}
#endif
