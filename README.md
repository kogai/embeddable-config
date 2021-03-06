# embeddable-config

## Note

```rs
// 1. Manually convert from file to aribitarly type
#[derive(Embeddable)]
struct Wrapped(T);
// or impl Embeddable for Wrapped {...};
let config: T = Wrapped::with_embedded_file("conf.yml");
let config = Config::with_file("conf.yml");

// Embeded data of file into executable

// 1. Impl Trait by auther of crate(very ideal)
// #[derive(Embeddable)] or impl Embeddable for T {...}
// fn with_embeded() -> Any {}
let config = Config::with_embeded_file("conf.yml");
```

* [x] yaml
* [x] json
* [x] toml

* [ ] log4rs
* [ ] clap-rs

* [x] nightly
* [x] 1.26.0
* [x] 1.25.0
* [x] 1.24.0
* [ ] Windows
