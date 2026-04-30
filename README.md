# Lambda
Compiled language based on LLVM, compiler written in Rust.

## Usage for building by command line
```
lambda
  [-o, --output <path>]          output file (default: a.out)
  [-O, --optimize <none|1|2|3>]  optimization level (default: 2)
  [-l, --link <lib>]             link a shared library (repeatable)
  [-L <path>]                    add a library search path (repeatable)
  [-M, --modules <path>]         add a module search path (repeatable)
  [--shared]                     compile as a shared library (.so / .dll)
  <files...>                     .lm source files and/or linker inputs
```

**Debug options**
```
  [--output-ir <path>]   write generated LLVM IR to a file
  [--debug-ast]          print the untyped and typed ASTs
  [--no-codegen]         stop after AST generation (no object/binary output)
```



### Compile options

**output:** output file path.

**optimize:** optimization level applied to the generated LLVM IR.

**link / L:** pass `-l`/`-L` flags to the linker. Can be repeated.

**modules / M:** add directories to the module search path. Can be repeated.

**shared:** produce a shared library instead of an executable.

**files:** a mix of `.lm` Lambda source files and linker inputs. Any file whose
extension is not `.lm` (e.g. `.o`, `.a`, `.so`) is passed directly to the
linker, making it easy to link against pre-compiled C, C++, or Rust objects:

## Building with a json file
```lambda
  --build, -B <file>
```

**build:** grabs compiler arguments from json file

**Example JSON**
```json
{
  "output": "main",
  "output_ir": "main.ll",
  "libs": [
    "raylib",
    "GL",
    "pthread",
    "dl",
    "rt",
    "X11"
  ],
  "lib_paths": [

  ],
  "module_paths": [
    "../lib/"
  ],
  "files": [
    "main.lm"
  ]
}
```


**Linking with other object files:**
```bash
# Compile a C file to an object
clang -c mylib.c -o mylib.o

# Link it alongside Lambda source
lambda -o myapp main.lm mylib.o -l m
```

---

## Module system

Source files can import other modules with `use`:

```lambda
use mymodule;
use core::imath;
```

The compiler searches for `mymodule.lm` in every directory on the module search
path (`-M` flags, plus `./` by default). Nested paths use `::` separators
(`core::imath` → `core/imath.lm`).

---

## Language features

### Primitive types

| Type      | Description                  |
|-----------|------------------------------|
| `int8`    | 8-bit signed integer         |
| `int16`   | 16-bit signed integer        |
| `int32`   | 32-bit signed integer        |
| `int64`   | 64-bit signed integer        |
| `uint8`   | 8-bit unsigned integer       |
| `uint16`  | 16-bit unsigned integer      |
| `uint32`  | 32-bit unsigned integer      |
| `uint64`  | 64-bit unsigned integer      |
| `float32` | 32-bit floating-point        |
| `float64` | 64-bit floating-point        |
| `bool`    | boolean (`true` / `false`)   |
| `str`     | immutable string slice       |

Type modifiers compose right-to-left:
- `T&` — reference to T
- `T*` — pointer to T
- `T[]` — slice of T
- `T[N]` — array of N T's

---

### Variable declarations

**Explicit type:**
```lambda
x: int32 = 42;
name: str = "hello";
```

**Inferred type (let):**
```lambda
let y = x + 1;
let flag = true;
```

`let` requires an initialiser; the type is inferred from the expression.

---

### Functions

```lambda
fn add(a: int32, b: int32) = int32 {
    return a + b;
}

// void (no = ReturnType)
fn print_sum(a: int32, b: int32) {
    cprint("done\n");
}
```

**Extern functions** declare a symbol defined in another object file / shared
library, or mark a symbol for export from a shared library:

```lambda
extern fn cprint(s: str);
```

**Entry point:** `main` must return `int8` (the process exit code).

```lambda
fn main() = int8 {
    return 0;
}
```

---

### Generics (templates)

Type parameters are declared with `<T, U, ...>` immediately after the keyword.
Lambda uses **monomorphization** — a separate concrete copy is generated for
each unique combination of type arguments.

#### Generic functions

```lambda
fn<T> identity(x: T) = T {
    return x;
}

fn<T> max(a: T, b: T) = T {
    if a > b { return a; }
    return b;
}
```

**Calling a generic function** uses `.< >` syntax at the call site:

```lambda
let v  = identity.<int32>(5);
let mx = max.<float64>(1.5, 2.5);
```

Generic functions can call other generic functions:

```lambda
fn<T> double_identity(x: T) = T {
    return identity.<T>(x);
}
```

#### Generic structs

```lambda
struct<T> Pair {
    a: T,
    b: T,
}

// Instantiate by using the type:
p: Pair<int32>;
p.a = 7;
p.b = 13;
```

