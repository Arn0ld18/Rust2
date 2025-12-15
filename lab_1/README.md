## 1. Is Rust single-threaded or multi-threaded? Is it synchronous or asynchronous?
- Rust supports both

## 2 What runtime Rust has? Does it use a GC (garbage collector)? 
- Rust has a minimal runtime (almost none by        default). No GC(rust uses ownership and borrowing)

## 3 What static typing means? What are the benefits of using it?
- Variable types are known at compile time.Early error detection, safer code

## 4 What is immutability? What is the benefit of using it?
- By default, variables can’t be changed after assignment. Safer code, easier reasoning

## 5 What are move semantics? What are borrowing rules? What is the benefit of using them?
- When assigning or passing ownership of a value, the original variable is no longer valid.
- 1)One mutable reference or many immutable references at a time, 2)References must not outlive the owner.
- Memory safety without GC, prevents data races

## 6 What are traits? How are they used? How do they compare to interfaces? 
- Collections of methods that types can implement.
- Define shared behavior across types.
- Similar, but more flexible (default impls)

## 7 What are lifetimes? Which problems do they solve?
- Annotations that describe how long references are valid.
- Prevent dangling references and ensure memory safety.

## 8 What are macros? Which problems do they solve?
- Metaprogramming tools that expand into Rust code
- Remove boilerplate, enable code generation.

## 9 What is the difference between &String and &str types (or between &Vec and &[u8] types)? Difference between fat and thin pointers? 
- String -> String obj, &str -> string slice.
- &Vec<u8> -> reference to the vector, &[u8] -> slice view into elements.
- Thin pointer -> just memory address, Fat pointer -> address + metadata

## 10 What are static and dynamic dispatches?
- Static dispatch -> method calls resolved at compile time
- Dynamic dispatch -> calls resolved at runtime via vtables
