pub const ADDRESS: u32 = 0x40012800;
/// status register
pub mod sr {
    pub const OFFSET: u32 = 0x0;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Regular channel start flag
        pub strt: bool,
        /// Injected channel start flag
        pub jstrt: bool,
        /// Injected channel end of conversion
        pub jeoc: bool,
        /// Regular channel end of conversion
        pub eoc: bool,
        /// Analog watchdog flag
        pub awd: bool,
    }
    pub struct Cache {
        /// Regular channel start flag
        pub strt: bool,
        /// Injected channel start flag
        pub jstrt: bool,
        /// Injected channel end of conversion
        pub jeoc: bool,
        /// Regular channel end of conversion
        pub eoc: bool,
        /// Analog watchdog flag
        pub awd: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            strt: ((value >> 4) & 0b1) > 0,
            jstrt: ((value >> 3) & 0b1) > 0,
            jeoc: ((value >> 2) & 0b1) > 0,
            eoc: ((value >> 1) & 0b1) > 0,
            awd: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            strt: ((value >> 4) & 0b1) > 0,
            jstrt: ((value >> 3) & 0b1) > 0,
            jeoc: ((value >> 2) & 0b1) > 0,
            eoc: ((value >> 1) & 0b1) > 0,
            awd: ((value >> 0) & 0b1) > 0,
        }
    }
    impl Cache {
        pub fn save(self) {
            // This will call ops::Drop below
        }
    }
    impl ::core::ops::Drop for Cache {
        fn drop(&mut self) {
            let value = 0
                | ((self.strt as u32) << 4)
                | ((self.jstrt as u32) << 3)
                | ((self.jeoc as u32) << 2)
                | ((self.eoc as u32) << 1)
                | ((self.awd as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// control register 1
pub mod cr1 {
    pub const OFFSET: u32 = 0x4;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Analog watchdog enable on regular channels
        pub awden: bool,
        /// Analog watchdog enable on injected channels
        pub jawden: bool,
        /// Discontinuous mode channel count
        pub discnum: bool,
        /// Discontinuous mode on injected channels
        pub jdiscen: bool,
        /// Discontinuous mode on regular channels
        pub discen: bool,
        /// Automatic injected group conversion
        pub jauto: bool,
        /// Enable the watchdog on a single channel in scan mode
        pub awdsgl: bool,
        /// Scan mode
        pub scan: bool,
        /// Interrupt enable for injected channels
        pub jeocie: bool,
        /// Analog watchdog interrupt enable
        pub awdie: bool,
        /// Interrupt enable for EOC
        pub eocie: bool,
        /// Analog watchdog channel select bits
        pub awdch: bool,
    }
    pub struct Cache {
        /// Analog watchdog enable on regular channels
        pub awden: bool,
        /// Analog watchdog enable on injected channels
        pub jawden: bool,
        /// Discontinuous mode channel count
        pub discnum: bool,
        /// Discontinuous mode on injected channels
        pub jdiscen: bool,
        /// Discontinuous mode on regular channels
        pub discen: bool,
        /// Automatic injected group conversion
        pub jauto: bool,
        /// Enable the watchdog on a single channel in scan mode
        pub awdsgl: bool,
        /// Scan mode
        pub scan: bool,
        /// Interrupt enable for injected channels
        pub jeocie: bool,
        /// Analog watchdog interrupt enable
        pub awdie: bool,
        /// Interrupt enable for EOC
        pub eocie: bool,
        /// Analog watchdog channel select bits
        pub awdch: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            awden: ((value >> 23) & 0b1) > 0,
            jawden: ((value >> 22) & 0b1) > 0,
            discnum: ((value >> 13) & 0b1) > 0,
            jdiscen: ((value >> 12) & 0b1) > 0,
            discen: ((value >> 11) & 0b1) > 0,
            jauto: ((value >> 10) & 0b1) > 0,
            awdsgl: ((value >> 9) & 0b1) > 0,
            scan: ((value >> 8) & 0b1) > 0,
            jeocie: ((value >> 7) & 0b1) > 0,
            awdie: ((value >> 6) & 0b1) > 0,
            eocie: ((value >> 5) & 0b1) > 0,
            awdch: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            awden: ((value >> 23) & 0b1) > 0,
            jawden: ((value >> 22) & 0b1) > 0,
            discnum: ((value >> 13) & 0b1) > 0,
            jdiscen: ((value >> 12) & 0b1) > 0,
            discen: ((value >> 11) & 0b1) > 0,
            jauto: ((value >> 10) & 0b1) > 0,
            awdsgl: ((value >> 9) & 0b1) > 0,
            scan: ((value >> 8) & 0b1) > 0,
            jeocie: ((value >> 7) & 0b1) > 0,
            awdie: ((value >> 6) & 0b1) > 0,
            eocie: ((value >> 5) & 0b1) > 0,
            awdch: ((value >> 0) & 0b1) > 0,
        }
    }
    impl Cache {
        pub fn save(self) {
            // This will call ops::Drop below
        }
    }
    impl ::core::ops::Drop for Cache {
        fn drop(&mut self) {
            let value = 0
                | ((self.awden as u32) << 23)
                | ((self.jawden as u32) << 22)
                | ((self.discnum as u32) << 13)
                | ((self.jdiscen as u32) << 12)
                | ((self.discen as u32) << 11)
                | ((self.jauto as u32) << 10)
                | ((self.awdsgl as u32) << 9)
                | ((self.scan as u32) << 8)
                | ((self.jeocie as u32) << 7)
                | ((self.awdie as u32) << 6)
                | ((self.eocie as u32) << 5)
                | ((self.awdch as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// control register 2
pub mod cr2 {
    pub const OFFSET: u32 = 0x8;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Temperature sensor and VREFINT enable
        pub tsvrefe: bool,
        /// Start conversion of regular channels
        pub swstart: bool,
        /// Start conversion of injected channels
        pub jswstart: bool,
        /// External trigger conversion mode for regular channels
        pub exttrig: bool,
        /// External event select for regular group
        pub extsel: bool,
        /// External trigger conversion mode for injected channels
        pub jexttrig: bool,
        /// External event select for injected group
        pub jextsel: bool,
        /// Data alignment
        pub align: bool,
        /// Direct memory access mode
        pub dma: bool,
        /// Reset calibration
        pub rstcal: bool,
        /// A/D calibration
        pub cal: bool,
        /// Continuous conversion
        pub cont: bool,
        /// A/D converter ON / OFF
        pub adon: bool,
    }
    pub struct Cache {
        /// Temperature sensor and VREFINT enable
        pub tsvrefe: bool,
        /// Start conversion of regular channels
        pub swstart: bool,
        /// Start conversion of injected channels
        pub jswstart: bool,
        /// External trigger conversion mode for regular channels
        pub exttrig: bool,
        /// External event select for regular group
        pub extsel: bool,
        /// External trigger conversion mode for injected channels
        pub jexttrig: bool,
        /// External event select for injected group
        pub jextsel: bool,
        /// Data alignment
        pub align: bool,
        /// Direct memory access mode
        pub dma: bool,
        /// Reset calibration
        pub rstcal: bool,
        /// A/D calibration
        pub cal: bool,
        /// Continuous conversion
        pub cont: bool,
        /// A/D converter ON / OFF
        pub adon: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            tsvrefe: ((value >> 23) & 0b1) > 0,
            swstart: ((value >> 22) & 0b1) > 0,
            jswstart: ((value >> 21) & 0b1) > 0,
            exttrig: ((value >> 20) & 0b1) > 0,
            extsel: ((value >> 17) & 0b1) > 0,
            jexttrig: ((value >> 15) & 0b1) > 0,
            jextsel: ((value >> 12) & 0b1) > 0,
            align: ((value >> 11) & 0b1) > 0,
            dma: ((value >> 8) & 0b1) > 0,
            rstcal: ((value >> 3) & 0b1) > 0,
            cal: ((value >> 2) & 0b1) > 0,
            cont: ((value >> 1) & 0b1) > 0,
            adon: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            tsvrefe: ((value >> 23) & 0b1) > 0,
            swstart: ((value >> 22) & 0b1) > 0,
            jswstart: ((value >> 21) & 0b1) > 0,
            exttrig: ((value >> 20) & 0b1) > 0,
            extsel: ((value >> 17) & 0b1) > 0,
            jexttrig: ((value >> 15) & 0b1) > 0,
            jextsel: ((value >> 12) & 0b1) > 0,
            align: ((value >> 11) & 0b1) > 0,
            dma: ((value >> 8) & 0b1) > 0,
            rstcal: ((value >> 3) & 0b1) > 0,
            cal: ((value >> 2) & 0b1) > 0,
            cont: ((value >> 1) & 0b1) > 0,
            adon: ((value >> 0) & 0b1) > 0,
        }
    }
    impl Cache {
        pub fn save(self) {
            // This will call ops::Drop below
        }
    }
    impl ::core::ops::Drop for Cache {
        fn drop(&mut self) {
            let value = 0
                | ((self.tsvrefe as u32) << 23)
                | ((self.swstart as u32) << 22)
                | ((self.jswstart as u32) << 21)
                | ((self.exttrig as u32) << 20)
                | ((self.extsel as u32) << 17)
                | ((self.jexttrig as u32) << 15)
                | ((self.jextsel as u32) << 12)
                | ((self.align as u32) << 11)
                | ((self.dma as u32) << 8)
                | ((self.rstcal as u32) << 3)
                | ((self.cal as u32) << 2)
                | ((self.cont as u32) << 1)
                | ((self.adon as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// sample time register 1
pub mod smpr1 {
    pub const OFFSET: u32 = 0xC;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([::SampleTime;8]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = ::SampleTime;
        fn index(&self, index: u8) -> &::SampleTime {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut ::SampleTime {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([::SampleTime;8]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = ::SampleTime;
        fn index(&self, index: u8) -> &::SampleTime {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut ::SampleTime {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b111) .into(),
            ((value >> 3) & 0b111) .into(),
            ((value >> 6) & 0b111) .into(),
            ((value >> 9) & 0b111) .into(),
            ((value >> 12) & 0b111) .into(),
            ((value >> 15) & 0b111) .into(),
            ((value >> 18) & 0b111) .into(),
            ((value >> 21) & 0b111) .into(),
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b111) .into(),
            ((value >> 3) & 0b111) .into(),
            ((value >> 6) & 0b111) .into(),
            ((value >> 9) & 0b111) .into(),
            ((value >> 12) & 0b111) .into(),
            ((value >> 15) & 0b111) .into(),
            ((value >> 18) & 0b111) .into(),
            ((value >> 21) & 0b111) .into(),
        ])
    }
    impl Cache {
        pub fn save(self) {
            // This will call ops::Drop below
        }
    }
    impl ::core::ops::Drop for Cache {
        fn drop(&mut self) {
            let value = 0
                | ((self.0[0] as u32) << 0)
                | ((self.0[1] as u32) << 3)
                | ((self.0[2] as u32) << 6)
                | ((self.0[3] as u32) << 9)
                | ((self.0[4] as u32) << 12)
                | ((self.0[5] as u32) << 15)
                | ((self.0[6] as u32) << 18)
                | ((self.0[7] as u32) << 21)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// sample time register 2
pub mod smpr2 {
    pub const OFFSET: u32 = 0x10;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([::SampleTime;10]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = ::SampleTime;
        fn index(&self, index: u8) -> &::SampleTime {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut ::SampleTime {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([::SampleTime;10]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = ::SampleTime;
        fn index(&self, index: u8) -> &::SampleTime {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut ::SampleTime {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b111) .into(),
            ((value >> 3) & 0b111) .into(),
            ((value >> 6) & 0b111) .into(),
            ((value >> 9) & 0b111) .into(),
            ((value >> 12) & 0b111) .into(),
            ((value >> 15) & 0b111) .into(),
            ((value >> 18) & 0b111) .into(),
            ((value >> 21) & 0b111) .into(),
            ((value >> 24) & 0b111) .into(),
            ((value >> 27) & 0b111) .into(),
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b111) .into(),
            ((value >> 3) & 0b111) .into(),
            ((value >> 6) & 0b111) .into(),
            ((value >> 9) & 0b111) .into(),
            ((value >> 12) & 0b111) .into(),
            ((value >> 15) & 0b111) .into(),
            ((value >> 18) & 0b111) .into(),
            ((value >> 21) & 0b111) .into(),
            ((value >> 24) & 0b111) .into(),
            ((value >> 27) & 0b111) .into(),
        ])
    }
    impl Cache {
        pub fn save(self) {
            // This will call ops::Drop below
        }
    }
    impl ::core::ops::Drop for Cache {
        fn drop(&mut self) {
            let value = 0
                | ((self.0[0] as u32) << 0)
                | ((self.0[1] as u32) << 3)
                | ((self.0[2] as u32) << 6)
                | ((self.0[3] as u32) << 9)
                | ((self.0[4] as u32) << 12)
                | ((self.0[5] as u32) << 15)
                | ((self.0[6] as u32) << 18)
                | ((self.0[7] as u32) << 21)
                | ((self.0[8] as u32) << 24)
                | ((self.0[9] as u32) << 27)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// injected channel data offset register x
pub mod jofr1 {
    pub const OFFSET: u32 = 0x14;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([u16;1]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = u16;
        fn index(&self, index: u8) -> &u16 {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut u16 {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([u16;1]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = u16;
        fn index(&self, index: u8) -> &u16 {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut u16 {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b111111111111) as u16,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b111111111111) as u16,
        ])
    }
    impl Cache {
        pub fn save(self) {
            // This will call ops::Drop below
        }
    }
    impl ::core::ops::Drop for Cache {
        fn drop(&mut self) {
            let value = 0
                | ((self.0[0] as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// injected channel data offset register x
pub mod jofr2 {
    pub const OFFSET: u32 = 0x18;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([u16;1]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = u16;
        fn index(&self, index: u8) -> &u16 {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut u16 {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([u16;1]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = u16;
        fn index(&self, index: u8) -> &u16 {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut u16 {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b111111111111) as u16,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b111111111111) as u16,
        ])
    }
    impl Cache {
        pub fn save(self) {
            // This will call ops::Drop below
        }
    }
    impl ::core::ops::Drop for Cache {
        fn drop(&mut self) {
            let value = 0
                | ((self.0[0] as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// injected channel data offset register x
pub mod jofr3 {
    pub const OFFSET: u32 = 0x1C;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([u16;1]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = u16;
        fn index(&self, index: u8) -> &u16 {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut u16 {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([u16;1]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = u16;
        fn index(&self, index: u8) -> &u16 {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut u16 {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b111111111111) as u16,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b111111111111) as u16,
        ])
    }
    impl Cache {
        pub fn save(self) {
            // This will call ops::Drop below
        }
    }
    impl ::core::ops::Drop for Cache {
        fn drop(&mut self) {
            let value = 0
                | ((self.0[0] as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// injected channel data offset register x
pub mod jofr4 {
    pub const OFFSET: u32 = 0x20;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([u16;1]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = u16;
        fn index(&self, index: u8) -> &u16 {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut u16 {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([u16;1]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = u16;
        fn index(&self, index: u8) -> &u16 {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut u16 {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b111111111111) as u16,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b111111111111) as u16,
        ])
    }
    impl Cache {
        pub fn save(self) {
            // This will call ops::Drop below
        }
    }
    impl ::core::ops::Drop for Cache {
        fn drop(&mut self) {
            let value = 0
                | ((self.0[0] as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// watchdog higher threshold register
pub mod htr {
    pub const OFFSET: u32 = 0x24;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Analog watchdog higher threshold
        pub ht: u16,
    }
    pub struct Cache {
        /// Analog watchdog higher threshold
        pub ht: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            ht: ((value >> 0) & 0b111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            ht: ((value >> 0) & 0b111111111111) as u16,
        }
    }
    impl Cache {
        pub fn save(self) {
            // This will call ops::Drop below
        }
    }
    impl ::core::ops::Drop for Cache {
        fn drop(&mut self) {
            let value = 0
                | ((self.ht as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// watchdog lower threshold register
pub mod ltr {
    pub const OFFSET: u32 = 0x28;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Analog watchdog lower threshold
        pub lt: u16,
    }
    pub struct Cache {
        /// Analog watchdog lower threshold
        pub lt: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            lt: ((value >> 0) & 0b111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            lt: ((value >> 0) & 0b111111111111) as u16,
        }
    }
    impl Cache {
        pub fn save(self) {
            // This will call ops::Drop below
        }
    }
    impl ::core::ops::Drop for Cache {
        fn drop(&mut self) {
            let value = 0
                | ((self.lt as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// regular sequence register 1
pub mod sqr1 {
    pub const OFFSET: u32 = 0x2C;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Regular channel sequence length
        pub l: u8,
        /// 16th conversion in regular sequence
        pub sq16: u8,
        /// 15th conversion in regular sequence
        pub sq15: u8,
        /// 14th conversion in regular sequence
        pub sq14: u8,
        /// 13th conversion in regular sequence
        pub sq13: u8,
    }
    pub struct Cache {
        /// Regular channel sequence length
        pub l: u8,
        /// 16th conversion in regular sequence
        pub sq16: u8,
        /// 15th conversion in regular sequence
        pub sq15: u8,
        /// 14th conversion in regular sequence
        pub sq14: u8,
        /// 13th conversion in regular sequence
        pub sq13: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            l: ((value >> 20) & 0b1111) as u8,
            sq16: ((value >> 15) & 0b1111) as u8,
            sq15: ((value >> 10) & 0b1111) as u8,
            sq14: ((value >> 5) & 0b1111) as u8,
            sq13: ((value >> 0) & 0b1111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            l: ((value >> 20) & 0b1111) as u8,
            sq16: ((value >> 15) & 0b1111) as u8,
            sq15: ((value >> 10) & 0b1111) as u8,
            sq14: ((value >> 5) & 0b1111) as u8,
            sq13: ((value >> 0) & 0b1111) as u8,
        }
    }
    impl Cache {
        pub fn save(self) {
            // This will call ops::Drop below
        }
    }
    impl ::core::ops::Drop for Cache {
        fn drop(&mut self) {
            let value = 0
                | ((self.l as u32) << 20)
                | ((self.sq16 as u32) << 15)
                | ((self.sq15 as u32) << 10)
                | ((self.sq14 as u32) << 5)
                | ((self.sq13 as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// regular sequence register 2
pub mod sqr2 {
    pub const OFFSET: u32 = 0x30;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([u8;6]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = u8;
        fn index(&self, index: u8) -> &u8 {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut u8 {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([u8;6]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = u8;
        fn index(&self, index: u8) -> &u8 {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut u8 {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b11111) as u8,
            ((value >> 5) & 0b11111) as u8,
            ((value >> 10) & 0b11111) as u8,
            ((value >> 15) & 0b11111) as u8,
            ((value >> 20) & 0b11111) as u8,
            ((value >> 25) & 0b11111) as u8,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b11111) as u8,
            ((value >> 5) & 0b11111) as u8,
            ((value >> 10) & 0b11111) as u8,
            ((value >> 15) & 0b11111) as u8,
            ((value >> 20) & 0b11111) as u8,
            ((value >> 25) & 0b11111) as u8,
        ])
    }
    impl Cache {
        pub fn save(self) {
            // This will call ops::Drop below
        }
    }
    impl ::core::ops::Drop for Cache {
        fn drop(&mut self) {
            let value = 0
                | ((self.0[0] as u32) << 0)
                | ((self.0[1] as u32) << 5)
                | ((self.0[2] as u32) << 10)
                | ((self.0[3] as u32) << 15)
                | ((self.0[4] as u32) << 20)
                | ((self.0[5] as u32) << 25)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// regular sequence register 3
pub mod sqr3 {
    pub const OFFSET: u32 = 0x34;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([u8;6]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = u8;
        fn index(&self, index: u8) -> &u8 {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut u8 {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([u8;6]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = u8;
        fn index(&self, index: u8) -> &u8 {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut u8 {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b11111) as u8,
            ((value >> 5) & 0b11111) as u8,
            ((value >> 10) & 0b11111) as u8,
            ((value >> 15) & 0b11111) as u8,
            ((value >> 20) & 0b11111) as u8,
            ((value >> 25) & 0b11111) as u8,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b11111) as u8,
            ((value >> 5) & 0b11111) as u8,
            ((value >> 10) & 0b11111) as u8,
            ((value >> 15) & 0b11111) as u8,
            ((value >> 20) & 0b11111) as u8,
            ((value >> 25) & 0b11111) as u8,
        ])
    }
    impl Cache {
        pub fn save(self) {
            // This will call ops::Drop below
        }
    }
    impl ::core::ops::Drop for Cache {
        fn drop(&mut self) {
            let value = 0
                | ((self.0[0] as u32) << 0)
                | ((self.0[1] as u32) << 5)
                | ((self.0[2] as u32) << 10)
                | ((self.0[3] as u32) << 15)
                | ((self.0[4] as u32) << 20)
                | ((self.0[5] as u32) << 25)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// injected sequence register
pub mod jsqr {
    pub const OFFSET: u32 = 0x38;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Injected sequence length
        pub jl: u8,
        /// 4th conversion in injected sequence
        pub jsq4: u8,
        /// 3rd conversion in injected sequence
        pub jsq3: u8,
        /// 2nd conversion in injected sequence
        pub jsq2: u8,
        /// 1st conversion in injected sequence
        pub jsq1: u8,
    }
    pub struct Cache {
        /// Injected sequence length
        pub jl: u8,
        /// 4th conversion in injected sequence
        pub jsq4: u8,
        /// 3rd conversion in injected sequence
        pub jsq3: u8,
        /// 2nd conversion in injected sequence
        pub jsq2: u8,
        /// 1st conversion in injected sequence
        pub jsq1: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            jl: ((value >> 20) & 0b11) as u8,
            jsq4: ((value >> 15) & 0b11) as u8,
            jsq3: ((value >> 10) & 0b11) as u8,
            jsq2: ((value >> 5) & 0b11) as u8,
            jsq1: ((value >> 0) & 0b11) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            jl: ((value >> 20) & 0b11) as u8,
            jsq4: ((value >> 15) & 0b11) as u8,
            jsq3: ((value >> 10) & 0b11) as u8,
            jsq2: ((value >> 5) & 0b11) as u8,
            jsq1: ((value >> 0) & 0b11) as u8,
        }
    }
    impl Cache {
        pub fn save(self) {
            // This will call ops::Drop below
        }
    }
    impl ::core::ops::Drop for Cache {
        fn drop(&mut self) {
            let value = 0
                | ((self.jl as u32) << 20)
                | ((self.jsq4 as u32) << 15)
                | ((self.jsq3 as u32) << 10)
                | ((self.jsq2 as u32) << 5)
                | ((self.jsq1 as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// injected data register x
pub mod jdr1 {
    pub const OFFSET: u32 = 0x3C;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// Injected data
    /// Access: read-only, Width: 16, Offset: 0
    /// Get Injected data
    pub fn jdata() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1111111111111111 << 0);
        value as u16
    }
}
/// injected data register x
pub mod jdr2 {
    pub const OFFSET: u32 = 0x40;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// Injected data
    /// Access: read-only, Width: 16, Offset: 0
    /// Get Injected data
    pub fn jdata() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1111111111111111 << 0);
        value as u16
    }
}
/// injected data register x
pub mod jdr3 {
    pub const OFFSET: u32 = 0x44;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// Injected data
    /// Access: read-only, Width: 16, Offset: 0
    /// Get Injected data
    pub fn jdata() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1111111111111111 << 0);
        value as u16
    }
}
/// injected data register x
pub mod jdr4 {
    pub const OFFSET: u32 = 0x48;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// Injected data
    /// Access: read-only, Width: 16, Offset: 0
    /// Get Injected data
    pub fn jdata() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1111111111111111 << 0);
        value as u16
    }
}
/// regular data register
pub mod dr {
    pub const OFFSET: u32 = 0x4C;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// Regular data
    /// Access: read-only, Width: 16, Offset: 0
    /// Get Regular data
    pub fn data() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1111111111111111 << 0);
        value as u16
    }
}
/// ADC2 global interrupt
pub const INTERRUPT_ADC: u32 = 18;
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0x40012800</baseAddress>
  <description>Analog to digital converter</description>
  <groupName>ADC</groupName>
  <interrupt>
    <description>ADC2 global interrupt</description>
    <name>ADC</name>
    <value>18</value>
  </interrupt>
  <name>ADC2</name>
  <registers>
    <register>
      <access>read-write</access>
      <addressOffset>0x0</addressOffset>
      <description>status register</description>
      <displayName>SR</displayName>
      <fields>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Regular channel start flag</description>
          <name>STRT</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Injected channel start
              flag</description>
          <name>JSTRT</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Injected channel end of
              conversion</description>
          <name>JEOC</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Regular channel end of
              conversion</description>
          <name>EOC</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Analog watchdog flag</description>
          <name>AWD</name>
        </field>
      </fields>
      <name>SR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x4</addressOffset>
      <description>control register 1</description>
      <displayName>CR1</displayName>
      <fields>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Analog watchdog enable on regular
              channels</description>
          <name>AWDEN</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Analog watchdog enable on injected
              channels</description>
          <name>JAWDEN</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Discontinuous mode channel
              count</description>
          <name>DISCNUM</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Discontinuous mode on injected
              channels</description>
          <name>JDISCEN</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Discontinuous mode on regular
              channels</description>
          <name>DISCEN</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Automatic injected group
              conversion</description>
          <name>JAUTO</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Enable the watchdog on a single channel
              in scan mode</description>
          <name>AWDSGL</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Scan mode</description>
          <name>SCAN</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt enable for injected
              channels</description>
          <name>JEOCIE</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Analog watchdog interrupt
              enable</description>
          <name>AWDIE</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt enable for EOC</description>
          <name>EOCIE</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>5</bitWidth>
          <description>Analog watchdog channel select
              bits</description>
          <name>AWDCH</name>
        </field>
      </fields>
      <name>CR1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x8</addressOffset>
      <description>control register 2</description>
      <displayName>CR2</displayName>
      <fields>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Temperature sensor and VREFINT
              enable</description>
          <name>TSVREFE</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Start conversion of regular
              channels</description>
          <name>SWSTART</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Start conversion of injected
              channels</description>
          <name>JSWSTART</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>External trigger conversion mode for
              regular channels</description>
          <name>EXTTRIG</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>3</bitWidth>
          <description>External event select for regular
              group</description>
          <name>EXTSEL</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>External trigger conversion mode for
              injected channels</description>
          <name>JEXTTRIG</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>3</bitWidth>
          <description>External event select for injected
              group</description>
          <name>JEXTSEL</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Data alignment</description>
          <name>ALIGN</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Direct memory access mode</description>
          <name>DMA</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Reset calibration</description>
          <name>RSTCAL</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>A/D calibration</description>
          <name>CAL</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Continuous conversion</description>
          <name>CONT</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>A/D converter ON / OFF</description>
          <name>ADON</name>
        </field>
      </fields>
      <name>CR2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xC</addressOffset>
      <description>sample time register 1</description>
      <displayName>SMPR1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Channel 10 sample time
              selection</description>
          <name>SMP10</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Channel 11 sample time
              selection</description>
          <name>SMP11</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Channel 12 sample time
              selection</description>
          <name>SMP12</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Channel 13 sample time
              selection</description>
          <name>SMP13</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Channel 14 sample time
              selection</description>
          <name>SMP14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Channel 15 sample time
              selection</description>
          <name>SMP15</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Channel 16 sample time
              selection</description>
          <name>SMP16</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Channel 17 sample time
              selection</description>
          <name>SMP17</name>
        </field>
      </fields>
      <name>SMPR1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x10</addressOffset>
      <description>sample time register 2</description>
      <displayName>SMPR2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Channel 0 sample time
              selection</description>
          <name>SMP0</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Channel 1 sample time
              selection</description>
          <name>SMP1</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Channel 2 sample time
              selection</description>
          <name>SMP2</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Channel 3 sample time
              selection</description>
          <name>SMP3</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Channel 4 sample time
              selection</description>
          <name>SMP4</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Channel 5 sample time
              selection</description>
          <name>SMP5</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Channel 6 sample time
              selection</description>
          <name>SMP6</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Channel 7 sample time
              selection</description>
          <name>SMP7</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Channel 8 sample time
              selection</description>
          <name>SMP8</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Channel 9 sample time
              selection</description>
          <name>SMP9</name>
        </field>
      </fields>
      <name>SMPR2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x14</addressOffset>
      <description>injected channel data offset register
          x</description>
      <displayName>JOFR1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
          <description>Data offset for injected channel
              x</description>
          <name>JOFFSET1</name>
        </field>
      </fields>
      <name>JOFR1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x18</addressOffset>
      <description>injected channel data offset register
          x</description>
      <displayName>JOFR2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
          <description>Data offset for injected channel
              x</description>
          <name>JOFFSET2</name>
        </field>
      </fields>
      <name>JOFR2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x1C</addressOffset>
      <description>injected channel data offset register
          x</description>
      <displayName>JOFR3</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
          <description>Data offset for injected channel
              x</description>
          <name>JOFFSET3</name>
        </field>
      </fields>
      <name>JOFR3</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x20</addressOffset>
      <description>injected channel data offset register
          x</description>
      <displayName>JOFR4</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
          <description>Data offset for injected channel
              x</description>
          <name>JOFFSET4</name>
        </field>
      </fields>
      <name>JOFR4</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x24</addressOffset>
      <description>watchdog higher threshold
          register</description>
      <displayName>HTR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
          <description>Analog watchdog higher
              threshold</description>
          <name>HT</name>
        </field>
      </fields>
      <name>HTR</name>
      <resetValue>0x00000FFF</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x28</addressOffset>
      <description>watchdog lower threshold
          register</description>
      <displayName>LTR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
          <description>Analog watchdog lower
              threshold</description>
          <name>LT</name>
        </field>
      </fields>
      <name>LTR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x2C</addressOffset>
      <description>regular sequence register 1</description>
      <displayName>SQR1</displayName>
      <fields>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Regular channel sequence
              length</description>
          <name>L</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>5</bitWidth>
          <description>16th conversion in regular
              sequence</description>
          <name>SQ16</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>5</bitWidth>
          <description>15th conversion in regular
              sequence</description>
          <name>SQ15</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>5</bitWidth>
          <description>14th conversion in regular
              sequence</description>
          <name>SQ14</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>5</bitWidth>
          <description>13th conversion in regular
              sequence</description>
          <name>SQ13</name>
        </field>
      </fields>
      <name>SQR1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x30</addressOffset>
      <description>regular sequence register 2</description>
      <displayName>SQR2</displayName>
      <fields>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>5</bitWidth>
          <description>12th conversion in regular
              sequence</description>
          <name>SQ12</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>5</bitWidth>
          <description>11th conversion in regular
              sequence</description>
          <name>SQ11</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>5</bitWidth>
          <description>10th conversion in regular
              sequence</description>
          <name>SQ10</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>5</bitWidth>
          <description>9th conversion in regular
              sequence</description>
          <name>SQ9</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>5</bitWidth>
          <description>8th conversion in regular
              sequence</description>
          <name>SQ8</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>5</bitWidth>
          <description>7th conversion in regular
              sequence</description>
          <name>SQ7</name>
        </field>
      </fields>
      <name>SQR2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x34</addressOffset>
      <description>regular sequence register 3</description>
      <displayName>SQR3</displayName>
      <fields>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>5</bitWidth>
          <description>6th conversion in regular
              sequence</description>
          <name>SQ6</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>5</bitWidth>
          <description>5th conversion in regular
              sequence</description>
          <name>SQ5</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>5</bitWidth>
          <description>4th conversion in regular
              sequence</description>
          <name>SQ4</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>5</bitWidth>
          <description>3rd conversion in regular
              sequence</description>
          <name>SQ3</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>5</bitWidth>
          <description>2nd conversion in regular
              sequence</description>
          <name>SQ2</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>5</bitWidth>
          <description>1st conversion in regular
              sequence</description>
          <name>SQ1</name>
        </field>
      </fields>
      <name>SQR3</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x38</addressOffset>
      <description>injected sequence register</description>
      <displayName>JSQR</displayName>
      <fields>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Injected sequence length</description>
          <name>JL</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>5</bitWidth>
          <description>4th conversion in injected
              sequence</description>
          <name>JSQ4</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>5</bitWidth>
          <description>3rd conversion in injected
              sequence</description>
          <name>JSQ3</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>5</bitWidth>
          <description>2nd conversion in injected
              sequence</description>
          <name>JSQ2</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>5</bitWidth>
          <description>1st conversion in injected
              sequence</description>
          <name>JSQ1</name>
        </field>
      </fields>
      <name>JSQR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x3C</addressOffset>
      <description>injected data register x</description>
      <displayName>JDR1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Injected data</description>
          <name>JDATA</name>
        </field>
      </fields>
      <name>JDR1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x40</addressOffset>
      <description>injected data register x</description>
      <displayName>JDR2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Injected data</description>
          <name>JDATA</name>
        </field>
      </fields>
      <name>JDR2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x44</addressOffset>
      <description>injected data register x</description>
      <displayName>JDR3</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Injected data</description>
          <name>JDATA</name>
        </field>
      </fields>
      <name>JDR3</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x48</addressOffset>
      <description>injected data register x</description>
      <displayName>JDR4</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Injected data</description>
          <name>JDATA</name>
        </field>
      </fields>
      <name>JDR4</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x4C</addressOffset>
      <description>regular data register</description>
      <displayName>DR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Regular data</description>
          <name>DATA</name>
        </field>
      </fields>
      <name>DR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
