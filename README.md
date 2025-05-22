# tin 🥫

> [!warning]
> Tin is a _super-alpha_ (read: does not currently work) programming language.

Tin is a programming language inspired by Rust, Kotlin, and Go. Some of its
key ideas include:

- __Algebraic data types__: `struct`s and `enum`s, like in Rust.
- __Typeclasses__: interfaces to constrain generic code, like in Rust.
- __Null safety__: the type system knows when types might be null, and makes
  you handle the null case, like in Kotlin.
- __Garbage collection__: values are garbage collected, rather than using a
  borrow checker like Rust does.
- __Compiles to native binaries__: instead of being interpreted or targeting
  a virtual machine, like in Go and Rust.
