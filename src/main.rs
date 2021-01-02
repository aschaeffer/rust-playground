mod builder_test;
mod glfw_test;
mod dependency_injection_test;

#[macro_use]
extern crate derive_builder;

fn main() {
    builder_test::get_point_inbound();
    glfw_test::glfw_all_tests();
    dependency_injection_test::dependency_injection_tests();
}
