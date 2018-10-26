extern crate cc;

fn main() {
    cc::Build::new()
               .file("tests/c/fastapprox.c")
               .compile("fastapprox");
}
