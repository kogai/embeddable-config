extern crate serde;
extern crate serde_json;

/// A macro generates from arbitrary file path and type definition which involve to the file to the data structure ebeddeded data of file.
///
/// # Example
///
/// ```
/// # extern crate serde;
/// # #[macro_use]
/// # extern crate serde_derive;
/// # extern crate serde_json;
/// # #[macro_use]
/// # extern crate embeddable_config;
///
/// # fn main() {
///
/// #[derive(Deserialize, PartialEq, Debug)]
/// struct Sample {
///     version: usize,
/// }
///
/// let instance = embedded!("../assets/simple.json", Sample);
/// assert_eq!(Sample { version: 1 }, instance.value().unwrap());
/// # }
/// ```
#[macro_export]
macro_rules! embedded {
    ($path:expr, $def:ty) => {{
        pub struct Embedded {
            internal: &'static [u8],
        }

        impl Embedded {
            fn new() -> Self {
                Embedded {
                    internal: include_bytes!($path),
                }
            }

            pub fn value(&self) -> serde_json::Result<$def> {
                serde_json::from_slice(&self.internal)
            }
        }

        Embedded::new()
    }}
}
