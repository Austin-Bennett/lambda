# Lambda
Compiled language based on LLVM, compiler written in rust

## Usage
`lambda [--output, -o <output> (default: a.out)] [--debug_ast] [--shared] [-l, --link <shared library>] [-L <linkage directory>] files...`

**output:** the output file path

**debug_ast:** print the AST as a debug step

**shared:** compile this as a shared (.so or .dll) library

**link:** link to a shared library, can pass this argument more than once

**L:** specify a linkage directory for linking to a file

**files:** files to compile

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
    public new(float32 x, float32 y) = Point {
        return Point(x, y);
    }

    public length2(self) = float32 {
        //self is a reference to this object
        //self.x is a reference aswell
        return *self.x * *self.x + *self.y * *self.y;
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

*Operator overloading is done inside modify blocks aswell*
```lambda
modify Point {
    //fn is replaced with operator, and the name of the operand
    public operator add(self, other: Point) = Point {
        return Point(*self.x + *other.x, *self.y + *other.y);
    }
}
```

*Operators*


| Operator |         Signature          | Description |
|:--------:|:--------------------------:|-------------|
|    +     |  add(self, other: T) = R   | a + b       |
|    -     |  sub(self, other: T) = R   | a - b       |
|    *     |  mul(self, other: T) = R   | a * b       |
|    /     |  div(self, other: T) = R   | a / b       |
|    =     | assign(self, other: T) = R | a = b       |
|   as T   |     convert(self) = T      | a as T      |
|   drop   |         drop(self)         | destructor  |