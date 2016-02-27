extern crate serde;

#[derive(Serialize, Deserialize)]
pub struct Foo {
    pub bar: String,
    pub baz: String,
}
