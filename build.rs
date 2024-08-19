fn main() {
    embuild::espidf::sysenv::output();
    println!("cargo:rustc-env=WIFI_AP_MODE=1"); // Set to 1 for AP mode, anything else for client mode (connects to your netwok)
    println!("cargo:rustc-env=WIFI_SSID=Boat_Hotspot"); // Replace with your own SSID (AP SSID or Network SSID)
    println!("cargo:rustc-env=WIFI_PASSWORD=password"); // Replace with your own password (AP password or Network password)
}
