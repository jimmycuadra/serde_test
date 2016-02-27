extern crate serde;
extern crate serde_json;

use serde_json::to_string;

use foo::Foo;

#[path = "generated_foo.rs"]
pub mod foo;

fn main() {
    let foo = Foo {
        bar: "bar".to_owned(),
        baz: "baz".to_owned(),
    };

    println!("{}", to_string(&foo).unwrap());
}
