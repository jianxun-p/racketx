use racketx::racket_println;

#[test]
fn racket_println_test1() {
    racket_println! {
        (f 1 (+ 2 3 4))
    };
}
