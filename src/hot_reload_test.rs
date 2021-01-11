use assets_manager::{
    Asset,
    AssetCache,
    loader
};
use serde::Deserialize;
use std::{thread, time};

// The struct you want to load
#[derive(Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

// Specify how you want the structure to be loaded
impl Asset for Point {
    // The extension of the files to look into
    const EXTENSION: &'static str = "json";

    // The serialization format (JSON)
    type Loader = loader::JsonLoader;
}

pub fn hot_reload_tests() {

    // Create a new cache to load assets under the "./assets" folder
    let cache = AssetCache::new("assets").unwrap();

    // Get a handle on the asset
    // This will load the file `./assets/common/position.ron`
    let handle = cache.load::<Point>("common.position").unwrap();

    // // Lock the asset for reading
    // // Any number of read locks can exist at the same time,
    // // but none can exist when the asset is reloaded
    // let point = handle.read();
    //
    // // // The asset is now ready to be used
    // // assert_eq!(point.x, 5);
    // // assert_eq!(point.y, -6);

    // // Loading the same asset retrieves it from the cache
    // let other_handle = cache.load("common.position").unwrap();
    // assert!(other_handle.ptr_eq(&handle));

    //
    let delay = time::Duration::from_millis(500);
    let mut is_zero = false;
    while !is_zero {
        thread::sleep(delay);

        // Reload all cached files that changed
        cache.hot_reload();

        // Assets are updated without any further work

        println!("Point({}, {})", handle.read().x, handle.read().y);

        is_zero = handle.read().x - handle.read().y == 0;

    }

}