# Research Rust

## The following technologies are evaluated

* How a graph database can be integrated (the main technology behind the entity system)
* How data from the entity system / graph database can be serialized and deserialized (JSON/TOML/...)
* How data from the entity system / graph database can be synchronized with other instances (gRPC)
* How data from the entity system / graph database can be provided to a web user interface (REST / GraphQL)
* How reactive streams can be integrated (the main technology behind the visual scripting system)
* How a scripting engine can be integrated (we prefer javascript) which is able to interact with the entity system

## Is Rust suitable for Inexor?

- [x] Easier Standard Technologies
  - [x] Logging
  - [x] Include static resources into the binary
  - [x] Serialization / Deserialization
    - [x] JSON (in fact Rust has a framework for different types of serialization in contrast to C++)
    - [x] TOML
  - [x] Math (Game Engine / Physics / Particle System)
    - [x] Linear Algebra
    - [x] Trigonometry
  - [x] Asset Manager + Hot reloading
    - [x] https://github.com/a1phyr/assets_manager
  - [ ] Profiling
    - [ ] https://crates.io/crates/profiling
    - [ ] https://optick.dev/
- [ ] Software Architecture / Design Patterns
  - [x] Builder Pattern
  - [x] Dependency Injection
  - [x] Borrow / RAII
- [x] Library Support
  - [x] GLFW Library
  - [x] Reactive Streams (Entity System / Visual Scripting)
  - [x] JavaScript Runtime (Entity System / Visual Scripting)
    - [x] Execute JavaScript Files
    - [x] Define functions in Rust which can be used in JavaScript
    - [x] Low-Level Access to the V8 library
  - [ ] Embeddable In-Memory Graph-Database (Entity System)
    - [x] CRUD Vertices
    - [x] Search Vertices
    - [x] CRUD Edges
    - [x] Search Edges
    - [x] CRUD Properties
    - [x] Search Properties
    - [ ] Complex Search (multiple hops)
  - [ ] Graph Database & Reactive Streams
  - [ ] GRPC
    - [ ] Server
    - [ ] Client
  - [ ] GraphQL
    - [ ] Server
  - [ ] HTTP2 / REST web server
  - [ ] Text User Interface
    - [ ] https://github.com/fdehau/tui-rs
- [ ] Integration Capabilities
  - [ ] Use Vulkan-Renderer as a Library (C++)
  - [ ] Cxx Wrapper for C++
- [ ] CI / Packaging
  - [x] Cargo Package Management
  - [ ] GitHub Actions Rust Build
  - [ ] Cross Platform
  - [ ] Build Snap
