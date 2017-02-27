pub const ADDRESS: u32 = 0x40001800;
/// control register 1
pub mod cr1 {
    pub const OFFSET: u32 = 0x0;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Clock division
        pub ckd: u8,
        /// Auto-reload preload enable
        pub arpe: u8,
        /// One-pulse mode
        pub opm: u8,
        /// Update request source
        pub urs: u8,
        /// Update disable
        pub udis: u8,
        /// Counter enable
        pub cen: u8,
    }
    pub struct Cache {
        /// Clock division
        pub ckd: u8,
        /// Auto-reload preload enable
        pub arpe: u8,
        /// One-pulse mode
        pub opm: u8,
        /// Update request source
        pub urs: u8,
        /// Update disable
        pub udis: u8,
        /// Counter enable
        pub cen: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            ckd: ((value >> 8) & 0b11) as u8,
            arpe: ((value >> 7) & 0b11) as u8,
            opm: ((value >> 3) & 0b11) as u8,
            urs: ((value >> 2) & 0b11) as u8,
            udis: ((value >> 1) & 0b11) as u8,
            cen: ((value >> 0) & 0b11) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            ckd: ((value >> 8) & 0b11) as u8,
            arpe: ((value >> 7) & 0b11) as u8,
            opm: ((value >> 3) & 0b11) as u8,
            urs: ((value >> 2) & 0b11) as u8,
            udis: ((value >> 1) & 0b11) as u8,
            cen: ((value >> 0) & 0b11) as u8,
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
                | ((self.ckd as u32) << 8)
                | ((self.arpe as u32) << 7)
                | ((self.opm as u32) << 3)
                | ((self.urs as u32) << 2)
                | ((self.udis as u32) << 1)
                | ((self.cen as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// control register 2
pub mod cr2 {
    pub const OFFSET: u32 = 0x4;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Master mode selection
        pub mms: u8,
    }
    pub struct Cache {
        /// Master mode selection
        pub mms: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            mms: ((value >> 4) & 0b111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            mms: ((value >> 4) & 0b111) as u8,
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
                | ((self.mms as u32) << 4)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// slave mode control register
pub mod smcr {
    pub const OFFSET: u32 = 0x8;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Master/Slave mode
        pub msm: bool,
        /// Trigger selection
        pub ts: bool,
        /// Slave mode selection
        pub sms: bool,
    }
    pub struct Cache {
        /// Master/Slave mode
        pub msm: bool,
        /// Trigger selection
        pub ts: bool,
        /// Slave mode selection
        pub sms: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            msm: ((value >> 7) & 0b1) > 0,
            ts: ((value >> 4) & 0b1) > 0,
            sms: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            msm: ((value >> 7) & 0b1) > 0,
            ts: ((value >> 4) & 0b1) > 0,
            sms: ((value >> 0) & 0b1) > 0,
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
                | ((self.msm as u32) << 7)
                | ((self.ts as u32) << 4)
                | ((self.sms as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// DMA/Interrupt enable register
pub mod dier {
    pub const OFFSET: u32 = 0xC;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Trigger interrupt enable
        pub tie: bool,
        /// Capture/Compare 2 interrupt enable
        pub cc2ie: bool,
        /// Capture/Compare 1 interrupt enable
        pub cc1ie: bool,
        /// Update interrupt enable
        pub uie: bool,
    }
    pub struct Cache {
        /// Trigger interrupt enable
        pub tie: bool,
        /// Capture/Compare 2 interrupt enable
        pub cc2ie: bool,
        /// Capture/Compare 1 interrupt enable
        pub cc1ie: bool,
        /// Update interrupt enable
        pub uie: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            tie: ((value >> 6) & 0b1) > 0,
            cc2ie: ((value >> 2) & 0b1) > 0,
            cc1ie: ((value >> 1) & 0b1) > 0,
            uie: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            tie: ((value >> 6) & 0b1) > 0,
            cc2ie: ((value >> 2) & 0b1) > 0,
            cc1ie: ((value >> 1) & 0b1) > 0,
            uie: ((value >> 0) & 0b1) > 0,
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
                | ((self.tie as u32) << 6)
                | ((self.cc2ie as u32) << 2)
                | ((self.cc1ie as u32) << 1)
                | ((self.uie as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// status register
pub mod sr {
    pub const OFFSET: u32 = 0x10;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Capture/compare 2 overcapture flag
        pub cc2of: bool,
        /// Capture/Compare 1 overcapture flag
        pub cc1of: bool,
        /// Trigger interrupt flag
        pub tif: bool,
        /// Capture/Compare 2 interrupt flag
        pub cc2if: bool,
        /// Capture/compare 1 interrupt flag
        pub cc1if: bool,
        /// Update interrupt flag
        pub uif: bool,
    }
    pub struct Cache {
        /// Capture/compare 2 overcapture flag
        pub cc2of: bool,
        /// Capture/Compare 1 overcapture flag
        pub cc1of: bool,
        /// Trigger interrupt flag
        pub tif: bool,
        /// Capture/Compare 2 interrupt flag
        pub cc2if: bool,
        /// Capture/compare 1 interrupt flag
        pub cc1if: bool,
        /// Update interrupt flag
        pub uif: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            cc2of: ((value >> 10) & 0b1) > 0,
            cc1of: ((value >> 9) & 0b1) > 0,
            tif: ((value >> 6) & 0b1) > 0,
            cc2if: ((value >> 2) & 0b1) > 0,
            cc1if: ((value >> 1) & 0b1) > 0,
            uif: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            cc2of: ((value >> 10) & 0b1) > 0,
            cc1of: ((value >> 9) & 0b1) > 0,
            tif: ((value >> 6) & 0b1) > 0,
            cc2if: ((value >> 2) & 0b1) > 0,
            cc1if: ((value >> 1) & 0b1) > 0,
            uif: ((value >> 0) & 0b1) > 0,
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
                | ((self.cc2of as u32) << 10)
                | ((self.cc1of as u32) << 9)
                | ((self.tif as u32) << 6)
                | ((self.cc2if as u32) << 2)
                | ((self.cc1if as u32) << 1)
                | ((self.uif as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// event generation register
pub mod egr {
    pub const OFFSET: u32 = 0x14;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// Trigger generation
    /// Access: write-only, Width: 1, Offset: 6
    /// Set Trigger generation
    pub fn tg(value: bool) {
        let value = value as u32;
        let value = value << 6;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Capture/compare 2 generation
    /// Access: write-only, Width: 1, Offset: 2
    /// Set Capture/compare 2 generation
    pub fn cc2g(value: bool) {
        let value = value as u32;
        let value = value << 2;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Capture/compare 1 generation
    /// Access: write-only, Width: 1, Offset: 1
    /// Set Capture/compare 1 generation
    pub fn cc1g(value: bool) {
        let value = value as u32;
        let value = value << 1;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Update generation
    /// Access: write-only, Width: 1, Offset: 0
    /// Set Update generation
    pub fn ug(value: bool) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
}
/// capture/compare mode register 1 (output mode)
pub mod ccmr1_output {
    pub const OFFSET: u32 = 0x18;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Output Compare 2 mode
        pub oc2m: u8,
        /// Output Compare 2 preload enable
        pub oc2pe: u8,
        /// Output Compare 2 fast enable
        pub oc2fe: u8,
        /// Capture/Compare 2 selection
        pub cc2s: u8,
        /// Output Compare 1 mode
        pub oc1m: u8,
        /// Output Compare 1 preload enable
        pub oc1pe: u8,
        /// Output Compare 1 fast enable
        pub oc1fe: u8,
        /// Capture/Compare 1 selection
        pub cc1s: u8,
    }
    pub struct Cache {
        /// Output Compare 2 mode
        pub oc2m: u8,
        /// Output Compare 2 preload enable
        pub oc2pe: u8,
        /// Output Compare 2 fast enable
        pub oc2fe: u8,
        /// Capture/Compare 2 selection
        pub cc2s: u8,
        /// Output Compare 1 mode
        pub oc1m: u8,
        /// Output Compare 1 preload enable
        pub oc1pe: u8,
        /// Output Compare 1 fast enable
        pub oc1fe: u8,
        /// Capture/Compare 1 selection
        pub cc1s: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            oc2m: ((value >> 12) & 0b111) as u8,
            oc2pe: ((value >> 11) & 0b111) as u8,
            oc2fe: ((value >> 10) & 0b111) as u8,
            cc2s: ((value >> 8) & 0b111) as u8,
            oc1m: ((value >> 4) & 0b111) as u8,
            oc1pe: ((value >> 3) & 0b111) as u8,
            oc1fe: ((value >> 2) & 0b111) as u8,
            cc1s: ((value >> 0) & 0b111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            oc2m: ((value >> 12) & 0b111) as u8,
            oc2pe: ((value >> 11) & 0b111) as u8,
            oc2fe: ((value >> 10) & 0b111) as u8,
            cc2s: ((value >> 8) & 0b111) as u8,
            oc1m: ((value >> 4) & 0b111) as u8,
            oc1pe: ((value >> 3) & 0b111) as u8,
            oc1fe: ((value >> 2) & 0b111) as u8,
            cc1s: ((value >> 0) & 0b111) as u8,
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
                | ((self.oc2m as u32) << 12)
                | ((self.oc2pe as u32) << 11)
                | ((self.oc2fe as u32) << 10)
                | ((self.cc2s as u32) << 8)
                | ((self.oc1m as u32) << 4)
                | ((self.oc1pe as u32) << 3)
                | ((self.oc1fe as u32) << 2)
                | ((self.cc1s as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// capture/compare mode register 1 (input mode)
pub mod ccmr1_input {
    pub const OFFSET: u32 = 0x18;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Input capture 2 filter
        pub ic2f: u8,
        /// Input capture 2 prescaler
        pub ic2psc: u8,
        /// Capture/Compare 2 selection
        pub cc2s: u8,
        /// Input capture 1 filter
        pub ic1f: u8,
        /// Input capture 1 prescaler
        pub ic1psc: u8,
        /// Capture/Compare 1 selection
        pub cc1s: u8,
    }
    pub struct Cache {
        /// Input capture 2 filter
        pub ic2f: u8,
        /// Input capture 2 prescaler
        pub ic2psc: u8,
        /// Capture/Compare 2 selection
        pub cc2s: u8,
        /// Input capture 1 filter
        pub ic1f: u8,
        /// Input capture 1 prescaler
        pub ic1psc: u8,
        /// Capture/Compare 1 selection
        pub cc1s: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            ic2f: ((value >> 12) & 0b1111) as u8,
            ic2psc: ((value >> 10) & 0b1111) as u8,
            cc2s: ((value >> 8) & 0b1111) as u8,
            ic1f: ((value >> 4) & 0b1111) as u8,
            ic1psc: ((value >> 2) & 0b1111) as u8,
            cc1s: ((value >> 0) & 0b1111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            ic2f: ((value >> 12) & 0b1111) as u8,
            ic2psc: ((value >> 10) & 0b1111) as u8,
            cc2s: ((value >> 8) & 0b1111) as u8,
            ic1f: ((value >> 4) & 0b1111) as u8,
            ic1psc: ((value >> 2) & 0b1111) as u8,
            cc1s: ((value >> 0) & 0b1111) as u8,
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
                | ((self.ic2f as u32) << 12)
                | ((self.ic2psc as u32) << 10)
                | ((self.cc2s as u32) << 8)
                | ((self.ic1f as u32) << 4)
                | ((self.ic1psc as u32) << 2)
                | ((self.cc1s as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// capture/compare enable register
pub mod ccer {
    pub const OFFSET: u32 = 0x20;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Capture/Compare 2 output Polarity
        pub cc2np: bool,
        /// Capture/Compare 2 output Polarity
        pub cc2p: bool,
        /// Capture/Compare 2 output enable
        pub cc2e: bool,
        /// Capture/Compare 1 output Polarity
        pub cc1np: bool,
        /// Capture/Compare 1 output Polarity
        pub cc1p: bool,
        /// Capture/Compare 1 output enable
        pub cc1e: bool,
    }
    pub struct Cache {
        /// Capture/Compare 2 output Polarity
        pub cc2np: bool,
        /// Capture/Compare 2 output Polarity
        pub cc2p: bool,
        /// Capture/Compare 2 output enable
        pub cc2e: bool,
        /// Capture/Compare 1 output Polarity
        pub cc1np: bool,
        /// Capture/Compare 1 output Polarity
        pub cc1p: bool,
        /// Capture/Compare 1 output enable
        pub cc1e: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            cc2np: ((value >> 7) & 0b1) > 0,
            cc2p: ((value >> 5) & 0b1) > 0,
            cc2e: ((value >> 4) & 0b1) > 0,
            cc1np: ((value >> 3) & 0b1) > 0,
            cc1p: ((value >> 1) & 0b1) > 0,
            cc1e: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            cc2np: ((value >> 7) & 0b1) > 0,
            cc2p: ((value >> 5) & 0b1) > 0,
            cc2e: ((value >> 4) & 0b1) > 0,
            cc1np: ((value >> 3) & 0b1) > 0,
            cc1p: ((value >> 1) & 0b1) > 0,
            cc1e: ((value >> 0) & 0b1) > 0,
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
                | ((self.cc2np as u32) << 7)
                | ((self.cc2p as u32) << 5)
                | ((self.cc2e as u32) << 4)
                | ((self.cc1np as u32) << 3)
                | ((self.cc1p as u32) << 1)
                | ((self.cc1e as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// counter
pub mod cnt {
    pub const OFFSET: u32 = 0x24;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// counter value
        pub cnt: u16,
    }
    pub struct Cache {
        /// counter value
        pub cnt: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            cnt: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            cnt: ((value >> 0) & 0b1111111111111111) as u16,
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
                | ((self.cnt as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// prescaler
pub mod psc {
    pub const OFFSET: u32 = 0x28;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Prescaler value
        pub psc: u16,
    }
    pub struct Cache {
        /// Prescaler value
        pub psc: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            psc: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            psc: ((value >> 0) & 0b1111111111111111) as u16,
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
                | ((self.psc as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// auto-reload register
pub mod arr {
    pub const OFFSET: u32 = 0x2C;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Auto-reload value
        pub arr: u16,
    }
    pub struct Cache {
        /// Auto-reload value
        pub arr: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            arr: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            arr: ((value >> 0) & 0b1111111111111111) as u16,
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
                | ((self.arr as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// capture/compare register 1
pub mod ccr1 {
    pub const OFFSET: u32 = 0x34;
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
            ((value >> 0) & 0b1111111111111111) as u16,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1111111111111111) as u16,
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
/// capture/compare register 2
pub mod ccr2 {
    pub const OFFSET: u32 = 0x38;
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
            ((value >> 0) & 0b1111111111111111) as u16,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1111111111111111) as u16,
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
/// TIM8 Break interrupt and TIM12 global interrupt
pub const INTERRUPT_TIM8_BRK_TIM12: u32 = 43;
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema" derivedFrom="TIM9">
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0x40001800</baseAddress>
  <description>General purpose timer</description>
  <groupName>TIM</groupName>
  <interrupt>
    <description>TIM8 Break interrupt and TIM12 global
        interrupt</description>
    <name>TIM8_BRK_TIM12</name>
    <value>43</value>
  </interrupt>
  <name>TIM12</name>
  <registers>
    <register>
      <access>read-write</access>
      <addressOffset>0x0</addressOffset>
      <description>control register 1</description>
      <displayName>CR1</displayName>
      <fields>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Clock division</description>
          <name>CKD</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Auto-reload preload enable</description>
          <name>ARPE</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>One-pulse mode</description>
          <name>OPM</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Update request source</description>
          <name>URS</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Update disable</description>
          <name>UDIS</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Counter enable</description>
          <name>CEN</name>
        </field>
      </fields>
      <name>CR1</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x4</addressOffset>
      <description>control register 2</description>
      <displayName>CR2</displayName>
      <fields>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Master mode selection</description>
          <name>MMS</name>
        </field>
      </fields>
      <name>CR2</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x8</addressOffset>
      <description>slave mode control register</description>
      <displayName>SMCR</displayName>
      <fields>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Master/Slave mode</description>
          <name>MSM</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Trigger selection</description>
          <name>TS</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Slave mode selection</description>
          <name>SMS</name>
        </field>
      </fields>
      <name>SMCR</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xC</addressOffset>
      <description>DMA/Interrupt enable register</description>
      <displayName>DIER</displayName>
      <fields>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Trigger interrupt enable</description>
          <name>TIE</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Capture/Compare 2 interrupt
              enable</description>
          <name>CC2IE</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Capture/Compare 1 interrupt
              enable</description>
          <name>CC1IE</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Update interrupt enable</description>
          <name>UIE</name>
        </field>
      </fields>
      <name>DIER</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x10</addressOffset>
      <description>status register</description>
      <displayName>SR</displayName>
      <fields>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Capture/compare 2 overcapture
              flag</description>
          <name>CC2OF</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Capture/Compare 1 overcapture
              flag</description>
          <name>CC1OF</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Trigger interrupt flag</description>
          <name>TIF</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Capture/Compare 2 interrupt
              flag</description>
          <name>CC2IF</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Capture/compare 1 interrupt
              flag</description>
          <name>CC1IF</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Update interrupt flag</description>
          <name>UIF</name>
        </field>
      </fields>
      <name>SR</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>write-only</access>
      <addressOffset>0x14</addressOffset>
      <description>event generation register</description>
      <displayName>EGR</displayName>
      <fields>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Trigger generation</description>
          <name>TG</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Capture/compare 2
              generation</description>
          <name>CC2G</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Capture/compare 1
              generation</description>
          <name>CC1G</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Update generation</description>
          <name>UG</name>
        </field>
      </fields>
      <name>EGR</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x18</addressOffset>
      <description>capture/compare mode register 1 (output
          mode)</description>
      <displayName>CCMR1_Output</displayName>
      <fields>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Output Compare 2 mode</description>
          <name>OC2M</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Output Compare 2 preload
              enable</description>
          <name>OC2PE</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Output Compare 2 fast
              enable</description>
          <name>OC2FE</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Capture/Compare 2
              selection</description>
          <name>CC2S</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Output Compare 1 mode</description>
          <name>OC1M</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Output Compare 1 preload
              enable</description>
          <name>OC1PE</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Output Compare 1 fast
              enable</description>
          <name>OC1FE</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Capture/Compare 1
              selection</description>
          <name>CC1S</name>
        </field>
      </fields>
      <name>CCMR1_Output</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x18</addressOffset>
      <alternateRegister>CCMR1_Output</alternateRegister>
      <description>capture/compare mode register 1 (input
          mode)</description>
      <displayName>CCMR1_Input</displayName>
      <fields>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Input capture 2 filter</description>
          <name>IC2F</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Input capture 2 prescaler</description>
          <name>IC2PSC</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Capture/Compare 2
              selection</description>
          <name>CC2S</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Input capture 1 filter</description>
          <name>IC1F</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Input capture 1 prescaler</description>
          <name>IC1PSC</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Capture/Compare 1
              selection</description>
          <name>CC1S</name>
        </field>
      </fields>
      <name>CCMR1_Input</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x20</addressOffset>
      <description>capture/compare enable
          register</description>
      <displayName>CCER</displayName>
      <fields>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Capture/Compare 2 output
              Polarity</description>
          <name>CC2NP</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Capture/Compare 2 output
              Polarity</description>
          <name>CC2P</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Capture/Compare 2 output
              enable</description>
          <name>CC2E</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Capture/Compare 1 output
              Polarity</description>
          <name>CC1NP</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Capture/Compare 1 output
              Polarity</description>
          <name>CC1P</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Capture/Compare 1 output
              enable</description>
          <name>CC1E</name>
        </field>
      </fields>
      <name>CCER</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x24</addressOffset>
      <description>counter</description>
      <displayName>CNT</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>counter value</description>
          <name>CNT</name>
        </field>
      </fields>
      <name>CNT</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x28</addressOffset>
      <description>prescaler</description>
      <displayName>PSC</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Prescaler value</description>
          <name>PSC</name>
        </field>
      </fields>
      <name>PSC</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x2C</addressOffset>
      <description>auto-reload register</description>
      <displayName>ARR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Auto-reload value</description>
          <name>ARR</name>
        </field>
      </fields>
      <name>ARR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x34</addressOffset>
      <description>capture/compare register 1</description>
      <displayName>CCR1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Capture/Compare 1 value</description>
          <name>CCR1</name>
        </field>
      </fields>
      <name>CCR1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x38</addressOffset>
      <description>capture/compare register 2</description>
      <displayName>CCR2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Capture/Compare 2 value</description>
          <name>CCR2</name>
        </field>
      </fields>
      <name>CCR2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
