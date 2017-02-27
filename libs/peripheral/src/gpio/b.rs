pub const ADDRESS: u32 = 0x40010C00;
/// Port configuration register low (GPIOn_CRL)
pub mod crl {
    pub const OFFSET: u32 = 0x0;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([u8;16]);
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
    pub struct Cache([u8;16]);
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
            ((value >> 0) & 0b11) as u8,
            ((value >> 2) & 0b11) as u8,
            ((value >> 4) & 0b11) as u8,
            ((value >> 6) & 0b11) as u8,
            ((value >> 8) & 0b11) as u8,
            ((value >> 10) & 0b11) as u8,
            ((value >> 12) & 0b11) as u8,
            ((value >> 14) & 0b11) as u8,
            ((value >> 16) & 0b11) as u8,
            ((value >> 18) & 0b11) as u8,
            ((value >> 20) & 0b11) as u8,
            ((value >> 22) & 0b11) as u8,
            ((value >> 24) & 0b11) as u8,
            ((value >> 26) & 0b11) as u8,
            ((value >> 28) & 0b11) as u8,
            ((value >> 30) & 0b11) as u8,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b11) as u8,
            ((value >> 2) & 0b11) as u8,
            ((value >> 4) & 0b11) as u8,
            ((value >> 6) & 0b11) as u8,
            ((value >> 8) & 0b11) as u8,
            ((value >> 10) & 0b11) as u8,
            ((value >> 12) & 0b11) as u8,
            ((value >> 14) & 0b11) as u8,
            ((value >> 16) & 0b11) as u8,
            ((value >> 18) & 0b11) as u8,
            ((value >> 20) & 0b11) as u8,
            ((value >> 22) & 0b11) as u8,
            ((value >> 24) & 0b11) as u8,
            ((value >> 26) & 0b11) as u8,
            ((value >> 28) & 0b11) as u8,
            ((value >> 30) & 0b11) as u8,
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
                | ((self.0[1] as u32) << 2)
                | ((self.0[2] as u32) << 4)
                | ((self.0[3] as u32) << 6)
                | ((self.0[4] as u32) << 8)
                | ((self.0[5] as u32) << 10)
                | ((self.0[6] as u32) << 12)
                | ((self.0[7] as u32) << 14)
                | ((self.0[8] as u32) << 16)
                | ((self.0[9] as u32) << 18)
                | ((self.0[10] as u32) << 20)
                | ((self.0[11] as u32) << 22)
                | ((self.0[12] as u32) << 24)
                | ((self.0[13] as u32) << 26)
                | ((self.0[14] as u32) << 28)
                | ((self.0[15] as u32) << 30)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Port configuration register high (GPIOn_CRL)
pub mod crh {
    pub const OFFSET: u32 = 0x4;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([u8;16]);
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
    pub struct Cache([u8;16]);
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
            ((value >> 0) & 0b11) as u8,
            ((value >> 2) & 0b11) as u8,
            ((value >> 4) & 0b11) as u8,
            ((value >> 6) & 0b11) as u8,
            ((value >> 8) & 0b11) as u8,
            ((value >> 10) & 0b11) as u8,
            ((value >> 12) & 0b11) as u8,
            ((value >> 14) & 0b11) as u8,
            ((value >> 16) & 0b11) as u8,
            ((value >> 18) & 0b11) as u8,
            ((value >> 20) & 0b11) as u8,
            ((value >> 22) & 0b11) as u8,
            ((value >> 24) & 0b11) as u8,
            ((value >> 26) & 0b11) as u8,
            ((value >> 28) & 0b11) as u8,
            ((value >> 30) & 0b11) as u8,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b11) as u8,
            ((value >> 2) & 0b11) as u8,
            ((value >> 4) & 0b11) as u8,
            ((value >> 6) & 0b11) as u8,
            ((value >> 8) & 0b11) as u8,
            ((value >> 10) & 0b11) as u8,
            ((value >> 12) & 0b11) as u8,
            ((value >> 14) & 0b11) as u8,
            ((value >> 16) & 0b11) as u8,
            ((value >> 18) & 0b11) as u8,
            ((value >> 20) & 0b11) as u8,
            ((value >> 22) & 0b11) as u8,
            ((value >> 24) & 0b11) as u8,
            ((value >> 26) & 0b11) as u8,
            ((value >> 28) & 0b11) as u8,
            ((value >> 30) & 0b11) as u8,
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
                | ((self.0[1] as u32) << 2)
                | ((self.0[2] as u32) << 4)
                | ((self.0[3] as u32) << 6)
                | ((self.0[4] as u32) << 8)
                | ((self.0[5] as u32) << 10)
                | ((self.0[6] as u32) << 12)
                | ((self.0[7] as u32) << 14)
                | ((self.0[8] as u32) << 16)
                | ((self.0[9] as u32) << 18)
                | ((self.0[10] as u32) << 20)
                | ((self.0[11] as u32) << 22)
                | ((self.0[12] as u32) << 24)
                | ((self.0[13] as u32) << 26)
                | ((self.0[14] as u32) << 28)
                | ((self.0[15] as u32) << 30)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Port input data register (GPIOn_IDR)
