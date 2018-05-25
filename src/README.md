[enter] is a simple crate to solve the CR/LF/CRLF problem from one line input
on different OS.

## example(s)

```rust
use enter::{ Enter, ENTER };

let mut str_with_enter = "Hello, World!".to_string();
str_with_enter.push_str(ENTER);
assert_eq!(str_with_enter.enter(), "Hello, World!");
```
