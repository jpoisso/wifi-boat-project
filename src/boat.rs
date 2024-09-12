use anyhow::Result;
use esp_idf_svc::hal::ledc::LedcDriver;

pub(crate) struct Boat<'a> {
    pub(crate) motor: Motor<'a>,
    pub(crate) rudder: Rudder<'a>,
}

pub(crate) struct Motor<'a> {
    pwm: LedcDriver<'a>,
    max_duty: u32,
}

impl<'a> Motor<'a> {
    pub(crate) fn new(pwm: LedcDriver<'a>) -> Self {
        let max_duty = pwm.get_max_duty();
        Motor { pwm, max_duty }
    }

    pub(crate) fn start(&mut self) -> Result<()> {
        self.pwm.enable()?;
        self.set_power(self.max_duty / 2)?;
        Ok(())
    }

    pub(crate) fn set_power(&mut self, value: u32) -> Result<()> {
        self.pwm.set_duty(value)?;
        Ok(())
    }
}

pub(crate) struct Rudder<'a> {
    servo: LedcDriver<'a>,
    min_duty: u32,
    max_duty: u32,
    min_angle: u32,
    max_angle: u32,
}

impl<'a> Rudder<'a> {
    pub(crate) fn new(servo: LedcDriver<'a>) -> Self {
        let max_duty = servo.get_max_duty();
        let min_duty = max_duty * 25 / 1000;
        let max_duty = max_duty * 125 / 1000;
        Rudder {
            servo,
            min_duty,
            max_duty,
            min_angle: 0,
            max_angle: 180,
        }
    }

    pub(crate) fn start(&mut self) -> Result<()> {
        self.servo.enable()?;
        self.set_angle(90)
    }

    pub(crate) fn set_angle(&mut self, angle: u32) -> Result<()> {
        let duty = map_angle_to_duty(
            angle,
            self.min_angle,
            self.max_angle,
            self.min_duty,
            self.max_duty,
        );
        self.servo.set_duty(duty)?;
        Ok(())
    }
}

fn map_angle_to_duty(x: u32, in_min: u32, in_max: u32, out_min: u32, out_max: u32) -> u32 {
    (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}
