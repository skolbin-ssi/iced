// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

using System;
using System.Linq;
using System.Text;
using Generator.Documentation.Rust;
using Generator.Enums;
using Generator.Enums.Encoder;
using Generator.IO;

namespace Generator.Encoder.Rust {
	[Generator(TargetLanguage.Rust)]
	sealed class RustInstrCreateGen : InstrCreateGen {
		readonly GeneratorContext generatorContext;
		readonly IdentifierConverter idConverter;
		readonly RustDocCommentWriter docWriter;
		readonly InstrCreateGenImpl gen;
		readonly GenCreateNameArgs genNames;
		readonly StringBuilder sb;

		public RustInstrCreateGen(GeneratorContext generatorContext)
			: base(generatorContext.Types) {
			this.generatorContext = generatorContext;
			idConverter = RustIdentifierConverter.Create();
			docWriter = new RustDocCommentWriter(idConverter);
			gen = new InstrCreateGenImpl(genTypes, idConverter, docWriter);
			genNames = GenCreateNameArgs.RustNames;
			sb = new StringBuilder();
		}

		protected override (TargetLanguage language, string id, string filename) GetFileInfo() =>
			(TargetLanguage.Rust, "Create", generatorContext.Types.Dirs.GetRustFilename("instruction.rs"));

		enum TryMethodKind {
			// Can never fail
			Normal,
			// Calls 'Result' method then unwraps() the result to panic. Marked as deprecated.
			Panic,
			// Returns a Result<T, E> with a `try_` method name prefix
			Result,
		}

		void WriteDocs(FileWriter writer, CreateMethod method, TryMethodKind kind = TryMethodKind.Normal, Action? writeSection = null) {
			if (kind != TryMethodKind.Normal && writeSection is null)
				throw new InvalidOperationException();
			var sectionTitle = kind switch {
				TryMethodKind.Normal => string.Empty,
				TryMethodKind.Panic => "Panics",
				TryMethodKind.Result => "Errors",
				_ => throw new InvalidOperationException(),
			};
			gen.WriteDocs(writer, method, sectionTitle, writeSection);
		}

		static void WriteMethodAttributes(FileWriter writer, CreateMethod method, bool inline, bool canFail, GenTryFlags flags) {
			if (!canFail)
				writer.WriteLine(RustConstants.AttributeMustUse);
			writer.WriteLine(inline ? RustConstants.AttributeInline : RustConstants.AttributeAllowMissingInlineInPublicItems);
			writer.WriteLine(RustConstants.AttributeNoRustFmt);
			if ((flags & GenTryFlags.TrivialCasts) != 0)
				writer.WriteLine(RustConstants.AttributeAllowTrivialCasts);
		}

		void WriteMethod(FileWriter writer, CreateMethod method, string name, TryMethodKind kind, GenTryFlags flags) {
			bool inline = kind == TryMethodKind.Panic || (flags & GenTryFlags.NoFooter) != 0;
			WriteMethodAttributes(writer, method, inline, kind == TryMethodKind.Result, flags);
			writer.Write($"pub fn {name}(");
			gen.WriteMethodDeclArgs(writer, method);
			if (kind == TryMethodKind.Result)
				writer.WriteLine(") -> Result<Self, IcedError> {");
			else
				writer.WriteLine(") -> Self {");
		}

		void WriteInitializeInstruction(FileWriter writer, CreateMethod method) {
			writer.WriteLine("let mut instruction = Self::default();");
			var args = method.Args;
			if (args.Count == 0 || args[0].Type != MethodArgType.Code)
				throw new InvalidOperationException();
			var codeName = idConverter.Argument(args[0].Name);
			writer.WriteLine($"instruction_internal::internal_set_code(&mut instruction, {codeName});");
		}

		void WriteInitializeInstruction(FileWriter writer, EnumValue code) {
			writer.WriteLine("let mut instruction = Self::default();");
			writer.WriteLine($"instruction_internal::internal_set_code(&mut instruction, {code.DeclaringType.Name(idConverter)}::{code.Name(idConverter)});");
		}

		static void WriteMethodFooter(FileWriter writer, int opCount, TryMethodKind kind) {
			writer.WriteLine();
			writer.WriteLine($"debug_assert_eq!(instruction.op_count(), {opCount});");
			if (kind == TryMethodKind.Result)
				writer.WriteLine("Ok(instruction)");
			else
				writer.WriteLine("instruction");
		}

		readonly struct GenerateTryMethodContext {
			public readonly FileWriter Writer;
			public readonly CreateMethod Method;
			public readonly int OpCount;
			public readonly string MethodName;
			public readonly string TryMethodName;

			public GenerateTryMethodContext(FileWriter writer, CreateMethod method, int opCount, string methodName) {
				Writer = writer;
				Method = method;
				OpCount = opCount;
				MethodName = methodName;
				TryMethodName = "try_" + methodName;
			}
		}

		readonly struct RustDeprecatedInfo {
			public readonly string Version;
			public readonly string Message;

			public RustDeprecatedInfo(string version, string message) {
				Version = version;
				Message = message;
			}
		}

