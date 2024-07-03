#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

fn main() {

    // Check if the `cfg.toml` file exists and has been filled out.
    if !std::path::Path::new("cfg.toml").exists() {
        panic!("You need to create a `cfg.toml` file with your Wi-Fi credentials!");
    }

    // The constant `CONFIG` is auto-generated by `toml_config.`
    let app_config = CONFIG;
    println!("Loaded cfg.toml.");

    embuild::espidf::sysenv::output();
}