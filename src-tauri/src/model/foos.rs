use serde::{Deserialize, Serialize};
use crate::model::foo::Foo;

#[derive(Serialize, Deserialize, Debug)]
pub struct Foos {
    pub baz: Vec<Foo>,
}
