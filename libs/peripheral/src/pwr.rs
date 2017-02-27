pub const ADDRESS: u32 = 0x40007000;
/// Power control register (PWR_CR)
pub mod cr {
    pub const OFFSET: u32 = 0x0;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Low Power Deep Sleep
        pub lpds: bool,
        /// Power Down Deep Sleep
        pub pdds: bool,
        /// Clear Wake-up Flag
        pub cwuf: bool,
        /// Clear STANDBY Flag
        pub csbf: bool,
        /// Power Voltage Detector Enable
        pub pvde: bool,
        /// PVD Level Selection
        pub pls: bool,
        /// Disable Backup Domain write protection
        pub dbp: bool,
    }
    pub struct Cache {
        /// Low Power Deep Sleep
        pub lpds: bool,
        /// Power Down Deep Sleep
        pub pdds: bool,
        /// Clear Wake-up Flag
        pub cwuf: bool,
        /// Clear STANDBY Flag
        pub csbf: bool,
        /// Power Voltage Detector Enable
        pub pvde: bool,
        /// PVD Level Selection
        pub pls: bool,
        /// Disable Backup Domain write protection
        pub dbp: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            lpds: ((value >> 0) & 0b1) > 0,
            pdds: ((value >> 1) & 0b1) > 0,
            cwuf: ((value >> 2) & 0b1) > 0,
            csbf: ((value >> 3) & 0b1) > 0,
            pvde: ((value >> 4) & 0b1) > 0,
            pls: ((value >> 5) & 0b1) > 0,
            dbp: ((value >> 8) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            lpds: ((value >> 0) & 0b1) > 0,
            pdds: ((value >> 1) & 0b1) > 0,
            cwuf: ((value >> 2) & 0b1) > 0,
            csbf: ((value >> 3) & 0b1) > 0,
            pvde: ((value >> 4) & 0b1) > 0,
            pls: ((value >> 5) & 0b1) > 0,
            dbp: ((value >> 8) & 0b1) > 0,
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
                | ((self.lpds as u32) << 0)
                | ((self.pdds as u32) << 1)
                | ((self.cwuf as u32) << 2)
                | ((self.csbf as u32) << 3)
                | ((self.pvde as u32) << 4)
                | ((self.pls as u32) << 5)
                | ((self.dbp as u32) << 8)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Power control register (PWR_CR)
pub mod csr {
    pub const OFFSET: u32 = 0x4;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// Wake-Up Flag
    /// Access: read-only, Width: 1, Offset: 0
    /// Get Wake-Up Flag
    pub fn wuf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
    /// STANDBY Flag
    /// Access: read-only, Width: 1, Offset: 1
    /// Get STANDBY Flag
    pub fn sbf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 1);
        value > 0
    }
    /// PVD Output
    /// Access: read-only, Width: 1, Offset: 2
    /// Get PVD Output
    pub fn pvdo() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 2);
        value > 0
    }
    /// Enable WKUP pin
    /// Access: read-write, Width: 1, Offset: 8
    /// Set Enable WKUP pin
    pub fn set_ewup(value: bool) {
        let value = value as u32;
        let value = value << 8;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get Enable WKUP pin
    pub fn get_ewup() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 8);
        value > 0
    }
}
/// PVD through EXTI line detection interrupt
pub const INTERRUPT_PVD: u32 = 1;
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0x40007000</baseAddress>
  <description>Power control</description>
  <groupName>PWR</groupName>
  <interrupt>
    <description>PVD through EXTI line detection
        interrupt</description>
    <name>PVD</name>
    <value>1</value>
  </interrupt>
  <name>PWR</name>
  <registers>
    <register>
      <access>read-write</access>
      <addressOffset>0x0</addressOffset>
      <description>Power control register
          (PWR_CR)</description>
      <displayName>CR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Low Power Deep Sleep</description>
          <name>LPDS</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Power Down Deep Sleep</description>
          <name>PDDS</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Clear Wake-up Flag</description>
          <name>CWUF</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Clear STANDBY Flag</description>
          <name>CSBF</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Power Voltage Detector
              Enable</description>
          <name>PVDE</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>3</bitWidth>
          <description>PVD Level Selection</description>
          <name>PLS</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Disable Backup Domain write
              protection</description>
          <name>DBP</name>
        </field>
      </fields>
      <name>CR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0x4</addressOffset>
      <description>Power control register
          (PWR_CR)</description>
      <displayName>CSR</displayName>
      <fields>
        <field>
          <access>read-only</access>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Wake-Up Flag</description>
          <name>WUF</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>STANDBY Flag</description>
          <name>SBF</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>PVD Output</description>
          <name>PVDO</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Enable WKUP pin</description>
          <name>EWUP</name>
        </field>
      </fields>
      <name>CSR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
