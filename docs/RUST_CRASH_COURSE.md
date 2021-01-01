# Rust Crash Course

## 1. Variables

```rust
fn main() {
    let hello = 1;
}
```

### Immutable by default

```rust
fn main() {
    let hello = 1;
    // COMPILE ERROR: reassignment of an immutable
    hello = 3;
}
```

```rust
fn main() {
    let mut hello = 1;
    hello = 3;
}
```

### Constants

```rust

// Defined in the global scope
const WORLD: i32 = 5;

fn main() {
    let mut hello = 1;
    hello = 3;
    
    // Specify the type of the constant (integer 32 bit)
    const WORLD_2: i32 = 5;
    println!("Hello, {}", WORLD);
}
```

### Type conversion & Use shadowing to reuse the same variable name

Java doesn't allow the same variable name:

```java
public class App {
  public static void main() {
    Integer helloNumber = 5;
    String helloString = "5";
    System.out.println(helloNumber);
    System.out.println(helloString);
  }
}
```

Rust does:

```rust
fn main() {
    let hello_str = "5";
    let hello = 3;
    let hello = hello_str.parse::<i32>().unwrap();
    println!("Hello, {}", hello);
}
```

### Specify the type explicitly

```rust
fn main() {
    let hello: i32 = 5;
    println!("Hello, {}", hello);
}
```

# 2. Data Types

Compiler can infer the type of a variable automatically

```rust
fn main() {
    let hello = 5; // => automatically infered
    
    let hello2 = "5".parse().unwrap(); // COMPILE ERROR => not possible to infer type

    let hello3: i32 = "5".parse().unwrap(); // ok
    println!("Hello, {}", hello2);
}
```

### Integer types

* Signed (prefix: i) and Unsigned (prefix: u)
* Length 8-bit, 16-bit, 32-bit, 64-bit, 128-bit

#### Casting

```rust
fn main() {
    let hello: u32 = 5;
    let world = hello as i32;
    println!("Hello, {}", hello2);
}
```

### Floating Point

* f32, f64

```rust
fn main() {
    let hello: f32 = 1.0;
    println!("Hello, {}", hello);
    let world: f64 = 2.0;
    println!("Hello, {}", world);
    let result = hello + 2.0;
    println!("Hello, {}", result); // => 3
}
```

#### No implicit type conversion

```rust
fn main() {
    let hello: f32 = 1.0;
    let world: f64 = 2.0;
    let result = hello + world; // COMPILE ERROR: two different data types!
}
```

#### Promote / demote

```rust
fn main() {
    let hello: f32 = 1.0;
    let world: f64 = 2.0;
    let result = hello + world as f32; // compiles, but loss of precision
    println!("Hello, {}", result); // => 3
}
```

```rust
fn main() {
    let hello: f32 = 1.0;
    let world: f64 = 2.0;
    let result = hello as f64 + world; // No loss of precision
    println!("Hello, {}", result); // => 3
}
```

### Boolean

```rust
fn main() {
    let hello: bool = true;
    let world: bool = false;
    println!("Hello, {} {}", hello, world);
}
```

### Char

* 4-Bytes (not only 1-Byte ASCII)
* Unicode scalar value

```rust
fn main() {
    let hello: char = 't';
    println!("Hello, {}", hello);
}
```

### Tuple

```rust
fn main() {
    let hello = (5, 2.0, 't');
    println!("Hello, {:?}", hello); // Debug-Print
}
```

#### Destruct tuple (like python)

```rust
fn main() {
    let hello = (5, 2.0, 't');
    let (x, y, z) = hello;
    println!("Hello, {} {} {}", x, y, z);
}
```

### Array

```rust
fn main() {
    let hello = [1, 2, 3]; // Fixed length
    println!("Hello, {:?}", hello);
}
```

```rust
fn main() {
    let hello: [i32; 5] = [1, 2, 3, 4, 5]; // Specify the type and length
    println!("Hello, {:?}", hello);
    println!("Hello, {:?}", hello[0]); // The first element
    println!("Hello, {:?}", hello[5]); // COMPILE ERROR: index out of bound
}
```

