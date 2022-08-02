# Light lang

Light is a staticaly typed compiled programming language that aims to be simple.
Note that this is in early stage for now and the compiler is unstable.

## How to use it

Here is a simple main function for light :

``` js
fn main(): number {
    return 0;
}
```

And of course the traditional "Hello World!" program :

``` js
fn puts(message: string): number;

fn main(): number {
   puts("Hello World!");
   return 0;
}
```

More informations can be found [here](specs.md).

## How to build

### Required

* Rust toolchain (use rustup)
* LLVM13 (for inkwell)

then use cargo to build the project (and launch tests).

### Compile and launch a program

Since `cargo install` is not supported yet. The best way to invoke the compiler
is to run the binary from the `target` directory.

``` sh
# For debug build
$ ./target/debug/lightc hello.lht

# For release build
$ ./target/release/lightc hello.lht

# Launch the executable
$ ./program
```

* `-c` option generates only objects files (like gcc and clang).
* `-o` option allows to specify the generated executable name. Default is `program`.
* `-p` prints the generated llvm-ir code (useful for debugging)

More options and their descriptions are described with the `-h` flag.

## TODO

Here a short list of features to implements

- A `byte` type for dynamic strings
- Each module should have its own namespace to avoid name collision.
- Implement `struct` member functions
- Implement casts (the compiler should check that casts are valid)
- Better error reporting with a proper error location (e.g rustc, gcc)
- Debug randoms errors because there's a lot of bugs
- Smarter checks to make sure a function returns in every paths.

## Credits

- [Logos](https://docs.rs/logos/latest/logos/)
- [inkwell](https://github.com/TheDan64/inkwell)

