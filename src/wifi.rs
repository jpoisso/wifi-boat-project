use anyhow::Result;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::modem::Modem,
    nvs::EspDefaultNvsPartition,
    wifi::{AccessPointConfiguration, AuthMethod, ClientConfiguration, Configuration, EspWifi},
};
use log::info;
use std::{thread::sleep, time::Duration};

pub(crate) fn setup_wifi<'a>(
    ssid: &'a str,
    password: &'a str,
    is_ap_mode: bool,
    modem: Modem
) -> Result<EspWifi<'a>> {
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;
    let ssid = ssid.try_into().unwrap();
    let password = password.try_into().unwrap();
    let configuration = if is_ap_mode {
        Configuration::AccessPoint(AccessPointConfiguration {
            ssid,
            password,
            max_connections: 4,
            auth_method: AuthMethod::WPA2WPA3Personal,
            ..Default::default()
        })
    } else {
        Configuration::Client(ClientConfiguration {
            ssid,
            password,
            ..Default::default()
        })
    };

    let mut esp_wifi = EspWifi::new(modem, sys_loop, Some(nvs))?;
    esp_wifi.set_configuration(&configuration)?;

    esp_wifi.start()?;
    esp_wifi.connect()?;
    while !esp_wifi.is_connected()? {
        info!("waiting for connection status from wifi driver...");
        sleep(Duration::new(1, 0));
    }
    Ok(esp_wifi)
}
