#![no_std]
#![deny(warnings)]
#![feature(asm)]

#[derive(Copy, Clone)]
pub enum ModerType {
    InputMode = 0b00,
    OutputMode = 0b01,
    AlternateMode = 0b10,
    AnalogMode = 0b11,
}
impl ::core::convert::From<u32> for ModerType {
    fn from(val: u32) -> ModerType {
        match val {
            0b00 => ModerType::InputMode,
            0b01 => ModerType::OutputMode,
            0b10 => ModerType::AlternateMode,
            0b11 => ModerType::AnalogMode,
            x => panic!("ModerType::From out of range: {}", x)
        }
    }
}
#[derive(Copy, Clone)]
pub enum PullDirection {
    None = 0b00,
    PullUp = 0b01,
    PullDown = 0b10,
    Reserved = 0b11,
}
impl ::core::convert::From<u32> for PullDirection {
    fn from(val: u32) -> PullDirection {
        match val {
            0b00 => PullDirection::None,
            0b01 => PullDirection::PullUp,
            0b10 => PullDirection::PullDown,
            0b11 => PullDirection::Reserved,
            x => panic!("PullDirection::From out of range: {}", x)
        }
    }
}
#[derive(Copy, Clone)]
pub enum GPIOSpeed {
    Low = 0b00,
    Medium = 0b01,
    High = 0b11,
}
impl ::core::convert::From<u32> for GPIOSpeed {
    fn from(val: u32) -> GPIOSpeed {
        match val {
            0b00 => GPIOSpeed::Low,
            0b01 => GPIOSpeed::Medium,
            0b10 => GPIOSpeed::Low,
            0b11 => GPIOSpeed::High,
            x => panic!("GPIOSpeed::From out of range: {}", x)
        }
    }
}
#[derive(Copy, Clone)]
pub enum CaptureCompareOneSection {
    Output = 0b00,
    InputTI1 = 0b01,
    InputTI2 = 0b10,
    InputTRC = 0b11,
}
impl ::core::convert::From<u32> for CaptureCompareOneSection {
    fn from(val: u32) -> CaptureCompareOneSection {
        match val {
            0b00 => CaptureCompareOneSection::Output,
            0b01 => CaptureCompareOneSection::InputTI1,
            0b10 => CaptureCompareOneSection::InputTI2,
            0b11 => CaptureCompareOneSection::InputTRC,
            x => panic!("CaptureCompareOneSection::From out of range: {}", x)
        }
    }
}
#[derive(Copy, Clone)]
pub enum MemoryMap {
    MainFlash = 0b00,
    SystemFlash = 0b01,
    EmbeddedSRAM = 0b11,
    FMCBank = 0b100,
}
impl ::core::convert::From<u32> for MemoryMap {
    fn from(val: u32) -> MemoryMap {
        match val {
            0b000 => MemoryMap::MainFlash,
            0b001 => MemoryMap::SystemFlash,
            0b010 => MemoryMap::MainFlash,
            0b011 => MemoryMap::EmbeddedSRAM,
            0b100 => MemoryMap::FMCBank,
            0b101 => MemoryMap::FMCBank,
            0b110 => MemoryMap::FMCBank,
            0b111 => MemoryMap::FMCBank,
            x => panic!("MemoryMap::From out of range: {}", x)
        }
    }
}
#[derive(Copy, Clone)]
pub enum CaptureCompareThreeSections {
    Output = 0b00,
    InputTI3 = 0b01,
    InputTI4 = 0b10,
    InputTRC = 0b11,
}
impl ::core::convert::From<u32> for CaptureCompareThreeSections {
    fn from(val: u32) -> CaptureCompareThreeSections {
        match val {
            0b00 => CaptureCompareThreeSections::Output,
            0b01 => CaptureCompareThreeSections::InputTI3,
            0b10 => CaptureCompareThreeSections::InputTI4,
            0b11 => CaptureCompareThreeSections::InputTRC,
            x => panic!("CaptureCompareThreeSections::From out of range: {}", x)
        }
    }
}
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum SampleTime {
    Cycles_1_5   = 0b000,
    Cycles_2_5   = 0b001,
    Cycles_4_5   = 0b010,
    Cycles_7_5   = 0b011,
    Cycles_19_5  = 0b100,
    Cycles_61_5  = 0b101,
    Cycles_181_5 = 0b110,
    Cycles_601_5 = 0b111,
}
impl ::core::convert::From<u32> for SampleTime {
    fn from(val: u32) -> SampleTime {
        match val {
            0b000 => SampleTime::Cycles_1_5,
            0b001 => SampleTime::Cycles_2_5,
            0b010 => SampleTime::Cycles_4_5,
            0b011 => SampleTime::Cycles_7_5,
            0b100 => SampleTime::Cycles_19_5,
            0b101 => SampleTime::Cycles_61_5,
            0b110 => SampleTime::Cycles_181_5,
            0b111 => SampleTime::Cycles_601_5,
            x => panic!("SampleTime::From out of range: {}", x)
        }
    }
}
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum SlaveMode {
    Disabled = 0b000,
    Encoder1 = 0b001,
    Encoder2 = 0b010,
    Encoder3 = 0b011,
    Reset = 0b100,
    Gated = 0b101,
    Trigger = 0b110,
    External = 0b111,
}
impl ::core::convert::From<u32> for SlaveMode {
    fn from(val: u32) -> SlaveMode {
        match val {
            0b000 => SlaveMode::Disabled,
            0b001 => SlaveMode::Encoder1,
            0b010 => SlaveMode::Encoder2,
            0b011 => SlaveMode::Encoder3,
            0b100 => SlaveMode::Reset,
            0b101 => SlaveMode::Gated,
            0b110 => SlaveMode::Trigger,
            0b111 => SlaveMode::External,
            x => panic!("SlaveMode::From out of range: {}", x)
        }
    }
}
pub trait CacheTrait {
    fn save(self);
}

#[no_mangle]
#[export_name = "__aeabi_memcpy"]
pub unsafe extern fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *dest.offset(i as isize) = *src.offset(i as isize);
        i += 1;
    }
    return dest
}

#[no_mangle]
#[export_name = "__aeabi_memclr4"]
pub unsafe extern fn memclr4(dest: *mut u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *dest.offset(i as isize) = 0;
        i += 1;
    }
    return dest
}

/// STM32F103xx
pub mod gpio;
pub mod dma;
pub mod tim;
pub mod i2c;
pub mod spi;
pub mod usart;
pub mod adc;
pub mod fsmc;
pub mod pwr;
pub mod rcc;
pub mod afio;
pub mod exti;
pub mod sdio;
pub mod rtc;
pub mod bkp;
pub mod iwdg;
pub mod wwdg;
pub mod can;
pub mod dac;
pub mod dbg;
pub mod crc;
pub mod flash;
pub mod nvic;
pub mod usb;
