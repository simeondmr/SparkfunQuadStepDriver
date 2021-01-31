// Written by Simeon Tornabene
// This library implement a driver for the "Sparkfun QuadStep" board
// Sparkfun QuadStep can drive up to four stepper but not simultaneously
// For more info about Sparkfun QuadStep board: http://cdn.sparkfun.com/datasheets/Widgets/A4983SETTR-T-Allegro-datasheet-123643.pdf

use gpio_cdev::{Chip, LineRequestFlags, LineHandle};
use embedded_hal::blocking::delay::DelayUs;
use linux_embedded_hal::Delay;

#[derive(Default)]
pub struct SparkfunQuadStep {
    torque: Option<u32>,
    step1: Option<u32>,
    step2: Option<u32>,
    step4: Option<u32>,
    step8: Option<u32>,
    step16: Option<u32>,
    ch1: Option<SparkfunQuadStepChannel>,
    ch2: Option<SparkfunQuadStepChannel>,
    ch3: Option<SparkfunQuadStepChannel>,
    ch4: Option<SparkfunQuadStepChannel>
}

#[derive(Default)]
struct SparkfunQuadStepChannel {
    mot_en: Option<LineHandle>,
    mot_stp: Option<LineHandle>,
    mot_dir: Option<LineHandle>,
    mot_ms1: Option<LineHandle>,
    mot_ms2: Option<LineHandle>,
    mot_ms3: Option<LineHandle>
}

pub enum Motor {
    Ch1,
    Ch2,
    Ch3,
    Ch4
}

pub enum StepSize {
    One,
    Two,
    Four,
    Eight,
    Sixteen
}

const STEPMIN: u32 = 800;

const STEP_MODE: ((u8,u8, u8), (u8,u8, u8), (u8,u8, u8),(u8,u8, u8),(u8,u8, u8)) = ((0, 0, 0), (1, 0, 0), (0, 1, 0), (1, 1, 0), (1, 1, 1));

