# racketx
Allows you to write Racket code in Rust. 

## Example
Here is an example of using the `racket` macro.
```rust
use racketx::racket;
pub fn main() {
    racket!{
        (define add_2_num (lambda (x y) (+ x y)))
        (define result (add_2_num 239 (- 9)))
    };
    assert_eq!(result, 230);
}
```

More examples can be found under the `examples` directory.


## Features
- [x] definition of constants
- [x] lambda
- [x] integer operations (+, -, *, /)
- [ ] negative integer (alternate solution: express with subtraction, e.g. `-8` would be `(- 8)`)
- [ ] non-integer
- [ ] conditional statements
- [ ] lists
- [ ] function definitions (alternate solution: use constants definition with lambda)
- [ ] modulo arithmetic
