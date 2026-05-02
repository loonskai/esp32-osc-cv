fn main() {
    embuild::espidf::sysenv::output();

    let cfg = std::fs::read_to_string("cfg.toml").expect("cfg.toml not found");
    let value: toml::Value = cfg.parse().unwrap();

    println!("cargo:rustc-env=WIFI_SSID={}", value["wifi"]["ssid"].as_str().unwrap());
    println!("cargo:rustc-env=WIFI_PASSWORD={}", value["wifi"]["password"].as_str().unwrap());
}
