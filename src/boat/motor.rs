use anyhow::Result;
use esp_idf_svc::hal::{
    ledc::{config::TimerConfig, LedcDriver, LedcTimerDriver},
    prelude::*,
};

pub(crate) struct Motor<'a> {
    pwm: LedcDriver<'a>,
}

impl<'a> Motor<'a> {
    pub(crate) fn new(pwm: LedcDriver<'a>) -> Self {
        Motor { pwm }
    }

    pub(crate) fn start(&mut self) -> Result<()> {
        self.pwm.enable()?;
        Ok(())
    }

    pub(crate) fn set_power(&mut self, value: u8) -> Result<()> {
        self.pwm.set_duty(value.into())?;
        Ok(())
    }
}

pub(crate) fn setup_motor() -> Result<Motor<'static>> {
    let peripherals = Peripherals::take()?;
    let pwm = LedcDriver::new(
        peripherals.ledc.channel0,
        LedcTimerDriver::new(
            peripherals.ledc.timer0,
            &TimerConfig::new().frequency(10.kHz().into()),
        )?,
        peripherals.pins.gpio5,
    )?;
    let mut motor = Motor::new(pwm);
    motor.start()?;
    Ok(motor)
}
