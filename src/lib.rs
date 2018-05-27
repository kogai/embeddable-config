extern crate serde;

#[cfg(test)]
#[macro_use]
extern crate serde_derive;

#[cfg(feature = "serde_json")]
extern crate serde_json;

#[cfg(feature = "serde_yaml")]
extern crate serde_yaml;

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
///     description: String
/// }
///
/// let instance = embedded!("../assets/simple.json", Sample);
/// assert_eq!(
///     Sample {
///         version: 1,
///         description: "This is sample configuration file".to_owned(),
///     },
///     instance.value().unwrap()
/// );
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

            // FIXME: If #[cfg(doctest)] has been released, I can remove those hacky solution.
            // For more information -> https://github.com/rust-lang/rust/issues/45599
            // #[cfg(any(test, feature = "test", feature = "serde_json"))]
            #[cfg(feature = "serde_json")]
            pub fn value(&self) -> serde_json::Result<$def> {
                serde_json::from_slice(&self.internal)
            }

            // #[cfg(any(test, feature = "test", feature = "serde_yaml"))]
            #[cfg(feature = "serde_yaml")]
            pub fn value(&self) -> serde_yaml::Result<$def> {
                serde_yaml::from_slice(&self.internal)
            }
        }

        Embedded::new()
    }}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "serde_yaml")]
    fn yaml() {
        #[derive(Deserialize, PartialEq, Debug)]
        struct Sample {
            version: usize,
            description: String,
        }
        let instance = embedded!("../assets/simple.yaml", Sample);
        assert_eq!(
            Sample {
                version: 1,
                description: "This is sample configuration file".to_owned(),
            },
            instance.value().unwrap()
        );
    }
}
