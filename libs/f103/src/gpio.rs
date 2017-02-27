use core::ops::{Index, IndexMut};
use ::Side;

pub fn enable(side: Side){

}

pub fn configure(side: Side) -> Configure {
    Configure {
        side: side
    }
}

pub struct Configure {
    side: Side
}

impl Configure {
    pub fn configure_input(&mut self, index: u8, config: PinInput) {
        if self.index > 8 {
            self.side.set_crh((self.index - 8) * 2, 0);
            self.side.set_crh((self.index - 8) * 2 + 1, config as u8);
        } else {
            self.side.set_crl(self.index * 2, 0);
            self.side.set_crl(self.index * 2 + 1, config as u8);;
        }
    }
    pub fn configure_output(&mut self, index: u8, speed: PinSpeed, config: PinOutput){
        if self.index > 8 {
            self.side.set_crh((self.index - 8) * 2, speed as u8);
            self.side.set_crh((self.index - 8) * 2 + 1, config as u8);
        } else {
            self.side.set_crl(self.index * 2, speed as u8);
            self.side.set_crl(self.index * 2 + 1, config as u8);;
        }
    }
}


pub enum PinSpeed {
    Output10Mhz = 0b01,
    Output2Mhz = 0b10,
    Output50Mhz = 0b11,
}

pub enum PinInput {
    Analog = 0b00,
    Floating = 0b01,
    PullUpPullDown = 0b10,
    Reserved = 0b11,
}
pub enum PinOutput {
    GeneralPushPull = 0b00,
    GeneralOpenDrain = 0b01,
    AlternatePushPull = 0b10,
    AlternateOpenDrain = 0b11,
}

impl<'a> ConfigurePin<'a>{
    pub fn mode(&mut self, mode: PinMode) -> Self {
        if self.index > 8 {
            self.configure.set_crh((self.index - 8) * 2, mode as u8);
        } else {
            self.configure.set_crl(self.index * 2, mode as u8);
        }
        self
    }

    pub fn input_config(&mut self, config: PinInputConfig) -> Self {
        if self.index > 8 {
            self.configure.set_crh((self.index - 8) * 2 + 1, config as u8);
        } else {
            self.configure.set_crl(self.index * 2 + 1, config as u8);
        }
        self
    }

    pub fn output_config(&mut self, config: PinOutputConfig) -> Self {
        if self.index > 8 {
            self.configure.set_crh((self.index - 8) * 2 + 1, config as u8);
        } else {
            self.configure.set_crl(self.index * 2 + 1, config as u8);
        }
        self
    }
}

