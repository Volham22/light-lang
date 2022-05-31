# Light lang

Light is a staticaly typed compiled programming language that aims to be simple.
Note that this is in early stage for now and the compiler is unstable.

## How to use it

Here is a simple main function for light :

```js
fn main(): number {
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

## Credits

- [Logos](https://docs.rs/logos/latest/logos/)
- [inkwell](https://github.com/TheDan64/inkwell)

