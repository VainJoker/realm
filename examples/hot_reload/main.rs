#[cfg(feature = "toml")]
fn main() {
    use std::{path::PathBuf, thread, time::Duration};

    use realme::{Adaptor, FileSource, Realme, TomlParser};

    // const CONFIGURATION1: &str = r#"key1 = "value""#;

    let realme = Realme::builder()
        .load(
            Adaptor::new(FileSource::<TomlParser>::new(PathBuf::from(
                "typos.toml",
            )))
            .watch(Duration::from_secs(3)),
        )
        .build()
        .expect("Building configuration object");

    // let value: String = realme
    //     .get("key1")
    //     .expect("Accessing configuration object")
    //     .try_into()
    //     .unwrap();

    loop {
        println!("'key1' Config element is: '{realme:?}'");
        thread::sleep(Duration::from_secs(5));
    }
}

#[cfg(not(feature = "toml"))]
fn main() {
    println!("Please enable toml feature");
    println!("cargo run --example simple --features toml");
}
