use racketx::racket;

fn main() {
    racket! {
        (define add_2_num (lambda (x y) (+ x y)))
        (define a 3)
        (define b ((lambda (x y z) (+ x y z)) 1 2 a))
        (define c (- 9))
        (define result (add_2_num b c))
    };
    assert_eq!(a, 3);
    assert_eq!(b, 6);
    assert_eq!(c, -9);
    assert_eq!(result, -3);
}
