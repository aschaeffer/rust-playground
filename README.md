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
  - [x] Gather Hardware Information
  - [ ] Profiling
    - [ ] https://crates.io/crates/profiling
    - [ ] https://optick.dev/
- [x] Software Architecture / Design Patterns
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
    - [ ] Graph Database & Reactive Streams -> Moved to the RECS-POC
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
  - [ ] Can we use whole objects created in the C++-Library?
- [ ] CI / Packaging
  - [x] Cargo Package Management
  - [x] Build Time Information
  - [x] GitHub Actions Rust Build
  - [ ] Cross Platform
  - [ ] Build Snap

## Conclusion

### Better or worse?

After a lot of tests and experiments: Is it better or worse? Is Rust suitable for Inexor?

| Topic | C++ | Rust |
| --- | --- | --- |
| Embedded Graph Database | Implementations of graph databases exists, but none of these can be embedded as a library | Graph Database can be embedded as library (Better) |
| Embedded JavaScript Runtime | Hard to compile, Hard to integrate, Integration have to be implemented by ourselves | V8 provided as cargo, runs immediately; Deno (Alternative to NodeJS) can be integrated as embedded library (Better) |
| Embedded Reactive Streams | Only one library, outdated, without maintainer | Multiple library candidates. The chosen one is easy, thread safe and maintained (Better) |
| Serialization | Extra Libraries needed | Rust provides a standardized way of serialization/deserialization and supports multiple formats. It's much more intuitive to use (Better) |
| Logging | Extra Library needed. More Features (Better) | Standard Library (Better) |
| Resources / Asset | Have to be implemented by ourselves, no Library found | Compile into binary (Equals), Ready to use Asset Manager Library (Better), Hot reloading of changed assets (Better) |
| Thread safety | Hard to archive | If the code compiles it's thread-safe (Better) |
| Memory Management | Pointers | Borrow-Checker (Compile-time): If the code compiles it will not crash (if you respect Result<T> and Option<T>) |
| Package Management | Conan | Cargo: Easy to use. Also has an ecosystem with crates.io (Better) |
| Libraries | More Libraries | Less Libraries (Rust is relatively new), but the right one's (Equals/Better) |
| Unit Tests | Extra Library needed. More Features | Standard Library, Conditional Compilation (cfg "test") |
| Dependency Injection | Extra Library needed | Extra Library needed. Both are compile time (Equals) |
| Cross Platform | Mostly platform independent | Mostly platform independent (Equals) |

### Risks

* Rust has new concepts; C++-Developers have to learn a lot.
* How can we access C++ Libraries from Rust and vice versa?
