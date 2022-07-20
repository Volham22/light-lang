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

## Pointers and heap allocation

Sometimes we don't know in the exact size of an array at compile time, so we
can't store it in the stack. That's where heap allocation comes in.

### Pointers
Pointers are declared this way :

```js
let my_ptr: ptr number = null;
```

We can a variable address like this :
```js
let answer: number = 42;
let ans_ptr: ptr number = addrof answer;
printf("%p", ans_ptr); // some address ... 0x[...]
```

### Heap allocation
We can use the libc's malloc function like this :

```js
let my_ptr: ptr number = malloc(8);
deref my_ptr = 32;
free(my_ptr);
```

#### Dynamic arrays
```js
let array_size: number = some_function(); // returns 5
let dyn_array: ptr number = malloc(8 * array_size);
dyn_array[1] = 43;
dyn_array[5] = 12; // Out of bounds access! May raise in a SIGSEGV on Unix systems.
```

### Void pointer
It acts like C's `void*`.

```js
let answer: number = 4;
let greeting: string = "hi mom!";
let void_ptr: ptr void = addrof answer;
void_ptr = addrof void_ptr;
printf("%p", void_ptr); // some address ... 0x[...]
```

**A void ptr cannot be dereferenced!**


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

### Heap allocated random array

```js
fn printf(msg: string, n: number): number;
fn puts(msg: string): number;
fn malloc(size: number): ptr void;
fn free(freed_ptr: ptr void): void;
fn rand(): number;
fn srand(seed: number): void;
fn time(_: ptr number): number;

fn print_number(n: number): void {
   printf("%d", n);
   puts("");
}

fn fill_array_of_random(arr: ptr number, size: number): void {
    srand(time(null));

    for let i: number = 0; i < size; i = i + 1; {
        arr[i] = rand() % 100;
    }
}

// Heap allocate an array of 100000 numbers
fn main(): void {
   let array_size: number = 100000;
   let dyn_arr: ptr number = malloc(8 * array_size);
   fill_array_of_random(dyn_arr, array_size);

   for let i: number = 0; i < array_size; i = i + 1; {
       print_number(dyn_arr[i]);
   }

   printf("%d", array_size); puts(" Numbers printed");
   free(dyn_arr);
}
```
