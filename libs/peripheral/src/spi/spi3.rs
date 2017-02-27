pub const ADDRESS: u32 = 0x40003C00;
/// control register 1
pub mod cr1 {
    pub const OFFSET: u32 = 0x0;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Bidirectional data mode enable
        pub bidimode: bool,
        /// Output enable in bidirectional mode
        pub bidioe: bool,
        /// Hardware CRC calculation enable
        pub crcen: bool,
        /// CRC transfer next
        pub crcnext: bool,
        /// Data frame format
        pub dff: bool,
        /// Receive only
        pub rxonly: bool,
        /// Software slave management
        pub ssm: bool,
        /// Internal slave select
        pub ssi: bool,
        /// Frame format
        pub lsbfirst: bool,
        /// SPI enable
        pub spe: bool,
        /// Baud rate control
        pub br: bool,
        /// Master selection
        pub mstr: bool,
        /// Clock polarity
        pub cpol: bool,
        /// Clock phase
        pub cpha: bool,
    }
    pub struct Cache {
        /// Bidirectional data mode enable
        pub bidimode: bool,
        /// Output enable in bidirectional mode
        pub bidioe: bool,
        /// Hardware CRC calculation enable
        pub crcen: bool,
        /// CRC transfer next
        pub crcnext: bool,
        /// Data frame format
        pub dff: bool,
        /// Receive only
        pub rxonly: bool,
        /// Software slave management
        pub ssm: bool,
        /// Internal slave select
        pub ssi: bool,
        /// Frame format
        pub lsbfirst: bool,
        /// SPI enable
        pub spe: bool,
        /// Baud rate control
        pub br: bool,
        /// Master selection
        pub mstr: bool,
        /// Clock polarity
        pub cpol: bool,
        /// Clock phase
        pub cpha: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            bidimode: ((value >> 15) & 0b1) > 0,
            bidioe: ((value >> 14) & 0b1) > 0,
            crcen: ((value >> 13) & 0b1) > 0,
            crcnext: ((value >> 12) & 0b1) > 0,
            dff: ((value >> 11) & 0b1) > 0,
            rxonly: ((value >> 10) & 0b1) > 0,
            ssm: ((value >> 9) & 0b1) > 0,
            ssi: ((value >> 8) & 0b1) > 0,
            lsbfirst: ((value >> 7) & 0b1) > 0,
            spe: ((value >> 6) & 0b1) > 0,
            br: ((value >> 3) & 0b1) > 0,
            mstr: ((value >> 2) & 0b1) > 0,
            cpol: ((value >> 1) & 0b1) > 0,
            cpha: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            bidimode: ((value >> 15) & 0b1) > 0,
            bidioe: ((value >> 14) & 0b1) > 0,
            crcen: ((value >> 13) & 0b1) > 0,
            crcnext: ((value >> 12) & 0b1) > 0,
            dff: ((value >> 11) & 0b1) > 0,
            rxonly: ((value >> 10) & 0b1) > 0,
            ssm: ((value >> 9) & 0b1) > 0,
            ssi: ((value >> 8) & 0b1) > 0,
            lsbfirst: ((value >> 7) & 0b1) > 0,
            spe: ((value >> 6) & 0b1) > 0,
            br: ((value >> 3) & 0b1) > 0,
            mstr: ((value >> 2) & 0b1) > 0,
            cpol: ((value >> 1) & 0b1) > 0,
            cpha: ((value >> 0) & 0b1) > 0,
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
                | ((self.bidimode as u32) << 15)
                | ((self.bidioe as u32) << 14)
                | ((self.crcen as u32) << 13)
                | ((self.crcnext as u32) << 12)
                | ((self.dff as u32) << 11)
                | ((self.rxonly as u32) << 10)
                | ((self.ssm as u32) << 9)
                | ((self.ssi as u32) << 8)
                | ((self.lsbfirst as u32) << 7)
                | ((self.spe as u32) << 6)
                | ((self.br as u32) << 3)
                | ((self.mstr as u32) << 2)
                | ((self.cpol as u32) << 1)
                | ((self.cpha as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// control register 2
pub mod cr2 {
    pub const OFFSET: u32 = 0x4;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Tx buffer empty interrupt enable
        pub txeie: bool,
        /// RX buffer not empty interrupt enable
        pub rxneie: bool,
        /// Error interrupt enable
        pub errie: bool,
        /// SS output enable
        pub ssoe: bool,
        /// Tx buffer DMA enable
        pub txdmaen: bool,
        /// Rx buffer DMA enable
        pub rxdmaen: bool,
    }
    pub struct Cache {
        /// Tx buffer empty interrupt enable
        pub txeie: bool,
        /// RX buffer not empty interrupt enable
        pub rxneie: bool,
        /// Error interrupt enable
        pub errie: bool,
        /// SS output enable
        pub ssoe: bool,
        /// Tx buffer DMA enable
        pub txdmaen: bool,
        /// Rx buffer DMA enable
        pub rxdmaen: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            txeie: ((value >> 7) & 0b1) > 0,
            rxneie: ((value >> 6) & 0b1) > 0,
            errie: ((value >> 5) & 0b1) > 0,
            ssoe: ((value >> 2) & 0b1) > 0,
            txdmaen: ((value >> 1) & 0b1) > 0,
            rxdmaen: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            txeie: ((value >> 7) & 0b1) > 0,
            rxneie: ((value >> 6) & 0b1) > 0,
            errie: ((value >> 5) & 0b1) > 0,
            ssoe: ((value >> 2) & 0b1) > 0,
            txdmaen: ((value >> 1) & 0b1) > 0,
            rxdmaen: ((value >> 0) & 0b1) > 0,
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
                | ((self.txeie as u32) << 7)
                | ((self.rxneie as u32) << 6)
                | ((self.errie as u32) << 5)
                | ((self.ssoe as u32) << 2)
                | ((self.txdmaen as u32) << 1)
                | ((self.rxdmaen as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// status register
pub mod sr {
    pub const OFFSET: u32 = 0x8;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// Busy flag
    /// Access: read-only, Width: 1, Offset: 7
    /// Get Busy flag
    pub fn bsy() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 7);
        value > 0
    }
    /// Overrun flag
    /// Access: read-only, Width: 1, Offset: 6
    /// Get Overrun flag
    pub fn ovr() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 6);
        value > 0
    }
    /// Mode fault
    /// Access: read-only, Width: 1, Offset: 5
    /// Get Mode fault
    pub fn modf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 5);
        value > 0
    }
    /// CRC error flag
    /// Access: read-write, Width: 1, Offset: 4
    /// Set CRC error flag
    pub fn set_crcerr(value: bool) {
        let value = value as u32;
        let value = value << 4;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get CRC error flag
    pub fn get_crcerr() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 4);
        value > 0
    }
    /// Underrun flag
    /// Access: read-only, Width: 1, Offset: 3
    /// Get Underrun flag
    pub fn udr() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 3);
        value > 0
    }
    /// Channel side
    /// Access: read-only, Width: 1, Offset: 2
    /// Get Channel side
    pub fn chside() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 2);
        value > 0
    }
    /// Transmit buffer empty
    /// Access: read-only, Width: 1, Offset: 1
    /// Get Transmit buffer empty
    pub fn txe() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 1);
        value > 0
    }
    /// Receive buffer not empty
    /// Access: read-only, Width: 1, Offset: 0
    /// Get Receive buffer not empty
    pub fn rxne() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
}
/// data register
pub mod dr {
    pub const OFFSET: u32 = 0xC;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Data register
        pub dr: u16,
    }
    pub struct Cache {
        /// Data register
        pub dr: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            dr: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            dr: ((value >> 0) & 0b1111111111111111) as u16,
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
/// CRC polynomial register
pub mod crcpr {
    pub const OFFSET: u32 = 0x10;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// CRC polynomial register
        pub crcpoly: u16,
    }
    pub struct Cache {
        /// CRC polynomial register
        pub crcpoly: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            crcpoly: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            crcpoly: ((value >> 0) & 0b1111111111111111) as u16,
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
                | ((self.crcpoly as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// RX CRC register
pub mod rxcrcr {
    pub const OFFSET: u32 = 0x14;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// Rx CRC register
    /// Access: read-only, Width: 16, Offset: 0
    /// Get Rx CRC register
    pub fn rxcrc() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1111111111111111 << 0);
        value as u16
    }
}
/// TX CRC register
pub mod txcrcr {
    pub const OFFSET: u32 = 0x18;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// Tx CRC register
    /// Access: read-only, Width: 16, Offset: 0
    /// Get Tx CRC register
    pub fn txcrc() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1111111111111111 << 0);
        value as u16
    }
}
/// I2S configuration register
pub mod i2scfgr {
    pub const OFFSET: u32 = 0x1C;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// I2S mode selection
        pub i2smod: bool,
        /// I2S Enable
        pub i2se: bool,
        /// I2S configuration mode
        pub i2scfg: bool,
        /// PCM frame synchronization
        pub pcmsync: bool,
        /// I2S standard selection
        pub i2sstd: bool,
        /// Steady state clock polarity
        pub ckpol: bool,
        /// Data length to be transferred
        pub datlen: bool,
        /// Channel length (number of bits per audio channel)
        pub chlen: bool,
    }
    pub struct Cache {
        /// I2S mode selection
        pub i2smod: bool,
        /// I2S Enable
        pub i2se: bool,
        /// I2S configuration mode
        pub i2scfg: bool,
        /// PCM frame synchronization
        pub pcmsync: bool,
        /// I2S standard selection
        pub i2sstd: bool,
        /// Steady state clock polarity
        pub ckpol: bool,
        /// Data length to be transferred
        pub datlen: bool,
        /// Channel length (number of bits per audio channel)
        pub chlen: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            i2smod: ((value >> 11) & 0b1) > 0,
            i2se: ((value >> 10) & 0b1) > 0,
            i2scfg: ((value >> 8) & 0b1) > 0,
            pcmsync: ((value >> 7) & 0b1) > 0,
            i2sstd: ((value >> 4) & 0b1) > 0,
            ckpol: ((value >> 3) & 0b1) > 0,
            datlen: ((value >> 1) & 0b1) > 0,
            chlen: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            i2smod: ((value >> 11) & 0b1) > 0,
            i2se: ((value >> 10) & 0b1) > 0,
            i2scfg: ((value >> 8) & 0b1) > 0,
            pcmsync: ((value >> 7) & 0b1) > 0,
            i2sstd: ((value >> 4) & 0b1) > 0,
            ckpol: ((value >> 3) & 0b1) > 0,
            datlen: ((value >> 1) & 0b1) > 0,
            chlen: ((value >> 0) & 0b1) > 0,
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
                | ((self.i2smod as u32) << 11)
                | ((self.i2se as u32) << 10)
                | ((self.i2scfg as u32) << 8)
                | ((self.pcmsync as u32) << 7)
                | ((self.i2sstd as u32) << 4)
                | ((self.ckpol as u32) << 3)
                | ((self.datlen as u32) << 1)
                | ((self.chlen as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// I2S prescaler register
pub mod i2spr {
    pub const OFFSET: u32 = 0x20;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Master clock output enable
        pub mckoe: bool,
        /// Odd factor for the prescaler
        pub odd: bool,
        /// I2S Linear prescaler
        pub i2sdiv: bool,
    }
    pub struct Cache {
        /// Master clock output enable
        pub mckoe: bool,
        /// Odd factor for the prescaler
        pub odd: bool,
        /// I2S Linear prescaler
        pub i2sdiv: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            mckoe: ((value >> 9) & 0b1) > 0,
            odd: ((value >> 8) & 0b1) > 0,
            i2sdiv: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            mckoe: ((value >> 9) & 0b1) > 0,
            odd: ((value >> 8) & 0b1) > 0,
            i2sdiv: ((value >> 0) & 0b1) > 0,
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
                | ((self.mckoe as u32) << 9)
                | ((self.odd as u32) << 8)
                | ((self.i2sdiv as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// SPI3 global interrupt
pub const INTERRUPT_SPI3: u32 = 51;
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema" derivedFrom="SPI1">
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0x40003C00</baseAddress>
  <description>Serial peripheral interface</description>
  <groupName>SPI</groupName>
  <interrupt>
    <description>SPI3 global interrupt</description>
    <name>SPI3</name>
    <value>51</value>
  </interrupt>
  <name>SPI3</name>
  <registers>
    <register>
      <access>read-write</access>
      <addressOffset>0x0</addressOffset>
      <description>control register 1</description>
      <displayName>CR1</displayName>
      <fields>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Bidirectional data mode
              enable</description>
          <name>BIDIMODE</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Output enable in bidirectional
              mode</description>
          <name>BIDIOE</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Hardware CRC calculation
              enable</description>
          <name>CRCEN</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>CRC transfer next</description>
          <name>CRCNEXT</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Data frame format</description>
          <name>DFF</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Receive only</description>
          <name>RXONLY</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Software slave management</description>
          <name>SSM</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Internal slave select</description>
          <name>SSI</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Frame format</description>
          <name>LSBFIRST</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>SPI enable</description>
          <name>SPE</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Baud rate control</description>
          <name>BR</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Master selection</description>
          <name>MSTR</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Clock polarity</description>
          <name>CPOL</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Clock phase</description>
          <name>CPHA</name>
        </field>
      </fields>
      <name>CR1</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x4</addressOffset>
      <description>control register 2</description>
      <displayName>CR2</displayName>
      <fields>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Tx buffer empty interrupt
              enable</description>
          <name>TXEIE</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RX buffer not empty interrupt
              enable</description>
          <name>RXNEIE</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Error interrupt enable</description>
          <name>ERRIE</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>SS output enable</description>
          <name>SSOE</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Tx buffer DMA enable</description>
          <name>TXDMAEN</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Rx buffer DMA enable</description>
          <name>RXDMAEN</name>
        </field>
      </fields>
      <name>CR2</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0x8</addressOffset>
      <description>status register</description>
      <displayName>SR</displayName>
      <fields>
        <field>
          <access>read-only</access>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Busy flag</description>
          <name>BSY</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Overrun flag</description>
          <name>OVR</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Mode fault</description>
          <name>MODF</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>CRC error flag</description>
          <name>CRCERR</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Underrun flag</description>
          <name>UDR</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Channel side</description>
          <name>CHSIDE</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Transmit buffer empty</description>
          <name>TXE</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Receive buffer not empty</description>
          <name>RXNE</name>
        </field>
      </fields>
      <name>SR</name>
      <resetValue>0x0002</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xC</addressOffset>
      <description>data register</description>
      <displayName>DR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Data register</description>
          <name>DR</name>
        </field>
      </fields>
      <name>DR</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x10</addressOffset>
      <description>CRC polynomial register</description>
      <displayName>CRCPR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>CRC polynomial register</description>
          <name>CRCPOLY</name>
        </field>
      </fields>
      <name>CRCPR</name>
      <resetValue>0x0007</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x14</addressOffset>
      <description>RX CRC register</description>
      <displayName>RXCRCR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Rx CRC register</description>
          <name>RxCRC</name>
        </field>
      </fields>
      <name>RXCRCR</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x18</addressOffset>
      <description>TX CRC register</description>
      <displayName>TXCRCR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>16</bitWidth>
          <description>Tx CRC register</description>
          <name>TxCRC</name>
        </field>
      </fields>
      <name>TXCRCR</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x1C</addressOffset>
      <description>I2S configuration register</description>
      <displayName>I2SCFGR</displayName>
      <fields>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>I2S mode selection</description>
          <name>I2SMOD</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>I2S Enable</description>
          <name>I2SE</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
          <description>I2S configuration mode</description>
          <name>I2SCFG</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>PCM frame synchronization</description>
          <name>PCMSYNC</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
          <description>I2S standard selection</description>
          <name>I2SSTD</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Steady state clock
              polarity</description>
          <name>CKPOL</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Data length to be
              transferred</description>
          <name>DATLEN</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Channel length (number of bits per audio
              channel)</description>
          <name>CHLEN</name>
        </field>
      </fields>
      <name>I2SCFGR</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x20</addressOffset>
      <description>I2S prescaler register</description>
      <displayName>I2SPR</displayName>
      <fields>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Master clock output enable</description>
          <name>MCKOE</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Odd factor for the
              prescaler</description>
          <name>ODD</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>I2S Linear prescaler</description>
          <name>I2SDIV</name>
        </field>
      </fields>
      <name>I2SPR</name>
      <resetValue>00000010</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
