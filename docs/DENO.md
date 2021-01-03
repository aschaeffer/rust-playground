# DENO & RUST

## Motivation

We want javascript as a scripting language within Inexor.

Deno can be embedded in a Rust application:

https://crates.io/crates/deno_core

It depends on the V8 javascript bindings:

https://github.com/denoland/rusty_v8

## Motivation II

We already tried to embed V8 directly in C++ code. This is hard to archive because of the build stack.

I expect that this is much easier to archive with rusty_v8 and deno.

## Motivation III

With this integration it would be possible to push forward the development of the reactive entity component system
because it's possible

## Example

https://github.com/SyrupThinker/deno/blob/559ce3f45e8989f2f7f9629eb09d1244ffbe6ce2/core/examples/hello_world.rs

## Resources

* https://deno.land/manual/embedding_deno
* https://crates.io/crates/deno_core
* https://docs.rs/deno_core/0.75.0/deno_core/
* https://github.com/denoland/rusty_v8
* https://t3n.de/news/deno-1-steckt-dino-logo-1280484/


Cube getRootCube()
List<Cube> getChildCubes(Cube cube)
void setCube(Cube cube)
