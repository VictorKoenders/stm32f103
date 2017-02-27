pub const ADDRESS: u32 = 0x40022000;
/// Flash access control register
pub mod acr {
    pub const OFFSET: u32 = 0x0;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// Latency
    /// Access: read-write, Width: 3, Offset: 0
    /// Set Latency
    pub fn set_latency(value: u8) {
        debug_assert!(value <= 0b111, "set_latency out of range");
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get Latency
    pub fn get_latency() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b111 << 0);
        value as u8
    }
    /// Flash half cycle access enable
    /// Access: read-write, Width: 1, Offset: 3
    /// Set Flash half cycle access enable
    pub fn set_hlfcya(value: bool) {
        let value = value as u32;
        let value = value << 3;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get Flash half cycle access enable
    pub fn get_hlfcya() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 3);
        value > 0
    }
    /// Prefetch buffer enable
    /// Access: read-write, Width: 1, Offset: 4
    /// Set Prefetch buffer enable
    pub fn set_prftbe(value: bool) {
        let value = value as u32;
        let value = value << 4;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get Prefetch buffer enable
    pub fn get_prftbe() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 4);
        value > 0
    }
    /// Prefetch buffer status
    /// Access: read-only, Width: 1, Offset: 5
    /// Get Prefetch buffer status
    pub fn prftbs() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 5);
        value > 0
    }
}
/// Flash key register
pub mod keyr {
    pub const OFFSET: u32 = 0x4;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// FPEC key
    /// Access: write-only, Width: 32, Offset: 0
    /// Set FPEC key
    pub fn key(value: u32) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
}
/// Flash option key register
pub mod optkeyr {
    pub const OFFSET: u32 = 0x8;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// Option byte key
    /// Access: write-only, Width: 32, Offset: 0
    /// Set Option byte key
    pub fn optkey(value: u32) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
}
/// Status register
pub mod sr {
    pub const OFFSET: u32 = 0xC;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// End of operation
    /// Access: read-write, Width: 1, Offset: 5
    /// Set End of operation
    pub fn set_eop(value: bool) {
        let value = value as u32;
        let value = value << 5;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get End of operation
    pub fn get_eop() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 5);
        value > 0
    }
    /// Write protection error
    /// Access: read-write, Width: 1, Offset: 4
    /// Set Write protection error
    pub fn set_wrprterr(value: bool) {
        let value = value as u32;
        let value = value << 4;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get Write protection error
    pub fn get_wrprterr() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 4);
        value > 0
    }
    /// Programming error
    /// Access: read-write, Width: 1, Offset: 2
    /// Set Programming error
    pub fn set_pgerr(value: bool) {
        let value = value as u32;
        let value = value << 2;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get Programming error
    pub fn get_pgerr() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 2);
        value > 0
    }
    /// Busy
    /// Access: read-only, Width: 1, Offset: 0
    /// Get Busy
    pub fn bsy() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
}
/// Control register
pub mod cr {
    pub const OFFSET: u32 = 0x10;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Programming
        pub pg: bool,
        /// Page Erase
        pub per: bool,
        /// Mass Erase
        pub mer: bool,
        /// Option byte programming
        pub optpg: bool,
        /// Option byte erase
        pub opter: bool,
        /// Start
        pub strt: bool,
        /// Lock
        pub lock: bool,
        /// Option bytes write enable
        pub optwre: bool,
        /// Error interrupt enable
        pub errie: bool,
        /// End of operation interrupt enable
        pub eopie: bool,
    }
    pub struct Cache {
        /// Programming
        pub pg: bool,
        /// Page Erase
        pub per: bool,
        /// Mass Erase
        pub mer: bool,
        /// Option byte programming
        pub optpg: bool,
        /// Option byte erase
        pub opter: bool,
        /// Start
        pub strt: bool,
        /// Lock
        pub lock: bool,
        /// Option bytes write enable
        pub optwre: bool,
        /// Error interrupt enable
        pub errie: bool,
        /// End of operation interrupt enable
        pub eopie: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            pg: ((value >> 0) & 0b1) > 0,
            per: ((value >> 1) & 0b1) > 0,
            mer: ((value >> 2) & 0b1) > 0,
            optpg: ((value >> 4) & 0b1) > 0,
            opter: ((value >> 5) & 0b1) > 0,
            strt: ((value >> 6) & 0b1) > 0,
            lock: ((value >> 7) & 0b1) > 0,
            optwre: ((value >> 9) & 0b1) > 0,
            errie: ((value >> 10) & 0b1) > 0,
            eopie: ((value >> 12) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            pg: ((value >> 0) & 0b1) > 0,
            per: ((value >> 1) & 0b1) > 0,
            mer: ((value >> 2) & 0b1) > 0,
            optpg: ((value >> 4) & 0b1) > 0,
            opter: ((value >> 5) & 0b1) > 0,
            strt: ((value >> 6) & 0b1) > 0,
            lock: ((value >> 7) & 0b1) > 0,
            optwre: ((value >> 9) & 0b1) > 0,
            errie: ((value >> 10) & 0b1) > 0,
            eopie: ((value >> 12) & 0b1) > 0,
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
                | ((self.pg as u32) << 0)
                | ((self.per as u32) << 1)
                | ((self.mer as u32) << 2)
                | ((self.optpg as u32) << 4)
                | ((self.opter as u32) << 5)
                | ((self.strt as u32) << 6)
                | ((self.lock as u32) << 7)
                | ((self.optwre as u32) << 9)
                | ((self.errie as u32) << 10)
                | ((self.eopie as u32) << 12)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Flash address register
pub mod ar {
    pub const OFFSET: u32 = 0x14;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// Flash Address
    /// Access: write-only, Width: 32, Offset: 0
    /// Set Flash Address
    pub fn far(value: u32) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
}
/// Option byte register
pub mod obr {
    pub const OFFSET: u32 = 0x1C;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// Option byte error
    /// Access: read-only, Width: 1, Offset: 0
    /// Get Option byte error
    pub fn opterr() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
    /// Read protection
    /// Access: read-only, Width: 1, Offset: 1
    /// Get Read protection
    pub fn rdprt() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 1);
        value > 0
    }
    /// WDG_SW
    /// Access: read-only, Width: 1, Offset: 2
    /// Get WDG_SW
    pub fn wdg_sw() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 2);
        value > 0
    }
    /// nRST_STOP
    /// Access: read-only, Width: 1, Offset: 3
    /// Get nRST_STOP
    pub fn nrst_stop() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 3);
        value > 0
    }
    /// nRST_STDBY
    /// Access: read-only, Width: 1, Offset: 4
    /// Get nRST_STDBY
    pub fn nrst_stdby() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 4);
        value > 0
    }
    /// Data0
    /// Access: read-only, Width: 8, Offset: 10
    /// Get Data0
    pub fn data0() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b11111111 << 10);
        value as u8
    }
    /// Data1
    /// Access: read-only, Width: 8, Offset: 18
    /// Get Data1
    pub fn data1() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b11111111 << 18);
        value as u8
    }
}
/// Write protection register
pub mod wrpr {
    pub const OFFSET: u32 = 0x20;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// Write protect
    /// Access: read-only, Width: 32, Offset: 0
    /// Get Write protect
    pub fn wrp() -> u32 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b11111111111111111111111111111111 << 0);
        value as u32
    }
}
/// Flash global interrupt
pub const INTERRUPT_FLASH: u32 = 4;
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0x40022000</baseAddress>
  <description>FLASH</description>
  <groupName>FLASH</groupName>
  <interrupt>
    <description>Flash global interrupt</description>
    <name>FLASH</name>
    <value>4</value>
  </interrupt>
  <name>FLASH</name>
  <registers>
    <register>
      <addressOffset>0x0</addressOffset>
      <description>Flash access control register</description>
      <displayName>ACR</displayName>
      <fields>
        <field>
          <access>read-write</access>
          <bitOffset>0</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Latency</description>
          <name>LATENCY</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Flash half cycle access
              enable</description>
          <name>HLFCYA</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Prefetch buffer enable</description>
          <name>PRFTBE</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Prefetch buffer status</description>
          <name>PRFTBS</name>
        </field>
      </fields>
      <name>ACR</name>
      <resetValue>0x00000030</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>write-only</access>
      <addressOffset>0x4</addressOffset>
      <description>Flash key register</description>
      <displayName>KEYR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>FPEC key</description>
          <name>KEY</name>
        </field>
      </fields>
      <name>KEYR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>write-only</access>
      <addressOffset>0x8</addressOffset>
      <description>Flash option key register</description>
      <displayName>OPTKEYR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>Option byte key</description>
          <name>OPTKEY</name>
        </field>
      </fields>
      <name>OPTKEYR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0xC</addressOffset>
      <description>Status register</description>
      <displayName>SR</displayName>
      <fields>
        <field>
          <access>read-write</access>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>End of operation</description>
          <name>EOP</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Write protection error</description>
          <name>WRPRTERR</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Programming error</description>
          <name>PGERR</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Busy</description>
          <name>BSY</name>
        </field>
      </fields>
      <name>SR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x10</addressOffset>
      <description>Control register</description>
      <displayName>CR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Programming</description>
          <name>PG</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Page Erase</description>
          <name>PER</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Mass Erase</description>
          <name>MER</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Option byte programming</description>
          <name>OPTPG</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Option byte erase</description>
          <name>OPTER</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Start</description>
          <name>STRT</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Lock</description>
          <name>LOCK</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Option bytes write enable</description>
          <name>OPTWRE</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Error interrupt enable</description>
          <name>ERRIE</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>End of operation interrupt
              enable</description>
          <name>EOPIE</name>
        </field>
      </fields>
      <name>CR</name>
      <resetValue>0x00000080</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>write-only</access>
      <addressOffset>0x14</addressOffset>
      <description>Flash address register</description>
      <displayName>AR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>Flash Address</description>
          <name>FAR</name>
        </field>
      </fields>
      <name>AR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x1C</addressOffset>
      <description>Option byte register</description>
      <displayName>OBR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Option byte error</description>
          <name>OPTERR</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Read protection</description>
          <name>RDPRT</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>WDG_SW</description>
          <name>WDG_SW</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>nRST_STOP</description>
          <name>nRST_STOP</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>nRST_STDBY</description>
          <name>nRST_STDBY</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>8</bitWidth>
          <description>Data0</description>
          <name>Data0</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>8</bitWidth>
          <description>Data1</description>
          <name>Data1</name>
        </field>
      </fields>
      <name>OBR</name>
      <resetValue>0x03FFFFFC</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x20</addressOffset>
      <description>Write protection register</description>
      <displayName>WRPR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>Write protect</description>
          <name>WRP</name>
        </field>
      </fields>
      <name>WRPR</name>
      <resetValue>0xFFFFFFFF</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
