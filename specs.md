# Light language
A staticaly typed language

## Variables
### Builtin types

* number: signed 64 bits numbers
* real: 64 bits floating point numbers
* bool: a boolean can be true or false
* string: TODO

### Syntax

Variables can be declared with the following syntax:
All variables are mutable and must be initialized with a value.

```
let <var_identifier>: <type> = <init_expr>;
```

## Control flow

### While loop

Classic while loop

```
while <expr> {
    <statement>*
}
```

### For loop
C-styled for loop

```
for <init>;<loop_expr>;<next_expr> {
    <statement>*
}
```

### Loop

Loops forever (syntactic sugar of while loop)

```
loop {

}
```

All loops can be stopped with the `break` keyword. An iteration can be skipped
with the `continue` keyword.

## Functions

By default functions are not exported (static keyword in C). A function must be
exported with the `export` keyword.

```
[export] fn <identifier>(<arg_identifier>: type, <other_identifier>: type): <return_type> {
    <statement>*
    return <expr>;
}

```

## Code samples

### Add

```
fn add(a: number, b: number): number {
return a + b;
}
```

### Factorial

```
fn fact(n: number): number {
    let result: number = 1;
    for i: number = 1; i <= n; i = i + 1 {
        result  = result * i;
    }

    return result;
}

```
