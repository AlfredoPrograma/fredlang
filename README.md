# Fredlang
## What is it?

**Fredlang** is a toy programming language built upon *Crafting Interpreters* tutorial book.
It is a beginner project implemented in order to accomplish multiple targets:

- [ ] Understand how programming languages are build (at least in most basic way)
- [ ] Learn core concepts about *parsers*, *lexers*, *AST* and other esotherical stuff
- [ ] Test out my knowledges at *Rust* programming language

## How is it build?
### Read module
Exposes core functions for read given source code from file or from repl

### Parsing module
Exposes parsing functions to get the corresponding lexemes based on the grammar of the language

### Scanner module
Exposes `scanner` function, which stores the state of the interpretation execution; like:
- Scan tokens based on the given source
- Store the location of the current scanning execution
- Store errors which occur during the scanning and parsing processes 

