use anyhow::Result;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::prelude::Peripherals,
    nvs::EspDefaultNvsPartition,
    wifi::{ClientConfiguration, Configuration, EspWifi},
};
use log::info;
use std::{thread::sleep, time::Duration};

pub(crate) fn setup_wifi<'a>(ssid: &'a str, password: &'a str) -> Result<EspWifi<'a>> {
    let peripherals = Peripherals::take()?;
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;
    let ssid = ssid.try_into().unwrap();
    let password = password.try_into().unwrap();
    let configuration = Configuration::Client(ClientConfiguration {
        ssid,
        password,
        ..Default::default()
    });
    let mut esp_wifi = EspWifi::new(peripherals.modem, sys_loop, Some(nvs))?;
    esp_wifi.set_configuration(&configuration)?;
    esp_wifi.start()?;
    esp_wifi.connect()?;
    while !esp_wifi.is_connected()? {
        info!("waiting for connected status from wifi endpoint...");
        sleep(Duration::new(1, 0));
    }
    Ok(esp_wifi)
}
