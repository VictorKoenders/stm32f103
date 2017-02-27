pub const ADDRESS: u32 = 0x40010400;
/// Interrupt mask register (EXTI_IMR)
pub mod imr {
    pub const OFFSET: u32 = 0x0;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([bool;19]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([bool;19]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
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
                | ((self.0[1] as u32) << 1)
                | ((self.0[2] as u32) << 2)
                | ((self.0[3] as u32) << 3)
                | ((self.0[4] as u32) << 4)
                | ((self.0[5] as u32) << 5)
                | ((self.0[6] as u32) << 6)
                | ((self.0[7] as u32) << 7)
                | ((self.0[8] as u32) << 8)
                | ((self.0[9] as u32) << 9)
                | ((self.0[10] as u32) << 10)
                | ((self.0[11] as u32) << 11)
                | ((self.0[12] as u32) << 12)
                | ((self.0[13] as u32) << 13)
                | ((self.0[14] as u32) << 14)
                | ((self.0[15] as u32) << 15)
                | ((self.0[16] as u32) << 16)
                | ((self.0[17] as u32) << 17)
                | ((self.0[18] as u32) << 18)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Event mask register (EXTI_EMR)
pub mod emr {
    pub const OFFSET: u32 = 0x4;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([bool;19]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([bool;19]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
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
                | ((self.0[1] as u32) << 1)
                | ((self.0[2] as u32) << 2)
                | ((self.0[3] as u32) << 3)
                | ((self.0[4] as u32) << 4)
                | ((self.0[5] as u32) << 5)
                | ((self.0[6] as u32) << 6)
                | ((self.0[7] as u32) << 7)
                | ((self.0[8] as u32) << 8)
                | ((self.0[9] as u32) << 9)
                | ((self.0[10] as u32) << 10)
                | ((self.0[11] as u32) << 11)
                | ((self.0[12] as u32) << 12)
                | ((self.0[13] as u32) << 13)
                | ((self.0[14] as u32) << 14)
                | ((self.0[15] as u32) << 15)
                | ((self.0[16] as u32) << 16)
                | ((self.0[17] as u32) << 17)
                | ((self.0[18] as u32) << 18)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Rising Trigger selection register (EXTI_RTSR)
pub mod rtsr {
    pub const OFFSET: u32 = 0x8;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([bool;19]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([bool;19]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
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
                | ((self.0[1] as u32) << 1)
                | ((self.0[2] as u32) << 2)
                | ((self.0[3] as u32) << 3)
                | ((self.0[4] as u32) << 4)
                | ((self.0[5] as u32) << 5)
                | ((self.0[6] as u32) << 6)
                | ((self.0[7] as u32) << 7)
                | ((self.0[8] as u32) << 8)
                | ((self.0[9] as u32) << 9)
                | ((self.0[10] as u32) << 10)
                | ((self.0[11] as u32) << 11)
                | ((self.0[12] as u32) << 12)
                | ((self.0[13] as u32) << 13)
                | ((self.0[14] as u32) << 14)
                | ((self.0[15] as u32) << 15)
                | ((self.0[16] as u32) << 16)
                | ((self.0[17] as u32) << 17)
                | ((self.0[18] as u32) << 18)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Falling Trigger selection register (EXTI_FTSR)
pub mod ftsr {
    pub const OFFSET: u32 = 0xC;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([bool;19]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([bool;19]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
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
                | ((self.0[1] as u32) << 1)
                | ((self.0[2] as u32) << 2)
                | ((self.0[3] as u32) << 3)
                | ((self.0[4] as u32) << 4)
                | ((self.0[5] as u32) << 5)
                | ((self.0[6] as u32) << 6)
                | ((self.0[7] as u32) << 7)
                | ((self.0[8] as u32) << 8)
                | ((self.0[9] as u32) << 9)
                | ((self.0[10] as u32) << 10)
                | ((self.0[11] as u32) << 11)
                | ((self.0[12] as u32) << 12)
                | ((self.0[13] as u32) << 13)
                | ((self.0[14] as u32) << 14)
                | ((self.0[15] as u32) << 15)
                | ((self.0[16] as u32) << 16)
                | ((self.0[17] as u32) << 17)
                | ((self.0[18] as u32) << 18)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Software interrupt event register (EXTI_SWIER)
pub mod swier {
    pub const OFFSET: u32 = 0x10;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([bool;19]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([bool;19]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
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
                | ((self.0[1] as u32) << 1)
                | ((self.0[2] as u32) << 2)
                | ((self.0[3] as u32) << 3)
                | ((self.0[4] as u32) << 4)
                | ((self.0[5] as u32) << 5)
                | ((self.0[6] as u32) << 6)
                | ((self.0[7] as u32) << 7)
                | ((self.0[8] as u32) << 8)
                | ((self.0[9] as u32) << 9)
                | ((self.0[10] as u32) << 10)
                | ((self.0[11] as u32) << 11)
                | ((self.0[12] as u32) << 12)
                | ((self.0[13] as u32) << 13)
                | ((self.0[14] as u32) << 14)
                | ((self.0[15] as u32) << 15)
                | ((self.0[16] as u32) << 16)
                | ((self.0[17] as u32) << 17)
                | ((self.0[18] as u32) << 18)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Pending register (EXTI_PR)
pub mod pr {
    pub const OFFSET: u32 = 0x14;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([bool;19]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([bool;19]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
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
                | ((self.0[1] as u32) << 1)
                | ((self.0[2] as u32) << 2)
                | ((self.0[3] as u32) << 3)
                | ((self.0[4] as u32) << 4)
                | ((self.0[5] as u32) << 5)
                | ((self.0[6] as u32) << 6)
                | ((self.0[7] as u32) << 7)
                | ((self.0[8] as u32) << 8)
                | ((self.0[9] as u32) << 9)
                | ((self.0[10] as u32) << 10)
                | ((self.0[11] as u32) << 11)
                | ((self.0[12] as u32) << 12)
                | ((self.0[13] as u32) << 13)
                | ((self.0[14] as u32) << 14)
                | ((self.0[15] as u32) << 15)
                | ((self.0[16] as u32) << 16)
                | ((self.0[17] as u32) << 17)
                | ((self.0[18] as u32) << 18)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Tamper interrupt
pub const INTERRUPT_TAMPER: u32 = 2;
/// EXTI Line0 interrupt
pub const INTERRUPT_EXTI0: u32 = 6;
/// EXTI Line1 interrupt
pub const INTERRUPT_EXTI1: u32 = 7;
/// EXTI Line2 interrupt
pub const INTERRUPT_EXTI2: u32 = 8;
/// EXTI Line3 interrupt
pub const INTERRUPT_EXTI3: u32 = 9;
/// EXTI Line4 interrupt
pub const INTERRUPT_EXTI4: u32 = 10;
/// EXTI Line[9:5] interrupts
pub const INTERRUPT_EXTI9_5: u32 = 23;
/// EXTI Line[15:10] interrupts
pub const INTERRUPT_EXTI15_10: u32 = 40;
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0x40010400</baseAddress>
  <description>EXTI</description>
  <groupName>EXTI</groupName>
  <interrupt>
    <description>Tamper interrupt</description>
    <name>TAMPER</name>
    <value>2</value>
  </interrupt>
  <interrupt>
    <description>EXTI Line0 interrupt</description>
    <name>EXTI0</name>
    <value>6</value>
  </interrupt>
  <interrupt>
    <description>EXTI Line1 interrupt</description>
    <name>EXTI1</name>
    <value>7</value>
  </interrupt>
  <interrupt>
    <description>EXTI Line2 interrupt</description>
    <name>EXTI2</name>
    <value>8</value>
  </interrupt>
  <interrupt>
    <description>EXTI Line3 interrupt</description>
    <name>EXTI3</name>
    <value>9</value>
  </interrupt>
  <interrupt>
    <description>EXTI Line4 interrupt</description>
    <name>EXTI4</name>
    <value>10</value>
  </interrupt>
  <interrupt>
    <description>EXTI Line[9:5] interrupts</description>
    <name>EXTI9_5</name>
    <value>23</value>
  </interrupt>
  <interrupt>
    <description>EXTI Line[15:10] interrupts</description>
    <name>EXTI15_10</name>
    <value>40</value>
  </interrupt>
  <name>EXTI</name>
  <registers>
    <register>
      <access>read-write</access>
      <addressOffset>0x0</addressOffset>
      <description>Interrupt mask register
          (EXTI_IMR)</description>
      <displayName>IMR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 0</description>
          <name>MR0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 1</description>
          <name>MR1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 2</description>
          <name>MR2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 3</description>
          <name>MR3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 4</description>
          <name>MR4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 5</description>
          <name>MR5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 6</description>
          <name>MR6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 7</description>
          <name>MR7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 8</description>
          <name>MR8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 9</description>
          <name>MR9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 10</description>
          <name>MR10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 11</description>
          <name>MR11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 12</description>
          <name>MR12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 13</description>
          <name>MR13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 14</description>
          <name>MR14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 15</description>
          <name>MR15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 16</description>
          <name>MR16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 17</description>
          <name>MR17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interrupt Mask on line 18</description>
          <name>MR18</name>
        </field>
      </fields>
      <name>IMR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x4</addressOffset>
      <description>Event mask register (EXTI_EMR)</description>
      <displayName>EMR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 0</description>
          <name>MR0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 1</description>
          <name>MR1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 2</description>
          <name>MR2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 3</description>
          <name>MR3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 4</description>
          <name>MR4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 5</description>
          <name>MR5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 6</description>
          <name>MR6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 7</description>
          <name>MR7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 8</description>
          <name>MR8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 9</description>
          <name>MR9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 10</description>
          <name>MR10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 11</description>
          <name>MR11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 12</description>
          <name>MR12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 13</description>
          <name>MR13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 14</description>
          <name>MR14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 15</description>
          <name>MR15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 16</description>
          <name>MR16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 17</description>
          <name>MR17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Mask on line 18</description>
          <name>MR18</name>
        </field>
      </fields>
      <name>EMR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x8</addressOffset>
      <description>Rising Trigger selection register
          (EXTI_RTSR)</description>
      <displayName>RTSR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Rising trigger event configuration of
              line 0</description>
          <name>TR0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Rising trigger event configuration of
              line 1</description>
          <name>TR1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Rising trigger event configuration of
              line 2</description>
          <name>TR2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Rising trigger event configuration of
              line 3</description>
          <name>TR3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Rising trigger event configuration of
              line 4</description>
          <name>TR4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Rising trigger event configuration of
              line 5</description>
          <name>TR5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Rising trigger event configuration of
              line 6</description>
          <name>TR6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Rising trigger event configuration of
              line 7</description>
          <name>TR7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Rising trigger event configuration of
              line 8</description>
          <name>TR8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Rising trigger event configuration of
              line 9</description>
          <name>TR9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Rising trigger event configuration of
              line 10</description>
          <name>TR10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Rising trigger event configuration of
              line 11</description>
          <name>TR11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Rising trigger event configuration of
              line 12</description>
          <name>TR12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Rising trigger event configuration of
              line 13</description>
          <name>TR13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Rising trigger event configuration of
              line 14</description>
          <name>TR14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Rising trigger event configuration of
              line 15</description>
          <name>TR15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Rising trigger event configuration of
              line 16</description>
          <name>TR16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Rising trigger event configuration of
              line 17</description>
          <name>TR17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Rising trigger event configuration of
              line 18</description>
          <name>TR18</name>
        </field>
      </fields>
      <name>RTSR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xC</addressOffset>
      <description>Falling Trigger selection register
          (EXTI_FTSR)</description>
      <displayName>FTSR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Falling trigger event configuration of
              line 0</description>
          <name>TR0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Falling trigger event configuration of
              line 1</description>
          <name>TR1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Falling trigger event configuration of
              line 2</description>
          <name>TR2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Falling trigger event configuration of
              line 3</description>
          <name>TR3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Falling trigger event configuration of
              line 4</description>
          <name>TR4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Falling trigger event configuration of
              line 5</description>
          <name>TR5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Falling trigger event configuration of
              line 6</description>
          <name>TR6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Falling trigger event configuration of
              line 7</description>
          <name>TR7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Falling trigger event configuration of
              line 8</description>
          <name>TR8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Falling trigger event configuration of
              line 9</description>
          <name>TR9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Falling trigger event configuration of
              line 10</description>
          <name>TR10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Falling trigger event configuration of
              line 11</description>
          <name>TR11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Falling trigger event configuration of
              line 12</description>
          <name>TR12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Falling trigger event configuration of
              line 13</description>
          <name>TR13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Falling trigger event configuration of
              line 14</description>
          <name>TR14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Falling trigger event configuration of
              line 15</description>
          <name>TR15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Falling trigger event configuration of
              line 16</description>
          <name>TR16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Falling trigger event configuration of
              line 17</description>
          <name>TR17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Falling trigger event configuration of
              line 18</description>
          <name>TR18</name>
        </field>
      </fields>
      <name>FTSR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x10</addressOffset>
      <description>Software interrupt event register
          (EXTI_SWIER)</description>
      <displayName>SWIER</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Software Interrupt on line
              0</description>
          <name>SWIER0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Software Interrupt on line
              1</description>
          <name>SWIER1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Software Interrupt on line
              2</description>
          <name>SWIER2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Software Interrupt on line
              3</description>
          <name>SWIER3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Software Interrupt on line
              4</description>
          <name>SWIER4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Software Interrupt on line
              5</description>
          <name>SWIER5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Software Interrupt on line
              6</description>
          <name>SWIER6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Software Interrupt on line
              7</description>
          <name>SWIER7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Software Interrupt on line
              8</description>
          <name>SWIER8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Software Interrupt on line
              9</description>
          <name>SWIER9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Software Interrupt on line
              10</description>
          <name>SWIER10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Software Interrupt on line
              11</description>
          <name>SWIER11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Software Interrupt on line
              12</description>
          <name>SWIER12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Software Interrupt on line
              13</description>
          <name>SWIER13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Software Interrupt on line
              14</description>
          <name>SWIER14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Software Interrupt on line
              15</description>
          <name>SWIER15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Software Interrupt on line
              16</description>
          <name>SWIER16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Software Interrupt on line
              17</description>
          <name>SWIER17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Software Interrupt on line
              18</description>
          <name>SWIER18</name>
        </field>
      </fields>
      <name>SWIER</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x14</addressOffset>
      <description>Pending register (EXTI_PR)</description>
      <displayName>PR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 0</description>
          <name>PR0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 1</description>
          <name>PR1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 2</description>
          <name>PR2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 3</description>
          <name>PR3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 4</description>
          <name>PR4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 5</description>
          <name>PR5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 6</description>
          <name>PR6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 7</description>
          <name>PR7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 8</description>
          <name>PR8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 9</description>
          <name>PR9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 10</description>
          <name>PR10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 11</description>
          <name>PR11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 12</description>
          <name>PR12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 13</description>
          <name>PR13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 14</description>
          <name>PR14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 15</description>
          <name>PR15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 16</description>
          <name>PR16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 17</description>
          <name>PR17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Pending bit 18</description>
          <name>PR18</name>
        </field>
      </fields>
      <name>PR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
