pub const ADDRESS: u32 = 0xE000E000;
/// Interrupt Controller Type Register
pub mod ictr {
    pub const OFFSET: u32 = 0x4;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// Total number of interrupt lines in groups
    /// Access: read-only, Width: 4, Offset: 0
    /// Get Total number of interrupt lines in groups
    pub fn intlinesnum() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1111 << 0);
        value as u8
    }
}
/// Software Triggered Interrupt Register
pub mod stir {
    pub const OFFSET: u32 = 0xF00;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// interrupt to be triggered
    /// Access: write-only, Width: 9, Offset: 0
    /// Set interrupt to be triggered
    pub fn intid(value: u16) {
        debug_assert!(value <= 0b111111111, "intid out of range");
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
}
/// Interrupt Set-Enable Register
pub mod iser0 {
    pub const OFFSET: u32 = 0x100;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// SETENA
        pub setena: u32,
    }
    pub struct Cache {
        /// SETENA
        pub setena: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            setena: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            setena: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.setena as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Interrupt Set-Enable Register
pub mod iser1 {
    pub const OFFSET: u32 = 0x104;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// SETENA
        pub setena: u32,
    }
    pub struct Cache {
        /// SETENA
        pub setena: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            setena: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            setena: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.setena as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Interrupt Clear-Enable Register
pub mod icer0 {
    pub const OFFSET: u32 = 0x180;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// CLRENA
        pub clrena: u32,
    }
    pub struct Cache {
        /// CLRENA
        pub clrena: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            clrena: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            clrena: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.clrena as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Interrupt Clear-Enable Register
pub mod icer1 {
    pub const OFFSET: u32 = 0x184;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// CLRENA
        pub clrena: u32,
    }
    pub struct Cache {
        /// CLRENA
        pub clrena: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            clrena: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            clrena: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.clrena as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Interrupt Set-Pending Register
pub mod ispr0 {
    pub const OFFSET: u32 = 0x200;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// SETPEND
        pub setpend: u32,
    }
    pub struct Cache {
        /// SETPEND
        pub setpend: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            setpend: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            setpend: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.setpend as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Interrupt Set-Pending Register
pub mod ispr1 {
    pub const OFFSET: u32 = 0x204;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// SETPEND
        pub setpend: u32,
    }
    pub struct Cache {
        /// SETPEND
        pub setpend: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            setpend: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            setpend: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.setpend as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Interrupt Clear-Pending Register
pub mod icpr0 {
    pub const OFFSET: u32 = 0x280;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// CLRPEND
        pub clrpend: u32,
    }
    pub struct Cache {
        /// CLRPEND
        pub clrpend: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            clrpend: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            clrpend: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.clrpend as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Interrupt Clear-Pending Register
pub mod icpr1 {
    pub const OFFSET: u32 = 0x284;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// CLRPEND
        pub clrpend: u32,
    }
    pub struct Cache {
        /// CLRPEND
        pub clrpend: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            clrpend: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            clrpend: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.clrpend as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Interrupt Active Bit Register
pub mod iabr0 {
    pub const OFFSET: u32 = 0x300;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// ACTIVE
    /// Access: read-only, Width: 32, Offset: 0
    /// Get ACTIVE
    pub fn active() -> u32 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b11111111111111111111111111111111 << 0);
        value as u32
    }
}
/// Interrupt Active Bit Register
pub mod iabr1 {
    pub const OFFSET: u32 = 0x304;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// ACTIVE
    /// Access: read-only, Width: 32, Offset: 0
    /// Get ACTIVE
    pub fn active() -> u32 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b11111111111111111111111111111111 << 0);
        value as u32
    }
}
/// Interrupt Priority Register
pub mod ipr0 {
    pub const OFFSET: u32 = 0x400;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([u8;4]);
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
    pub struct Cache([u8;4]);
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
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
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
                | ((self.0[1] as u32) << 8)
                | ((self.0[2] as u32) << 16)
                | ((self.0[3] as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Interrupt Priority Register
pub mod ipr1 {
    pub const OFFSET: u32 = 0x404;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([u8;4]);
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
    pub struct Cache([u8;4]);
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
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
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
                | ((self.0[1] as u32) << 8)
                | ((self.0[2] as u32) << 16)
                | ((self.0[3] as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Interrupt Priority Register
pub mod ipr2 {
    pub const OFFSET: u32 = 0x408;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([u8;4]);
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
    pub struct Cache([u8;4]);
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
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
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
                | ((self.0[1] as u32) << 8)
                | ((self.0[2] as u32) << 16)
                | ((self.0[3] as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Interrupt Priority Register
pub mod ipr3 {
    pub const OFFSET: u32 = 0x40C;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([u8;4]);
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
    pub struct Cache([u8;4]);
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
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
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
                | ((self.0[1] as u32) << 8)
                | ((self.0[2] as u32) << 16)
                | ((self.0[3] as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Interrupt Priority Register
pub mod ipr4 {
    pub const OFFSET: u32 = 0x410;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([u8;4]);
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
    pub struct Cache([u8;4]);
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
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
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
                | ((self.0[1] as u32) << 8)
                | ((self.0[2] as u32) << 16)
                | ((self.0[3] as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Interrupt Priority Register
pub mod ipr5 {
    pub const OFFSET: u32 = 0x414;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([u8;4]);
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
    pub struct Cache([u8;4]);
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
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
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
                | ((self.0[1] as u32) << 8)
                | ((self.0[2] as u32) << 16)
                | ((self.0[3] as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Interrupt Priority Register
pub mod ipr6 {
    pub const OFFSET: u32 = 0x418;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([u8;4]);
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
    pub struct Cache([u8;4]);
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
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
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
                | ((self.0[1] as u32) << 8)
                | ((self.0[2] as u32) << 16)
                | ((self.0[3] as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Interrupt Priority Register
pub mod ipr7 {
    pub const OFFSET: u32 = 0x41C;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([u8;4]);
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
    pub struct Cache([u8;4]);
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
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
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
                | ((self.0[1] as u32) << 8)
                | ((self.0[2] as u32) << 16)
                | ((self.0[3] as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Interrupt Priority Register
pub mod ipr8 {
    pub const OFFSET: u32 = 0x420;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([u8;4]);
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
    pub struct Cache([u8;4]);
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
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
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
                | ((self.0[1] as u32) << 8)
                | ((self.0[2] as u32) << 16)
                | ((self.0[3] as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Interrupt Priority Register
pub mod ipr9 {
    pub const OFFSET: u32 = 0x424;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([u8;4]);
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
    pub struct Cache([u8;4]);
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
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
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
                | ((self.0[1] as u32) << 8)
                | ((self.0[2] as u32) << 16)
                | ((self.0[3] as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Interrupt Priority Register
pub mod ipr10 {
    pub const OFFSET: u32 = 0x428;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([u8;4]);
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
    pub struct Cache([u8;4]);
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
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
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
                | ((self.0[1] as u32) << 8)
                | ((self.0[2] as u32) << 16)
                | ((self.0[3] as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Interrupt Priority Register
pub mod ipr11 {
    pub const OFFSET: u32 = 0x42C;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([u8;4]);
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
    pub struct Cache([u8;4]);
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
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
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
                | ((self.0[1] as u32) << 8)
                | ((self.0[2] as u32) << 16)
                | ((self.0[3] as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Interrupt Priority Register
pub mod ipr12 {
    pub const OFFSET: u32 = 0x430;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([u8;4]);
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
    pub struct Cache([u8;4]);
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
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
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
                | ((self.0[1] as u32) << 8)
                | ((self.0[2] as u32) << 16)
                | ((self.0[3] as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Interrupt Priority Register
pub mod ipr13 {
    pub const OFFSET: u32 = 0x434;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([u8;4]);
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
    pub struct Cache([u8;4]);
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
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
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
                | ((self.0[1] as u32) << 8)
                | ((self.0[2] as u32) << 16)
                | ((self.0[3] as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Interrupt Priority Register
pub mod ipr14 {
    pub const OFFSET: u32 = 0x438;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([u8;4]);
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
    pub struct Cache([u8;4]);
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
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
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
                | ((self.0[1] as u32) << 8)
                | ((self.0[2] as u32) << 16)
                | ((self.0[3] as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <addressBlock>
    <offset>0x0</offset>
    <size>0x1001</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0xE000E000</baseAddress>
  <description>Nested Vectored Interrupt
      Controller</description>
  <groupName>NVIC</groupName>
  <name>NVIC</name>
  <registers>
    <register>
      <access>read-only</access>
      <addressOffset>0x4</addressOffset>
      <description>Interrupt Controller Type
          Register</description>
      <displayName>ICTR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Total number of interrupt lines in
              groups</description>
          <name>INTLINESNUM</name>
        </field>
      </fields>
      <name>ICTR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>write-only</access>
      <addressOffset>0xF00</addressOffset>
      <description>Software Triggered Interrupt
          Register</description>
      <displayName>STIR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>9</bitWidth>
          <description>interrupt to be triggered</description>
          <name>INTID</name>
        </field>
      </fields>
      <name>STIR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x100</addressOffset>
      <description>Interrupt Set-Enable Register</description>
      <displayName>ISER0</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>SETENA</description>
          <name>SETENA</name>
        </field>
      </fields>
      <name>ISER0</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x104</addressOffset>
      <description>Interrupt Set-Enable Register</description>
      <displayName>ISER1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>SETENA</description>
          <name>SETENA</name>
        </field>
      </fields>
      <name>ISER1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x180</addressOffset>
      <description>Interrupt Clear-Enable
          Register</description>
      <displayName>ICER0</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>CLRENA</description>
          <name>CLRENA</name>
        </field>
      </fields>
      <name>ICER0</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x184</addressOffset>
      <description>Interrupt Clear-Enable
          Register</description>
      <displayName>ICER1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>CLRENA</description>
          <name>CLRENA</name>
        </field>
      </fields>
      <name>ICER1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x200</addressOffset>
      <description>Interrupt Set-Pending Register</description>
      <displayName>ISPR0</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>SETPEND</description>
          <name>SETPEND</name>
        </field>
      </fields>
      <name>ISPR0</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x204</addressOffset>
      <description>Interrupt Set-Pending Register</description>
      <displayName>ISPR1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>SETPEND</description>
          <name>SETPEND</name>
        </field>
      </fields>
      <name>ISPR1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x280</addressOffset>
      <description>Interrupt Clear-Pending
          Register</description>
      <displayName>ICPR0</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>CLRPEND</description>
          <name>CLRPEND</name>
        </field>
      </fields>
      <name>ICPR0</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x284</addressOffset>
      <description>Interrupt Clear-Pending
          Register</description>
      <displayName>ICPR1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>CLRPEND</description>
          <name>CLRPEND</name>
        </field>
      </fields>
      <name>ICPR1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x300</addressOffset>
      <description>Interrupt Active Bit Register</description>
      <displayName>IABR0</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>ACTIVE</description>
          <name>ACTIVE</name>
        </field>
      </fields>
      <name>IABR0</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x304</addressOffset>
      <description>Interrupt Active Bit Register</description>
      <displayName>IABR1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>ACTIVE</description>
          <name>ACTIVE</name>
        </field>
      </fields>
      <name>IABR1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x400</addressOffset>
      <description>Interrupt Priority Register</description>
      <displayName>IPR0</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N0</description>
          <name>IPR_N0</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N1</description>
          <name>IPR_N1</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N2</description>
          <name>IPR_N2</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N3</description>
          <name>IPR_N3</name>
        </field>
      </fields>
      <name>IPR0</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x404</addressOffset>
      <description>Interrupt Priority Register</description>
      <displayName>IPR1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N0</description>
          <name>IPR_N0</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N1</description>
          <name>IPR_N1</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N2</description>
          <name>IPR_N2</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N3</description>
          <name>IPR_N3</name>
        </field>
      </fields>
      <name>IPR1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x408</addressOffset>
      <description>Interrupt Priority Register</description>
      <displayName>IPR2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N0</description>
          <name>IPR_N0</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N1</description>
          <name>IPR_N1</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N2</description>
          <name>IPR_N2</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N3</description>
          <name>IPR_N3</name>
        </field>
      </fields>
      <name>IPR2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x40C</addressOffset>
      <description>Interrupt Priority Register</description>
      <displayName>IPR3</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N0</description>
          <name>IPR_N0</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N1</description>
          <name>IPR_N1</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N2</description>
          <name>IPR_N2</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N3</description>
          <name>IPR_N3</name>
        </field>
      </fields>
      <name>IPR3</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x410</addressOffset>
      <description>Interrupt Priority Register</description>
      <displayName>IPR4</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N0</description>
          <name>IPR_N0</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N1</description>
          <name>IPR_N1</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N2</description>
          <name>IPR_N2</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N3</description>
          <name>IPR_N3</name>
        </field>
      </fields>
      <name>IPR4</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x414</addressOffset>
      <description>Interrupt Priority Register</description>
      <displayName>IPR5</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N0</description>
          <name>IPR_N0</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N1</description>
          <name>IPR_N1</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N2</description>
          <name>IPR_N2</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N3</description>
          <name>IPR_N3</name>
        </field>
      </fields>
      <name>IPR5</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x418</addressOffset>
      <description>Interrupt Priority Register</description>
      <displayName>IPR6</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N0</description>
          <name>IPR_N0</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N1</description>
          <name>IPR_N1</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N2</description>
          <name>IPR_N2</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N3</description>
          <name>IPR_N3</name>
        </field>
      </fields>
      <name>IPR6</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x41C</addressOffset>
      <description>Interrupt Priority Register</description>
      <displayName>IPR7</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N0</description>
          <name>IPR_N0</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N1</description>
          <name>IPR_N1</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N2</description>
          <name>IPR_N2</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N3</description>
          <name>IPR_N3</name>
        </field>
      </fields>
      <name>IPR7</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x420</addressOffset>
      <description>Interrupt Priority Register</description>
      <displayName>IPR8</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N0</description>
          <name>IPR_N0</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N1</description>
          <name>IPR_N1</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N2</description>
          <name>IPR_N2</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N3</description>
          <name>IPR_N3</name>
        </field>
      </fields>
      <name>IPR8</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x424</addressOffset>
      <description>Interrupt Priority Register</description>
      <displayName>IPR9</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N0</description>
          <name>IPR_N0</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N1</description>
          <name>IPR_N1</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N2</description>
          <name>IPR_N2</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N3</description>
          <name>IPR_N3</name>
        </field>
      </fields>
      <name>IPR9</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x428</addressOffset>
      <description>Interrupt Priority Register</description>
      <displayName>IPR10</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N0</description>
          <name>IPR_N0</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N1</description>
          <name>IPR_N1</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N2</description>
          <name>IPR_N2</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N3</description>
          <name>IPR_N3</name>
        </field>
      </fields>
      <name>IPR10</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x42C</addressOffset>
      <description>Interrupt Priority Register</description>
      <displayName>IPR11</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N0</description>
          <name>IPR_N0</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N1</description>
          <name>IPR_N1</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N2</description>
          <name>IPR_N2</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N3</description>
          <name>IPR_N3</name>
        </field>
      </fields>
      <name>IPR11</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x430</addressOffset>
      <description>Interrupt Priority Register</description>
      <displayName>IPR12</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N0</description>
          <name>IPR_N0</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N1</description>
          <name>IPR_N1</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N2</description>
          <name>IPR_N2</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N3</description>
          <name>IPR_N3</name>
        </field>
      </fields>
      <name>IPR12</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x434</addressOffset>
      <description>Interrupt Priority Register</description>
      <displayName>IPR13</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N0</description>
          <name>IPR_N0</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N1</description>
          <name>IPR_N1</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N2</description>
          <name>IPR_N2</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N3</description>
          <name>IPR_N3</name>
        </field>
      </fields>
      <name>IPR13</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x438</addressOffset>
      <description>Interrupt Priority Register</description>
      <displayName>IPR14</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N0</description>
          <name>IPR_N0</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N1</description>
          <name>IPR_N1</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N2</description>
          <name>IPR_N2</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IPR_N3</description>
          <name>IPR_N3</name>
        </field>
      </fields>
      <name>IPR14</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
