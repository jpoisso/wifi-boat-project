fn main() {
    embuild::espidf::sysenv::output();
    println!("cargo:rustc-env=WIFI_SSID=YOUR_SSID");
    println!("cargo:rustc-env=WIFI_PASSWORD=YOUR_PASSWORD");
}
