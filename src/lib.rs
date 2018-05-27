extern crate serde;

#[cfg(test)]
#[macro_use]
extern crate quote;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct Sample {
    version: usize,
}

pub struct EmbeddedT {
    internal: &'static [u8],
}

impl EmbeddedT {
    pub fn new() -> Self {
        EmbeddedT {
            internal: include_bytes!("../assets/simple.json"),
        }
    }
    pub fn value<'a, T>(&self) -> T
    where
        T: serde::Deserialize<'a>,
    {
        serde_json::from_slice(&self.internal).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_file() {
        let z = EmbeddedT::new();
        assert_eq!(Sample { version: 1 }, z.value());
    }
}
