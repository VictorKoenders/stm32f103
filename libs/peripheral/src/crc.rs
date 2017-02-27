pub const ADDRESS: u32 = 0x40023000;
/// Data register
pub mod dr {
    pub const OFFSET: u32 = 0x0;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Data Register
        pub dr: u32,
    }
    pub struct Cache {
        /// Data Register
        pub dr: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            dr: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            dr: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.dr as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Independent Data register
pub mod idr {
    pub const OFFSET: u32 = 0x4;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Independent Data register
        pub idr: u8,
    }
    pub struct Cache {
        /// Independent Data register
        pub idr: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            idr: ((value >> 0) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            idr: ((value >> 0) & 0b11111111) as u8,
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
                | ((self.idr as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Control register
pub mod cr {
    pub const OFFSET: u32 = 0x8;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// Reset bit
    /// Access: write-only, Width: 1, Offset: 0
    /// Set Reset bit
    pub fn reset(value: bool) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
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
  <baseAddress>0x40023000</baseAddress>
  <description>CRC calculation unit</description>
  <groupName>CRC</groupName>
  <name>CRC</name>
  <registers>
    <register>
      <access>read-write</access>
      <addressOffset>0x0</addressOffset>
      <description>Data register</description>
      <displayName>DR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>Data Register</description>
          <name>DR</name>
        </field>
      </fields>
      <name>DR</name>
      <resetValue>0xFFFFFFFF</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x4</addressOffset>
      <description>Independent Data register</description>
      <displayName>IDR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>Independent Data register</description>
          <name>IDR</name>
        </field>
      </fields>
      <name>IDR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>write-only</access>
      <addressOffset>0x8</addressOffset>
      <description>Control register</description>
      <displayName>CR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Reset bit</description>
          <name>RESET</name>
        </field>
      </fields>
      <name>CR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