Caution: runtime errors can still occur with runtime variables:

```rust
fn main() {
    let hello: [i32; 5] = [1, 2, 3, 4, 5];
    let index = 10;
    println!("Hello, {:?}", hello[index]); // RUNTIME ERROR: index out of bound
}
```

## 3. Functions

```rust
fn hello(x: i32) {
    println!("Hello, {:?}", x);
}

fn main() {
    hello(5)
}
```

```rust
fn add(x: i32) -> i32 {
    return x + 5;
}

fn main() {
    println!("{}", add(5));
}
```

```rust
fn add(x: i32) -> i32 {
    // return can be omitted, but the semicolon have to be omitted also
    x + 5
}

fn main() {
    println!("{}", add(5));
}
```

### Expressions

In Rust use the expression style:

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    println!("{}", add(5, 7));
}
```

```rust
fn add(x: i32, y: i32) -> i32 {
    let z = x + y;
    z
}

fn main() {
    println!("{}", add(5, 7));
}
```

## 4. If expressions

```rust
fn main() {
    let hello: i32 = 5;
    
    // No parenthesis
    if hello > 2 {
        println!("Hello!");
    } else if hello == 5 {
        println!("World!");
    } else {
        println!("World!");
    }
}
```

```rust
fn main() {
    let hello: i32 = 5;

    // don't do this in rust:
    let mut name: &str = "";
    if hello > 3 {
        name = "joh";
    } else {
        name = "mike";
    }
    println!("Hello {}", name);
}
```

```rust
fn main() {
    let hello: i32 = 5;

    // In rust we write instead (a bit like a ternary operator, but much more powerful)
    let name: &str () = if hello > 3 {
        "joh"
    } else {
        "mike"
    }; // <= semicolon needed

    println!("Hello {}", name);
}
```

```rust
fn main() {
    let hello: i32 = 5;
    let name: &str () = if hello > 3 {
        "joh"
    } else {
        5 // COMPILE ERROR: incompatible / incorrect return types
    };

    println!("Hello {}", name);
}
```

## 5. Loops

### Endless Loops

```rust
fn main() {
    loop {
        println!("Hello");
    }
}
```

```rust
fn main() {
    let mut result: i32 = 0;
    let mut counter: i32 = 0;
    loop {
        println!("Hello");
        counter += 1;
        if counter == 10 {
            result = counter * 2;
            break;
        }
    }
    println("{}", result);
}
```

More elegant using expression:

```rust
fn main() {
    let mut counter: i32 = 0;
    let result: () = loop { // use loop as expression
        println!("Hello");
        counter += 1;
        if counter == 10 {
            // break acts like a return statement:
            // return value of the expression
            break counter * 2;
        }
    }; // again: semicolon needed
    println("{}", result);
}
```

### while

```rust
fn main() {
    let mut counter: i32 = 0;
    while counter < 10 {
        println!("Hello");
        counter += 1;
    }
}
```

### for

Not like in C++ and Java, more like Python ranges:

```rust
fn main() {
    for i in 0..10 {
        println!("Hello {}", i);
    }
}
```

#### Iterator

```rust
fn main() {
    let array = [1, 2, 3, 4];
    for i in array.iter() {
        println!("Hello {}", i);
    }
}
```

#### Reverse Iterator

```rust
fn main() {
    let array = [1, 2, 3, 4];
    for i in array.iter().rev() {
        println!("Hello {}", i);
    }
}
```

## 6. Ownership

### Basic Types

Integers are one of the basic types, which can be copied

```rust
fn main() {
    let x = 5;
    let y = x; // Copy
    println!("{}", y);
}
```

```rust
fn main() {
    let n = 10;
    println!("{}", n);
    p(n);
}

fn p(n: i32) {
    println!("{}", n);
}
```

#### Pass by copy

```rust
fn main() {
    let n = 10;
    p(n); // Passed by copy
    println!("{}", n);
}

