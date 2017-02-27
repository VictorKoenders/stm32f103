pub const ADDRESS: u32 = 0x40003000;
/// Key register (IWDG_KR)
pub mod kr {
    pub const OFFSET: u32 = 0x0;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// Key value
    /// Access: write-only, Width: 16, Offset: 0
    /// Set Key value
    pub fn key(value: u16) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
}
/// Prescaler register (IWDG_PR)
pub mod pr {
    pub const OFFSET: u32 = 0x4;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Prescaler divider
        pub pr: u8,
    }
    pub struct Cache {
        /// Prescaler divider
        pub pr: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            pr: ((value >> 0) & 0b111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            pr: ((value >> 0) & 0b111) as u8,
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
                | ((self.pr as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Reload register (IWDG_RLR)
pub mod rlr {
    pub const OFFSET: u32 = 0x8;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Watchdog counter reload value
        pub rl: u16,
    }
    pub struct Cache {
        /// Watchdog counter reload value
        pub rl: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            rl: ((value >> 0) & 0b111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            rl: ((value >> 0) & 0b111111111111) as u16,
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
                | ((self.rl as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Status register (IWDG_SR)
pub mod sr {
    pub const OFFSET: u32 = 0xC;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// Watchdog prescaler value update
    /// Access: read-only, Width: 1, Offset: 0
    /// Get Watchdog prescaler value update
    pub fn pvu() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
    /// Watchdog counter reload value update
    /// Access: read-only, Width: 1, Offset: 1
    /// Get Watchdog counter reload value update
    pub fn rvu() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 1);
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
  <baseAddress>0x40003000</baseAddress>
  <description>Independent watchdog</description>
  <groupName>IWDG</groupName>
  <name>IWDG</name>
  <registers>
    <register>
      <access>write-only</access>
      <addressOffset>0x0</addressOffset>
      <description>Key register (IWDG_KR)</description>
      <displayName>KR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Key value</description>
          <name>KEY</name>
        </field>
      </fields>
      <name>KR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x4</addressOffset>
      <description>Prescaler register (IWDG_PR)</description>
      <displayName>PR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Prescaler divider</description>
          <name>PR</name>
        </field>
      </fields>
      <name>PR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x8</addressOffset>
      <description>Reload register (IWDG_RLR)</description>
      <displayName>RLR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
          <description>Watchdog counter reload
              value</description>
          <name>RL</name>
        </field>
      </fields>
      <name>RLR</name>
      <resetValue>0x00000FFF</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0xC</addressOffset>
      <description>Status register (IWDG_SR)</description>
      <displayName>SR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Watchdog prescaler value
              update</description>
          <name>PVU</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Watchdog counter reload value
              update</description>
          <name>RVU</name>
        </field>
      </fields>
      <name>SR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
