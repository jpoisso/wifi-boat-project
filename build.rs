fn main() {
    embuild::espidf::sysenv::output();
    println!("cargo:rustc-env=WIFI_AP_MODE=1"); // Set to anything else for client mode (connects to your netwok)
    println!("cargo:rustc-env=WIFI_SSID=WIFI_BOAT_HOTSPOT"); // Replace with your own SSID (AP SSID or Network SSID)
    println!("cargo:rustc-env=WIFI_PASSWORD=password"); // Replace with your own password (AP password or Network password)
}
