use indradb::{Datastore, MemoryDatastore, Transaction, Vertex, Edge, Type, RangeVertexQuery, SpecificVertexQuery, VertexQueryExt};
use serde_json::{json, Result, Value};

pub fn indradb_tests() {

    // New Datastore (in memory)
    let graph_database = MemoryDatastore::default();

    // Open a transaction
    let transaction = graph_database.transaction().unwrap();

    // Define a new type (entity type)
    let type_camera = Type::new("camera").unwrap();

    println!("Number of vertices: {}", transaction.get_vertex_count().unwrap());

    // Create an instance (entity instance)
    let camera_1 = Vertex::new(type_camera.clone());
    let result = transaction.create_vertex(&camera_1);
    if result.is_ok() {
        println!("Camera 1 created");
    }
    println!("Number of vertices: {}", transaction.get_vertex_count().unwrap());

    let type_player = Type::new("player").unwrap();

    let player_1 = Vertex::new(type_player.clone());
    let result = transaction.create_vertex(&player_1);
    if result.is_ok() {
        println!("Player 1 created");
    }
    println!("Number of vertices: {}", transaction.get_vertex_count().unwrap());

    println!("Set property name of the player");
    let q = SpecificVertexQuery::single(player_1.id);
    let player_1_name = json!("Peter Penacka");
    transaction.set_vertex_properties(q.clone().property("name"), &player_1_name);

    // Query all entities of type camera
    let cameras = transaction.get_vertices(RangeVertexQuery::new(1000).t(type_camera)).unwrap();

    println!("Query all entities of type camera");
    for camera in cameras {
        println!("{} UUID: {} ", camera.t.0, camera.id);

    }

    // Query all entities of type player
    let players = transaction.get_vertices(RangeVertexQuery::new(1000).t(type_player)).unwrap();

    println!("Query all entities of type player");
    for player in players {
        println!("{} UUID: {} ", player.t.0, player.id);
        let vertex_properties = transaction.get_all_vertex_properties(SpecificVertexQuery::single(player.id));
        if vertex_properties.is_ok() {
            for prop in vertex_properties.unwrap()[0].props.iter() {
                println!("  {} = {} ", prop.name, prop.value.to_string());
            }
        }
    }

    // Define a new type (relation type)
    // let rel_type_points_to = Type::new("points_to").unwrap();


}
