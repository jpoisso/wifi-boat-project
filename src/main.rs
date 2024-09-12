mod boat;
mod server;
mod wifi;

use anyhow::Result;
use boat::{Motor, Rudder};
use esp_idf_svc::hal::{
    ledc::{config::TimerConfig, LedcDriver, LedcTimerDriver, Resolution},
    prelude::*,
};
use log::info;
use std::{thread::sleep, time::Duration};

const WIFI_SSID: &str = env!("WIFI_SSID");
const WIFI_PASSWORD: &str = env!("WIFI_PASSWORD");
const WIFI_AP_MODE: &str = env!("WIFI_AP_MODE");

fn main() -> Result<()> {
    // Initialize ESP-IDF system and logging.
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;

    info!("connecting to wifi...");
    let wifi = wifi::setup_wifi(
        WIFI_SSID,
        WIFI_PASSWORD,
        WIFI_AP_MODE == "1",
        peripherals.modem,
    )?;
    sleep(Duration::from_secs(1));
    let ip = wifi.sta_netif().get_ip_info()?.ip;
    info!("connected to wifi with ip: {ip:?}");

    // Create the boat and its components.
    let motor_channel = peripherals.ledc.channel0;
    let motor_timer = peripherals.ledc.timer0;
    let motor_timer_config = TimerConfig::new()
        .frequency(50.kHz().into())
        .resolution(Resolution::Bits8);
    let motor_pwm_pin = peripherals.pins.gpio5;
    let motor_timer_driver = LedcTimerDriver::new(motor_timer, &motor_timer_config)?;
    let motor_pwm = LedcDriver::new(motor_channel, motor_timer_driver, motor_pwm_pin)?;

    let mut motor = Motor::new(motor_pwm);
    motor.start()?;

    let servo_channel = peripherals.ledc.channel1;
    let servo_timer = peripherals.ledc.timer1;
    let servo_timer_config = TimerConfig::default()
        .frequency(50.Hz())
        .resolution(Resolution::Bits14);
    let servo_pwm_pin = peripherals.pins.gpio6;
    let servo_timer_driver = LedcTimerDriver::new(servo_timer, &servo_timer_config)?;
    let servo = LedcDriver::new(servo_channel, servo_timer_driver, servo_pwm_pin)?;

    let mut rudder = Rudder::new(servo);
    rudder.start()?;

    let boat = boat::Boat { motor, rudder };

    // Set up http server and keep a reference to it (otherwise it drops out of scope).
    let _server = server::setup_server(boat)?;

    // Keep the main thread alive by sleeping periodically.
    loop {
        info!(
            "server is still running. wifi status: {}",
            wifi.is_connected()?
        );
        sleep(Duration::from_secs(20));
    }
}
