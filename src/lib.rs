extern crate serde;

#[cfg(test)]
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::path::Path;

trait Embeddable {
    fn from_file<P: AsRef<Path>>(path: P) -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_file() {
        use std::fs;
        use std::io::Read;

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        struct T {
            version: usize,
        };
        impl Embeddable for T {
            fn from_file<P: AsRef<Path>>(path: P) -> Self {
                let x = match fs::File::open(path) {
                    Ok(mut file) => {
                        let mut buf = Vec::new();
                        let _ = file.read_to_end(&mut buf);
                        buf
                    }
                    Err(e) => unreachable!(e),
                };
                let x = serde_json::from_slice::<T>(&x).unwrap();
                x
            }
        }
        assert_eq!(T { version: 1 }, T::from_file("assets/simple.json"));
    }
}
