#[cfg(feature = "toml")]
use realme::{Adaptor, Realme, StringSource, TomlParser};
use realme::{EnvParser, EnvSource};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Config {
    key: String,
    like: String,
    #[serde(default)]
    default: String,
}

#[cfg(feature = "toml")]
fn main() {
    const CONFIGURATION1: &str = r#"
    key="{{env}}"
    like="like"
    "#;

    let realme = Realme::builder()
        .load(Adaptor::new(Box::new(StringSource::<TomlParser>::new(
            CONFIGURATION1,
        ))))
        .load(Adaptor::new(Box::new(EnvSource::<EnvParser>::new(
            "REALM_",
        ))))
        .build();

    match realme {
        Ok(realme) => {
            let config: Config = realme.try_deserialize().unwrap();
            println!("{config:?}");
        }
        Err(e) => {
            println!("Error: {e}");
        }
    }
}

#[cfg(not(feature = "toml"))]
fn main() {
    println!("Please enable toml feature");
    println!("cargo run --example simple --features toml");
}
