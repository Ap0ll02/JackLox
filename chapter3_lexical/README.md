# Chapter 3 Of Compilers (The Dragon Book)

In this chapter we explored lexical analysis. The programming exercise problem 1 asked us to create a lexical analyzer for a set of tokens
and attribute values brought up in figure 3.10

I decided to make this analyzer print out your tokens at the end, along with show you in real time what the last token type it recognized was.

## Installing

You can install from crates.io! Simply run `cargo install chapter3_lexical` and you will have the project.

## Running

To run this crate, simply use (assuming default paths)
- `/home/USER/.cargo/bin/chapter3_lexical` to run
- Add the cargo installation path to your shells $PATH. After sourcing run `chapter3_lexical`

If your path differs please note I am using the default path assumption, it is up to you to run it 
from a custom path.

## Flags and Usability

Add a flag to ignore the introduction text if you'd like quicker access to the live lexing.

Simply run your rust program with whichever path you need (I will show as if the path is in your shells $PATH)

```
chapter3_lexical -q
chapter3_lexical --quick
```

Using either flag `-q` or `--quick` you can bypass the introduction

## Ending your input 

To stop input you have a few options, such as 
- standard process killing with `ctrl+c`
- Hitting <ENTER> 
- Typing a valid lexme for the recognized token type "EOF", which can be any capitalization of "eof"