pub mod idr {
    pub const OFFSET: u32 = 0x8;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// Get Port input data
    pub fn idr(index: u8) -> bool {
        debug_assert!(index < 16, "idr out of range");
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << (0 + index * 1));
        value > 0
    }
}
/// Port output data register (GPIOn_ODR)
pub mod odr {
    pub const OFFSET: u32 = 0xC;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([bool;16]);
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
    pub struct Cache([bool;16]);
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
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Port bit set/reset register (GPIOn_BSRR)
pub mod bsrr {
    pub const OFFSET: u32 = 0x10;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// Set Set bit 0
    pub fn bs(index: u8, value: bool) {
        debug_assert!(index < 16, "bs out of range");
        let value = value as u32;
        let value = value << (0 + index * 1);
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Set Reset bit 0
    pub fn br(index: u8, value: bool) {
        debug_assert!(index < 16, "br out of range");
        let value = value as u32;
        let value = value << (16 + index * 1);
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
}
/// Port bit reset register (GPIOn_BRR)
pub mod brr {
    pub const OFFSET: u32 = 0x14;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// Set Reset bit 0
    pub fn br(index: u8, value: bool) {
        debug_assert!(index < 16, "br out of range");
        let value = value as u32;
        let value = value << (0 + index * 1);
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
}
/// Port configuration lock register
pub mod lckr {
    pub const OFFSET: u32 = 0x18;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Port A Lock bit 0
        pub lck0: bool,
        /// Port A Lock bit 1
        pub lck1: bool,
        /// Port A Lock bit 2
        pub lck2: bool,
        /// Port A Lock bit 3
        pub lck3: bool,
        /// Port A Lock bit 4
        pub lck4: bool,
        /// Port A Lock bit 5
        pub lck5: bool,
        /// Port A Lock bit 6
        pub lck6: bool,
        /// Port A Lock bit 7
        pub lck7: bool,
        /// Port A Lock bit 8
        pub lck8: bool,
        /// Port A Lock bit 9
        pub lck9: bool,
        /// Port A Lock bit 10
        pub lck10: bool,
        /// Port A Lock bit 11
        pub lck11: bool,
        /// Port A Lock bit 12
        pub lck12: bool,
        /// Port A Lock bit 13
        pub lck13: bool,
        /// Port A Lock bit 14
        pub lck14: bool,
        /// Port A Lock bit 15
        pub lck15: bool,
        /// Lock key
        pub lckk: bool,
    }
    pub struct Cache {
        /// Port A Lock bit 0
        pub lck0: bool,
        /// Port A Lock bit 1
        pub lck1: bool,
        /// Port A Lock bit 2
        pub lck2: bool,
        /// Port A Lock bit 3
        pub lck3: bool,
        /// Port A Lock bit 4
        pub lck4: bool,
        /// Port A Lock bit 5
        pub lck5: bool,
        /// Port A Lock bit 6
        pub lck6: bool,
        /// Port A Lock bit 7
        pub lck7: bool,
        /// Port A Lock bit 8
        pub lck8: bool,
        /// Port A Lock bit 9
        pub lck9: bool,
        /// Port A Lock bit 10
        pub lck10: bool,
        /// Port A Lock bit 11
        pub lck11: bool,
        /// Port A Lock bit 12
        pub lck12: bool,
        /// Port A Lock bit 13
        pub lck13: bool,
        /// Port A Lock bit 14
        pub lck14: bool,
        /// Port A Lock bit 15
        pub lck15: bool,
        /// Lock key
        pub lckk: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            lck0: ((value >> 0) & 0b1) > 0,
            lck1: ((value >> 1) & 0b1) > 0,
            lck2: ((value >> 2) & 0b1) > 0,
            lck3: ((value >> 3) & 0b1) > 0,
            lck4: ((value >> 4) & 0b1) > 0,
            lck5: ((value >> 5) & 0b1) > 0,
            lck6: ((value >> 6) & 0b1) > 0,
            lck7: ((value >> 7) & 0b1) > 0,
            lck8: ((value >> 8) & 0b1) > 0,
            lck9: ((value >> 9) & 0b1) > 0,
            lck10: ((value >> 10) & 0b1) > 0,
            lck11: ((value >> 11) & 0b1) > 0,
            lck12: ((value >> 12) & 0b1) > 0,
            lck13: ((value >> 13) & 0b1) > 0,
            lck14: ((value >> 14) & 0b1) > 0,
            lck15: ((value >> 15) & 0b1) > 0,
            lckk: ((value >> 16) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            lck0: ((value >> 0) & 0b1) > 0,
            lck1: ((value >> 1) & 0b1) > 0,
            lck2: ((value >> 2) & 0b1) > 0,
            lck3: ((value >> 3) & 0b1) > 0,
            lck4: ((value >> 4) & 0b1) > 0,
            lck5: ((value >> 5) & 0b1) > 0,
            lck6: ((value >> 6) & 0b1) > 0,
            lck7: ((value >> 7) & 0b1) > 0,
            lck8: ((value >> 8) & 0b1) > 0,
            lck9: ((value >> 9) & 0b1) > 0,
            lck10: ((value >> 10) & 0b1) > 0,
            lck11: ((value >> 11) & 0b1) > 0,
            lck12: ((value >> 12) & 0b1) > 0,
            lck13: ((value >> 13) & 0b1) > 0,
            lck14: ((value >> 14) & 0b1) > 0,
            lck15: ((value >> 15) & 0b1) > 0,
            lckk: ((value >> 16) & 0b1) > 0,
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
                | ((self.lck0 as u32) << 0)
                | ((self.lck1 as u32) << 1)
                | ((self.lck2 as u32) << 2)
                | ((self.lck3 as u32) << 3)
                | ((self.lck4 as u32) << 4)
                | ((self.lck5 as u32) << 5)
                | ((self.lck6 as u32) << 6)
                | ((self.lck7 as u32) << 7)
                | ((self.lck8 as u32) << 8)
                | ((self.lck9 as u32) << 9)
                | ((self.lck10 as u32) << 10)
                | ((self.lck11 as u32) << 11)
                | ((self.lck12 as u32) << 12)
                | ((self.lck13 as u32) << 13)
                | ((self.lck14 as u32) << 14)
                | ((self.lck15 as u32) << 15)
                | ((self.lckk as u32) << 16)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema" derivedFrom="GPIOA">
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0x40010C00</baseAddress>
  <description>General purpose I/O</description>
  <groupName>GPIO</groupName>
  <name>GPIOB</name>
  <registers>
    <register>
      <access>read-write</access>
      <addressOffset>0x0</addressOffset>
      <description>Port configuration register low
          (GPIOn_CRL)</description>
      <displayName>CRL</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Port n.0 mode bits</description>
          <name>MODE0</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Port n.0 configuration
              bits</description>
          <name>CNF0</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Port n.1 mode bits</description>
          <name>MODE1</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Port n.1 configuration
              bits</description>
          <name>CNF1</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Port n.2 mode bits</description>
          <name>MODE2</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Port n.2 configuration
              bits</description>
          <name>CNF2</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Port n.3 mode bits</description>
          <name>MODE3</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Port n.3 configuration
              bits</description>
          <name>CNF3</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Port n.4 mode bits</description>
          <name>MODE4</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Port n.4 configuration
              bits</description>
          <name>CNF4</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Port n.5 mode bits</description>
          <name>MODE5</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Port n.5 configuration
              bits</description>
          <name>CNF5</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Port n.6 mode bits</description>
          <name>MODE6</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Port n.6 configuration
              bits</description>
          <name>CNF6</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Port n.7 mode bits</description>
          <name>MODE7</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Port n.7 configuration
              bits</description>
          <name>CNF7</name>
        </field>
      </fields>
      <name>CRL</name>
      <resetValue>0x44444444</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x4</addressOffset>
      <description>Port configuration register high
          (GPIOn_CRL)</description>
      <displayName>CRH</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Port n.8 mode bits</description>
          <name>MODE8</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Port n.8 configuration
              bits</description>
          <name>CNF8</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Port n.9 mode bits</description>
          <name>MODE9</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Port n.9 configuration
              bits</description>
          <name>CNF9</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Port n.10 mode bits</description>
          <name>MODE10</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Port n.10 configuration
              bits</description>
          <name>CNF10</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Port n.11 mode bits</description>
          <name>MODE11</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Port n.11 configuration
              bits</description>
          <name>CNF11</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Port n.12 mode bits</description>
          <name>MODE12</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Port n.12 configuration
              bits</description>
          <name>CNF12</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Port n.13 mode bits</description>
          <name>MODE13</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Port n.13 configuration
              bits</description>
          <name>CNF13</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Port n.14 mode bits</description>
          <name>MODE14</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Port n.14 configuration
              bits</description>
          <name>CNF14</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Port n.15 mode bits</description>
          <name>MODE15</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Port n.15 configuration
              bits</description>
          <name>CNF15</name>
        </field>
      </fields>
      <name>CRH</name>
      <resetValue>0x44444444</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x8</addressOffset>
      <description>Port input data register
          (GPIOn_IDR)</description>
      <displayName>IDR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port input data</description>
          <name>IDR0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port input data</description>
          <name>IDR1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port input data</description>
          <name>IDR2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port input data</description>
          <name>IDR3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port input data</description>
          <name>IDR4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port input data</description>
          <name>IDR5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port input data</description>
          <name>IDR6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port input data</description>
          <name>IDR7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port input data</description>
          <name>IDR8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port input data</description>
          <name>IDR9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port input data</description>
          <name>IDR10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port input data</description>
          <name>IDR11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port input data</description>
          <name>IDR12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port input data</description>
          <name>IDR13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port input data</description>
          <name>IDR14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port input data</description>
          <name>IDR15</name>
        </field>
      </fields>
      <name>IDR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xC</addressOffset>
      <description>Port output data register
          (GPIOn_ODR)</description>
      <displayName>ODR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port output data</description>
          <name>ODR0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port output data</description>
          <name>ODR1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port output data</description>
          <name>ODR2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port output data</description>
          <name>ODR3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port output data</description>
          <name>ODR4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port output data</description>
          <name>ODR5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port output data</description>
          <name>ODR6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port output data</description>
          <name>ODR7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port output data</description>
          <name>ODR8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port output data</description>
          <name>ODR9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port output data</description>
          <name>ODR10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port output data</description>
          <name>ODR11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port output data</description>
          <name>ODR12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port output data</description>
          <name>ODR13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port output data</description>
          <name>ODR14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port output data</description>
          <name>ODR15</name>
        </field>
      </fields>
      <name>ODR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>write-only</access>
      <addressOffset>0x10</addressOffset>
      <description>Port bit set/reset register
          (GPIOn_BSRR)</description>
      <displayName>BSRR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Set bit 0</description>
          <name>BS0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Set bit 1</description>
          <name>BS1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Set bit 1</description>
          <name>BS2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Set bit 3</description>
          <name>BS3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Set bit 4</description>
          <name>BS4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Set bit 5</description>
          <name>BS5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Set bit 6</description>
          <name>BS6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Set bit 7</description>
          <name>BS7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Set bit 8</description>
          <name>BS8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Set bit 9</description>
          <name>BS9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Set bit 10</description>
          <name>BS10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Set bit 11</description>
          <name>BS11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Set bit 12</description>
          <name>BS12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Set bit 13</description>
          <name>BS13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Set bit 14</description>
          <name>BS14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Set bit 15</description>
          <name>BS15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Reset bit 0</description>
          <name>BR0</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Reset bit 1</description>
          <name>BR1</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Reset bit 2</description>
          <name>BR2</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Reset bit 3</description>
          <name>BR3</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Reset bit 4</description>
          <name>BR4</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Reset bit 5</description>
          <name>BR5</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Reset bit 6</description>
          <name>BR6</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Reset bit 7</description>
          <name>BR7</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Reset bit 8</description>
          <name>BR8</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Reset bit 9</description>
          <name>BR9</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Reset bit 10</description>
          <name>BR10</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Reset bit 11</description>
          <name>BR11</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Reset bit 12</description>
          <name>BR12</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Reset bit 13</description>
          <name>BR13</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Reset bit 14</description>
          <name>BR14</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Reset bit 15</description>
          <name>BR15</name>
        </field>
      </fields>
      <name>BSRR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>write-only</access>
      <addressOffset>0x14</addressOffset>
      <description>Port bit reset register
          (GPIOn_BRR)</description>
      <displayName>BRR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Reset bit 0</description>
          <name>BR0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Reset bit 1</description>
          <name>BR1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Reset bit 1</description>
          <name>BR2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Reset bit 3</description>
          <name>BR3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Reset bit 4</description>
          <name>BR4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Reset bit 5</description>
          <name>BR5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Reset bit 6</description>
          <name>BR6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Reset bit 7</description>
          <name>BR7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Reset bit 8</description>
          <name>BR8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Reset bit 9</description>
          <name>BR9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Reset bit 10</description>
          <name>BR10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Reset bit 11</description>
          <name>BR11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Reset bit 12</description>
          <name>BR12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Reset bit 13</description>
          <name>BR13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Reset bit 14</description>
          <name>BR14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Reset bit 15</description>
          <name>BR15</name>
        </field>
      </fields>
      <name>BRR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x18</addressOffset>
      <description>Port configuration lock
          register</description>
      <displayName>LCKR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port A Lock bit 0</description>
          <name>LCK0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port A Lock bit 1</description>
          <name>LCK1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port A Lock bit 2</description>
          <name>LCK2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port A Lock bit 3</description>
          <name>LCK3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port A Lock bit 4</description>
          <name>LCK4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port A Lock bit 5</description>
          <name>LCK5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port A Lock bit 6</description>
          <name>LCK6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port A Lock bit 7</description>
          <name>LCK7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port A Lock bit 8</description>
          <name>LCK8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port A Lock bit 9</description>
          <name>LCK9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port A Lock bit 10</description>
          <name>LCK10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port A Lock bit 11</description>
          <name>LCK11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port A Lock bit 12</description>
          <name>LCK12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port A Lock bit 13</description>
          <name>LCK13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port A Lock bit 14</description>
          <name>LCK14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port A Lock bit 15</description>
          <name>LCK15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Lock key</description>
          <name>LCKK</name>
        </field>
      </fields>
      <name>LCKR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
