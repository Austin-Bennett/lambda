# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Build
cargo build

# Run (takes one or more .lm source files)
# even though its a normal executable, output with the .lme to distinguish it as a lambda executable
cargo run -- -o test.lme test.lm test_import.lm

# Test
cargo test

# Check without building
cargo check
```

This project requires the nightly Rust toolchain (uses `#![feature(try_trait_v2)]` and `#![feature(deref_pure_trait)]`) and LLVM 21.1 (via Inkwell).

## Architecture

Lambda is a compiled language (`.lm` files) that targets LLVM IR. The pipeline has these stages:

```
Source text
  → Lexer (tokens)
  → Untyped AST (parse)
  → Typed AST (type-check)
  → LLVM IR codegen
```

### Lexer (`src/lexer/`)

Tokenizes `.lm` source into a flat token stream. `token.rs` defines `TokenType` with variants `ExpressionToken`, `StatementToken`, `FeatureToken`, `KeywordToken`. Specialized sub-parsers in `token_parsers/` handle keywords, imports, expressions, and user-defined types.

### Untyped AST (`src/ast/`)

Parses tokens into an AST without type resolution. Top-level items (functions, structs) live in `items/`; statement types in `statements/`. `ty.rs` represents the syntactic type grammar: `Typename`, `Reference` (`T&`), `Pointer` (`T*`), `Slice` (`T[]`), `Array` (`T[N]`).

### Type System & Typed AST (`src/typed_ast/`)

`typing/tcontext.rs` — `TypeContext` holds the canonical set of types (primitives: `int8`–`int64`, `uint8`–`uint64`, `float32`/`float64`, `bool`) and maps `TypeId` ↔ `TypeInfo`. `typing/scope.rs` manages symbol tables for variable resolution. `typing/ty.rs` defines `TypeId`, `TypeInfo`, and `StructId`. The `ast/` sub-tree mirrors the untyped AST but every node carries a resolved `TypeId`.

### Compiler (`src/compiler/`)

`mod.rs` — `Compiler` is the top-level orchestrator. It owns a `ModuleMap` of source modules, a `TypeContext`, and accumulates errors/warnings (`CompileMessage`). Key methods: `add_module`, `create_typed_ast`, `raise_compile_errors`, `raise_compile_warnings`.

`modules.rs` — handles module loading, dependency tracking, and resolution of `use` imports.

`codegen.rs` — emits LLVM IR using Inkwell. Walks the typed AST and builds LLVM values/functions.

`compile_message.rs` — diagnostic structs with source-location info for error/warning reporting.

### Constant Evaluation (`src/consteval/`)

Evaluates compile-time constant expressions before codegen.

### Common Utilities (`src/common/`)

`sourcemap.rs` / `source_owner.rs` — track source file locations for diagnostics. `operator.rs` — operator enum. `utils/runtime_static.rs` — wrapper that lets LLVM `Context` be stored in a `static`-like fashion (required because Inkwell types are tied to a `Context` lifetime).

## Language Syntax (Lambda)

```
use module_name              // import

struct Name {
    field: type,
}

fn name(param: type) = ReturnType {  // = ReturnType omitted for void
    x: int64 = expr;
    return expr;
}
```

Type modifiers compose right-to-left: `T&*[][N]` = array of N slices of pointers to references of T.

## Entry Point Flow (`src/main.rs`)

1. Parse CLI args (list of `.lm` files) via `clap`.
2. Create an Inkwell `Context` wrapped in `RuntimeStatic`.
3. For each file: `compiler.add_module(...)` — lexes and parses into untyped AST.
4. `raise_compile_errors` — abort if parse errors.
5. `compiler.create_typed_ast()` — type-check all modules.
6. `raise_compile_errors` — abort if type errors.
7. Print untyped and typed ASTs (debug output; codegen emission is the next step).
