mod builder_test;

#[macro_use]
extern crate derive_builder;

fn main() {
    builder_test::get_point_inbound();
}
