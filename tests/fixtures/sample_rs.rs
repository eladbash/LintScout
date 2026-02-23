#![allow(clippy::all)]

fn main() {
    #[allow(clippy::needless_return)]
    let x = 1;

    #[allow(clippy::too_many_arguments)]
    fn foo() {}
}
