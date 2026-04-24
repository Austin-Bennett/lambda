# Lambda
Compiled language based on LLVM, compiler written in rust

## Usage
```
lambda 
[--output, -o <output> (default: a.out)]
[--shared]
[-l, --link <shared library>]
[-L <linkage directory>]
[--optimize, -O <none, 1, 2, 3>]
files...
```
**Debug Options**
```
[--output-ir <path>]
[--debug-ast]
[--no-codegen]
 ```

### Compile options
**output:** the output file path

**debug-ast:** print the AST as a debug step

**shared:** compile this as a shared (.so or .dll) library

**link:** link to a shared library, can pass this argument more than once

**L:** specify a linkage directory for linking to a file

**optimize:** optimization level to apply to the generated IR

**files:** files to compile

### Debug Options

**output-ir:** output the generated llvm ir to the specified path

**no-codegen:** Tell the compiler to stop after AST generation


## Lang features

### Structures

**Syntax:**

```lambda
struct MyStruct {
  x: int32,
  y: uint32,
}
```

**Allows the programmer to define their own data types**

*Note: There is no syntax for constructing a structure yet*


### Functions

**Syntax:**

```lambda
[extern] fn add(a: int32, b: int32) = int32 {
  return a + b;
}
```

**A function is a callable item of code**

declare using the following structure:
`[extern] fn NAME(arg1: type1, arg2: type2 ... argn: typen) = return_type`
**Extern tells the compiler either a) the symbol is defined elsewhere or b) this symbol should be exported to a shared library**


### Statements

#### Expressions

An expression is anything that performs actions on values, for example, `a+b` is an expression, `2b` is an expression, and `add(a, b)` is also an expression

**Notes:**

putting a number in front of any identifier (such as the `2b` example) is the same as multiplication
all primitive types can use the syntax `a(b)` for multiplication as well (i.e `2(2)` will return `4`)


#### Operators

|                         Name                          | Token |
|:-----------------------------------------------------:|:-----:|
|                       Addition                        |   +   |
|                Subtraction / Negation                 |   -   |
|             Multiplication / Dereference              |   *   |
|                       Division                        |   /   |
|                        Assign                         |   =   |
| Address Of / Bitwise AND / Boolean NS<sup>1</sup> AND |   &   |
|        Bitwise OR / Boolean NS<sup>1</sup> OR         |  \|   |
|                      Bitwise XOR                      |   ^   |
|                 Bitwise / Boolean NOT                 |   !   |
|                  Bitwise Shift Right                  |  \>>  |
|                  Bitwise Shift Left                   |  <<   |
|                      Boolean AND                      |  &&   |
|                      Boolean OR                       | \|\|  |

1. NS stands for Non-Short Circuiting
2. Bitwise operators can only be applied to integer types
3. All of these operators have assignment variants (+=)

#### Variable Declarations


**Syntax:**

`a: int32 = 2 + 2;`

Declare any variable using the structure:

`NAME: TYPE = EXPRESSION`


#### Return Statements

**Syntax:**

`return 2a; //return 2 * a`

returns a value from a function

#### Casting

**Syntax**
```lambda
a: i32 = 2;
b: i64 = a as i64 + 2
```

casts a value to the specified type if it can
the structure is:

`VALUE as TYPE`

**Notes:**
any primitive (floatN, intN, uintN) can be cast between each-other
any pointer can be cast to a usize, and any integer to a pointer
and a boolean can be cast as a uint8 but not vice-versa

#### Pointer arithmetic

`&ident` gives a reference to an identifier

`ref as *type` casts a reference to a pointer

`*ptr` turns the pointer into a reference again

`*ref` reads the reference

```lambda
a: i32 = 10;
a_ref: i32& = &a;
a_ptr = a_ref as i32*;
a_ref_2 = *a_ptr;
a_clone: i32 = *a_ref_2;
a_clone = 3;
*a_ref = a_clone;
```



#### if statements
```lambda
if [boolean] {
    ...code
} else <if [boolean]> {
    ...code
} ...
```

#### while statements
```lambda
while boolean {
    ...code
}
```

#### structure initialization

*Syntax:*
```lambda
struct myStruct {
    //public makes this member usable outside the structures context
    public a: int32,
    b: int32,
}

//initializer is basically a function that takes 
//input variables and returns the structure
//like rust- custom constructors must be made as static methods
my_struct: myStruct = myStruct(1, 2);

//the . operator allows one to access a structures inner members
a: int32 = myStruct.a;
```




#### modify blocks

similar to a rust impl block, but can be added to any type even outside its defining module

*Syntax*

```lambda
struct Point {
    public float32 x;
    public float32 y;
}

modify Point {
    public length2(self) = float32 {
        //self is a reference to this object
        return self.x * self.x + self.y * self.y;
    }
}

//modify primitive types too- you can modify any valid type
modify int32 {
    public max(self, other: int32) = int32 {
        if *self > other {
            return *self;
        }
        return other;
    }
}
```


*Operator Overloading*

this is also done inside a modify block:

*Syntax*
```lambda
//operator overloads are always public, if a public is put before a operator method
//its allowed, but will cause a compiler warning
operator operator_name(self, args...) = result { ... }
```

*Operator methods:*

```lambda
//addition, a + b = c
operator add(self, other: T) = R { ... }

//subtraction, a - b = c
operator sub(self, other: T) = R { ... }

//multiplication, a * b = c
operator mul(self, other: T) = R { ... }

//division, a / b = c
operator div(self, other: T) = R { ... }

//assignment, a = b
//note that if T is Self, then it overrides the default move/copy operator for struct types, 
//you cannot override it on primitives however, no other operator supports this besides assign
operator assign(self, other: T) = R { ... }

//comparison operator, a < b, a <= b, a > b, a >= b, a == b, a != b
//returns 0 if equal, 1 if greater than, and -1 if less than
//maybe todo: implement enums in lambda and replace this with a enum
operator cmp(self, other: T) = int8 { ... }

//drop operator / destructor operator
operator drop(self) { ... }
```