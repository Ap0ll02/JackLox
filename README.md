# JackLox - Crafting Interpreters Follow Along

My code and notes for the crafting interpreters book will be in this repo. I will structure it to have a folder per chapter
with a README in each, along with the code. If this is not applicable there may not be a folder or content for a chapter.

## The Visitor Pattern

### Degining an Interface

We need a set of functions for each expression type:

- VisitBinaryExpr()
- VisitLiteralExpr()
- etc.

### Note on Visitor and R in Java

Abstract accept method will return some visitor, relating to one of the expressions.
This will ensure all expressions have an accept method.

Abstract Accept method

Interface must implement the visit methods 

Each of the expressions must implement their own abstract visit method.

The expressions visitor must implement the visit concretely.
