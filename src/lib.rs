extern crate serde;

#[cfg(test)]
#[macro_use]
extern crate quote;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use serde::{Deserialize, Serialize};

#[macro_export]
macro_rules! embedded {
    ($path:expr, $def:ty) => {{
        pub struct EmbeddedT {
            internal: &'static [u8],
        }

        impl EmbeddedT {
            pub fn new() -> Self {
                EmbeddedT {
                    internal: include_bytes!($path),
                }
            }

            pub fn value<'a, T>(&self) -> T
            where
                T: serde::Deserialize<'a>,
            {
                serde_json::from_slice(&self.internal).unwrap()
            }
        }

        EmbeddedT::new()
    }}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    struct Sample {
        version: usize,
    }

    #[test]
    fn from_file() {
        let instance = embedded!("../assets/simple.json", Sample);
        assert_eq!(Sample { version: 1 }, instance.value());
    }
}
