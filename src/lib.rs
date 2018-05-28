extern crate serde;

#[cfg(test)]
#[macro_use]
extern crate serde_derive;

#[cfg(feature = "serde_json")]
extern crate serde_json;

#[cfg(feature = "serde_yaml")]
extern crate serde_yaml;

#[cfg(feature = "toml")]
extern crate toml;

#[cfg(test)]
extern crate log4rs;

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
/// # fn main() {
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
            #[cfg(feature = "serde_json")]
            pub fn value(&self) -> serde_json::Result<$def> {
                serde_json::from_slice(&self.internal)
            }

            #[cfg(feature = "serde_yaml")]
            pub fn value(&self) -> serde_yaml::Result<$def> {
                serde_yaml::from_slice(&self.internal)
            }

            #[cfg(feature = "toml")]
            pub fn value(&self) -> Result<$def, toml::de::Error> {
                toml::from_slice(&self.internal)
            }
        }

        Embedded::new()
    }}
}

pub enum Embeddable {
    Log4rs,
}

macro_rules! from_file {
    ($path:expr, $kind:path) => {{
        match $kind {
            Embeddable::Log4rs => {
                let path = path.as_ref().to_path_buf();
                let format = Format::from_path(&path)?;
                let source = read_config(&path)?;
                // An Err here could come because mtime isn't available, so don't bail
                let modified = fs::metadata(&path).and_then(|m| m.modified()).ok();
                let config = format.parse(&source)?;

                let refresh_rate = config.refresh_rate();
                let config = embedded!($path, log4rs::file::RawConfig);
                match log4rs::init_config(config) {
                    Ok(handle) => {
                        if let Some(refresh_rate) = refresh_rate {
                            ConfigReloader::start(
                                path,
                                format,
                                refresh_rate,
                                source,
                                modified,
                                deserializers,
                                handle,
                            );
                        }
                        Ok(())
                    }
                    Err(e) => Err(e.into()),
                }
            }
            _ => unreachable!()
        }
    }}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "serde_yaml")]
    fn support_yaml() {
        #[derive(Deserialize, PartialEq, Debug)]
        pub struct Sample {
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

    #[test]
    #[cfg(feature = "serde_yaml")]
    fn from_yaml() {
        // TODO: Replace to actual configuration.
        let actual = from_file!("../assets/simple.yaml", Embeddable::Log4rs);
        // println!("{:?}", actual.value());
    }

    #[test]
    #[cfg(feature = "toml")]
    fn support_toml() {
        #[derive(Deserialize, PartialEq, Debug)]
        pub struct Sample {
            version: usize,
            description: String,
        }
        let instance = embedded!("../assets/simple.toml", Sample);
        assert_eq!(
            Sample {
                version: 1,
                description: "This is sample configuration file".to_owned(),
            },
            instance.value().unwrap()
        );
    }
}
