use indradb::{Datastore, MemoryDatastore, Transaction, Vertex, Edge, Type, RangeVertexQuery, SpecificVertexQuery, VertexQueryExt, EdgeKey, SpecificEdgeQuery, EdgePropertyQuery, EdgeQueryExt, EdgeDirection};
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
    let rel_type_looks_at = Type::new("looks_at").unwrap();

    println!("Player 1 has {} incoming edges of type looks_at", transaction.get_edge_count(player_1.id, Some(&rel_type_looks_at.clone()), EdgeDirection::Inbound).unwrap());

    let edge_key = EdgeKey::new(player_1.id, rel_type_looks_at.clone(), camera_1.id);

    // Record the start and end time. Round off the the nanoseconds off the
    // start time, since some implementations may not have that level of
    // accuracy.
    let create_edge_result = transaction.create_edge(&edge_key.clone());
    if create_edge_result.is_ok() {
        println!("Created edge: Camera 1 --[looks_at]--> Player 1");
    }

    println!("Player 1 has {} incoming edges of type looks_at", transaction.get_edge_count(player_1.id, Some(&rel_type_looks_at.clone()), EdgeDirection::Inbound).unwrap());

    let edge_query = SpecificEdgeQuery::single(edge_key);
    let edge_property_query_looks_at_distance = edge_query.clone().property("distance");

    println!("Edge looks_at has {} properties", transaction.get_edge_properties(edge_property_query_looks_at_distance.clone()).unwrap().len());
    let looks_at_distance = json!(32.0);
    let create_edge_property_result = transaction.set_edge_properties(
        edge_property_query_looks_at_distance.clone(),
        &looks_at_distance
    );
    if create_edge_property_result.is_ok() {
        println!("Created edge property: looksAt.distance");
    }
    println!("Edge looks_at has {} properties", transaction.get_edge_properties(edge_property_query_looks_at_distance.clone()).unwrap().len());

    let edges = transaction.get_edges(edge_query.clone()).unwrap();

    println!("Query all relations of type look at");
    for edge in edges.iter() {
        println!("Edge {}--[{}]-->{}: Date: {} ",
                 edge.key.outbound_id,
                 edge.key.t.0,
                 edge.key.inbound_id,
                 edge.created_datetime.to_string()
        );

        let edge_properties = transaction.get_all_edge_properties(edge_query.clone());
        if edge_properties.is_ok() {
            let edge_properties = edge_properties.unwrap()[0].clone();
            let edge_key = edge_properties.edge.key;
            for edge_property in edge_properties.props.iter() {
                println!("  {}--[{}]-->{}: {} = {} ",
                         edge_key.outbound_id,
                         edge_key.t.0,
                         edge_key.inbound_id,
                         edge_property.name,
                         edge_property.value.to_string()
                );
            }
        }
    }
}
