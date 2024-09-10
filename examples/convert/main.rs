#[cfg(feature = "toml")]

use realm::Realm;
#[cfg(feature = "toml")]
use serde::{Serialize, Deserialize};
#[cfg(feature = "toml")]
// 定义一个示例结构体
#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
}

#[cfg(feature = "toml")]
fn main() {
    let person_toml = toml::toml!{
        name = "John"
        age = 30
    };
        

    let person_realm = Realm::try_serialize(&person_toml).unwrap(); 
    println!("{person_realm:#?}");
    let person: Person = person_realm.try_deserialize().unwrap();
    println!("{person:#?}");
}

#[cfg(not(feature = "toml"))]
fn main() {
    println!("Please enable toml feature");
    println!("cargo run --example convert --features toml");
}