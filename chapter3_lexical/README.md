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
