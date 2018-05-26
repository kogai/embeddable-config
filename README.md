```rs
let config = Config::with_file("conf.yml");
// Embeded data of file into executable

// 1. Impl Trait by auther of crate(very ideal)
// #[derive(Embeddable)] or impl Embeddable for T {...}
// fn with_embeded() -> Any {}
let config = Config::with_embeded_file("conf.yml");

// 1. Or manually convert from file to aribitarly type
let config: T = Embeddable::from_type::<T>("conf.yml);
```