		[Flags]
		enum GenTryFlags : uint {
			None			= 0,
			CanFail			= 1,
			NoFooter		= 2,
			TrivialCasts	= 4,
		}
		void GenerateTryMethods(FileWriter writer, CreateMethod method, int opCount, GenTryFlags flags, Action<GenerateTryMethodContext, TryMethodKind> genBody, Action<TryMethodKind>? writeError, string? methodName = null, RustDeprecatedInfo? deprecatedInfo = null) {
			if (((flags & GenTryFlags.CanFail) != 0) != (writeError is not null))
				throw new InvalidOperationException();
			methodName ??= gen.GetCreateName(method, genNames);
			var ctx = new GenerateTryMethodContext(writer, method, opCount, methodName);

			string? deprecMsg = null;
			if (deprecatedInfo is RustDeprecatedInfo deprec)
				deprecMsg = $"#[deprecated(since = \"{deprec.Version}\", note = \"{deprec.Message}\")]";

			if ((flags & GenTryFlags.CanFail) != 0) {
				if (writeError is null)
					throw new InvalidOperationException();
				const TryMethodKind kind = TryMethodKind.Result;
				Action docsWriteError = () => writeError(kind);
				WriteDocs(ctx.Writer, ctx.Method, kind, docsWriteError);
				if (deprecMsg is not null)
					ctx.Writer.WriteLine(deprecMsg);
				WriteMethod(ctx.Writer, ctx.Method, ctx.TryMethodName, kind, flags);
				using (ctx.Writer.Indent()) {
					genBody(ctx, kind);
					if ((flags & GenTryFlags.NoFooter) == 0)
						WriteMethodFooter(ctx.Writer, ctx.OpCount, kind);
				}
				ctx.Writer.WriteLine("}");
			}
			else {
				const TryMethodKind kind = TryMethodKind.Normal;
				WriteDocs(ctx.Writer, ctx.Method);
				if (deprecMsg is not null)
					ctx.Writer.WriteLine(deprecMsg);
				WriteMethod(ctx.Writer, ctx.Method, ctx.MethodName, kind, flags);
				using (ctx.Writer.Indent()) {
					genBody(ctx, kind);
					if ((flags & GenTryFlags.NoFooter) == 0)
						WriteMethodFooter(ctx.Writer, ctx.OpCount, kind);
				}
				ctx.Writer.WriteLine("}");
			}

			if ((flags & GenTryFlags.CanFail) != 0) {
				ctx.Writer.WriteLine();
				if (writeError is null)
					throw new InvalidOperationException();
				const TryMethodKind kind = TryMethodKind.Panic;
				Action docsWriteError = () => writeError(kind);
				WriteDocs(ctx.Writer, ctx.Method, kind, docsWriteError);
				ctx.Writer.WriteLine($"#[deprecated(since = \"1.10.0\", note = \"This method can panic, use {ctx.TryMethodName}() instead\")]");
				ctx.Writer.WriteLine(RustConstants.AttributeAllowUnwrapUsed);
				WriteMethod(ctx.Writer, ctx.Method, ctx.MethodName, kind, GenTryFlags.None);
				using (ctx.Writer.Indent()) {
					sb.Clear();
					sb.Append("Instruction::");
					sb.Append(ctx.TryMethodName);
					sb.Append('(');
					for (int i = 0; i < ctx.Method.Args.Count; i++) {
						if (i > 0)
							sb.Append(", ");
						var argName = idConverter.Argument(ctx.Method.Args[i].Name);
						sb.Append(argName);
					}
					sb.Append(").unwrap()");
					ctx.Writer.WriteLine(sb.ToString());
				}
				ctx.Writer.WriteLine("}");
			}
		}

		static string GetErrorString(TryMethodKind kind, string message) {
			string word = kind switch {
				TryMethodKind.Panic => "Panics",
				TryMethodKind.Result => "Fails",
				_ => throw new InvalidOperationException(),
			};
			return $"{word} {message}";
		}

		protected override void GenCreate(FileWriter writer, CreateMethod method, InstructionGroup group) {
			int opCount = method.Args.Count - 1;
			if (InstrCreateGenImpl.HasTryMethod(method)) {
				Action<TryMethodKind> writeError = kind => docWriter.WriteLine(writer, GetErrorString(kind, "if the immediate is invalid"));
				GenerateTryMethods(writer, method, opCount, GenTryFlags.CanFail, GenCreateBody, writeError);
			}
			else
				GenerateTryMethods(writer, method, opCount, GenTryFlags.None, GenCreateBody, null);
		}

