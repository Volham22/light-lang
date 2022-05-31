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

    - Rust toolchain (use rustup)
    - LLVM13 (for inkwell)
    
then use cargo to build the project (and launch tests).

## Credits

- [Logos](https://docs.rs/logos/latest/logos/)
- [inkwell](https://github.com/TheDan64/inkwell)

