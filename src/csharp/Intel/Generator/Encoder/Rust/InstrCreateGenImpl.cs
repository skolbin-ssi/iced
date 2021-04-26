// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

using System;
using System.Text;
using Generator.Documentation.Rust;
using Generator.IO;

namespace Generator.Encoder.Rust {
	sealed class GenCreateNameArgs {
#pragma warning disable CS8618 // Non-nullable field is uninitialized. Consider declaring as nullable.
		public string CreatePrefix;
		public string Register;
		public string Memory;
		public string Int32;
		public string UInt32;
		public string Int64;
		public string UInt64;
#pragma warning restore CS8618 // Non-nullable field is uninitialized. Consider declaring as nullable.

		public static readonly GenCreateNameArgs RustNames = new GenCreateNameArgs {
			CreatePrefix = "with",
			Register = "_reg",
			Memory = "_mem",
			Int32 = "_i32",
			UInt32 = "_u32",
			Int64 = "_i64",
			UInt64 = "_u64",
		};
	}

	sealed class InstrCreateGenImpl {
		readonly GenTypes genTypes;
		readonly IdentifierConverter idConverter;
		readonly RustDocCommentWriter docWriter;
		readonly StringBuilder sb;

		public InstrCreateGenImpl(GenTypes genTypes, IdentifierConverter idConverter, RustDocCommentWriter docWriter) {
			this.genTypes = genTypes;
			this.idConverter = idConverter;
			this.docWriter = docWriter;
			sb = new StringBuilder();
		}

		public void WriteDocs(FileWriter writer, CreateMethod method, string sectionTitle, Action? writeSection) {
			const string typeName = "Instruction";
			docWriter.BeginWrite(writer);
			foreach (var doc in method.Docs)
				docWriter.WriteDocLine(writer, doc, typeName);
			docWriter.WriteLine(writer, string.Empty);
			if (writeSection is not null) {
				docWriter.WriteLine(writer, $"# {sectionTitle}");
				docWriter.WriteLine(writer, string.Empty);
				writeSection();
				docWriter.WriteLine(writer, string.Empty);
			}
			docWriter.WriteLine(writer, "# Arguments");
			docWriter.WriteLine(writer, string.Empty);
			for (int i = 0; i < method.Args.Count; i++) {
				var arg = method.Args[i];
				docWriter.Write($"* `{idConverter.Argument(arg.Name)}`: ");
				docWriter.WriteDocLine(writer, arg.Doc, typeName);
			}
			docWriter.EndWrite(writer);
		}

		static bool IsSnakeCase(string s) {
			foreach (var c in s) {
				if (!(char.IsLower(c) || char.IsDigit(c) || c == '_'))
					return false;
			}
			return true;
		}

		public void WriteMethodDeclArgs(FileWriter writer, CreateMethod method) {
			bool comma = false;
			foreach (var arg in method.Args) {
				if (comma)
					writer.Write(", ");
				comma = true;
				var argName = idConverter.Argument(arg.Name);
				if (!IsSnakeCase(argName)) {
					writer.Write(RustConstants.AttributeAllowNonSnakeCase);
					writer.Write(" ");
				}
				writer.Write(argName);
				writer.Write(": ");
				switch (arg.Type) {
				case MethodArgType.Code:
					writer.Write(genTypes[TypeIds.Code].Name(idConverter));
					break;
				case MethodArgType.Register:
					writer.Write(genTypes[TypeIds.Register].Name(idConverter));
					break;
				case MethodArgType.RepPrefixKind:
					writer.Write(genTypes[TypeIds.RepPrefixKind].Name(idConverter));
					break;
				case MethodArgType.Memory:
					writer.Write("MemoryOperand");
					break;
				case MethodArgType.UInt8:
					writer.Write("u8");
					break;
				case MethodArgType.UInt16:
					writer.Write("u16");
					break;
				case MethodArgType.Int32:
					writer.Write("i32");
					break;
				case MethodArgType.PreferedInt32:
				case MethodArgType.UInt32:
					writer.Write("u32");
					break;
				case MethodArgType.Int64:
					writer.Write("i64");
					break;
				case MethodArgType.UInt64:
					writer.Write("u64");
					break;
				case MethodArgType.ByteSlice:
					writer.Write("&[u8]");
					break;
				case MethodArgType.WordSlice:
					writer.Write("&[u16]");
					break;
				case MethodArgType.DwordSlice:
					writer.Write("&[u32]");
					break;
				case MethodArgType.QwordSlice:
					writer.Write("&[u64]");
					break;
				case MethodArgType.ByteArray:
				case MethodArgType.WordArray:
				case MethodArgType.DwordArray:
				case MethodArgType.QwordArray:
				case MethodArgType.BytePtr:
				case MethodArgType.WordPtr:
				case MethodArgType.DwordPtr:
				case MethodArgType.QwordPtr:
				case MethodArgType.ArrayIndex:
				case MethodArgType.ArrayLength:
				default:
					throw new InvalidOperationException();
				}
			}
		}

