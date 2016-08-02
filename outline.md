# Rust Intro

## Why Another Programming Language?

    * Rust is a language which bears memory safety in mind

    * Compile language, which makes it blazing fast

    * Tough but kind compiler
   
## Memory Safety? How Unsafe Could It Be?

    * Shared mutable states is the root of all evil

    * Rust deals with shared instead of mutable

## OK Then, Show Me Something

### Variable Bindings

### Functions

### Primitive Types

### If

### Loops

### Vectors

### Structs

### Enums

### Match

### Method Syntax

### Generics

### Traits

### Closures

### Cargo
    
## Ownership and Borrowing

### The Ownership

### Borrowing and Reference
    
    * Shared but not mutable state: `&T`

    * Not shared but mutalbe state: `&mut T`

    * The Rules:

        1. Any borrow must last for a scope no greater than that of the owner.

        2. Either one of the following must be true:
            
            * One of more references (`&T`) to the resource.

            * Exactly one mutable reference (`&mut T`) to the resource.

### Why So Serious, Compiler?

    1. Iterator invalidation

    2. Use after free -> Dangling pointer

## But I Need Both!

### Concurrency in Rust
