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
    step16: Option<LineHandle>,
    mot_en1: Option<LineHandle>,
    mot_dir1: Option<LineHandle>,
    mot_ms11: Option<LineHandle>,
    mot_ms12: Option<LineHandle>,
    mot_ms13: Option<LineHandle>,
    mot_en2: Option<LineHandle>,
    mot_dir2: Option<LineHandle>,
    mot_ms21: Option<LineHandle>,
    mot_ms22: Option<LineHandle>,
    mot_ms23: Option<LineHandle>,
    mot_en3: Option<LineHandle>,
    mot_dir3: Option<LineHandle>,
    mot_ms31: Option<LineHandle>,
    mot_ms32: Option<LineHandle>,
    mot_ms33: Option<LineHandle>,
    mot_en4: Option<LineHandle>,
    mot_dir4: Option<LineHandle>,
    mot_ms41: Option<LineHandle>,
    mot_ms42: Option<LineHandle>,
    mot_ms43: Option<LineHandle>,
    raspberry_ctrl_pin: RaspberryControlPin
}

#[derive(Default)]
struct RaspberryControlPin {
    pin15: Option<LineHandle>,
    pin11: Option<LineHandle>,
    pin23: Option<LineHandle>,
    pin24: Option<LineHandle>,
}

pub enum Motor {
    Ch1,
    Ch2,
    Ch3,
    Ch4
}

pub enum StepSize {
    ONE,
    TWO,
    FOUR,
    EIGHT,
    SIXTEEN
}

const STEPMIN: u32 = 800;

impl SparkfunQuadStep {
    //maybe pwd pin???
    pub fn new() -> SparkfunQuadStep {
        let mut pin15 = Chip::new("/dev/gpiochip0").unwrap()
            .get_line(15).unwrap()
            .request(LineRequestFlags::OUTPUT, 1, "SparkfunQuadStepDriver").unwrap();
        let mut pin11 = Chip::new("/dev/gpiochip0").unwrap()
            .get_line(11).unwrap()
            .request(LineRequestFlags::OUTPUT, 1, "SparkfunQuadStepDriver").unwrap();
        let mut pin23 = Chip::new("/dev/gpiochip0").unwrap()
            .get_line(23).unwrap()
            .request(LineRequestFlags::OUTPUT, 1, "SparkfunQuadStepDriver").unwrap();
        let mut pin24 = Chip::new("/dev/gpiochip0").unwrap()
            .get_line(24).unwrap()
            .request(LineRequestFlags::OUTPUT, 1, "SparkfunQuadStepDriver").unwrap();
        pin15.set_value(0);
        pin11.set_value(0);
        pin23.set_value(0);
        pin24.set_value(0);
        SparkfunQuadStep {
            raspberry_ctrl_pin : RaspberryControlPin {
                pin15: Some(pin15),
                pin11: Some(pin11),
                pin23: Some(pin23),
                pin24: Some(pin24)
            },
            ..Default::default()
        }
    }

    pub fn add_mot(&mut self, mot: Motor, mot_en: u32, mot_dir: u32, mot_ms1: u32, mot_ms2: u32, mot_ms3: u32) {
        let mot_en = Chip::new("/dev/gpiochip0").unwrap()
            .get_line(mot_en).unwrap()
            .request(LineRequestFlags::OUTPUT, 1, "SparkfunQuadStepDriver").unwrap();
        let mot_dir = Chip::new("/dev/gpiochip0").unwrap()
            .get_line(mot_dir).unwrap()
            .request(LineRequestFlags::OUTPUT, 1, "SparkfunQuadStepDriver").unwrap();
        let mot_ms1 = Chip::new("/dev/gpiochip0").unwrap()
            .get_line(mot_ms1).unwrap()
            .request(LineRequestFlags::OUTPUT, 1, "SparkfunQuadStepDriver")
            .unwrap();
        let mot_ms2 = Chip::new("/dev/gpiochip0").unwrap()
            .get_line(mot_ms2).unwrap()
            .request(LineRequestFlags::OUTPUT, 1, "SparkfunQuadStepDriver").unwrap();
        let mot_ms3 = Chip::new("/dev/gpiochip0").unwrap()
            .get_line(mot_ms3).unwrap()
            .request(LineRequestFlags::OUTPUT, 1, "SparkfunQuadStepDriver").unwrap();
        mot_en.set_value(1);
        mot_dir.set_value(0);
        match mot {
            Ch1 => {
                self.mot_en1 = Some(mot_en);
                self.mot_dir1 = Some(mot_dir);
                self.mot_ms11 = Some(mot_ms1);
                self.mot_ms12 = Some(mot_ms2);
                self.mot_ms13 = Some(mot_ms3);
            }
            Ch2 => {
                self.mot_en2 = Some(mot_en);
                self.mot_dir2 = Some(mot_dir);
                self.mot_ms21 = Some(mot_ms1);
                self.mot_ms22 = Some(mot_ms2);
                self.mot_ms23 = Some(mot_ms3);
            }
            Ch3 => {
                self.mot_en3 = Some(mot_en);
                self.mot_dir3 = Some(mot_dir);
                self.mot_ms31 = Some(mot_ms1);
                self.mot_ms32 = Some(mot_ms2);
                self.mot_ms33 = Some(mot_ms3);
            }
            Ch4 => {
                self.mot_en4 = Some(mot_en);
                self.mot_dir4 = Some(mot_dir);
                self.mot_ms41 = Some(mot_ms1);
                self.mot_ms42 = Some(mot_ms2);
                self.mot_ms43 = Some(mot_ms3);
            }
        }
    }

    fn curr_cntl(&mut self, step: StepSize) {
        match step {
            StepSize::ONE => self.step1 = Some(STEPMIN + (self.torque.unwrap() * 260)),
            StepSize::TWO => self.step1 = Some(STEPMIN + (self.torque.unwrap() * 260) / 2),
            StepSize::FOUR => self.step1 = Some(STEPMIN + (self.torque.unwrap() * 260) / 4),
            StepSize::EIGHT => self.step1 = Some(STEPMIN + (self.torque.unwrap() * 260) / 8),
            StepSize::SIXTEEN => self.step1 = Some(STEPMIN + (self.torque.unwrap() * 260) / 16)
        }
    }

    pub fn mot_run(&mut self, mot: Motor, step_size: StepSize, number_step: i32, torque: i32) {
        let dir = if number_step > 0 { 1 } else { 0 };
        let number_step = number_step.abs();
        if let mot = Motor::Ch1 {
            let a = self.mot_dir1.as_ref().unwrap().set_value(dir);
            if let step_size = StepSize::ONE {
                self.curr_cntl(StepSize::ONE);
                self.mot_ms11.as_ref().unwrap().set_value(0);
                self.mot_ms12.as_ref().unwrap().set_value(0);
                self.mot_ms13.as_ref().unwrap().set_value(0);
                self.mot_en1.as_ref().unwrap().set_value(0);
                let mut delay = Delay{};
                for i in 0..number_step - 1 {
                    self.raspberry_ctrl_pin.pin11.as_ref().unwrap().set_value(1);
                    delay.delay_us(self.step1.unwrap());
                    self.raspberry_ctrl_pin.pin11.as_ref().unwrap().set_value(0);
                    delay.delay_us(self.step1.unwrap());
                }
                self.raspberry_ctrl_pin.pin11.as_ref().unwrap().set_value(0);
                self.mot_en1.as_ref().unwrap().set_value(1);
            }
        }
    }
}