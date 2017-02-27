pub const ADDRESS: u32 = 0x40007400;
/// Control register (DAC_CR)
pub mod cr {
    pub const OFFSET: u32 = 0x0;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([bool;14]);
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
    pub struct Cache([bool;14]);
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
            ((value >> 6) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
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
                | ((self.0[4] as u32) << 6)
                | ((self.0[5] as u32) << 8)
                | ((self.0[6] as u32) << 12)
                | ((self.0[7] as u32) << 16)
                | ((self.0[8] as u32) << 17)
                | ((self.0[9] as u32) << 18)
                | ((self.0[10] as u32) << 19)
                | ((self.0[11] as u32) << 22)
                | ((self.0[12] as u32) << 24)
                | ((self.0[13] as u32) << 28)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// DAC software trigger register (DAC_SWTRIGR)
pub mod swtrigr {
    pub const OFFSET: u32 = 0x4;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// Set DAC channel1 software trigger
    pub fn swtrig(index: u8, value: bool) {
        debug_assert!(index < 2, "swtrig out of range");
        let value = value as u32;
        let value = value << (0 + index * 1);
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
}
/// DAC channel1 12-bit right-aligned data holding register(DAC_DHR12R1)
pub mod dhr12r1 {
    pub const OFFSET: u32 = 0x8;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// DAC channel1 12-bit right-aligned data
        pub dacc1dhr: u16,
    }
    pub struct Cache {
        /// DAC channel1 12-bit right-aligned data
        pub dacc1dhr: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            dacc1dhr: ((value >> 0) & 0b111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            dacc1dhr: ((value >> 0) & 0b111111111111) as u16,
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
                | ((self.dacc1dhr as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// DAC channel1 12-bit left aligned data holding register (DAC_DHR12L1)
pub mod dhr12l1 {
    pub const OFFSET: u32 = 0xC;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// DAC channel1 12-bit left-aligned data
        pub dacc1dhr: u16,
    }
    pub struct Cache {
        /// DAC channel1 12-bit left-aligned data
        pub dacc1dhr: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            dacc1dhr: ((value >> 4) & 0b111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            dacc1dhr: ((value >> 4) & 0b111111111111) as u16,
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
                | ((self.dacc1dhr as u32) << 4)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// DAC channel1 8-bit right aligned data holding register (DAC_DHR8R1)
pub mod dhr8r1 {
    pub const OFFSET: u32 = 0x10;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// DAC channel1 8-bit right-aligned data
        pub dacc1dhr: u8,
    }
    pub struct Cache {
        /// DAC channel1 8-bit right-aligned data
        pub dacc1dhr: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            dacc1dhr: ((value >> 0) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            dacc1dhr: ((value >> 0) & 0b11111111) as u8,
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
                | ((self.dacc1dhr as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// DAC channel2 12-bit right aligned data holding register (DAC_DHR12R2)
pub mod dhr12r2 {
    pub const OFFSET: u32 = 0x14;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// DAC channel2 12-bit right-aligned data
        pub dacc2dhr: u16,
    }
    pub struct Cache {
        /// DAC channel2 12-bit right-aligned data
        pub dacc2dhr: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            dacc2dhr: ((value >> 0) & 0b111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            dacc2dhr: ((value >> 0) & 0b111111111111) as u16,
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
                | ((self.dacc2dhr as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// DAC channel2 12-bit left aligned data holding register (DAC_DHR12L2)
pub mod dhr12l2 {
    pub const OFFSET: u32 = 0x18;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// DAC channel2 12-bit left-aligned data
        pub dacc2dhr: u16,
    }
    pub struct Cache {
        /// DAC channel2 12-bit left-aligned data
        pub dacc2dhr: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            dacc2dhr: ((value >> 4) & 0b111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            dacc2dhr: ((value >> 4) & 0b111111111111) as u16,
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
                | ((self.dacc2dhr as u32) << 4)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// DAC channel2 8-bit right-aligned data holding register (DAC_DHR8R2)
pub mod dhr8r2 {
    pub const OFFSET: u32 = 0x1C;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// DAC channel2 8-bit right-aligned data
        pub dacc2dhr: u8,
    }
    pub struct Cache {
        /// DAC channel2 8-bit right-aligned data
        pub dacc2dhr: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            dacc2dhr: ((value >> 0) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            dacc2dhr: ((value >> 0) & 0b11111111) as u8,
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
                | ((self.dacc2dhr as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Dual DAC 12-bit right-aligned data holding register (DAC_DHR12RD), Bits 31:28 Reserved, Bits 15:12 Reserved
pub mod dhr12rd {
    pub const OFFSET: u32 = 0x20;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// DAC channel1 12-bit right-aligned data
        pub dacc1dhr: u16,
        /// DAC channel2 12-bit right-aligned data
        pub dacc2dhr: u16,
    }
    pub struct Cache {
        /// DAC channel1 12-bit right-aligned data
        pub dacc1dhr: u16,
        /// DAC channel2 12-bit right-aligned data
        pub dacc2dhr: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            dacc1dhr: ((value >> 0) & 0b111111111111) as u16,
            dacc2dhr: ((value >> 16) & 0b111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            dacc1dhr: ((value >> 0) & 0b111111111111) as u16,
            dacc2dhr: ((value >> 16) & 0b111111111111) as u16,
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
                | ((self.dacc1dhr as u32) << 0)
                | ((self.dacc2dhr as u32) << 16)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// DUAL DAC 12-bit left aligned data holding register (DAC_DHR12LD), Bits 19:16 Reserved, Bits 3:0 Reserved
pub mod dhr12ld {
    pub const OFFSET: u32 = 0x24;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// DAC channel1 12-bit left-aligned data
        pub dacc1dhr: u16,
        /// DAC channel2 12-bit right-aligned data
        pub dacc2dhr: u16,
    }
    pub struct Cache {
        /// DAC channel1 12-bit left-aligned data
        pub dacc1dhr: u16,
        /// DAC channel2 12-bit right-aligned data
        pub dacc2dhr: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            dacc1dhr: ((value >> 4) & 0b111111111111) as u16,
            dacc2dhr: ((value >> 20) & 0b111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            dacc1dhr: ((value >> 4) & 0b111111111111) as u16,
            dacc2dhr: ((value >> 20) & 0b111111111111) as u16,
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
                | ((self.dacc1dhr as u32) << 4)
                | ((self.dacc2dhr as u32) << 20)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// DUAL DAC 8-bit right aligned data holding register (DAC_DHR8RD), Bits 31:16 Reserved
pub mod dhr8rd {
    pub const OFFSET: u32 = 0x28;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// DAC channel1 8-bit right-aligned data
        pub dacc1dhr: u8,
        /// DAC channel2 8-bit right-aligned data
        pub dacc2dhr: u8,
    }
    pub struct Cache {
        /// DAC channel1 8-bit right-aligned data
        pub dacc1dhr: u8,
        /// DAC channel2 8-bit right-aligned data
        pub dacc2dhr: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            dacc1dhr: ((value >> 0) & 0b11111111) as u8,
            dacc2dhr: ((value >> 8) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            dacc1dhr: ((value >> 0) & 0b11111111) as u8,
            dacc2dhr: ((value >> 8) & 0b11111111) as u8,
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
                | ((self.dacc1dhr as u32) << 0)
                | ((self.dacc2dhr as u32) << 8)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// DAC channel1 data output register (DAC_DOR1)
pub mod dor1 {
    pub const OFFSET: u32 = 0x2C;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// DAC channel1 data output
    /// Access: read-only, Width: 12, Offset: 0
    /// Get DAC channel1 data output
    pub fn dacc1dor() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b111111111111 << 0);
        value as u16
    }
}
/// DAC channel2 data output register (DAC_DOR2)
pub mod dor2 {
    pub const OFFSET: u32 = 0x30;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// DAC channel2 data output
    /// Access: read-only, Width: 12, Offset: 0
    /// Get DAC channel2 data output
    pub fn dacc2dor() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b111111111111 << 0);
        value as u16
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
  <baseAddress>0x40007400</baseAddress>
  <description>Digital to analog converter</description>
  <groupName>DAC</groupName>
  <name>DAC</name>
  <registers>
    <register>
      <access>read-write</access>
      <addressOffset>0x0</addressOffset>
      <description>Control register (DAC_CR)</description>
      <displayName>CR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DAC channel1 enable</description>
          <name>EN1</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DAC channel1 output buffer
              disable</description>
          <name>BOFF1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DAC channel1 trigger
              enable</description>
          <name>TEN1</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>3</bitWidth>
          <description>DAC channel1 trigger
              selection</description>
          <name>TSEL1</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>2</bitWidth>
          <description>DAC channel1 noise/triangle wave
              generation enable</description>
          <name>WAVE1</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>4</bitWidth>
          <description>DAC channel1 mask/amplitude
              selector</description>
          <name>MAMP1</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DAC channel1 DMA enable</description>
          <name>DMAEN1</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DAC channel2 enable</description>
          <name>EN2</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DAC channel2 output buffer
              disable</description>
          <name>BOFF2</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DAC channel2 trigger
              enable</description>
          <name>TEN2</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>3</bitWidth>
          <description>DAC channel2 trigger
              selection</description>
          <name>TSEL2</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>2</bitWidth>
          <description>DAC channel2 noise/triangle wave
              generation enable</description>
          <name>WAVE2</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>4</bitWidth>
          <description>DAC channel2 mask/amplitude
              selector</description>
          <name>MAMP2</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DAC channel2 DMA enable</description>
          <name>DMAEN2</name>
        </field>
      </fields>
      <name>CR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>write-only</access>
      <addressOffset>0x4</addressOffset>
      <description>DAC software trigger register
          (DAC_SWTRIGR)</description>
      <displayName>SWTRIGR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DAC channel1 software
              trigger</description>
          <name>SWTRIG1</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DAC channel2 software
              trigger</description>
          <name>SWTRIG2</name>
        </field>
      </fields>
      <name>SWTRIGR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x8</addressOffset>
      <description>DAC channel1 12-bit right-aligned data
          holding register(DAC_DHR12R1)</description>
      <displayName>DHR12R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
          <description>DAC channel1 12-bit right-aligned
              data</description>
          <name>DACC1DHR</name>
        </field>
      </fields>
      <name>DHR12R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xC</addressOffset>
      <description>DAC channel1 12-bit left aligned data
          holding register (DAC_DHR12L1)</description>
      <displayName>DHR12L1</displayName>
      <fields>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>12</bitWidth>
          <description>DAC channel1 12-bit left-aligned
              data</description>
          <name>DACC1DHR</name>
        </field>
      </fields>
      <name>DHR12L1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x10</addressOffset>
      <description>DAC channel1 8-bit right aligned data
          holding register (DAC_DHR8R1)</description>
      <displayName>DHR8R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DAC channel1 8-bit right-aligned
              data</description>
          <name>DACC1DHR</name>
        </field>
      </fields>
      <name>DHR8R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x14</addressOffset>
      <description>DAC channel2 12-bit right aligned data
          holding register (DAC_DHR12R2)</description>
      <displayName>DHR12R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
          <description>DAC channel2 12-bit right-aligned
              data</description>
          <name>DACC2DHR</name>
        </field>
      </fields>
      <name>DHR12R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x18</addressOffset>
      <description>DAC channel2 12-bit left aligned data
          holding register (DAC_DHR12L2)</description>
      <displayName>DHR12L2</displayName>
      <fields>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>12</bitWidth>
          <description>DAC channel2 12-bit left-aligned
              data</description>
          <name>DACC2DHR</name>
        </field>
      </fields>
      <name>DHR12L2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x1C</addressOffset>
      <description>DAC channel2 8-bit right-aligned data
          holding register (DAC_DHR8R2)</description>
      <displayName>DHR8R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DAC channel2 8-bit right-aligned
              data</description>
          <name>DACC2DHR</name>
        </field>
      </fields>
      <name>DHR8R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x20</addressOffset>
      <description>Dual DAC 12-bit right-aligned data holding
          register (DAC_DHR12RD), Bits 31:28 Reserved, Bits 15:12
          Reserved</description>
      <displayName>DHR12RD</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
          <description>DAC channel1 12-bit right-aligned
              data</description>
          <name>DACC1DHR</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>12</bitWidth>
          <description>DAC channel2 12-bit right-aligned
              data</description>
          <name>DACC2DHR</name>
        </field>
      </fields>
      <name>DHR12RD</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x24</addressOffset>
      <description>DUAL DAC 12-bit left aligned data holding
          register (DAC_DHR12LD), Bits 19:16 Reserved, Bits 3:0
          Reserved</description>
      <displayName>DHR12LD</displayName>
      <fields>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>12</bitWidth>
          <description>DAC channel1 12-bit left-aligned
              data</description>
          <name>DACC1DHR</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>12</bitWidth>
          <description>DAC channel2 12-bit right-aligned
              data</description>
          <name>DACC2DHR</name>
        </field>
      </fields>
      <name>DHR12LD</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x28</addressOffset>
      <description>DUAL DAC 8-bit right aligned data holding
          register (DAC_DHR8RD), Bits 31:16 Reserved</description>
      <displayName>DHR8RD</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DAC channel1 8-bit right-aligned
              data</description>
          <name>DACC1DHR</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DAC channel2 8-bit right-aligned
              data</description>
          <name>DACC2DHR</name>
        </field>
      </fields>
      <name>DHR8RD</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x2C</addressOffset>
      <description>DAC channel1 data output register
          (DAC_DOR1)</description>
      <displayName>DOR1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
          <description>DAC channel1 data output</description>
          <name>DACC1DOR</name>
        </field>
      </fields>
      <name>DOR1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x30</addressOffset>
      <description>DAC channel2 data output register
          (DAC_DOR2)</description>
      <displayName>DOR2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
          <description>DAC channel2 data output</description>
          <name>DACC2DOR</name>
        </field>
      </fields>
      <name>DOR2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
