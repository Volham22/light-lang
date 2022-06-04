# Light language
A staticaly typed language

## Variables
### Builtin types

* number: signed 64 bits numbers
* real: 64 bits floating point numbers
* bool: a boolean can be true or false
* string: it's just strings literals yet

### Syntax

Variables can be declared with the following syntax:
All variables are mutable and must be initialized with a value.

```js
let <var_identifier>: <type> = <init_expr>;
```

## Control flow

### While loop

Classic while loop

```js
while <expr> {
    <statement>*
}
```

### For loop
C-styled for loop (a syntactic sugar of while loops)

```js
for <init>;<loop_expr>;<next_expr> {
    <statement>*
}
```

### Loop
__Not implemented yet__
Loops forever (syntactic sugar of while loop)

```js
loop {

}
```

All loops can be stopped with the `break` keyword. An iteration can be skipped
with the `continue` keyword.

## Functions

By default functions are not exported (static keyword in C). A function must be
exported with the `export` keyword.

Note that the main function is exported by default to avoid linker
errors. So we can omit the `export` keyword on main function.

```js
[export] fn <identifier>(<arg_identifier>: type, <other_identifier>: type): <return_type> {
    <statement>*
    return <expr>;
}

```

## Arrays

We can declare static arrays and use it as C-styled arrays. Arrays are 0 indexed.

```js
let my_array: [number; 10] = 0;

for let i: number = 0; i < 10; i = i + 1; {
    my_array[i];
}
```

## Code samples

### Hello World!

Note that since the standard library is not yet implemented some C standard
library's functions must be forward declared.

``` js
// Forward declare C library functions
// stdio.h
//      puts

fn puts(message: string): number;

fn main(): number {
   puts("Hello World!");
   return 0;
}
```

### Add

```js
fn add(a: number, b: number): number {
    return a + b;
}
```

### Factorial

```js
fn fact(n: number): number {
    let result: number = 1;

    for i: number = 1; i <= n; i = i + 1; {
        result  = result * i;
    }

    return result;
}
```

### Random array
This program will fill an array of 10 random number and print the array

```js
// This program will fill an array of 10 random number and print the array

// Forward declare C library functions
// stdio.h
//      printf
//      puts
// stdlib.h
//      srand
//      rand
// time.h
//      time.h

fn puts(message: string): number;
fn printf(format: string, n: number): number;
fn rand(): number;
fn srand(seed: number): void;
fn time(_: number): number;

fn print_number(n: number): void {
   printf("%d", n);
}

fn main(): number {
   srand(time(0));
   let array: [number; 10] = 0;

   for let i: number = 0; i < 10; i = i + 1; {
        array[i] = rand() % 100;
        print_number(array[i]);
        puts(""); // newline
   }

   return 0;
}
```