#### Generic modify blocks

```lambda
modify<T> Pair<T> {
    public fn sum(self) = T {
        return self.a + self.b;
    }
}
```

Pattern matching in the modify type lets you restrict instantiation:

```lambda
// Only applies when the second type arg is int32
modify<T> Pair<T> { ... }          // applies to Pair<anything>
modify<T> Pair<T, int32> { ... }   // applies only when second arg is int32
```

---

### Structures

```lambda
struct Vec2 {
    x: float32,
    y: float32,
}
```

Members are private by default. Use `public` to expose them:

```lambda
struct Point {
    public x: float32,
    public y: float32,
}
```

Access members with `.`:

```lambda
v: Vec2;
v.x = 1.0;
v.y = 2.0;
```

---

### Modify blocks

A `modify` block adds methods and operator overloads to any type. It can appear
anywhere — even in a different file from the type definition.

```lambda
modify Vec2 {
    // Methods are private by default; public makes them callable from outside
    public fn length_sq(self) = float32 {
        return self.x * self.x + self.y * self.y;
    }

    // Private helper — only callable from within other Vec2 methods
    fn internal_helper(self) = float32 { ... }
}
```

`self` inside a method is a reference to the receiver; mutating it mutates the
original value.

#### Operator overloads

Operator overloads are declared inside a modify block using `fn operator`:

```lambda
modify Vec2 {
    fn operator add(self, rhs: Vec2) = Vec2 {
        r: Vec2;
        r.x = self.x + rhs.x;
        r.y = self.y + rhs.y;
        return r;
    }

    // cmp: return negative / zero / positive (int8)
    fn operator cmp(self, rhs: Vec2) = int8 {
        if self.x < rhs.x { return -1; }
        if self.x > rhs.x { return  1; }
        return 0;
    }
}
```

Operator overloads are always publicly accessible.

**Available operators:**

| Name     | Token(s)              | Notes |
|----------|-----------------------|-------|
| `add`    | `a + b`               | |
| `sub`    | `a - b`               | |
| `mul`    | `a * b`               | |
| `div`    | `a / b`               | |
| `cmp`    | `==` `!=` `<` `>` `<=` `>=` | return `int8`: negative / zero / positive |
| `drop`   | (destructor)          | called when value goes out of scope |

---

### Statements

#### Operators

| Name                         | Token(s)         | Notes |
|------------------------------|------------------|-------|
| Addition                     | `+`              | |
| Subtraction / Negation       | `-`              | |
| Multiplication / Dereference | `*`              | |
| Division                     | `/`              | |
| Assignment                   | `=`              | |
| Address-of / Bitwise AND / Non-short-circuit AND | `&` | |
| Bitwise OR / Non-short-circuit OR | `\|`        | |
| Bitwise XOR                  | `^`              | |
| Bitwise / Boolean NOT        | `!`              | |
| Shift right                  | `>>`             | integers only |
| Shift left                   | `<<`             | integers only |
| Boolean AND (short-circuit)  | `&&`             | |
| Boolean OR (short-circuit)   | `\|\|`           | |

All binary operators have compound-assignment variants: `+=`, `-=`, `*=`, `/=`,
`&=`, `|=`, `^=`, `>>=`, `<<=`, `&&=`, `||=`.

#### Casting

```lambda
big: int64 = 300;
let small = big as int32;   // 300
let byte  = big as int8;    // 44  (300 mod 256)

fl: float32 = 7.9;
let i = fl as int32;        // 7 (truncate toward zero)
```

Any primitive (`intN`, `uintN`, `floatN`, `bool`) can be cast to any other
primitive. Pointers and integers are mutually castable.

#### if / else

```lambda
if condition {
    ...
} else if other {
    ...
} else {
    ...
}
```

#### while

```lambda
while condition {
    ...
}
```

#### Pointer / reference operations

```lambda
a: int32 = 10;
r: int32& = &a;          // reference to a
p: int32* = r as int32*; // reference -> pointer
r2: int32& = *p;         // pointer -> reference
let val: int32 = *r2;    // read through reference
*r = 99;                 // write through reference
```

#### Lambdas

*Of course lambda has lambdas!*

**Syntax:**
```lambda
my_lambda: int32(int32) = lambda(x: int32) = int32 { return x + 1; }
let x = my_lambda(5); //x = 6, 5 + 1 = 6
```

---

### FFI

Declare external C / system functions with `extern fn`:

```lambda
extern fn printf(fmt: str) = int32;
```

Link against pre-compiled object files, archives, or shared libraries by
passing them on the command line alongside `.lm` files (see **Compile options**
above). The compiler routes by extension — anything that isn't `.lm` goes
straight to the linker.
