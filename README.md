# norm-email 📧

[![Crates.io](https://img.shields.io/crates/v/norm-email.svg)](https://crates.io/crates/norm-email)
[![Documentation](https://docs.rs/norm-email/badge.svg)](https://docs.rs/norm-email)
[![License](https://img.shields.io/github/license/orhanbalci/norm-email.svg)](https://github.com/orhanbalci/norm-email/blob/master/LICENSE)


Email normalization for your services. This crate is ported from Python library
[email-normalize](https://github.com/gmr/email-normalize) written by
[@gmr](https://github.com/gmr).


## 📦 Cargo.toml

```toml
[dependencies]
norm-email = {git = "https://github.com/orhanbalci/norm-email"}
```

## 🔧 Example

```rust
fn main() {
    let normalizer = Normalizer::new();
    let result = normalizer.normalize("orhan.balci@gmail.com").unwrap();
    println!("{:#?}", result.mx_records);
    println!("{}", result.mailbox_provider.unwrap());
    println!("{}", result.normalized_address);
}
```

## 🖨️ Output

```text
[
    MxRecord {
        priority: 30,
        host: "alt3.gmail-smtp-in.l.google.com.",
    },
    MxRecord {
        priority: 20,
        host: "alt2.gmail-smtp-in.l.google.com.",
    },
    MxRecord {
        priority: 5,
        host: "gmail-smtp-in.l.google.com.",
    },
    MxRecord {
        priority: 10,
        host: "alt1.gmail-smtp-in.l.google.com.",
    },
    MxRecord {
        priority: 40,
        host: "alt4.gmail-smtp-in.l.google.com.",
    },
]
GOOGLE
orhanbalci@gmail.com
```


## 📝 License

Licensed under MIT License ([LICENSE](LICENSE)).

### 🚧 Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, as defined in the MIT license, shall be licensed as above, without any additional terms or conditions.
