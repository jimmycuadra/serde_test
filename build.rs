extern crate serde_codegen;
extern crate syntex;

use std::path::Path;

fn main() {
    let src = Path::new("src/foo.rs");
    let dst = Path::new("src/generated_foo.rs");

    let mut registry = syntex::Registry::new();

    serde_codegen::register(&mut registry);
    registry.expand("", &src, &dst).unwrap();
}
