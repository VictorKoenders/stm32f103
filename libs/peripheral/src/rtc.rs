pub const ADDRESS: u32 = 0x40002800;
/// RTC Control Register High
pub mod crh {
    pub const OFFSET: u32 = 0x0;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Second interrupt Enable
        pub secie: bool,
        /// Alarm interrupt Enable
        pub alrie: bool,
        /// Overflow interrupt Enable
        pub owie: bool,
    }
    pub struct Cache {
        /// Second interrupt Enable
        pub secie: bool,
        /// Alarm interrupt Enable
        pub alrie: bool,
        /// Overflow interrupt Enable
        pub owie: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            secie: ((value >> 0) & 0b1) > 0,
            alrie: ((value >> 1) & 0b1) > 0,
            owie: ((value >> 2) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            secie: ((value >> 0) & 0b1) > 0,
            alrie: ((value >> 1) & 0b1) > 0,
            owie: ((value >> 2) & 0b1) > 0,
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
                | ((self.secie as u32) << 0)
                | ((self.alrie as u32) << 1)
                | ((self.owie as u32) << 2)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// RTC Control Register Low
pub mod crl {
    pub const OFFSET: u32 = 0x4;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// Second Flag
    /// Access: read-write, Width: 1, Offset: 0
    /// Set Second Flag
    pub fn set_secf(value: bool) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get Second Flag
    pub fn get_secf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
    /// Alarm Flag
    /// Access: read-write, Width: 1, Offset: 1
    /// Set Alarm Flag
    pub fn set_alrf(value: bool) {
        let value = value as u32;
        let value = value << 1;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get Alarm Flag
    pub fn get_alrf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 1);
        value > 0
    }
    /// Overflow Flag
    /// Access: read-write, Width: 1, Offset: 2
    /// Set Overflow Flag
    pub fn set_owf(value: bool) {
        let value = value as u32;
        let value = value << 2;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get Overflow Flag
    pub fn get_owf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 2);
        value > 0
    }
    /// Registers Synchronized Flag
    /// Access: read-write, Width: 1, Offset: 3
    /// Set Registers Synchronized Flag
    pub fn set_rsf(value: bool) {
        let value = value as u32;
        let value = value << 3;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get Registers Synchronized Flag
    pub fn get_rsf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 3);
        value > 0
    }
    /// Configuration Flag
    /// Access: read-write, Width: 1, Offset: 4
    /// Set Configuration Flag
    pub fn set_cnf(value: bool) {
        let value = value as u32;
        let value = value << 4;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get Configuration Flag
    pub fn get_cnf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 4);
        value > 0
    }
    /// RTC operation OFF
    /// Access: read-only, Width: 1, Offset: 5
    /// Get RTC operation OFF
    pub fn rtoff() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 5);
        value > 0
    }
}
/// RTC Prescaler Load Register High
pub mod prlh {
    pub const OFFSET: u32 = 0x8;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// RTC Prescaler Load Register High
    /// Access: write-only, Width: 4, Offset: 0
    /// Set RTC Prescaler Load Register High
    pub fn prlh(value: u8) {
        debug_assert!(value <= 0b1111, "prlh out of range");
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
}
/// RTC Prescaler Load Register Low
pub mod prll {
    pub const OFFSET: u32 = 0xC;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// RTC Prescaler Divider Register Low
    /// Access: write-only, Width: 16, Offset: 0
    /// Set RTC Prescaler Divider Register Low
    pub fn prll(value: u16) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
}
/// RTC Prescaler Divider Register High
pub mod divh {
    pub const OFFSET: u32 = 0x10;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// RTC prescaler divider register high
    /// Access: read-only, Width: 4, Offset: 0
    /// Get RTC prescaler divider register high
    pub fn divh() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1111 << 0);
        value as u8
    }
}
/// RTC Prescaler Divider Register Low
pub mod divl {
    pub const OFFSET: u32 = 0x14;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// RTC prescaler divider register Low
    /// Access: read-only, Width: 16, Offset: 0
    /// Get RTC prescaler divider register Low
    pub fn divl() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1111111111111111 << 0);
        value as u16
    }
}
/// RTC Counter Register High
pub mod cnth {
    pub const OFFSET: u32 = 0x18;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// RTC counter register high
        pub cnth: u16,
    }
    pub struct Cache {
        /// RTC counter register high
        pub cnth: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            cnth: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            cnth: ((value >> 0) & 0b1111111111111111) as u16,
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
                | ((self.cnth as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// RTC Counter Register Low
pub mod cntl {
    pub const OFFSET: u32 = 0x1C;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// RTC counter register Low
        pub cntl: u16,
    }
    pub struct Cache {
        /// RTC counter register Low
        pub cntl: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            cntl: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            cntl: ((value >> 0) & 0b1111111111111111) as u16,
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
                | ((self.cntl as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// RTC Alarm Register High
pub mod alrh {
    pub const OFFSET: u32 = 0x20;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// RTC alarm register high
    /// Access: write-only, Width: 16, Offset: 0
    /// Set RTC alarm register high
    pub fn alrh(value: u16) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
}
/// RTC Alarm Register Low
pub mod alrl {
    pub const OFFSET: u32 = 0x24;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// RTC alarm register low
    /// Access: write-only, Width: 16, Offset: 0
    /// Set RTC alarm register low
    pub fn alrl(value: u16) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
}
/// RTC global interrupt
pub const INTERRUPT_RTC: u32 = 3;
/// RTC Alarms through EXTI line interrupt
pub const INTERRUPT_RTCALARM: u32 = 41;
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0x40002800</baseAddress>
  <description>Real time clock</description>
  <groupName>RTC</groupName>
  <interrupt>
    <description>RTC global interrupt</description>
    <name>RTC</name>
    <value>3</value>
  </interrupt>
  <interrupt>
    <description>RTC Alarms through EXTI line
        interrupt</description>
    <name>RTCAlarm</name>
    <value>41</value>
  </interrupt>
  <name>RTC</name>
  <registers>
    <register>
      <access>read-write</access>
      <addressOffset>0x0</addressOffset>
      <description>RTC Control Register High</description>
      <displayName>CRH</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Second interrupt Enable</description>
          <name>SECIE</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Alarm interrupt Enable</description>
          <name>ALRIE</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Overflow interrupt Enable</description>
          <name>OWIE</name>
        </field>
      </fields>
      <name>CRH</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0x4</addressOffset>
      <description>RTC Control Register Low</description>
      <displayName>CRL</displayName>
      <fields>
        <field>
          <access>read-write</access>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Second Flag</description>
          <name>SECF</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Alarm Flag</description>
          <name>ALRF</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Overflow Flag</description>
          <name>OWF</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Registers Synchronized
              Flag</description>
          <name>RSF</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Configuration Flag</description>
          <name>CNF</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RTC operation OFF</description>
          <name>RTOFF</name>
        </field>
      </fields>
      <name>CRL</name>
      <resetValue>0x00000020</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>write-only</access>
      <addressOffset>0x8</addressOffset>
      <description>RTC Prescaler Load Register
          High</description>
      <displayName>PRLH</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>RTC Prescaler Load Register
              High</description>
          <name>PRLH</name>
        </field>
      </fields>
      <name>PRLH</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>write-only</access>
      <addressOffset>0xC</addressOffset>
      <description>RTC Prescaler Load Register
          Low</description>
      <displayName>PRLL</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>RTC Prescaler Divider Register
              Low</description>
          <name>PRLL</name>
        </field>
      </fields>
      <name>PRLL</name>
      <resetValue>0x8000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x10</addressOffset>
      <description>RTC Prescaler Divider Register
          High</description>
      <displayName>DIVH</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>RTC prescaler divider register
              high</description>
          <name>DIVH</name>
        </field>
      </fields>
      <name>DIVH</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x14</addressOffset>
      <description>RTC Prescaler Divider Register
          Low</description>
      <displayName>DIVL</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>RTC prescaler divider register
              Low</description>
          <name>DIVL</name>
        </field>
      </fields>
      <name>DIVL</name>
      <resetValue>0x8000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x18</addressOffset>
      <description>RTC Counter Register High</description>
      <displayName>CNTH</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>RTC counter register high</description>
          <name>CNTH</name>
        </field>
      </fields>
      <name>CNTH</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x1C</addressOffset>
      <description>RTC Counter Register Low</description>
      <displayName>CNTL</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>RTC counter register Low</description>
          <name>CNTL</name>
        </field>
      </fields>
      <name>CNTL</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>write-only</access>
      <addressOffset>0x20</addressOffset>
      <description>RTC Alarm Register High</description>
      <displayName>ALRH</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>RTC alarm register high</description>
          <name>ALRH</name>
        </field>
      </fields>
      <name>ALRH</name>
      <resetValue>0xFFFF</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>write-only</access>
      <addressOffset>0x24</addressOffset>
      <description>RTC Alarm Register Low</description>
      <displayName>ALRL</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>RTC alarm register low</description>
          <name>ALRL</name>
        </field>
      </fields>
      <name>ALRL</name>
      <resetValue>0xFFFF</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