impl SparkfunQuadStep {
    pub fn new() -> SparkfunQuadStep {
        SparkfunQuadStep {
            ch1: Some(SparkfunQuadStepChannel {
                ..Default::default()
            }),
            ch2: Some(SparkfunQuadStepChannel {
                ..Default::default()
            }),
            ch3: Some(SparkfunQuadStepChannel {
                ..Default::default()
            }),
            ch4: Some(SparkfunQuadStepChannel {
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    //This method allow to add a new stepper
    pub fn add_mot(&mut self, mot: Motor, mot_stp:u32, mot_en: u32, mot_dir: u32, mot_ms1: u32, mot_ms2: u32, mot_ms3: u32) -> Result<(), gpio_cdev::Error> {
        let mut gpio_chip = Chip::new("/dev/gpiochip0")?;
        let mot_stp = gpio_chip
            .get_line(mot_stp)?
            .request(LineRequestFlags::OUTPUT, 1, "SparkfunQuadStepDriver")?;
        let mot_en = gpio_chip
            .get_line(mot_en)?
            .request(LineRequestFlags::OUTPUT, 1, "SparkfunQuadStepDriver")?;
        let mot_dir = gpio_chip
            .get_line(mot_dir)?
            .request(LineRequestFlags::OUTPUT, 1, "SparkfunQuadStepDriver")?;
        let mot_ms1 = gpio_chip
            .get_line(mot_ms1)?
            .request(LineRequestFlags::OUTPUT, 1, "SparkfunQuadStepDriver")?;
        let mot_ms2 = gpio_chip
            .get_line(mot_ms2)?
            .request(LineRequestFlags::OUTPUT, 1, "SparkfunQuadStepDriver")?;
        let mot_ms3 = gpio_chip
            .get_line(mot_ms3)?
            .request(LineRequestFlags::OUTPUT, 1, "SparkfunQuadStepDriver")?;
        mot_en.set_value(1)?;
        mot_dir.set_value(0)?;
        let ch = match mot {
            Motor::Ch1 => self.ch1.as_mut().unwrap(),
            Motor::Ch2 => self.ch2.as_mut().unwrap(),
            Motor::Ch3 => self.ch3.as_mut().unwrap(),
            Motor::Ch4 =>self.ch4.as_mut().unwrap()
        };
        ch.mot_stp = Some(mot_stp);
        ch.mot_en = Some(mot_en);
        ch.mot_dir = Some(mot_dir);
        ch.mot_ms1 = Some(mot_ms1);
        ch.mot_ms2 = Some(mot_ms2);
        ch.mot_ms3 = Some(mot_ms3);
        Ok(())
    }

    fn curr_ctrl(&mut self, step: &StepSize) {
        match step {
            StepSize::One => self.step1 = Some(STEPMIN + (self.torque.unwrap() * 260)),
            StepSize::Two => self.step2 = Some(STEPMIN + (self.torque.unwrap() * 260) / 2),
            StepSize::Four => self.step4 = Some(STEPMIN + (self.torque.unwrap() * 260) / 4),
            StepSize::Eight => self.step8 = Some(STEPMIN + (self.torque.unwrap() * 260) / 8),
            StepSize::Sixteen => self.step16 = Some(STEPMIN + (self.torque.unwrap() * 260) / 16)
        }
    }

    pub fn stall(&mut self, mot: Motor) -> Result<(), gpio_cdev::Error> {
        match mot {
            Motor::Ch1 => self.ch1.as_mut().unwrap().mot_en.as_mut().unwrap().set_value(0)?,
            Motor::Ch2 => self.ch2.as_mut().unwrap().mot_en.as_mut().unwrap().set_value(0)?,
            Motor::Ch3 => self.ch3.as_mut().unwrap().mot_en.as_mut().unwrap().set_value(0)?,
            Motor::Ch4 => self.ch4.as_mut().unwrap().mot_en.as_mut().unwrap().set_value(0)?
        };
       Ok(())
    }

    //This method allow you to drive a stepper motor of a specific channel
    pub fn run(&mut self, mot: Motor, step_size: StepSize, num_step: i32, torque: u32) -> Result<(), gpio_cdev::Error> {
        let dir = if num_step > 0 { 1 } else { 0 };
        let num_step = num_step.abs();
        self.torque = Some(torque);
        self.curr_ctrl(&step_size);
        let ch = match mot {
            Motor::Ch1 => self.ch1.as_ref().unwrap(),
            Motor::Ch2 => self.ch2.as_ref().unwrap(),
            Motor::Ch3 => self.ch3.as_ref().unwrap(),
            Motor::Ch4 => self.ch4.as_ref().unwrap()
        };
        let ms_pin = match step_size {
            StepSize::One=> STEP_MODE.0,
            StepSize::Two => STEP_MODE.1,
            StepSize::Four => STEP_MODE.2,
            StepSize::Eight => STEP_MODE.3,
            StepSize::Sixteen => STEP_MODE.4
        };
        ch.mot_dir.as_ref().unwrap().set_value(dir)?;
        ch.mot_ms1.as_ref().unwrap().set_value(ms_pin.0)?;
        ch.mot_ms2.as_ref().unwrap().set_value(ms_pin.1)?;
        ch.mot_ms3.as_ref().unwrap().set_value(ms_pin.2)?;
        ch.mot_en.as_ref().unwrap().set_value(0)?;
        let mut delay = Delay { };
        for _ in 0..num_step {
            ch.mot_stp.as_ref().unwrap().set_value(1)?;
            delay.delay_us(self.step1.unwrap());
            ch.mot_stp.as_ref().unwrap().set_value(0)?;
            delay.delay_us(self.step1.unwrap());
        }
        ch.mot_stp.as_ref().unwrap().set_value(0)?;
        ch.mot_en.as_ref().unwrap().set_value(1)?;
        Ok(())
    }

    // Really important: use this method only if you are using the BUS header of the board(you must weld the BUS enable on the rear of the board)
    // With this method the Sparkfun QuadStep board can drive all four stepper using only the BUS
    // Note: at the moment, using the BUS header you cannot specify the the step size
    pub fn run_by_bus(stp_pin: u32, en_pin: u32, dir_pin: u32, num_step: i32, torque: u32) -> Result<(), gpio_cdev::Error> {
        let mut gpio_chip = Chip::new("/dev/gpiochip0")?;
        let stp_pin = gpio_chip
            .get_line(stp_pin)?
            .request(LineRequestFlags::OUTPUT, 1, "SparkfunQuadStepDriver")?;
        let en_pin = gpio_chip
            .get_line(en_pin)?
            .request(LineRequestFlags::OUTPUT, 1, "SparkfunQuadStepDriver")?;
        let dir_pin = gpio_chip
            .get_line(dir_pin)?
            .request(LineRequestFlags::OUTPUT, 1, "SparkfunQuadStepDriver")?;
        let dir = if num_step > 0 { 1 } else { 0 };
        let num_step = num_step.abs();
        let step = STEPMIN + (torque * 260);
        en_pin.set_value(0)?;
        dir_pin.set_value(dir)?;
        let mut delay = Delay { };
        for _ in 0..num_step {
            stp_pin.set_value(1)?;
            delay.delay_us(step);
            stp_pin.set_value(0)?;
            delay.delay_us(step);
        }
        stp_pin.set_value(0)?;
        en_pin.set_value(1)?;
        Ok(())
    }
}