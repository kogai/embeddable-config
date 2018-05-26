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

- [ ] yaml
- [ ] json
- [ ] toml
- [ ] xml

- [ ] log4rs
- [ ] clap-rs

- [ ] nightly
- [ ] 1.26.0
- [ ] 1.25.0
- [ ] 1.24.0
