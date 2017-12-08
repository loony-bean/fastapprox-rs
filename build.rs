extern crate gcc;

fn main() {
    gcc::Build::new()
               .file("tests/c/fastapprox.c")
               .compile("fastapprox");
}