fn p(n: i32) {
    println!("{}", n);
}
```

### Complex Types

String is not a basic type!

```rust
fn main() {
    let s1 = String::from("first"); // pointer to the heap location
    let s2 = s1; // s1 moved (copy would be very expensive!)
    // s1 is now invalidated
    // s2 is now the owner
    // if s2 goes out of scope, the memory gets released
    println!("{}", s1); // COMPILE ERROR: s1 no more available
}
```


```rust
fn main() {
    let s = String::from("abc");
    takes_ownership(s); // Takes the ownership of s
    println!("{}", s); // COMPILE ERROR: ownership has been taken by function
}

fn takes_ownership(s: String) {
    println!("inside function {}", s)
}
```

This works:

```rust
fn main() {
    let s = String::from("abc");
    takes_ownership(s.clone()); // Clone the string
    println!("{}", s);
}

fn takes_ownership(s: String) {
    println!("inside function {}", s)
}
```

#### Ownership moved

Short version:

```rust
fn main() {
    let s = hello();
    println!("{}", s);
}

fn hello() -> String {
    String::from("test")
} // ownership moved to "s"
```

Long version:

```rust
fn main() {
    let s = hello(); // Moved
    println!("{}", s);
}

fn hello() -> String {
    let a = String::from("test");
    a
} // a is out of scope; instead dropping the value, it's moved to "s"
```

#### No more ownership: dropping the variable

```rust
fn main() {
    hello(); // The result of the function is not assigned to a variable => a is dropped and no more accessible
}

fn hello() -> String {
    String::from("test");
    a
} // a is out of scope; instead dropping the value, it's moved to "s"
```

## 7. References

### Move-Move is really inconvenient

```rust
fn main() {
    let s = String::from("abc");
    let out = hello(s);
    println!("{}", s);
}

fn hello(a: String) -> String {
    println!("Inside {}", a);
    a
}
```

### Passing a reference

```rust
fn main() {
    let s = String::from("abc"); // s is the owner of the string
    hello(&s); // Passing a reference of "s"
    println!("{}", s);
}

fn hello(a: &String) { // <- Expecting a reference
    println!("Inside {}", a); // print without taking the ownership of the string
    // No return value
}
```

### References are immutable

```rust
fn main() {
    let s = String::from("abc");
    hello(&s);
    println!("{}", s);
}

fn hello(a: &String) {
    println!("Inside {}", a);
    a.push_str("suffix") // COMPILE ERROR: references are read only; mutation is not possible
}
```

### Mutable references

```rust
fn main() {
    let s = String::from("abc");
    hello(&mut s); // COMPILE ERROR: after returning from the function, s cannot be modified because s is immutable
    println!("{}", s);
}

fn hello(a: &mut String) { // The reference is marked as mutable
    println!("Inside {}", a);
    a.push_str("suffix") // The reference is modified
}
```

```rust
fn main() {
    let mut s = String::from("abc"); // Now s is mutable
    hello(&mut s); // so the modified reference can be assigned to the mutable "s"
    println!("{}", s);
}

fn hello(a: &mut String) { // The reference is marked as mutable
    println!("Inside {}", a);
    a.push_str("suffix")
}
```

### Thread safety: Borrow Checks

In Rust there is only one writer allowed at a time. This prevents race conditions.
Means: It's not possible to have multiple threads that writes on the same memory location.

```rust
fn main() {
    let mut s = String::from("abc");
    let z = &mut s;
    let t = &mut s; // COMPILE ERROR: the mutable is borrowed multiple times
    hello(z);
    println!("{}", s);
}

fn hello(a: &mut String) {
    println!("Inside {}", a);
    a.push_str("suffix")
}
```

But it's allowed to have multiple immutable references.

### Thread safety: Lifetime Checks

```rust
fn main() {
    let s = hello();
   println!("{}", s);
}

fn hello() -> &String {
    let s = String::from("hello");
    s
} // COMPILE ERROR: s is dropped at the end of the function. Cannot return a reference on a dropped variable
```
