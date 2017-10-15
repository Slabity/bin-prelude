#[macro_use]
extern crate bin_prelude;

#[test]
fn main() {
    bin_prelude!(
        (print "This")
        (print "is")
        (print "a")
        (print "test")
    );
}
