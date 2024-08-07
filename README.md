# Rust Memory Ownership and Borrowing

**Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.**

Limitations of Rust:
- Steep learning curve
- Fewer libraries and frameworks
- Smaller community (yet it is very active and helpful)
- A static type system that can be restrictive

**IDEs to use**

- RustRover: an IDE free for Non-Commercial use.

  No configuration needed for running and debugging Rust code

- Visual Studio Code:

  Extensions
  - rust-analyzer
  - CodeLLDB (for debugging under Linux)

> cargo init

## Memory management approaches
- Manual memory management: c, c++
    - Pros: Full control, Fast, predictable, and efficient
    - Cons: Prone to memory leaks, buffer overflows, and dangling pointers
- Garbage collection: Java, C#
    - Pros: Automatic memory management, reduces the risk of errors
    - Cons: Unpredictable, can cause performance issues
- Ownership and borrowing: Rust
    - Pros: Full control, no garbage collector, no runtime overhead
    - Cons: Learning curve
- Other approaches:
  - Automatic memory management with reference counting: Swift
  - Automatic reference counting: Objective-C
  - Automatic memory management: Python, JavaScript

### Rust is strongly typed and statically typed
- All types must be known at compile time
- Assigning a value of one type to a variable of another type will result in a compile-time error
- Rust has type inference

#### Primitive types (This means that they're built-in to the language)
- Scalar types (contain a single value; there are five primitive scalar types)
  - Integers
    - Signed: i8, i16, i32, i64, i128, isize
    - Unsigned: u8, u16, u32, u64, u128, usize
  - Floating-point numbers
    - f32, f64
  - Booleans
    - bool (1 byte in size) (true, false)
  - Characters (a 4-byte data type; used to store single characters)
    - char (4 bytes in size, represents a Unicode Scalar Value)
  - Unit type
    - Only possible value is an empty tuple: ()
    
- Compound types (contain multiple values in one type; there are four primitive compound types)
  - Tuples (fixed-size list of elements of different types)

    *let tuple: (i32, f64, u8) = (500, 6.4, 1);*
  - Arrays (fixed-size list of elements of the same type; arrays are immutable by default)
  
    *let a: [i32; 5] = [1, 2, 3, 4, 5];*
  - Slices: reference to a contiguous sequence of elements in a collection. They are dynamic in size.
            
        - Array slices: &[T], &mut [T]
              let a = [1, 2, 3, 4, 5];
              let slice = &a[1..3];
        - String slices: &str
              let s = String::from("hello");
              let slice = &str[1..3];
  - Strings: There are two types of strings in Rust: String (String Object) and &str (String literal).
  
        - String: Growable (mutable), heap-allocated data structure. It is not in the core language, but provided by the standard library.
                let s = String::from("hello");
                let name = String::new();
                name.push_str("hello");
        - &str: String slice, a reference to a sequence of UTF-8 bytes stored elsewhere. It is a primitive type, and it is immutable (not growable).
                let s = "hello";

Variables with a known, fixed size will be allocated on the stack

Variables with an unknown or changeable size at compile time will be allocated on the heap

#### Mutability
- Variables are immutable by default

#### Scope
- The scope of a variable is the range within a program for which the variable is valid

### Stack and heap

Languages with Garbage Collectors e.g. Java, C#, Python, JavaScript, will often
abstract away complexities of the stack and heap from the developers.

Rust wants you to understand how they work so that you can make efficient design decisions.

- Stack
  - LIFO (Last In, First Out)
  - Fixed size
  - Fast access
  - Data stored in the stack must have a known, fixed size

- Heap
  - Less organized
  - Stores data of unknown size at compile time or a size that might change
  - Slower access
  - Data stored in the heap is allocated at runtime and must be cleaned up when it is no longer needed

#### Ownership
- Ownership is a set of rules that governs how Rust manages memory. It helps ensure
the memory safety of your program without needing a garbage collector.

## Managing memory with ownership

**The two pillars of Rust are speed and safety.**

### Rules of Ownership
- Each value in Rust has an owner
- There can be only one owner at a time
- When the owner goes out of scope, the value will be dropped

### Rust borrow checker enforces these rules at compile time
1. Ensure lifetimes are correct
2. Prevent moving a value while it is borrowed
3. You cannot move the value twice
4. You cannot access a place while it is mutably borrowed
5. You cannot mutate a place while it is immutably borrowed
6. All variables are initialized before use

**Rust borrow checker is a punishment.**
**Rust borrow checker is your friend. Let it help you. Don't fight it.**

### Copyable data types
- Primitive data types
- Known size at compile time
- Stored on the stack
- Data types that implement the Copy trait

**Data types that implement the Copy trait "ignore" the Ownership Rules.**

**The copy trait is implemented for all primitive data types stored on the stack.**

### Non-copyable data types
- Data types that are stored on the heap

### Copy and Clone Traits

- Modify the value of the previous owner won't change the value of the current owenr
- Inheritating `#[derive(Clone)]`; It is deep copy.

## Borrowing values by reference

"We call the action of creating a reference borrowing. As in real life, if a person
owns something, you can borrow it from them. When you're done, you have to give it back.
You don't own it" -- The Rust Book, Chapter 4

There are two types of references
1. Immutable References
2. Mutable Referenecs
   
### Immutable References

declare references:

    let x: String = String::from("Hello World!");
    let y: &String = &x;

### Mutable Referenecs

To make a variable mutable, you add *mut* in front of the variable name.
 
To get a mut reference to variable, you add *&mut* in front of a variable.

To declare a variable as a mut reference to other variable, addd *: &mut variable_type* after the variable.

    let mut x: String = String::from("Hello World!");
    let y: &mut String = &mut x;

### Reference Restrictions
- References can never be null
- Multiple immutable references to the same are allowed
- Only one mutable reference to the same value per scope
- No mixing of mutable and immutable referenecs at the same time


### String Slices

- String 
  - Standard Library (std::string module)
  - Growable
  - Mutable
  - Own their data
- String Slice
  - Reference to a portion of String
  - Don't own the data they reference
  - Data type &str
  - Immutable
- How to choose between String and String Slice
  - Use String when you need to own the string data (modify, etc. heap allocated and growable)
  - Use &str when you only need to borrow a string (a simple reference).
  - Consider performance (If you don't need to modify the string, use &str is more efficient)

## Lifetimes and borrow checker

- borrow checker
  - Compares scopes to determine whether all borrows are valid
  
- lifetime
  - variables are out of scope
  - explicitly annotating the variable's lifetime
  - lifetime annotation
    - Explicitly defines a generic liftime for parameters
    - Must begin with an apostrophe (') symbol
    - Names are conventionally single lowercase letters
  
    code not working:

        fn best_fuel(x: &str, y; &str) -> &str {
          if x.len() > y.len() {
            x
          } else {
            y
          }
        }

    code that works:

        fn best_fuel<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
              x
            } else {
              y
            }
        }

    the lifetime annotation tells the lifetime of return value is as long as the lifetime of the two input parameters x and y

### Lifetime Elision rules (the compiler will apply them in the code)

- Set of rules for the compiler to analyze reference lifetimes
- Describes situations that do not require erxplicit lifetime annotations
- If any ambiguity remains, explicit annotation will b required
- There are currently three Elision rules
  - Rule #1. Each input parameter that is a reference is assigned its own lifetime
  - Rule #2. If there is exactly one input lifetime, assign it to all output lifetimes
  - Rule #3. If there is a &self or &mut self input parameter, its lifetime will be assigned to all output lifetimes
  - To be determined

If after applying the three rules, you still can't identify the lifetime, explicit lifetime rules should be provided.

