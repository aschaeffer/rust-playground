use num_cpus::*;

pub fn hardware_tests() {
    println!("Number of logical cores is {}", num_cpus::get());
    println!("Number of physical cores is {}", num_cpus::get_physical());
}
