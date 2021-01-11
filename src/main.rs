mod builder_test;
mod dependency_injection_test;
mod threading_test;
mod logging;
mod random;
mod vector_test;
mod ansi_test;
mod time_duration_test;
mod hardware_test;
mod linear_algebra_test;
mod trigonometry_test;
mod regex_test;
mod bidule_test;
mod deno_test;
// mod glfw_test;
mod static_resources_test;
mod bidule_glfw_test;
mod indradb_test;
mod serde_test;
mod hot_reload_test;
mod build_time_information_test;

#[macro_use]
extern crate shadow_rs;

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
    linear_algebra_test::linear_algebra_tests();
    trigonometry_test::trigonometry_tests();
    regex_test::regex_tests();
    bidule_test::bidule_tests();
    // glfw_test::glfw_all_tests();
    static_resources_test::static_resources_tests();
    deno_test::deno_tests();
    indradb_test::indradb_tests();
    // bidule_glfw_test::bidule_glfw_tests();
    serde_test::serde_tests();
    hot_reload_test::hot_reload_tests();
    build_time_information_test::build_time_information_tests();
}
