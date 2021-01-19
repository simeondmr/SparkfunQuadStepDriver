extern crate sysfs_gpio;

use sysfs_gpio::Pin;
use sysfs_gpio::Direction::Out;

pub struct SparkfunQuadStep {
    torque: Pin,
    step1: Pin,
    step2: Pin,
    step4: Pin,
    step8: Pin,
    step16: Pin,
    mot_en1: Pin,
    mot_dir1: Pin,
    mot_ms11: Pin,
    mot_ms12: Pin,
    mot_ms13: Pin,
    mot_en2: Pin,
    mot_dir2: Pin,
    mot_ms21: Pin,
    mot_ms22: Pin,
    mot_ms23: Pin,
    mot_en3: Pin,
    mot_dir3: Pin,
    mot_ms31: Pin,
    mot_ms32: Pin,
    mot_ms33: Pin,
    mot_en4: Pin,
    mot_dir4: Pin,
    mot_ms41: Pin,
    mot_ms42: Pin,
    mot_ms43: Pin
}

pub enum Motor {
    Ch1,
    Ch2,
    Ch3,
    Ch4
}

impl SparkfunQuadStep {
    pub fn new(&mut self, mot: Motor, mot_en: u64, mot_dir: u64, mot_ms1: u64, mot_ms2: u64, mot_ms3: u64) {
        let mot_en = Pin::new(mot_en);
        mot_en.set_direction(Out);
        let mot_dir = Pin::new(mot_dir);
        mot_dir.set_direction(Out);
        let mot_ms1 = Pin::new(mot_ms1);
        mot_ms1.set_direction(Out);
        let mot_ms2 = Pin::new(mot_ms2);
        mot_ms2.set_direction(Out);
        let mot_ms3 = Pin::new(mot_ms3);
        mot_ms3.set_direction(Out);
        match mot {
            Ch1 => {
                self.mot_en1 = mot_en;
                self.mot_dir1 = mot_dir;
                self.mot_ms11 = mot_ms1;
                self.mot_ms12 = mot_ms2;
                self.mot_ms13 = mot_ms3;
            }
            Ch2 => {
                self.mot_en2 = mot_en;
                self.mot_dir2 = mot_dir;
                self.mot_ms21 = mot_ms1;
                self.mot_ms22 = mot_ms2;
                self.mot_ms23 = mot_ms3;
            }
            Ch3 => {
                self.mot_en3 = mot_en;
                self.mot_dir3 = mot_dir;
                self.mot_ms31 = mot_ms1;
                self.mot_ms32 = mot_ms2;
                self.mot_ms33 = mot_ms3;
            }
            Ch4 => {
                self.mot_en4 = mot_en;
                self.mot_dir4 = mot_dir;
                self.mot_ms41 = mot_ms1;
                self.mot_ms42 = mot_ms2;
                self.mot_ms43 = mot_ms3;
            }
        }
    }
}