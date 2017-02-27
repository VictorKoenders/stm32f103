pub const ADDRESS: u32 = 0x40005400;
/// Control register 1
pub mod cr1 {
    pub const OFFSET: u32 = 0x0;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Software reset
        pub swrst: bool,
        /// SMBus alert
        pub alert: bool,
        /// Packet error checking
        pub pec: bool,
        /// Acknowledge/PEC Position (for data reception)
        pub pos: bool,
        /// Acknowledge enable
        pub ack: bool,
        /// Stop generation
        pub stop: bool,
        /// Start generation
        pub start: bool,
        /// Clock stretching disable (Slave mode)
        pub nostretch: bool,
        /// General call enable
        pub engc: bool,
        /// PEC enable
        pub enpec: bool,
        /// ARP enable
        pub enarp: bool,
        /// SMBus type
        pub smbtype: bool,
        /// SMBus mode
        pub smbus: bool,
        /// Peripheral enable
        pub pe: bool,
    }
    pub struct Cache {
        /// Software reset
        pub swrst: bool,
        /// SMBus alert
        pub alert: bool,
        /// Packet error checking
        pub pec: bool,
        /// Acknowledge/PEC Position (for data reception)
        pub pos: bool,
        /// Acknowledge enable
        pub ack: bool,
        /// Stop generation
        pub stop: bool,
        /// Start generation
        pub start: bool,
        /// Clock stretching disable (Slave mode)
        pub nostretch: bool,
        /// General call enable
        pub engc: bool,
        /// PEC enable
        pub enpec: bool,
        /// ARP enable
        pub enarp: bool,
        /// SMBus type
        pub smbtype: bool,
        /// SMBus mode
        pub smbus: bool,
        /// Peripheral enable
        pub pe: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            swrst: ((value >> 15) & 0b1) > 0,
            alert: ((value >> 13) & 0b1) > 0,
            pec: ((value >> 12) & 0b1) > 0,
            pos: ((value >> 11) & 0b1) > 0,
            ack: ((value >> 10) & 0b1) > 0,
            stop: ((value >> 9) & 0b1) > 0,
            start: ((value >> 8) & 0b1) > 0,
            nostretch: ((value >> 7) & 0b1) > 0,
            engc: ((value >> 6) & 0b1) > 0,
            enpec: ((value >> 5) & 0b1) > 0,
            enarp: ((value >> 4) & 0b1) > 0,
            smbtype: ((value >> 3) & 0b1) > 0,
            smbus: ((value >> 1) & 0b1) > 0,
            pe: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            swrst: ((value >> 15) & 0b1) > 0,
            alert: ((value >> 13) & 0b1) > 0,
            pec: ((value >> 12) & 0b1) > 0,
            pos: ((value >> 11) & 0b1) > 0,
            ack: ((value >> 10) & 0b1) > 0,
            stop: ((value >> 9) & 0b1) > 0,
            start: ((value >> 8) & 0b1) > 0,
            nostretch: ((value >> 7) & 0b1) > 0,
            engc: ((value >> 6) & 0b1) > 0,
            enpec: ((value >> 5) & 0b1) > 0,
            enarp: ((value >> 4) & 0b1) > 0,
            smbtype: ((value >> 3) & 0b1) > 0,
            smbus: ((value >> 1) & 0b1) > 0,
            pe: ((value >> 0) & 0b1) > 0,
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
                | ((self.swrst as u32) << 15)
                | ((self.alert as u32) << 13)
                | ((self.pec as u32) << 12)
                | ((self.pos as u32) << 11)
                | ((self.ack as u32) << 10)
                | ((self.stop as u32) << 9)
                | ((self.start as u32) << 8)
                | ((self.nostretch as u32) << 7)
                | ((self.engc as u32) << 6)
                | ((self.enpec as u32) << 5)
                | ((self.enarp as u32) << 4)
                | ((self.smbtype as u32) << 3)
                | ((self.smbus as u32) << 1)
                | ((self.pe as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Control register 2
pub mod cr2 {
    pub const OFFSET: u32 = 0x4;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// DMA last transfer
        pub last: bool,
        /// DMA requests enable
        pub dmaen: bool,
        /// Buffer interrupt enable
        pub itbufen: bool,
        /// Event interrupt enable
        pub itevten: bool,
        /// Error interrupt enable
        pub iterren: bool,
        /// Peripheral clock frequency
        pub freq: bool,
    }
    pub struct Cache {
        /// DMA last transfer
        pub last: bool,
        /// DMA requests enable
        pub dmaen: bool,
        /// Buffer interrupt enable
        pub itbufen: bool,
        /// Event interrupt enable
        pub itevten: bool,
        /// Error interrupt enable
        pub iterren: bool,
        /// Peripheral clock frequency
        pub freq: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            last: ((value >> 12) & 0b1) > 0,
            dmaen: ((value >> 11) & 0b1) > 0,
            itbufen: ((value >> 10) & 0b1) > 0,
            itevten: ((value >> 9) & 0b1) > 0,
            iterren: ((value >> 8) & 0b1) > 0,
            freq: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            last: ((value >> 12) & 0b1) > 0,
            dmaen: ((value >> 11) & 0b1) > 0,
            itbufen: ((value >> 10) & 0b1) > 0,
            itevten: ((value >> 9) & 0b1) > 0,
            iterren: ((value >> 8) & 0b1) > 0,
            freq: ((value >> 0) & 0b1) > 0,
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
                | ((self.last as u32) << 12)
                | ((self.dmaen as u32) << 11)
                | ((self.itbufen as u32) << 10)
                | ((self.itevten as u32) << 9)
                | ((self.iterren as u32) << 8)
                | ((self.freq as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Own address register 1
pub mod oar1 {
    pub const OFFSET: u32 = 0x8;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Addressing mode (slave mode)
        pub addmode: bool,
        /// Interface address
        pub add10: bool,
        /// Interface address
        pub add7: bool,
        /// Interface address
        pub add0: bool,
    }
    pub struct Cache {
        /// Addressing mode (slave mode)
        pub addmode: bool,
        /// Interface address
        pub add10: bool,
        /// Interface address
        pub add7: bool,
        /// Interface address
        pub add0: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            addmode: ((value >> 15) & 0b1) > 0,
            add10: ((value >> 8) & 0b1) > 0,
            add7: ((value >> 1) & 0b1) > 0,
            add0: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            addmode: ((value >> 15) & 0b1) > 0,
            add10: ((value >> 8) & 0b1) > 0,
            add7: ((value >> 1) & 0b1) > 0,
            add0: ((value >> 0) & 0b1) > 0,
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
                | ((self.addmode as u32) << 15)
                | ((self.add10 as u32) << 8)
                | ((self.add7 as u32) << 1)
                | ((self.add0 as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Own address register 2
pub mod oar2 {
    pub const OFFSET: u32 = 0xC;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Interface address
        pub add2: u8,
        /// Dual addressing mode enable
        pub endual: u8,
    }
    pub struct Cache {
        /// Interface address
        pub add2: u8,
        /// Dual addressing mode enable
        pub endual: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            add2: ((value >> 1) & 0b1111111) as u8,
            endual: ((value >> 0) & 0b1111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            add2: ((value >> 1) & 0b1111111) as u8,
            endual: ((value >> 0) & 0b1111111) as u8,
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
                | ((self.add2 as u32) << 1)
                | ((self.endual as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Data register
pub mod dr {
    pub const OFFSET: u32 = 0x10;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// 8-bit data register
        pub dr: u8,
    }
    pub struct Cache {
        /// 8-bit data register
        pub dr: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            dr: ((value >> 0) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            dr: ((value >> 0) & 0b11111111) as u8,
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
/// Status register 1
pub mod sr1 {
    pub const OFFSET: u32 = 0x14;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// SMBus alert
    /// Access: read-write, Width: 1, Offset: 15
    /// Set SMBus alert
    pub fn set_smbalert(value: bool) {
        let value = value as u32;
        let value = value << 15;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get SMBus alert
    pub fn get_smbalert() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 15);
        value > 0
    }
    /// Timeout or Tlow error
    /// Access: read-write, Width: 1, Offset: 14
    /// Set Timeout or Tlow error
    pub fn set_timeout(value: bool) {
        let value = value as u32;
        let value = value << 14;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get Timeout or Tlow error
    pub fn get_timeout() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 14);
        value > 0
    }
    /// PEC Error in reception
    /// Access: read-write, Width: 1, Offset: 12
    /// Set PEC Error in reception
    pub fn set_pecerr(value: bool) {
        let value = value as u32;
        let value = value << 12;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get PEC Error in reception
    pub fn get_pecerr() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 12);
        value > 0
    }
    /// Overrun/Underrun
    /// Access: read-write, Width: 1, Offset: 11
    /// Set Overrun/Underrun
    pub fn set_ovr(value: bool) {
        let value = value as u32;
        let value = value << 11;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get Overrun/Underrun
    pub fn get_ovr() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 11);
        value > 0
    }
    /// Acknowledge failure
    /// Access: read-write, Width: 1, Offset: 10
    /// Set Acknowledge failure
    pub fn set_af(value: bool) {
        let value = value as u32;
        let value = value << 10;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get Acknowledge failure
    pub fn get_af() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 10);
        value > 0
    }
    /// Arbitration lost (master mode)
    /// Access: read-write, Width: 1, Offset: 9
    /// Set Arbitration lost (master mode)
    pub fn set_arlo(value: bool) {
        let value = value as u32;
        let value = value << 9;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get Arbitration lost (master mode)
    pub fn get_arlo() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 9);
        value > 0
    }
    /// Bus error
    /// Access: read-write, Width: 1, Offset: 8
    /// Set Bus error
    pub fn set_berr(value: bool) {
        let value = value as u32;
        let value = value << 8;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get Bus error
    pub fn get_berr() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 8);
        value > 0
    }
    /// Data register empty (transmitters)
    /// Access: read-only, Width: 1, Offset: 7
    /// Get Data register empty (transmitters)
    pub fn txe() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 7);
        value > 0
    }
    /// Data register not empty (receivers)
    /// Access: read-only, Width: 1, Offset: 6
    /// Get Data register not empty (receivers)
    pub fn rxne() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 6);
        value > 0
    }
    /// Stop detection (slave mode)
    /// Access: read-only, Width: 1, Offset: 4
    /// Get Stop detection (slave mode)
    pub fn stopf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 4);
        value > 0
    }
    /// 10-bit header sent (Master mode)
    /// Access: read-only, Width: 1, Offset: 3
    /// Get 10-bit header sent (Master mode)
    pub fn add10() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 3);
        value > 0
    }
    /// Byte transfer finished
    /// Access: read-only, Width: 1, Offset: 2
    /// Get Byte transfer finished
    pub fn btf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 2);
        value > 0
    }
    /// Address sent (master mode)/matched (slave mode)
    /// Access: read-only, Width: 1, Offset: 1
    /// Get Address sent (master mode)/matched (slave mode)
    pub fn addr() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 1);
        value > 0
    }
    /// Start bit (Master mode)
    /// Access: read-only, Width: 1, Offset: 0
    /// Get Start bit (Master mode)
    pub fn sb() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
}
/// Status register 2
pub mod sr2 {
    pub const OFFSET: u32 = 0x18;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// acket error checking register
    /// Access: read-only, Width: 8, Offset: 8
    /// Get acket error checking register
    pub fn pec() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b11111111 << 8);
        value as u8
    }
    /// Dual flag (Slave mode)
    /// Access: read-only, Width: 1, Offset: 7
    /// Get Dual flag (Slave mode)
    pub fn dualf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 7);
        value > 0
    }
    /// SMBus host header (Slave mode)
    /// Access: read-only, Width: 1, Offset: 6
    /// Get SMBus host header (Slave mode)
    pub fn smbhost() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 6);
        value > 0
    }
    /// SMBus device default address (Slave mode)
    /// Access: read-only, Width: 1, Offset: 5
    /// Get SMBus device default address (Slave mode)
    pub fn smbdefault() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 5);
        value > 0
    }
    /// General call address (Slave mode)
    /// Access: read-only, Width: 1, Offset: 4
    /// Get General call address (Slave mode)
    pub fn gencall() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 4);
        value > 0
    }
    /// Transmitter/receiver
    /// Access: read-only, Width: 1, Offset: 2
    /// Get Transmitter/receiver
    pub fn tra() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 2);
        value > 0
    }
    /// Bus busy
    /// Access: read-only, Width: 1, Offset: 1
    /// Get Bus busy
    pub fn busy() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 1);
        value > 0
    }
    /// Master/slave
    /// Access: read-only, Width: 1, Offset: 0
    /// Get Master/slave
    pub fn msl() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
}
/// Clock control register
pub mod ccr {
    pub const OFFSET: u32 = 0x1C;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// I2C master mode selection
        pub f_s: bool,
        /// Fast mode duty cycle
        pub duty: bool,
        /// Clock control register in Fast/Standard mode (Master mode)
        pub ccr: bool,
    }
    pub struct Cache {
        /// I2C master mode selection
        pub f_s: bool,
        /// Fast mode duty cycle
        pub duty: bool,
        /// Clock control register in Fast/Standard mode (Master mode)
        pub ccr: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            f_s: ((value >> 15) & 0b1) > 0,
            duty: ((value >> 14) & 0b1) > 0,
            ccr: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            f_s: ((value >> 15) & 0b1) > 0,
            duty: ((value >> 14) & 0b1) > 0,
            ccr: ((value >> 0) & 0b1) > 0,
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
                | ((self.f_s as u32) << 15)
                | ((self.duty as u32) << 14)
                | ((self.ccr as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// TRISE register
pub mod trise {
    pub const OFFSET: u32 = 0x20;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Maximum rise time in Fast/Standard mode (Master mode)
        pub trise: u8,
    }
    pub struct Cache {
        /// Maximum rise time in Fast/Standard mode (Master mode)
        pub trise: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            trise: ((value >> 0) & 0b111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            trise: ((value >> 0) & 0b111111) as u8,
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
                | ((self.trise as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// I2C1 event interrupt
pub const INTERRUPT_I2C1_EV: u32 = 31;
/// I2C1 error interrupt
pub const INTERRUPT_I2C1_ER: u32 = 32;
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0x40005400</baseAddress>
  <description>Inter integrated circuit</description>
  <groupName>I2C</groupName>
  <interrupt>
    <description>I2C1 event interrupt</description>
    <name>I2C1_EV</name>
    <value>31</value>
  </interrupt>
  <interrupt>
    <description>I2C1 error interrupt</description>
    <name>I2C1_ER</name>
    <value>32</value>
  </interrupt>
  <name>I2C1</name>
  <registers>
    <register>
      <access>read-write</access>
      <addressOffset>0x0</addressOffset>
      <description>Control register 1</description>
      <displayName>CR1</displayName>
      <fields>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Software reset</description>
          <name>SWRST</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>SMBus alert</description>
          <name>ALERT</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Packet error checking</description>
          <name>PEC</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Acknowledge/PEC Position (for data
              reception)</description>
          <name>POS</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Acknowledge enable</description>
          <name>ACK</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Stop generation</description>
          <name>STOP</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Start generation</description>
          <name>START</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Clock stretching disable (Slave
              mode)</description>
          <name>NOSTRETCH</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>General call enable</description>
          <name>ENGC</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>PEC enable</description>
          <name>ENPEC</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ARP enable</description>
          <name>ENARP</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>SMBus type</description>
          <name>SMBTYPE</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>SMBus mode</description>
          <name>SMBUS</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Peripheral enable</description>
          <name>PE</name>
        </field>
      </fields>
      <name>CR1</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x4</addressOffset>
      <description>Control register 2</description>
      <displayName>CR2</displayName>
      <fields>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DMA last transfer</description>
          <name>LAST</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DMA requests enable</description>
          <name>DMAEN</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Buffer interrupt enable</description>
          <name>ITBUFEN</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event interrupt enable</description>
          <name>ITEVTEN</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Error interrupt enable</description>
          <name>ITERREN</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>6</bitWidth>
          <description>Peripheral clock frequency</description>
          <name>FREQ</name>
        </field>
      </fields>
      <name>CR2</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x8</addressOffset>
      <description>Own address register 1</description>
      <displayName>OAR1</displayName>
      <fields>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Addressing mode (slave
              mode)</description>
          <name>ADDMODE</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Interface address</description>
          <name>ADD10</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>7</bitWidth>
          <description>Interface address</description>
          <name>ADD7</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Interface address</description>
          <name>ADD0</name>
        </field>
      </fields>
      <name>OAR1</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xC</addressOffset>
      <description>Own address register 2</description>
      <displayName>OAR2</displayName>
      <fields>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>7</bitWidth>
          <description>Interface address</description>
          <name>ADD2</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Dual addressing mode
              enable</description>
          <name>ENDUAL</name>
        </field>
      </fields>
      <name>OAR2</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x10</addressOffset>
      <description>Data register</description>
      <displayName>DR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>8-bit data register</description>
          <name>DR</name>
        </field>
      </fields>
      <name>DR</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0x14</addressOffset>
      <description>Status register 1</description>
      <displayName>SR1</displayName>
      <fields>
        <field>
          <access>read-write</access>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>SMBus alert</description>
          <name>SMBALERT</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Timeout or Tlow error</description>
          <name>TIMEOUT</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>PEC Error in reception</description>
          <name>PECERR</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Overrun/Underrun</description>
          <name>OVR</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Acknowledge failure</description>
          <name>AF</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Arbitration lost (master
              mode)</description>
          <name>ARLO</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Bus error</description>
          <name>BERR</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Data register empty
              (transmitters)</description>
          <name>TxE</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Data register not empty
              (receivers)</description>
          <name>RxNE</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Stop detection (slave
              mode)</description>
          <name>STOPF</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>10-bit header sent (Master
              mode)</description>
          <name>ADD10</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Byte transfer finished</description>
          <name>BTF</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Address sent (master mode)/matched
              (slave mode)</description>
          <name>ADDR</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Start bit (Master mode)</description>
          <name>SB</name>
        </field>
      </fields>
      <name>SR1</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x18</addressOffset>
      <description>Status register 2</description>
      <displayName>SR2</displayName>
      <fields>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>acket error checking
              register</description>
          <name>PEC</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Dual flag (Slave mode)</description>
          <name>DUALF</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>SMBus host header (Slave
              mode)</description>
          <name>SMBHOST</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>SMBus device default address (Slave
              mode)</description>
          <name>SMBDEFAULT</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>General call address (Slave
              mode)</description>
          <name>GENCALL</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Transmitter/receiver</description>
          <name>TRA</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Bus busy</description>
          <name>BUSY</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Master/slave</description>
          <name>MSL</name>
        </field>
      </fields>
      <name>SR2</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x1C</addressOffset>
      <description>Clock control register</description>
      <displayName>CCR</displayName>
      <fields>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>I2C master mode selection</description>
          <name>F_S</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Fast mode duty cycle</description>
          <name>DUTY</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
          <description>Clock control register in Fast/Standard
              mode (Master mode)</description>
          <name>CCR</name>
        </field>
      </fields>
      <name>CCR</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x20</addressOffset>
      <description>TRISE register</description>
      <displayName>TRISE</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>6</bitWidth>
          <description>Maximum rise time in Fast/Standard mode
              (Master mode)</description>
          <name>TRISE</name>
        </field>
      </fields>
      <name>TRISE</name>
      <resetValue>0x0002</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