		void GenCreateBody(GenerateTryMethodContext ctx, TryMethodKind kind) {
			WriteInitializeInstruction(ctx.Writer, ctx.Method);
			var args = ctx.Method.Args;
			var codeName = idConverter.Argument(args[0].Name);
			var opKindStr = genTypes[TypeIds.OpKind].Name(idConverter);
			var registerStr = genTypes[TypeIds.OpKind][nameof(OpKind.Register)].Name(idConverter);
			var memoryStr = genTypes[TypeIds.OpKind][nameof(OpKind.Memory)].Name(idConverter);
			var immediate64Str = genTypes[TypeIds.OpKind][nameof(OpKind.Immediate64)].Name(idConverter);
			var immediate8_2ndStr = genTypes[TypeIds.OpKind][nameof(OpKind.Immediate8_2nd)].Name(idConverter);
			bool multipleInts = args.Where(a => a.Type == MethodArgType.Int32 || a.Type == MethodArgType.UInt32).Count() > 1;
			string methodName;
			for (int i = 1; i < args.Count; i++) {
				int op = i - 1;
				var arg = args[i];
				ctx.Writer.WriteLine();
				switch (arg.Type) {
				case MethodArgType.Register:
					ctx.Writer.WriteLine($"const_assert_eq!({opKindStr}::{registerStr} as u32, 0);");
					ctx.Writer.WriteLine($"//instruction_internal::internal_set_op{op}_kind(&mut instruction, {opKindStr}::{registerStr});");
					ctx.Writer.WriteLine($"instruction_internal::internal_set_op{op}_register(&mut instruction, {idConverter.Argument(arg.Name)});");
					break;

				case MethodArgType.Memory:
					ctx.Writer.WriteLine($"instruction_internal::internal_set_op{op}_kind(&mut instruction, {opKindStr}::{memoryStr});");
					ctx.Writer.WriteLine($"Instruction::init_memory_operand(&mut instruction, &{idConverter.Argument(arg.Name)});");
					break;

				case MethodArgType.Int32:
				case MethodArgType.UInt32:
					methodName = arg.Type == MethodArgType.Int32 ? "initialize_signed_immediate" : "initialize_unsigned_immediate";
					var castType = arg.Type == MethodArgType.Int32 ? " as i64" : " as u64";
					ctx.Writer.WriteLine($"instruction_internal::{methodName}(&mut instruction, {op}, {idConverter.Argument(arg.Name)}{castType})?;");
					break;

				case MethodArgType.Int64:
				case MethodArgType.UInt64:
					methodName = arg.Type == MethodArgType.Int64 ? "initialize_signed_immediate" : "initialize_unsigned_immediate";
					ctx.Writer.WriteLine($"instruction_internal::{methodName}(&mut instruction, {op}, {idConverter.Argument(arg.Name)})?;");
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
		}

		protected override void GenCreateBranch(FileWriter writer, CreateMethod method) {
			if (method.Args.Count != 2)
				throw new InvalidOperationException();
			Action<TryMethodKind> writeError = kind => docWriter.WriteLine(writer, GetErrorString(kind, "if the created instruction doesn't have a near branch operand"));
			GenerateTryMethods(writer, method, 1, GenTryFlags.CanFail, GenCreateBranch, writeError, RustInstrCreateGenNames.with_branch);
		}

		void GenCreateBranch(GenerateTryMethodContext ctx, TryMethodKind kind) {
			WriteInitializeInstruction(ctx.Writer, ctx.Method);
			ctx.Writer.WriteLine();
			ctx.Writer.WriteLine($"instruction_internal::internal_set_op0_kind(&mut instruction, instruction_internal::get_near_branch_op_kind({idConverter.Argument(ctx.Method.Args[0].Name)}, 0)?);");
			ctx.Writer.WriteLine($"instruction.set_near_branch64({idConverter.Argument(ctx.Method.Args[1].Name)});");
		}

		protected override void GenCreateFarBranch(FileWriter writer, CreateMethod method) {
			if (method.Args.Count != 3)
				throw new InvalidOperationException();
			Action<TryMethodKind> writeError = kind => docWriter.WriteLine(writer, GetErrorString(kind, "if the created instruction doesn't have a far branch operand"));
			GenerateTryMethods(writer, method, 1, GenTryFlags.CanFail, GenCreateFarBranch, writeError, RustInstrCreateGenNames.with_far_branch);
		}

		void GenCreateFarBranch(GenerateTryMethodContext ctx, TryMethodKind kind) {
			WriteInitializeInstruction(ctx.Writer, ctx.Method);
			ctx.Writer.WriteLine();
			ctx.Writer.WriteLine($"instruction_internal::internal_set_op0_kind(&mut instruction, instruction_internal::get_far_branch_op_kind({idConverter.Argument(ctx.Method.Args[0].Name)}, 0)?);");
			ctx.Writer.WriteLine($"instruction.set_far_branch_selector({idConverter.Argument(ctx.Method.Args[1].Name)});");
			ctx.Writer.WriteLine($"instruction.set_far_branch32({idConverter.Argument(ctx.Method.Args[2].Name)});");
		}

		protected override void GenCreateXbegin(FileWriter writer, CreateMethod method) {
			if (method.Args.Count != 2)
				throw new InvalidOperationException();
			Action<TryMethodKind> writeError = kind => WriteAddrSizeOrBitnessPanic(writer, method, kind);
			GenerateTryMethods(writer, method, 1, GenTryFlags.CanFail, GenCreateXbegin, writeError, RustInstrCreateGenNames.with_xbegin);
		}

		void GenCreateXbegin(GenerateTryMethodContext ctx, TryMethodKind kind) {
			ctx.Writer.WriteLine($"let mut instruction = Self::default();");
			var opKindName = genTypes[TypeIds.OpKind].Name(idConverter);
			var codeName = codeType.Name(idConverter);
			ctx.Writer.WriteLine();
			ctx.Writer.WriteLine($"match bitness {{");
			ctx.Writer.WriteLine($"	16 => {{");
			ctx.Writer.WriteLine($"		instruction_internal::internal_set_code(&mut instruction, {codeName}::{codeType[nameof(Code.Xbegin_rel16)].Name(idConverter)});");
			ctx.Writer.WriteLine($"		instruction_internal::internal_set_op0_kind(&mut instruction, {opKindName}::{genTypes[TypeIds.OpKind][nameof(OpKind.NearBranch32)].Name(idConverter)});");
			ctx.Writer.WriteLine($"		instruction.set_near_branch32({idConverter.Argument(ctx.Method.Args[1].Name)} as u32);");
			ctx.Writer.WriteLine($"	}}");
			ctx.Writer.WriteLine();
			ctx.Writer.WriteLine($"	32 => {{");
			ctx.Writer.WriteLine($"		instruction_internal::internal_set_code(&mut instruction, {codeName}::{codeType[nameof(Code.Xbegin_rel32)].Name(idConverter)});");
			ctx.Writer.WriteLine($"		instruction_internal::internal_set_op0_kind(&mut instruction, {opKindName}::{genTypes[TypeIds.OpKind][nameof(OpKind.NearBranch32)].Name(idConverter)});");
			ctx.Writer.WriteLine($"		instruction.set_near_branch32({idConverter.Argument(ctx.Method.Args[1].Name)} as u32);");
			ctx.Writer.WriteLine($"	}}");
			ctx.Writer.WriteLine();
			ctx.Writer.WriteLine($"	64 => {{");
			ctx.Writer.WriteLine($"		instruction_internal::internal_set_code(&mut instruction, {codeName}::{codeType[nameof(Code.Xbegin_rel32)].Name(idConverter)});");
			ctx.Writer.WriteLine($"		instruction_internal::internal_set_op0_kind(&mut instruction, {opKindName}::{genTypes[TypeIds.OpKind][nameof(OpKind.NearBranch64)].Name(idConverter)});");
			ctx.Writer.WriteLine($"		instruction.set_near_branch64({idConverter.Argument(ctx.Method.Args[1].Name)});");
			ctx.Writer.WriteLine($"	}}");
			ctx.Writer.WriteLine();
			ctx.Writer.WriteLine($"	_ => return Err(IcedError::new(\"Invalid bitness\")),");
			ctx.Writer.WriteLine($"}}");
		}

		protected override bool CallGenCreateMemory64 => true;
		protected override void GenCreateMemory64(FileWriter writer, CreateMethod method) {
			if (method.Args.Count != 4)
				throw new InvalidOperationException();

			int memOp, regOp;
			string name, newName;
			if (method.Args[1].Type == MethodArgType.UInt64) {
				memOp = 0;
				regOp = 1;
				name = RustInstrCreateGenNames.with_mem64_reg;
				newName = "with_mem_reg";
			}
			else {
				memOp = 1;
				regOp = 0;
				name = RustInstrCreateGenNames.with_reg_mem64;
				newName = "with_reg_mem";
			}

			var deprec = new RustDeprecatedInfo("1.11.0", $"Use {newName}() with a MemoryOperand arg instead");
			GenerateTryMethods(writer, method, 2, GenTryFlags.NoFooter, (ctx, _) => GenCreateMemory64(ctx, memOp, regOp), null, name, deprecatedInfo: deprec);
		}

		void GenCreateMemory64(GenerateTryMethodContext ctx, int memOp, int regOp) {
			var regNone = genTypes[TypeIds.Register][nameof(Register.None)];
			var regStr = $"{regNone.DeclaringType.Name(idConverter)}::{regNone.Name(idConverter)}";
			var addrStr = idConverter.Argument(ctx.Method.Args[1 + memOp].Name);
			var segPrefStr = idConverter.Argument(ctx.Method.Args[3].Name);
			var memOpStr = $"MemoryOperand::with_base_displ_size_bcst_seg({regStr}, {addrStr} as i64, 8, false, {segPrefStr})";
			var regOpStr = idConverter.Argument(ctx.Method.Args[1 + regOp].Name);
			var codeStr = idConverter.Argument(ctx.Method.Args[0].Name);

			if (memOp == 0)
				ctx.Writer.WriteLine($"Instruction::with_mem_reg({codeStr}, {memOpStr}, {regOpStr})");
			else
				ctx.Writer.WriteLine($"Instruction::with_reg_mem({codeStr}, {regOpStr}, {memOpStr})");
		}

		static void WriteComma(FileWriter writer) => writer.Write(", ");
		void Write(FileWriter writer, EnumValue value) => writer.Write($"{value.DeclaringType.Name(idConverter)}::{value.Name(idConverter)}");
		void Write(FileWriter writer, MethodArg arg) => writer.Write(idConverter.Argument(arg.Name));

		void WriteAddrSizeOrBitnessPanic(FileWriter writer, CreateMethod method, TryMethodKind kind) {
			var arg = method.Args[0];
			if (arg.Name != "addressSize" && arg.Name != "bitness")
				throw new InvalidOperationException();
			docWriter.WriteLine(writer, GetErrorString(kind, $"if `{idConverter.Argument(arg.Name)}` is not one of 16, 32, 64."));
		}

		protected override void GenCreateString_Reg_SegRSI(FileWriter writer, CreateMethod method, StringMethodKind kind, string methodBaseName, EnumValue code, EnumValue register) {
			var methodName = idConverter.Method("With" + methodBaseName);
			Action<TryMethodKind> writeError = kind => WriteAddrSizeOrBitnessPanic(writer, method, kind);
			GenerateTryMethods(writer, method, 1, GenTryFlags.CanFail | GenTryFlags.NoFooter, (ctx, _) => GenCreateString_Reg_SegRSI(ctx, kind, code, register), writeError, methodName);
		}

		void GenCreateString_Reg_SegRSI(GenerateTryMethodContext ctx, StringMethodKind kind, EnumValue code, EnumValue register) {
			ctx.Writer.Write("instruction_internal::with_string_reg_segrsi(");
			switch (kind) {
			case StringMethodKind.Full:
				if (ctx.Method.Args.Count != 3)
					throw new InvalidOperationException();
				Write(ctx.Writer, code);
				WriteComma(ctx.Writer);
				Write(ctx.Writer, ctx.Method.Args[0]);
				WriteComma(ctx.Writer);
				Write(ctx.Writer, register);
				WriteComma(ctx.Writer);
				Write(ctx.Writer, ctx.Method.Args[1]);
				WriteComma(ctx.Writer);
				Write(ctx.Writer, ctx.Method.Args[2]);
				break;
			case StringMethodKind.Rep:
				if (ctx.Method.Args.Count != 1)
					throw new InvalidOperationException();
				Write(ctx.Writer, code);
				WriteComma(ctx.Writer);
				Write(ctx.Writer, ctx.Method.Args[0]);
				WriteComma(ctx.Writer);
				Write(ctx.Writer, register);
				WriteComma(ctx.Writer);
				Write(ctx.Writer, genTypes[TypeIds.Register][nameof(Register.None)]);
				WriteComma(ctx.Writer);
				Write(ctx.Writer, genTypes[TypeIds.RepPrefixKind][nameof(RepPrefixKind.Repe)]);
				break;
			case StringMethodKind.Repe:
			case StringMethodKind.Repne:
			default:
				throw new InvalidOperationException();
			}
			ctx.Writer.WriteLine(")");
		}

		protected override void GenCreateString_Reg_ESRDI(FileWriter writer, CreateMethod method, StringMethodKind kind, string methodBaseName, EnumValue code, EnumValue register) {
			var methodName = idConverter.Method("With" + methodBaseName);
			Action<TryMethodKind> writeError = kind => WriteAddrSizeOrBitnessPanic(writer, method, kind);
			GenerateTryMethods(writer, method, 1, GenTryFlags.CanFail | GenTryFlags.NoFooter, (ctx, _) => GenCreateString_Reg_ESRDI(ctx, kind, code, register), writeError, methodName);
		}

		void GenCreateString_Reg_ESRDI(GenerateTryMethodContext ctx, StringMethodKind kind, EnumValue code, EnumValue register) {
			ctx.Writer.Write("instruction_internal::with_string_reg_esrdi(");
			switch (kind) {
			case StringMethodKind.Full:
				if (ctx.Method.Args.Count != 2)
					throw new InvalidOperationException();
				Write(ctx.Writer, code);
				WriteComma(ctx.Writer);
				Write(ctx.Writer, ctx.Method.Args[0]);
				WriteComma(ctx.Writer);
				Write(ctx.Writer, register);
				WriteComma(ctx.Writer);
				Write(ctx.Writer, ctx.Method.Args[1]);
				break;
			case StringMethodKind.Repe:
			case StringMethodKind.Repne:
				if (ctx.Method.Args.Count != 1)
					throw new InvalidOperationException();
				Write(ctx.Writer, code);
				WriteComma(ctx.Writer);
				Write(ctx.Writer, ctx.Method.Args[0]);
				WriteComma(ctx.Writer);
				Write(ctx.Writer, register);
				WriteComma(ctx.Writer);
				Write(ctx.Writer, kind == StringMethodKind.Repe ? genTypes[TypeIds.RepPrefixKind][nameof(RepPrefixKind.Repe)] : genTypes[TypeIds.RepPrefixKind][nameof(RepPrefixKind.Repne)]);
				break;
			case StringMethodKind.Rep:
			default:
				throw new InvalidOperationException();
			}
			ctx.Writer.WriteLine(")");
		}

		protected override void GenCreateString_ESRDI_Reg(FileWriter writer, CreateMethod method, StringMethodKind kind, string methodBaseName, EnumValue code, EnumValue register) {
			var methodName = idConverter.Method("With" + methodBaseName);
			Action<TryMethodKind> writeError = kind => WriteAddrSizeOrBitnessPanic(writer, method, kind);
			GenerateTryMethods(writer, method, 1, GenTryFlags.CanFail | GenTryFlags.NoFooter, (ctx, _) => GenCreateString_ESRDI_Reg(ctx, kind, code, register), writeError, methodName);
		}

		void GenCreateString_ESRDI_Reg(GenerateTryMethodContext ctx, StringMethodKind kind, EnumValue code, EnumValue register) {
			ctx.Writer.Write("instruction_internal::with_string_esrdi_reg(");
			switch (kind) {
			case StringMethodKind.Full:
				if (ctx.Method.Args.Count != 2)
					throw new InvalidOperationException();
				Write(ctx.Writer, code);
				WriteComma(ctx.Writer);
				Write(ctx.Writer, ctx.Method.Args[0]);
				WriteComma(ctx.Writer);
				Write(ctx.Writer, register);
				WriteComma(ctx.Writer);
				Write(ctx.Writer, ctx.Method.Args[1]);
				break;
			case StringMethodKind.Rep:
				if (ctx.Method.Args.Count != 1)
					throw new InvalidOperationException();
				Write(ctx.Writer, code);
				WriteComma(ctx.Writer);
				Write(ctx.Writer, ctx.Method.Args[0]);
				WriteComma(ctx.Writer);
				Write(ctx.Writer, register);
				WriteComma(ctx.Writer);
				Write(ctx.Writer, genTypes[TypeIds.RepPrefixKind][nameof(RepPrefixKind.Repe)]);
				break;
			case StringMethodKind.Repe:
			case StringMethodKind.Repne:
			default:
				throw new InvalidOperationException();
			}
			ctx.Writer.WriteLine(")");
		}

		protected override void GenCreateString_SegRSI_ESRDI(FileWriter writer, CreateMethod method, StringMethodKind kind, string methodBaseName, EnumValue code) {
			var methodName = idConverter.Method("With" + methodBaseName);
			Action<TryMethodKind> writeError = kind => WriteAddrSizeOrBitnessPanic(writer, method, kind);
			GenerateTryMethods(writer, method, 1, GenTryFlags.CanFail | GenTryFlags.NoFooter, (ctx, _) => GenCreateString_SegRSI_ESRDI(ctx, kind, code), writeError, methodName);
		}

		void GenCreateString_SegRSI_ESRDI(GenerateTryMethodContext ctx, StringMethodKind kind, EnumValue code) {
			ctx.Writer.Write("instruction_internal::with_string_segrsi_esrdi(");
			switch (kind) {
			case StringMethodKind.Full:
				if (ctx.Method.Args.Count != 3)
					throw new InvalidOperationException();
				Write(ctx.Writer, code);
				WriteComma(ctx.Writer);
				Write(ctx.Writer, ctx.Method.Args[0]);
				WriteComma(ctx.Writer);
				Write(ctx.Writer, ctx.Method.Args[1]);
				WriteComma(ctx.Writer);
				Write(ctx.Writer, ctx.Method.Args[2]);
				break;
			case StringMethodKind.Repe:
			case StringMethodKind.Repne:
				if (ctx.Method.Args.Count != 1)
					throw new InvalidOperationException();
				Write(ctx.Writer, code);
				WriteComma(ctx.Writer);
				Write(ctx.Writer, ctx.Method.Args[0]);
				WriteComma(ctx.Writer);
				Write(ctx.Writer, genTypes[TypeIds.Register][nameof(Register.None)]);
				WriteComma(ctx.Writer);
				Write(ctx.Writer, kind == StringMethodKind.Repe ? genTypes[TypeIds.RepPrefixKind][nameof(RepPrefixKind.Repe)] : genTypes[TypeIds.RepPrefixKind][nameof(RepPrefixKind.Repne)]);
				break;
			case StringMethodKind.Rep:
			default:
				throw new InvalidOperationException();
			}
			ctx.Writer.WriteLine(")");
		}

		protected override void GenCreateString_ESRDI_SegRSI(FileWriter writer, CreateMethod method, StringMethodKind kind, string methodBaseName, EnumValue code) {
			var methodName = idConverter.Method("With" + methodBaseName);
			Action<TryMethodKind> writeError = kind => WriteAddrSizeOrBitnessPanic(writer, method, kind);
			GenerateTryMethods(writer, method, 1, GenTryFlags.CanFail | GenTryFlags.NoFooter, (ctx, _) => GenCreateString_ESRDI_SegRSI(ctx, kind, code), writeError, methodName);
		}

		void GenCreateString_ESRDI_SegRSI(GenerateTryMethodContext ctx, StringMethodKind kind, EnumValue code) {
			ctx.Writer.Write("instruction_internal::with_string_esrdi_segrsi(");
			switch (kind) {
			case StringMethodKind.Full:
				if (ctx.Method.Args.Count != 3)
					throw new InvalidOperationException();
				Write(ctx.Writer, code);
				WriteComma(ctx.Writer);
				Write(ctx.Writer, ctx.Method.Args[0]);
				WriteComma(ctx.Writer);
				Write(ctx.Writer, ctx.Method.Args[1]);
				WriteComma(ctx.Writer);
				Write(ctx.Writer, ctx.Method.Args[2]);
				break;
			case StringMethodKind.Rep:
				if (ctx.Method.Args.Count != 1)
					throw new InvalidOperationException();
				Write(ctx.Writer, code);
				WriteComma(ctx.Writer);
				Write(ctx.Writer, ctx.Method.Args[0]);
				WriteComma(ctx.Writer);
				Write(ctx.Writer, genTypes[TypeIds.Register][nameof(Register.None)]);
				WriteComma(ctx.Writer);
				Write(ctx.Writer, genTypes[TypeIds.RepPrefixKind][nameof(RepPrefixKind.Repe)]);
				break;
			case StringMethodKind.Repe:
			case StringMethodKind.Repne:
			default:
				throw new InvalidOperationException();
			}
			ctx.Writer.WriteLine(")");
		}

		protected override void GenCreateMaskmov(FileWriter writer, CreateMethod method, string methodBaseName, EnumValue code) {
			var methodName = idConverter.Method("With" + methodBaseName);
			Action<TryMethodKind> writeError = kind => WriteAddrSizeOrBitnessPanic(writer, method, kind);
			GenerateTryMethods(writer, method, 1, GenTryFlags.CanFail | GenTryFlags.NoFooter, (ctx, _) => GenCreateMaskmov(ctx, code), writeError, methodName);
		}

		void GenCreateMaskmov(GenerateTryMethodContext ctx, EnumValue code) {
			ctx.Writer.Write("instruction_internal::with_maskmov(");
			if (ctx.Method.Args.Count != 4)
				throw new InvalidOperationException();
			Write(ctx.Writer, code);
			WriteComma(ctx.Writer);
			Write(ctx.Writer, ctx.Method.Args[0]);
			WriteComma(ctx.Writer);
			Write(ctx.Writer, ctx.Method.Args[1]);
			WriteComma(ctx.Writer);
			Write(ctx.Writer, ctx.Method.Args[2]);
			WriteComma(ctx.Writer);
			Write(ctx.Writer, ctx.Method.Args[3]);
			ctx.Writer.WriteLine(")");
		}

		protected override void GenCreateDeclareData(FileWriter writer, CreateMethod method, DeclareDataKind kind) {
			EnumValue code;
			string setValueName;
			string methodName;
			switch (kind) {
			case DeclareDataKind.Byte:
				code = codeType[nameof(Code.DeclareByte)];
				setValueName = "set_declare_byte_value";
				methodName = RustInstrCreateGenNames.with_declare_byte;
				break;

			case DeclareDataKind.Word:
				code = codeType[nameof(Code.DeclareWord)];
				setValueName = "set_declare_word_value";
				methodName = RustInstrCreateGenNames.with_declare_word;
				break;

			case DeclareDataKind.Dword:
				code = codeType[nameof(Code.DeclareDword)];
				setValueName = "set_declare_dword_value";
				methodName = RustInstrCreateGenNames.with_declare_dword;
				break;

			case DeclareDataKind.Qword:
				code = codeType[nameof(Code.DeclareQword)];
				setValueName = "set_declare_qword_value";
				methodName = RustInstrCreateGenNames.with_declare_qword;
				break;

			default:
				throw new InvalidOperationException();
			}
			methodName = RustInstrCreateGenNames.AppendArgCount(methodName, method.Args.Count);

			writer.WriteLine();
			Action<TryMethodKind> writeError = kind => WriteDeclareDataError(writer, kind);
			GenerateTryMethods(writer, method, 0, GenTryFlags.CanFail, (ctx, _) => GenCreateDeclareData(ctx, code, setValueName), writeError, methodName);
		}

		void GenCreateDeclareData(GenerateTryMethodContext ctx, EnumValue code, string setValueName) {
			WriteInitializeInstruction(ctx.Writer, code);
			ctx.Writer.WriteLine($"instruction_internal::internal_set_declare_data_len(&mut instruction, {ctx.Method.Args.Count});");
			ctx.Writer.WriteLine();
			for (int i = 0; i < ctx.Method.Args.Count; i++)
				ctx.Writer.WriteLine($"instruction.try_{setValueName}({i}, {idConverter.Argument(ctx.Method.Args[i].Name)})?;");
		}

		static string GetDbErrorMsg(TryMethodKind kind) => GetErrorString(kind, "if `db` feature wasn't enabled");
		void WriteDeclareDataError(FileWriter writer, TryMethodKind kind) =>
			docWriter.WriteLine(writer, GetDbErrorMsg(kind));

		void WriteDataError(FileWriter writer, TryMethodKind kind, CreateMethod method, string extra) {
			var msg1 = GetErrorString(kind, $"if `{idConverter.Argument(method.Args[0].Name)}.len()` {extra}");
			docWriter.WriteLine(writer, $"- {msg1}");
			docWriter.WriteLine(writer, $"- {GetDbErrorMsg(kind)}");
		}

		void GenCreateDeclareDataSlice(FileWriter writer, CreateMethod method, int elemSize, EnumValue code, string methodName, string setDeclValueName) {
			writer.WriteLine();
			Action<TryMethodKind> writeError = kind => WriteDataError(writer, kind, method, $"is not 1-{16 / elemSize}");
			GenerateTryMethods(writer, method, 0, GenTryFlags.CanFail, (ctx, _) => GenCreateDeclareDataSlice(ctx, elemSize, code, setDeclValueName), writeError, methodName);
		}

		void GenCreateDeclareDataSlice(GenerateTryMethodContext ctx, int elemSize, EnumValue code, string setDeclValueName) {
			var dataName = idConverter.Argument(ctx.Method.Args[0].Name);
			ctx.Writer.WriteLine($"if {dataName}.len().wrapping_sub(1) > {16 / elemSize} - 1 {{");
			using (ctx.Writer.Indent())
				ctx.Writer.WriteLine("return Err(IcedError::new(\"Invalid slice length\"));");
			ctx.Writer.WriteLine("}");
			ctx.Writer.WriteLine();
			WriteInitializeInstruction(ctx.Writer, code);
			ctx.Writer.WriteLine($"instruction_internal::internal_set_declare_data_len(&mut instruction, {dataName}.len() as u32);");
			ctx.Writer.WriteLine();
			ctx.Writer.WriteLine($"for i in {dataName}.iter().enumerate() {{");
			using (ctx.Writer.Indent())
				ctx.Writer.WriteLine($"instruction.try_{setDeclValueName}(i.0, *i.1)?;");
			ctx.Writer.WriteLine("}");
		}

		protected override void GenCreateDeclareDataArray(FileWriter writer, CreateMethod method, DeclareDataKind kind, ArrayType arrayType) {
			switch (kind) {
			case DeclareDataKind.Byte:
				switch (arrayType) {
				case ArrayType.BytePtr:
				case ArrayType.ByteArray:
					break;

				case ArrayType.ByteSlice:
					GenCreateDeclareDataSlice(writer, method, 1, codeType[nameof(Code.DeclareByte)], RustInstrCreateGenNames.with_declare_byte, "set_declare_byte_value");
					break;

				default:
					throw new InvalidOperationException();
				}
				break;

			case DeclareDataKind.Word:
				switch (arrayType) {
				case ArrayType.BytePtr:
				case ArrayType.WordPtr:
				case ArrayType.ByteArray:
				case ArrayType.WordArray:
					break;

				case ArrayType.ByteSlice:
					writer.WriteLine();
					Action<TryMethodKind> writeError = kind => WriteDataError(writer, kind, method, $"is not 2-16 or not a multiple of 2");
					const GenTryFlags flags = GenTryFlags.CanFail | GenTryFlags.TrivialCasts;
					GenerateTryMethods(writer, method, 0, flags, (ctx, _) => GenerateDeclWordByteSlice(ctx), writeError, RustInstrCreateGenNames.with_declare_word_slice_u8);
					break;

					void GenerateDeclWordByteSlice(GenerateTryMethodContext ctx) {
						var dataName = idConverter.Argument(method.Args[0].Name);
						writer.WriteLine($"if {dataName}.len().wrapping_sub(1) > 16 - 1 || ({dataName}.len() & 1) != 0 {{");
						using (writer.Indent())
							ctx.Writer.WriteLine("return Err(IcedError::new(\"Invalid slice length\"));");
						writer.WriteLine("}");
						writer.WriteLine();
						WriteInitializeInstruction(writer, codeType[nameof(Code.DeclareWord)]);
						writer.WriteLine($"instruction_internal::internal_set_declare_data_len(&mut instruction, {dataName}.len() as u32 / 2);");
						writer.WriteLine();
						writer.WriteLine($"for i in 0..{dataName}.len() / 2 {{");
						using (writer.Indent()) {
							writer.WriteLine($"let v = unsafe {{ u16::from_le(ptr::read_unaligned({dataName}.get_unchecked(i * 2) as *const _ as *const u16)) }};");
							writer.WriteLine("instruction.try_set_declare_word_value(i, v)?;");
						}
						writer.WriteLine("}");
					}

				case ArrayType.WordSlice:
					GenCreateDeclareDataSlice(writer, method, 2, codeType[nameof(Code.DeclareWord)], RustInstrCreateGenNames.with_declare_word, "set_declare_word_value");
					break;

				default:
					throw new InvalidOperationException();
				}
				break;

			case DeclareDataKind.Dword:
				switch (arrayType) {
				case ArrayType.BytePtr:
				case ArrayType.DwordPtr:
				case ArrayType.ByteArray:
				case ArrayType.DwordArray:
					break;

				case ArrayType.ByteSlice:
					writer.WriteLine();
					Action<TryMethodKind> writeError = kind => WriteDataError(writer, kind, method, $"is not 4-16 or not a multiple of 4");
					const GenTryFlags flags = GenTryFlags.CanFail | GenTryFlags.TrivialCasts;
					GenerateTryMethods(writer, method, 0, flags, (ctx, _) => GenerateDeclDwordByteSlice(ctx), writeError, RustInstrCreateGenNames.with_declare_dword_slice_u8);
					break;

					void GenerateDeclDwordByteSlice(GenerateTryMethodContext ctx) {
						var dataName = idConverter.Argument(method.Args[0].Name);
						writer.WriteLine($"if {dataName}.len().wrapping_sub(1) > 16 - 1 || ({dataName}.len() & 3) != 0 {{");
						using (writer.Indent())
							ctx.Writer.WriteLine("return Err(IcedError::new(\"Invalid slice length\"));");
						writer.WriteLine("}");
						writer.WriteLine();
						WriteInitializeInstruction(writer, codeType[nameof(Code.DeclareDword)]);
						writer.WriteLine($"instruction_internal::internal_set_declare_data_len(&mut instruction, {dataName}.len() as u32 / 4);");
						writer.WriteLine();
						writer.WriteLine($"for i in 0..{dataName}.len() / 4 {{");
						using (writer.Indent()) {
							writer.WriteLine($"let v = unsafe {{ u32::from_le(ptr::read_unaligned({dataName}.get_unchecked(i * 4) as *const _ as *const u32)) }};");
							writer.WriteLine("instruction.try_set_declare_dword_value(i, v)?;");
						}
						writer.WriteLine("}");
					}

				case ArrayType.DwordSlice:
					GenCreateDeclareDataSlice(writer, method, 4, codeType[nameof(Code.DeclareDword)], RustInstrCreateGenNames.with_declare_dword, "set_declare_dword_value");
					break;

				default:
					throw new InvalidOperationException();
				}
				break;

			case DeclareDataKind.Qword:
				switch (arrayType) {
				case ArrayType.BytePtr:
				case ArrayType.QwordPtr:
				case ArrayType.ByteArray:
				case ArrayType.QwordArray:
					break;

				case ArrayType.ByteSlice:
					writer.WriteLine();
					Action<TryMethodKind> writeError = kind => WriteDataError(writer, kind, method, $"is not 8-16 or not a multiple of 8");
					const GenTryFlags flags = GenTryFlags.CanFail | GenTryFlags.TrivialCasts;
					GenerateTryMethods(writer, method, 0, flags, (ctx, _) => GenerateDeclQwordByteSlice(ctx), writeError, RustInstrCreateGenNames.with_declare_qword_slice_u8);
					break;

					void GenerateDeclQwordByteSlice(GenerateTryMethodContext ctx) {
						var dataName = idConverter.Argument(method.Args[0].Name);
						writer.WriteLine($"if {dataName}.len().wrapping_sub(1) > 16 - 1 || ({dataName}.len() & 7) != 0 {{");
						using (writer.Indent())
							ctx.Writer.WriteLine("return Err(IcedError::new(\"Invalid slice length\"));");
						writer.WriteLine("}");
						writer.WriteLine();
						WriteInitializeInstruction(writer, codeType[nameof(Code.DeclareQword)]);
						writer.WriteLine($"instruction_internal::internal_set_declare_data_len(&mut instruction, {dataName}.len() as u32 / 8);");
						writer.WriteLine();
						writer.WriteLine($"for i in 0..{dataName}.len() / 8 {{");
						using (writer.Indent()) {
							writer.WriteLine($"let v = unsafe {{ u64::from_le(ptr::read_unaligned({dataName}.get_unchecked(i * 8) as *const _ as *const u64)) }};");
							writer.WriteLine("instruction.try_set_declare_qword_value(i, v)?;");
						}
						writer.WriteLine("}");
					}

				case ArrayType.QwordSlice:
					GenCreateDeclareDataSlice(writer, method, 8, codeType[nameof(Code.DeclareQword)], RustInstrCreateGenNames.with_declare_qword, "set_declare_qword_value");
					break;

				default:
					throw new InvalidOperationException();
				}
				break;

			default:
				throw new InvalidOperationException();
			}
		}

		protected override void GenCreateDeclareDataArrayLength(FileWriter writer, CreateMethod method, DeclareDataKind kind, ArrayType arrayType) {
		}
	}
}
