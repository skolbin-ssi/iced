// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

using System;
using System.Collections.Generic;
using System.Diagnostics.CodeAnalysis;
using System.Linq;
using System.Text;
using Generator.Constants;
using Generator.Enums;
using Generator.Tables;

namespace Generator.Encoder {
	enum InstructionOperand {
		Register,
		Memory,
		Imm32,
		Imm64,
	}

	sealed class InstructionGroup {
		public List<InstructionDef> Defs { get; }
		public InstructionOperand[] Operands { get; }
		public InstructionGroup(InstructionOperand[] operands) {
			Operands = operands;
			Defs = new List<InstructionDef>();
		}
		public override string ToString() {
			var sb = new StringBuilder();
			sb.Append(Defs[0].Code.RawName);
			sb.Append(" - (");
			for (int i = 0; i < Operands.Length; i++) {
				if (i > 0)
					sb.Append(", ");
				sb.Append(Operands[i].ToString());
			}
			sb.Append(')');
			return sb.ToString();
		}
	}

	sealed class InstructionGroups {
		readonly GenTypes genTypes;
		readonly HashSet<EnumValue> ignoredCodes;

		public InstructionGroups(GenTypes genTypes) {
			this.genTypes = genTypes;
			ignoredCodes = new HashSet<EnumValue>();

			foreach (var def in genTypes.GetObject<InstructionDefs>(TypeIds.InstructionDefs).Defs) {
				if ((def.Flags1 & InstructionDefFlags1.NoInstruction) != 0)
					ignoredCodes.Add(def.Code);
				foreach (var opKind in def.OpKindDefs) {
					switch (opKind.OperandEncoding) {
					case OperandEncoding.None:
					case OperandEncoding.NearBranch:
					case OperandEncoding.Xbegin:
					case OperandEncoding.AbsNearBranch:
					case OperandEncoding.FarBranch:
					case OperandEncoding.SegRSI:
					case OperandEncoding.SegRDI:
					case OperandEncoding.ESRDI:
						ignoredCodes.Add(def.Code);
						break;

					case OperandEncoding.Immediate:
					case OperandEncoding.ImpliedConst:
					case OperandEncoding.ImpliedRegister:
					case OperandEncoding.SegRBX:
					case OperandEncoding.RegImm:
					case OperandEncoding.RegOpCode:
					case OperandEncoding.RegModrmReg:
					case OperandEncoding.RegModrmRm:
					case OperandEncoding.RegMemModrmRm:
					case OperandEncoding.RegVvvvv:
					case OperandEncoding.MemModrmRm:
					case OperandEncoding.MemOffset:
						break;

					default:
						throw new InvalidOperationException();
					}
				}
			}
		}

		sealed class OpComparer : IEqualityComparer<InstructionOperand[]> {
			public bool Equals([AllowNull] InstructionOperand[] x, [AllowNull] InstructionOperand[] y) {
				if (x!.Length != y!.Length)
					return false;
				for (int i = 0; i < x.Length; i++) {
					if (x[i] != y[i])
						return false;
				}
				return true;
			}

			public int GetHashCode([DisallowNull] InstructionOperand[] obj) {
				int hc = 0;
				foreach (var o in obj)
					hc = HashCode.Combine(hc, o);
				return hc;
			}
		}

		public InstructionGroup[] GetGroups() {
			var groups = new Dictionary<InstructionOperand[], InstructionGroup>(new OpComparer());

			foreach (var def in genTypes.GetObject<InstructionDefs>(TypeIds.InstructionDefs).Defs) {
				if (ignoredCodes.Contains(def.Code))
					continue;

				foreach (var ops in GetOperands(def.OpKindDefs)) {
					if (!groups.TryGetValue(ops, out var group))
						groups.Add(ops, group = new InstructionGroup(ops));
					group.Defs.Add(def);
				}
			}

			var result = groups.Values.ToArray();
			Array.Sort(result, (a, b) => {
				int c = a.Operands.Length - b.Operands.Length;
				if (c != 0)
					return c;
				for (int i = 0; i < a.Operands.Length; i++) {
					c = GetOrder(a.Operands[i]) - GetOrder(b.Operands[i]);
					if (c != 0)
						return c;
				}
				return 0;
			});
			return result;

			static int GetOrder(InstructionOperand op) =>
				op switch {
					InstructionOperand.Register => 0,
					InstructionOperand.Imm32 => 1,
					InstructionOperand.Imm64 => 2,
					InstructionOperand.Memory => 3,
					_ => throw new InvalidOperationException(),
				};
		}

		static IEnumerable<InstructionOperand[]> GetOperands(OpCodeOperandKindDef[] opKinds) {
			if (opKinds.Length == 0) {
				yield return Array.Empty<InstructionOperand>();
				yield break;
			}
			if (IcedConstants.MaxOpCount != 5)
				throw new InvalidOperationException();
			var ops = new InstructionOperand[IcedConstants.MaxOpCount][];
			for (int i = 0; i < ops.Length; i++)
				ops[i] = Array.Empty<InstructionOperand>();
			for (int i = 0; i < opKinds.Length; i++)
				ops[i] = GetOperand(opKinds[i]);
			foreach (var o0 in ops[0]) {
				if (opKinds.Length == 1)
					yield return new[] { o0 };
				else {
					foreach (var o1 in ops[1]) {
						if (opKinds.Length == 2)
							yield return new[] { o0, o1 };
						else {
							foreach (var o2 in ops[2]) {
								if (opKinds.Length == 3)
									yield return new[] { o0, o1, o2 };
								else {
									foreach (var o3 in ops[3]) {
										if (opKinds.Length == 4)
											yield return new[] { o0, o1, o2, o3 };
										else {
											foreach (var o4 in ops[4]) {
												if (opKinds.Length == 5)
													yield return new[] { o0, o1, o2, o3, o4 };
												else
													throw new InvalidOperationException();
											}
										}
									}
								}
							}
						}
					}
				}
			}
		}

		static InstructionOperand[] GetOperand(OpCodeOperandKindDef def) {
			switch (def.OperandEncoding) {
			case OperandEncoding.Immediate:
				if (def.ImmediateSize == 64)
					return new[] { InstructionOperand.Imm64 };
				return new[] { InstructionOperand.Imm32 };

			case OperandEncoding.ImpliedConst:
				return new[] { InstructionOperand.Imm32 };

			case OperandEncoding.ImpliedRegister:
			case OperandEncoding.RegImm:
			case OperandEncoding.RegOpCode:
			case OperandEncoding.RegModrmReg:
			case OperandEncoding.RegModrmRm:
			case OperandEncoding.RegVvvvv:
				return new[] { InstructionOperand.Register };

			case OperandEncoding.RegMemModrmRm:
				return new[] { InstructionOperand.Register, InstructionOperand.Memory };

			case OperandEncoding.SegRBX:
			case OperandEncoding.MemModrmRm:
			case OperandEncoding.MemOffset:
				return new[] { InstructionOperand.Memory };

			case OperandEncoding.None:
			case OperandEncoding.NearBranch:
			case OperandEncoding.Xbegin:
			case OperandEncoding.AbsNearBranch:
			case OperandEncoding.FarBranch:
			case OperandEncoding.SegRSI:
			case OperandEncoding.SegRDI:
			case OperandEncoding.ESRDI:
			default:
				throw new InvalidOperationException();
			}
		}
	}
}
