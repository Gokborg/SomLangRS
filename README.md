# somelang-rs
A rust rewrite of SomLangJS with syntax changes

- The current goal is to get all the SomLangJS features in this project with the new syntax.
- The introduction will show exactly what features are currently implemented.

## How To Develop With SomLang
**Code & Compile**: in examples folder there is a 'test.som' that it by default generates asm from

**URCL compilation:** the assembly will generate and be printed to console, for now just copy from there

**Rust compilation:** 
- in somoutput folder there is a 'program.rs' file that generates from the program in 'test.som'
- The 'program.rs' file will run and you will see the output of it in console
- Also a 'program.exe' file generates which you can use.

*These are all subject to change*

## Introduction to SomLang (Current Features Only)

| Arithmetic | Boolean |
| ------|------|
| A + B | A == B |
| A - B |  |
| A * B | A > B |
| A / B | A < B |
|       | A >= B |
|       | A <= B |


### Variables
```
a: uint = 5;
a = 1;
```

### Conditionals
```
a: uint = 10;
if a > 10 {
}
```
