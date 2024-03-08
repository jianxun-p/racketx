use racketx::print_tokenstream;

#[test]
fn print_tokenstream_test1() {
    print_tokenstream! {
        (define a 1)
    };
}
