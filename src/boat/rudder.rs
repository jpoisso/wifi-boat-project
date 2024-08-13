use std::sync::{Arc, Mutex};

use anyhow::Result;
use esp_idf_svc::hal::{
    ledc::{self, config::TimerConfig, LedcDriver, LedcTimerDriver},
    prelude::*,
};

pub(crate) struct Rudder<'a> {
    servo: Arc<Mutex<LedcDriver<'a>>>,
    max_angle: u32,
    min_angle: u32,
}

impl<'a> Rudder<'a> {
    pub(crate) fn new(servo: Arc<Mutex<LedcDriver<'a>>>) -> Self {
        let max_duty = servo.lock().unwrap().get_max_duty();
        let max_angle = max_duty / 8;
        let min_angle = max_duty / 40;
        Rudder {
            servo,
            max_angle,
            min_angle,
        }
    }

    pub(crate) fn start(&mut self) -> Result<()> {
        self.servo.lock().unwrap().enable()?;
        Ok(())
    }

    pub(crate) fn set_angle(&mut self, angle: u32) -> Result<()> {
        let duty = self.interpolate(angle);
        self.servo.lock().unwrap().set_duty(duty)?;
        Ok(())
    }

    fn interpolate(&mut self, angle: u32) -> u32 {
        angle * (self.max_angle - self.min_angle) / 180 + self.min_angle
    }
}

pub(crate) fn setup_rudder() -> Result<Rudder<'static>> {
    let peripherals = Peripherals::take()?;
    let servo_driver = LedcTimerDriver::new(
        peripherals.ledc.timer1,
        &TimerConfig::new()
            .frequency(50.Hz())
            .resolution(ledc::Resolution::Bits14),
    )?;
    let servo = Arc::new(Mutex::new(
        LedcDriver::new(
            peripherals.ledc.channel1,
            servo_driver,
            peripherals.pins.gpio5,
        )
        .unwrap(),
    ));
    let mut rudder = Rudder::new(servo);
    rudder.start()?;
    Ok(rudder)
}
