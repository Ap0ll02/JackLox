# Interesting Things I Read (Saturday Meeting, June 14th)

## Error Recovery

There are a few general methods of error handling/Recovery

- Panic Mode
- Phrase Level
- Error Productions
- Global Correction

Each one has its' own quirks.

### Panic Mode

This is the most basic one, where you can implement it relatively easily.

Given some erroneous input, such as a simple missing semicolon:
```
int x = 0
int y = x + 8;
```

The parser might recognize a sequence of tokens like:

`int_id id assignop num int_id ...`

It attempts to match this against grammar productions, but fails when it expects a delimiter (like a semicolon) and encounters the next declaration (int).

At this point, panic mode recovery kicks in. The parser will discard tokens until it finds a synchronizing token such as ;. This might look like:

```
int x = 0 int (error detected)
int x = 0
int x =
int x
int
... no delimeter detected. Missing delimeter.
```

Eventually, once the parser reaches a recognizable point (like a semicolon), it resumes normal parsing. While this strategy may skip over parts of valid input, it prevents cascading errors and ensures that further syntax can still be analyzed.

### Phrase Level

Small local corrections to code, like add a missing semicolon or delete an extraneous one. These corrections are small and local, its' biggest drawback is
that it has a hard time with recovery when the error was detected later than it occurred.

### Error Productions (My favorite, in theory)

Create productions in our grammar for errors, so we can match common errors with a grammar production! This sounds like a way to give really exact locations on error feedback.
Along with a great way to catch a lot of common errors or very odd errors that are otherwise hard to detect.

