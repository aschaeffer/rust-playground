mod builder_test;
mod dependency_injection_test;
mod threading_test;
mod logging;
mod random;
mod vector_test;
mod ansi_test;
mod time_duration_test;
mod hardware_test;
mod glfw_test;

#[macro_use]
extern crate derive_builder;

fn main() {
    builder_test::get_point_inbound();
    threading_test::threading_tests();
    dependency_injection_test::dependency_injection_tests();
    logging::logging_tests();
    random::random_tests();
    vector_test::vector_tests();
    ansi_test::ansi_tests();
    time_duration_test::time_duration_tests();
    hardware_test::hardware_tests();
    glfw_test::glfw_all_tests();
}
