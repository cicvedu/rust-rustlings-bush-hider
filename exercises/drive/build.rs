fn main() {
    let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    // Set the environment variable
    // if driver3.rs also has: std::env::set_var("TEST_FOO", timestamp.to_string());
    // std::env::set_var("TEST_FOO", timestamp.to_string());
    
    println!("cargo:rustc-env=TEST_FOO={}", timestamp.to_string());

    // Add "pass" feature
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
