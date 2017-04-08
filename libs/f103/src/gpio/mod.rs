implement_enum!(PortMode: u8 {
    AnalogInput = 0b0000,
    FloatingInput = 0b0100,
    PullUpPullDownInput = 0b1000,

    OutputPushPull10mhz = 0b0001,
    OutputOpenDrain10mhz = 0b0101,
    AlternativePushPull10mhz = 0b1001,
    AlternativeOpenDrain10mhz = 0b1101,
    
    OutputPushPull2mhz = 0b0010,
    OutputOpenDrain2mhz = 0b0110,
    AlternativePushPull2mhz = 0b1010,
    AlternativeOpenDrain2mhz = 0b1110,
    
    OutputPushPull50mhz = 0b0011,
    OutputOpenDrain50mhz = 0b0111,
    AlternativePushPull50mhz = 0b1011,
    AlternativeOpenDrain50mhz = 0b1111,
});

macro_rules! implement_gpio {
    ($name:ident, $address:expr) => {
        pub mod $name {
            pub use super::PortMode;

            use core::ops::{Index, IndexMut};
            use core::ptr;
            
            #[derive(Default)]
            pub struct PortConfiguration([PortMode;16]);
            impl PortConfiguration {
                pub fn read() -> PortConfiguration {
                    unsafe {
                        let value: u64 = ptr::read_volatile($address.0 as u64 as *const _);
                        // TODO: Macro this?
                        PortConfiguration([
                            PortMode::from((value & 0b1111) as u8),
                            PortMode::from(((value >> 4) & 0b1111) as u8),
                            PortMode::from(((value >> 8) & 0b1111) as u8),
                            PortMode::from(((value >> 12) & 0b1111) as u8),
                            PortMode::from(((value >> 16) & 0b1111) as u8),
                            PortMode::from(((value >> 20) & 0b1111) as u8),
                            PortMode::from(((value >> 24) & 0b1111) as u8),
                            PortMode::from(((value >> 28) & 0b1111) as u8),
                            PortMode::from(((value >> 32) & 0b1111) as u8),
                            PortMode::from(((value >> 36) & 0b1111) as u8),
                            PortMode::from(((value >> 40) & 0b1111) as u8),
                            PortMode::from(((value >> 44) & 0b1111) as u8),
                            PortMode::from(((value >> 48) & 0b1111) as u8),
                            PortMode::from(((value >> 52) & 0b1111) as u8),
                            PortMode::from(((value >> 54) & 0b1111) as u8),
                            PortMode::from(((value >> 58) & 0b1111) as u8),
                        ])
                    }
                }

                pub fn save(self) {
                    let mut value: u64 = 0;
                    for i in 0..16 {
                        value = (self.0[i] as u64) << (i * 4);
                    }
                    unsafe { ptr::write_volatile($address.0 as u64 as *mut _, value); }
                }
            }

            impl Index<u8> for PortConfiguration {
                type Output = PortMode;
                fn index(&self, i: u8) -> &PortMode {
                    &self.0[i as usize]
                }
            }

            impl IndexMut<u8> for PortConfiguration {
                fn index_mut (&mut self, i: u8) -> &mut PortMode {
                    &mut self.0[i as usize]
                }
            }

            pub struct Input(u16);

            impl Input {
                pub fn read() -> Input {
                    let value = unsafe { ptr::read_volatile(($address.0 + 0x08) as u32 as *const u32) };
                    Input(value as u16)
                }
            }

            impl Index<u8> for Input {
                type Output = bool;
                fn index(&self, index: u8) -> &bool {
                    if (self.0 & (1 << index)) > 0 { &::TRUE } else { &::FALSE }
                }
            }

            pub struct Output(u16);

            impl Output {
                pub fn read() -> Output {
                    let value = unsafe { ptr::read_volatile(($address.0 + 0x0C) as u32 as *const u32) };
                    Output(value as u16)
                }

                pub fn set(&mut self, index: u8, value: bool) -> &mut Self {
                    if value {
                        self.0 |= 1 << index;
                    } else {
                        self.0 &= !(1 << index);
                    }
                    self
                }

                pub fn save(self) {

                }
            }

            impl Index<u8> for Output {
                type Output = bool;
                fn index(&self, index: u8) -> &bool {
                    if (self.0 & (1 << index)) > 0 { &::TRUE } else { &::FALSE }
                }
            }

            

        }
    }
}

implement_gpio!(a, ::address::GPIOA);
// implement_gpio!(b, ::address::GPIOB);
// implement_gpio!(c, ::address::GPIOC);
// implement_gpio!(d, ::address::GPIOD);
// implement_gpio!(e, ::address::GPIOE);
// implement_gpio!(f, ::address::GPIOF);
// implement_gpio!(g, ::address::GPIOG);