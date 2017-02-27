pub const ADDRESS: u32 = 0x40006C04;
/// Backup data register (BKP_DR)
pub mod dr1 {
    pub const OFFSET: u32 = 0x0;
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
/// Backup data register (BKP_DR)
pub mod dr2 {
    pub const OFFSET: u32 = 0x4;
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
/// Backup data register (BKP_DR)
pub mod dr3 {
    pub const OFFSET: u32 = 0x8;
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
/// Backup data register (BKP_DR)
pub mod dr4 {
    pub const OFFSET: u32 = 0xC;
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
/// Backup data register (BKP_DR)
pub mod dr5 {
    pub const OFFSET: u32 = 0x10;
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
/// Backup data register (BKP_DR)
pub mod dr6 {
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
/// Backup data register (BKP_DR)
pub mod dr7 {
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
/// Backup data register (BKP_DR)
pub mod dr8 {
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
/// Backup data register (BKP_DR)
pub mod dr9 {
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
/// Backup data register (BKP_DR)
pub mod dr10 {
    pub const OFFSET: u32 = 0x24;
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
/// Backup data register (BKP_DR)
pub mod dr11 {
    pub const OFFSET: u32 = 0x3C;
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
/// Backup data register (BKP_DR)
pub mod dr12 {
    pub const OFFSET: u32 = 0x40;
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
/// Backup data register (BKP_DR)
pub mod dr13 {
    pub const OFFSET: u32 = 0x44;
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
/// Backup data register (BKP_DR)
pub mod dr14 {
    pub const OFFSET: u32 = 0x48;
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
/// Backup data register (BKP_DR)
pub mod dr15 {
    pub const OFFSET: u32 = 0x4C;
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
/// Backup data register (BKP_DR)
pub mod dr16 {
    pub const OFFSET: u32 = 0x50;
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
/// Backup data register (BKP_DR)
pub mod dr17 {
    pub const OFFSET: u32 = 0x54;
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
/// Backup data register (BKP_DR)
pub mod dr18 {
    pub const OFFSET: u32 = 0x58;
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
/// Backup data register (BKP_DR)
pub mod dr19 {
    pub const OFFSET: u32 = 0x5C;
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
/// Backup data register (BKP_DR)
pub mod dr20 {
    pub const OFFSET: u32 = 0x60;
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
/// Backup data register (BKP_DR)
pub mod dr21 {
    pub const OFFSET: u32 = 0x64;
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
/// Backup data register (BKP_DR)
pub mod dr22 {
    pub const OFFSET: u32 = 0x68;
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
/// Backup data register (BKP_DR)
pub mod dr23 {
    pub const OFFSET: u32 = 0x6C;
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
/// Backup data register (BKP_DR)
pub mod dr24 {
    pub const OFFSET: u32 = 0x70;
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
/// Backup data register (BKP_DR)
pub mod dr25 {
    pub const OFFSET: u32 = 0x74;
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
/// Backup data register (BKP_DR)
pub mod dr26 {
    pub const OFFSET: u32 = 0x78;
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
/// Backup data register (BKP_DR)
pub mod dr27 {
    pub const OFFSET: u32 = 0x7C;
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
/// Backup data register (BKP_DR)
pub mod dr28 {
    pub const OFFSET: u32 = 0x80;
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
/// Backup data register (BKP_DR)
pub mod dr29 {
    pub const OFFSET: u32 = 0x84;
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
/// Backup data register (BKP_DR)
pub mod dr30 {
    pub const OFFSET: u32 = 0x88;
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
/// Backup data register (BKP_DR)
pub mod dr31 {
    pub const OFFSET: u32 = 0x8C;
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
/// Backup data register (BKP_DR)
pub mod dr32 {
    pub const OFFSET: u32 = 0x90;
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
/// Backup data register (BKP_DR)
pub mod dr33 {
    pub const OFFSET: u32 = 0x94;
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
/// Backup data register (BKP_DR)
pub mod dr34 {
    pub const OFFSET: u32 = 0x98;
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
/// Backup data register (BKP_DR)
pub mod dr35 {
    pub const OFFSET: u32 = 0x9C;
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
/// Backup data register (BKP_DR)
pub mod dr36 {
    pub const OFFSET: u32 = 0xA0;
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
/// Backup data register (BKP_DR)
pub mod dr37 {
    pub const OFFSET: u32 = 0xA4;
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
/// Backup data register (BKP_DR)
pub mod dr38 {
    pub const OFFSET: u32 = 0xA8;
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
/// Backup data register (BKP_DR)
pub mod dr39 {
    pub const OFFSET: u32 = 0xAC;
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
/// Backup data register (BKP_DR)
pub mod dr40 {
    pub const OFFSET: u32 = 0xB0;
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
/// Backup data register (BKP_DR)
pub mod dr41 {
    pub const OFFSET: u32 = 0xB4;
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
/// Backup data register (BKP_DR)
pub mod dr42 {
    pub const OFFSET: u32 = 0xB8;
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
/// RTC clock calibration register (BKP_RTCCR)
pub mod rtccr {
    pub const OFFSET: u32 = 0x28;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Calibration value
        pub cal: u8,
        /// Calibration Clock Output
        pub cco: u8,
        /// Alarm or second output enable
        pub asoe: u8,
        /// Alarm or second output selection
        pub asos: u8,
    }
    pub struct Cache {
        /// Calibration value
        pub cal: u8,
        /// Calibration Clock Output
        pub cco: u8,
        /// Alarm or second output enable
        pub asoe: u8,
        /// Alarm or second output selection
        pub asos: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            cal: ((value >> 0) & 0b1111111) as u8,
            cco: ((value >> 7) & 0b1111111) as u8,
            asoe: ((value >> 8) & 0b1111111) as u8,
            asos: ((value >> 9) & 0b1111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            cal: ((value >> 0) & 0b1111111) as u8,
            cco: ((value >> 7) & 0b1111111) as u8,
            asoe: ((value >> 8) & 0b1111111) as u8,
            asos: ((value >> 9) & 0b1111111) as u8,
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
                | ((self.cal as u32) << 0)
                | ((self.cco as u32) << 7)
                | ((self.asoe as u32) << 8)
                | ((self.asos as u32) << 9)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Backup control register (BKP_CR)
pub mod cr {
    pub const OFFSET: u32 = 0x2C;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Tamper pin enable
        pub tpe: bool,
        /// Tamper pin active level
        pub tpal: bool,
    }
    pub struct Cache {
        /// Tamper pin enable
        pub tpe: bool,
        /// Tamper pin active level
        pub tpal: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            tpe: ((value >> 0) & 0b1) > 0,
            tpal: ((value >> 1) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            tpe: ((value >> 0) & 0b1) > 0,
            tpal: ((value >> 1) & 0b1) > 0,
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
                | ((self.tpe as u32) << 0)
                | ((self.tpal as u32) << 1)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// BKP_CSR control/status register (BKP_CSR)
pub mod csr {
    pub const OFFSET: u32 = 0x30;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// Clear Tamper event
    /// Access: write-only, Width: 1, Offset: 0
    /// Set Clear Tamper event
    pub fn cte(value: bool) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Clear Tamper Interrupt
    /// Access: write-only, Width: 1, Offset: 1
    /// Set Clear Tamper Interrupt
    pub fn cti(value: bool) {
        let value = value as u32;
        let value = value << 1;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Tamper Pin interrupt enable
    /// Access: read-write, Width: 1, Offset: 2
    /// Set Tamper Pin interrupt enable
    pub fn set_tpie(value: bool) {
        let value = value as u32;
        let value = value << 2;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get Tamper Pin interrupt enable
    pub fn get_tpie() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 2);
        value > 0
    }
    /// Tamper Event Flag
    /// Access: read-only, Width: 1, Offset: 8
    /// Get Tamper Event Flag
    pub fn tef() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 8);
        value > 0
    }
    /// Tamper Interrupt Flag
    /// Access: read-only, Width: 1, Offset: 9
    /// Get Tamper Interrupt Flag
    pub fn tif() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 9);
        value > 0
    }
}
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0x40006C04</baseAddress>
  <description>Backup registers</description>
  <groupName>BKP</groupName>
  <name>BKP</name>
  <registers>
    <register>
      <access>read-write</access>
      <addressOffset>0x0</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>D1</name>
        </field>
      </fields>
      <name>DR1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x4</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>D2</name>
        </field>
      </fields>
      <name>DR2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x8</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR3</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>D3</name>
        </field>
      </fields>
      <name>DR3</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xC</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR4</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>D4</name>
        </field>
      </fields>
      <name>DR4</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x10</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR5</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>D5</name>
        </field>
      </fields>
      <name>DR5</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x14</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR6</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>D6</name>
        </field>
      </fields>
      <name>DR6</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x18</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR7</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>D7</name>
        </field>
      </fields>
      <name>DR7</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x1C</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR8</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>D8</name>
        </field>
      </fields>
      <name>DR8</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x20</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR9</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>D9</name>
        </field>
      </fields>
      <name>DR9</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x24</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR10</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>D10</name>
        </field>
      </fields>
      <name>DR10</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x3C</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR11</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>DR11</name>
        </field>
      </fields>
      <name>DR11</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x40</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR12</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>DR12</name>
        </field>
      </fields>
      <name>DR12</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x44</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR13</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>DR13</name>
        </field>
      </fields>
      <name>DR13</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x48</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR14</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>D14</name>
        </field>
      </fields>
      <name>DR14</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x4C</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR15</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>D15</name>
        </field>
      </fields>
      <name>DR15</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x50</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR16</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>D16</name>
        </field>
      </fields>
      <name>DR16</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x54</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR17</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>D17</name>
        </field>
      </fields>
      <name>DR17</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x58</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR18</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>D18</name>
        </field>
      </fields>
      <name>DR18</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x5C</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR19</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>D19</name>
        </field>
      </fields>
      <name>DR19</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x60</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR20</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>D20</name>
        </field>
      </fields>
      <name>DR20</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x64</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR21</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>D21</name>
        </field>
      </fields>
      <name>DR21</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x68</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR22</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>D22</name>
        </field>
      </fields>
      <name>DR22</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x6C</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR23</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>D23</name>
        </field>
      </fields>
      <name>DR23</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x70</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR24</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>D24</name>
        </field>
      </fields>
      <name>DR24</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x74</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR25</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>D25</name>
        </field>
      </fields>
      <name>DR25</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x78</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR26</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>D26</name>
        </field>
      </fields>
      <name>DR26</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x7C</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR27</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>D27</name>
        </field>
      </fields>
      <name>DR27</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x80</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR28</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>D28</name>
        </field>
      </fields>
      <name>DR28</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x84</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR29</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>D29</name>
        </field>
      </fields>
      <name>DR29</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x88</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR30</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>D30</name>
        </field>
      </fields>
      <name>DR30</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x8C</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR31</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>D31</name>
        </field>
      </fields>
      <name>DR31</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x90</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR32</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>D32</name>
        </field>
      </fields>
      <name>DR32</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x94</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR33</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>D33</name>
        </field>
      </fields>
      <name>DR33</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x98</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR34</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>D34</name>
        </field>
      </fields>
      <name>DR34</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x9C</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR35</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>D35</name>
        </field>
      </fields>
      <name>DR35</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xA0</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR36</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>D36</name>
        </field>
      </fields>
      <name>DR36</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xA4</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR37</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>D37</name>
        </field>
      </fields>
      <name>DR37</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xA8</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR38</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>D38</name>
        </field>
      </fields>
      <name>DR38</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xAC</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR39</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>D39</name>
        </field>
      </fields>
      <name>DR39</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xB0</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR40</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>D40</name>
        </field>
      </fields>
      <name>DR40</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xB4</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR41</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>D41</name>
        </field>
      </fields>
      <name>DR41</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xB8</addressOffset>
      <description>Backup data register (BKP_DR)</description>
      <displayName>DR42</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Backup data</description>
          <name>D42</name>
        </field>
      </fields>
      <name>DR42</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x28</addressOffset>
      <description>RTC clock calibration register
          (BKP_RTCCR)</description>
      <displayName>RTCCR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>7</bitWidth>
          <description>Calibration value</description>
          <name>CAL</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Calibration Clock Output</description>
          <name>CCO</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Alarm or second output
              enable</description>
          <name>ASOE</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Alarm or second output
              selection</description>
          <name>ASOS</name>
        </field>
      </fields>
      <name>RTCCR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x2C</addressOffset>
      <description>Backup control register
          (BKP_CR)</description>
      <displayName>CR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Tamper pin enable</description>
          <name>TPE</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Tamper pin active level</description>
          <name>TPAL</name>
        </field>
      </fields>
      <name>CR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0x30</addressOffset>
      <description>BKP_CSR control/status register
          (BKP_CSR)</description>
      <displayName>CSR</displayName>
      <fields>
        <field>
          <access>write-only</access>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Clear Tamper event</description>
          <name>CTE</name>
        </field>
        <field>
          <access>write-only</access>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Clear Tamper Interrupt</description>
          <name>CTI</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Tamper Pin interrupt
              enable</description>
          <name>TPIE</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Tamper Event Flag</description>
          <name>TEF</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Tamper Interrupt Flag</description>
          <name>TIF</name>
        </field>
      </fields>
      <name>CSR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