		public static bool Is64BitArgument(MethodArgType type) {
			switch (type) {
			case MethodArgType.Code:
			case MethodArgType.Register:
			case MethodArgType.RepPrefixKind:
			case MethodArgType.Memory:
			case MethodArgType.UInt8:
			case MethodArgType.UInt16:
			case MethodArgType.Int32:
			case MethodArgType.UInt32:
			case MethodArgType.PreferedInt32:
			case MethodArgType.ByteSlice:
			case MethodArgType.WordSlice:
			case MethodArgType.DwordSlice:
			case MethodArgType.QwordSlice:
				return false;

			case MethodArgType.Int64:
			case MethodArgType.UInt64:
				return true;

			case MethodArgType.ArrayIndex:
			case MethodArgType.ArrayLength:
				// Never used, but if they're used in the future, they should be converted to u32 types if RustJS
				throw new InvalidOperationException();

			case MethodArgType.ByteArray:
			case MethodArgType.WordArray:
			case MethodArgType.DwordArray:
			case MethodArgType.QwordArray:
			case MethodArgType.BytePtr:
			case MethodArgType.WordPtr:
			case MethodArgType.DwordPtr:
			case MethodArgType.QwordPtr:
			default:
				throw new ArgumentOutOfRangeException(nameof(type));
			}
		}

		public string GetCreateName(CreateMethod method, GenCreateNameArgs genNames) => GetCreateName(sb, method, genNames);

		public static string GetCreateName(StringBuilder sb, CreateMethod method, GenCreateNameArgs genNames) {
			if (method.Args.Count == 0 || method.Args[0].Type != MethodArgType.Code)
				throw new InvalidOperationException();

			sb.Clear();
			sb.Append(genNames.CreatePrefix);
			var args = method.Args;
			for (int i = 1; i < args.Count; i++) {
				var arg = args[i];
				switch (arg.Type) {
				case MethodArgType.Register:
					sb.Append(genNames.Register);
					break;
				case MethodArgType.Memory:
					sb.Append(genNames.Memory);
					break;
				case MethodArgType.Int32:
					sb.Append(genNames.Int32);
					break;
				case MethodArgType.UInt32:
					sb.Append(genNames.UInt32);
					break;
				case MethodArgType.Int64:
					sb.Append(genNames.Int64);
					break;
				case MethodArgType.UInt64:
					sb.Append(genNames.UInt64);
					break;

				case MethodArgType.Code:
				case MethodArgType.RepPrefixKind:
				case MethodArgType.UInt8:
				case MethodArgType.UInt16:
				case MethodArgType.PreferedInt32:
				case MethodArgType.ArrayIndex:
				case MethodArgType.ArrayLength:
				case MethodArgType.ByteArray:
				case MethodArgType.WordArray:
				case MethodArgType.DwordArray:
				case MethodArgType.QwordArray:
				case MethodArgType.ByteSlice:
				case MethodArgType.WordSlice:
				case MethodArgType.DwordSlice:
				case MethodArgType.QwordSlice:
				case MethodArgType.BytePtr:
				case MethodArgType.WordPtr:
				case MethodArgType.DwordPtr:
				case MethodArgType.QwordPtr:
				default:
					throw new InvalidOperationException();
				}
			}

			return sb.ToString();
		}

		static bool HasImmediateArg_8_16_32_64(CreateMethod method) {
			foreach (var arg in method.Args) {
				switch (arg.Type) {
				case MethodArgType.UInt8:
				case MethodArgType.UInt16:
				case MethodArgType.Int32:
				case MethodArgType.UInt32:
				case MethodArgType.Int64:
				case MethodArgType.UInt64:
					return true;
				}
			}
			return false;
		}

		// Assumes it's a generic with_*() method (not a specialized method such as with_movsb() etc)
		public static bool HasTryMethod(CreateMethod method) =>
			HasImmediateArg_8_16_32_64(method);
	}
}
