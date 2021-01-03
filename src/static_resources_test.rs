use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "static/"]
struct Asset;

pub fn static_resources_tests() {
    for file in Asset::iter() {
        println!("{}", file.as_ref());
    }
    if Asset::get("lorem_ipsum.txt").is_none() {
        panic!("lorem_ipsum.txt should exist");
    }
    let lorem_ipsum = Asset::get("lorem_ipsum.txt").unwrap();
    let result = std::str::from_utf8(lorem_ipsum.as_ref());
    if result.is_ok() {
        println!("{}", result.unwrap());
    } else {
        println!("Could not find lorem_ipsum.txt")
    }
}
