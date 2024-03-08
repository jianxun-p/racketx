use racketx::dbgstringify;

#[test]
fn dbgstringify_test1() {
    println!(
        "{}",
        dbgstringify! {
            (define b (a + 1))
        }
    );
}
