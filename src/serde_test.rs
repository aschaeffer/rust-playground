use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

#[derive(Serialize)]
struct Config {
    ip: String,
    port: Option<u16>,
    keys: Keys,
}

#[derive(Serialize)]
struct Keys {
    github: String,
    travis: Option<String>,
}

fn json_test() {

    let data = r#"
    {
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    }"#;

    let r: serde_json::Result<Person> = serde_json::from_str(data);
    if r.is_ok() {
        let p: Person = r.unwrap();
        println!("{:?}", serde_json::to_string(&p));
    } else {
        println!("Failed to deserialize json string");
    }

}

fn toml_test() {
    let data = "foo = 'bar'";

    let value = data.parse::<toml::Value>().unwrap();

    assert_eq!(value["foo"].as_str(), Some("bar"));

    let config = Config {
        ip: "127.0.0.1".to_string(),
        port: None,
        keys: Keys {
            github: "xxxxxxxxxxxxxxxxx".to_string(),
            travis: Some("yyyyyyyyyyyyyyyyy".to_string()),
        },
    };

    let toml = toml::to_string(&config).unwrap();



}

pub fn serde_tests() {
    json_test();
    toml_test();
}