pub const ADDRESS: u32 = 0x40004400;
/// Status register
pub mod sr {
    pub const OFFSET: u32 = 0x0;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// CTS flag
    /// Access: read-write, Width: 1, Offset: 9
    /// Set CTS flag
    pub fn set_cts(value: bool) {
        let value = value as u32;
        let value = value << 9;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get CTS flag
    pub fn get_cts() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 9);
        value > 0
    }
    /// LIN break detection flag
    /// Access: read-write, Width: 1, Offset: 8
    /// Set LIN break detection flag
    pub fn set_lbd(value: bool) {
        let value = value as u32;
        let value = value << 8;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get LIN break detection flag
    pub fn get_lbd() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 8);
        value > 0
    }
    /// Transmit data register empty
    /// Access: read-only, Width: 1, Offset: 7
    /// Get Transmit data register empty
    pub fn txe() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 7);
        value > 0
    }
    /// Transmission complete
    /// Access: read-write, Width: 1, Offset: 6
    /// Set Transmission complete
    pub fn set_tc(value: bool) {
        let value = value as u32;
        let value = value << 6;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get Transmission complete
    pub fn get_tc() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 6);
        value > 0
    }
    /// Read data register not empty
    /// Access: read-write, Width: 1, Offset: 5
    /// Set Read data register not empty
    pub fn set_rxne(value: bool) {
        let value = value as u32;
        let value = value << 5;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get Read data register not empty
    pub fn get_rxne() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 5);
        value > 0
    }
    /// IDLE line detected
    /// Access: read-only, Width: 1, Offset: 4
    /// Get IDLE line detected
    pub fn idle() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 4);
        value > 0
    }
    /// Overrun error
    /// Access: read-only, Width: 1, Offset: 3
    /// Get Overrun error
    pub fn ore() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 3);
        value > 0
    }
    /// Noise error flag
    /// Access: read-only, Width: 1, Offset: 2
    /// Get Noise error flag
    pub fn ne() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 2);
        value > 0
    }
    /// Framing error
    /// Access: read-only, Width: 1, Offset: 1
    /// Get Framing error
    pub fn fe() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 1);
        value > 0
    }
    /// Parity error
    /// Access: read-only, Width: 1, Offset: 0
    /// Get Parity error
    pub fn pe() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
}
/// Data register
pub mod dr {
    pub const OFFSET: u32 = 0x4;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Data value
        pub dr: u16,
    }
    pub struct Cache {
        /// Data value
        pub dr: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            dr: ((value >> 0) & 0b111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            dr: ((value >> 0) & 0b111111111) as u16,
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
/// Baud rate register
pub mod brr {
    pub const OFFSET: u32 = 0x8;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// mantissa of USARTDIV
        pub div_mantissa: u16,
        /// fraction of USARTDIV
        pub div_fraction: u16,
    }
    pub struct Cache {
        /// mantissa of USARTDIV
        pub div_mantissa: u16,
        /// fraction of USARTDIV
        pub div_fraction: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            div_mantissa: ((value >> 4) & 0b111111111111) as u16,
            div_fraction: ((value >> 0) & 0b111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            div_mantissa: ((value >> 4) & 0b111111111111) as u16,
            div_fraction: ((value >> 0) & 0b111111111111) as u16,
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
                | ((self.div_mantissa as u32) << 4)
                | ((self.div_fraction as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Control register 1
pub mod cr1 {
    pub const OFFSET: u32 = 0xC;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// USART enable
        pub ue: bool,
        /// Word length
        pub m: bool,
        /// Wakeup method
        pub wake: bool,
        /// Parity control enable
        pub pce: bool,
        /// Parity selection
        pub ps: bool,
        /// PE interrupt enable
        pub peie: bool,
        /// TXE interrupt enable
        pub txeie: bool,
        /// Transmission complete interrupt enable
        pub tcie: bool,
        /// RXNE interrupt enable
        pub rxneie: bool,
        /// IDLE interrupt enable
        pub idleie: bool,
        /// Transmitter enable
        pub te: bool,
        /// Receiver enable
        pub re: bool,
        /// Receiver wakeup
        pub rwu: bool,
        /// Send break
        pub sbk: bool,
    }
    pub struct Cache {
        /// USART enable
        pub ue: bool,
        /// Word length
        pub m: bool,
        /// Wakeup method
        pub wake: bool,
        /// Parity control enable
        pub pce: bool,
        /// Parity selection
        pub ps: bool,
        /// PE interrupt enable
        pub peie: bool,
        /// TXE interrupt enable
        pub txeie: bool,
        /// Transmission complete interrupt enable
        pub tcie: bool,
        /// RXNE interrupt enable
        pub rxneie: bool,
        /// IDLE interrupt enable
        pub idleie: bool,
        /// Transmitter enable
        pub te: bool,
        /// Receiver enable
        pub re: bool,
        /// Receiver wakeup
        pub rwu: bool,
        /// Send break
        pub sbk: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            ue: ((value >> 13) & 0b1) > 0,
            m: ((value >> 12) & 0b1) > 0,
            wake: ((value >> 11) & 0b1) > 0,
            pce: ((value >> 10) & 0b1) > 0,
            ps: ((value >> 9) & 0b1) > 0,
            peie: ((value >> 8) & 0b1) > 0,
            txeie: ((value >> 7) & 0b1) > 0,
            tcie: ((value >> 6) & 0b1) > 0,
            rxneie: ((value >> 5) & 0b1) > 0,
            idleie: ((value >> 4) & 0b1) > 0,
            te: ((value >> 3) & 0b1) > 0,
            re: ((value >> 2) & 0b1) > 0,
            rwu: ((value >> 1) & 0b1) > 0,
            sbk: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            ue: ((value >> 13) & 0b1) > 0,
            m: ((value >> 12) & 0b1) > 0,
            wake: ((value >> 11) & 0b1) > 0,
            pce: ((value >> 10) & 0b1) > 0,
            ps: ((value >> 9) & 0b1) > 0,
            peie: ((value >> 8) & 0b1) > 0,
            txeie: ((value >> 7) & 0b1) > 0,
            tcie: ((value >> 6) & 0b1) > 0,
            rxneie: ((value >> 5) & 0b1) > 0,
            idleie: ((value >> 4) & 0b1) > 0,
            te: ((value >> 3) & 0b1) > 0,
            re: ((value >> 2) & 0b1) > 0,
            rwu: ((value >> 1) & 0b1) > 0,
            sbk: ((value >> 0) & 0b1) > 0,
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
                | ((self.ue as u32) << 13)
                | ((self.m as u32) << 12)
                | ((self.wake as u32) << 11)
                | ((self.pce as u32) << 10)
                | ((self.ps as u32) << 9)
                | ((self.peie as u32) << 8)
                | ((self.txeie as u32) << 7)
                | ((self.tcie as u32) << 6)
                | ((self.rxneie as u32) << 5)
                | ((self.idleie as u32) << 4)
                | ((self.te as u32) << 3)
                | ((self.re as u32) << 2)
                | ((self.rwu as u32) << 1)
                | ((self.sbk as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Control register 2
pub mod cr2 {
    pub const OFFSET: u32 = 0x10;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// LIN mode enable
        pub linen: bool,
        /// STOP bits
        pub stop: bool,
        /// Clock enable
        pub clken: bool,
        /// Clock polarity
        pub cpol: bool,
        /// Clock phase
        pub cpha: bool,
        /// Last bit clock pulse
        pub lbcl: bool,
        /// LIN break detection interrupt enable
        pub lbdie: bool,
        /// lin break detection length
        pub lbdl: bool,
        /// Address of the USART node
        pub add: bool,
    }
    pub struct Cache {
        /// LIN mode enable
        pub linen: bool,
        /// STOP bits
        pub stop: bool,
        /// Clock enable
        pub clken: bool,
        /// Clock polarity
        pub cpol: bool,
        /// Clock phase
        pub cpha: bool,
        /// Last bit clock pulse
        pub lbcl: bool,
        /// LIN break detection interrupt enable
        pub lbdie: bool,
        /// lin break detection length
        pub lbdl: bool,
        /// Address of the USART node
        pub add: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            linen: ((value >> 14) & 0b1) > 0,
            stop: ((value >> 12) & 0b1) > 0,
            clken: ((value >> 11) & 0b1) > 0,
            cpol: ((value >> 10) & 0b1) > 0,
            cpha: ((value >> 9) & 0b1) > 0,
            lbcl: ((value >> 8) & 0b1) > 0,
            lbdie: ((value >> 6) & 0b1) > 0,
            lbdl: ((value >> 5) & 0b1) > 0,
            add: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            linen: ((value >> 14) & 0b1) > 0,
            stop: ((value >> 12) & 0b1) > 0,
            clken: ((value >> 11) & 0b1) > 0,
            cpol: ((value >> 10) & 0b1) > 0,
            cpha: ((value >> 9) & 0b1) > 0,
            lbcl: ((value >> 8) & 0b1) > 0,
            lbdie: ((value >> 6) & 0b1) > 0,
            lbdl: ((value >> 5) & 0b1) > 0,
            add: ((value >> 0) & 0b1) > 0,
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
                | ((self.linen as u32) << 14)
                | ((self.stop as u32) << 12)
                | ((self.clken as u32) << 11)
                | ((self.cpol as u32) << 10)
                | ((self.cpha as u32) << 9)
                | ((self.lbcl as u32) << 8)
                | ((self.lbdie as u32) << 6)
                | ((self.lbdl as u32) << 5)
                | ((self.add as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Control register 3
pub mod cr3 {
    pub const OFFSET: u32 = 0x14;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// CTS interrupt enable
        pub ctsie: bool,
        /// CTS enable
        pub ctse: bool,
        /// RTS enable
        pub rtse: bool,
        /// DMA enable transmitter
        pub dmat: bool,
        /// DMA enable receiver
        pub dmar: bool,
        /// Smartcard mode enable
        pub scen: bool,
        /// Smartcard NACK enable
        pub nack: bool,
        /// Half-duplex selection
        pub hdsel: bool,
        /// IrDA low-power
        pub irlp: bool,
        /// IrDA mode enable
        pub iren: bool,
        /// Error interrupt enable
        pub eie: bool,
    }
    pub struct Cache {
        /// CTS interrupt enable
        pub ctsie: bool,
        /// CTS enable
        pub ctse: bool,
        /// RTS enable
        pub rtse: bool,
        /// DMA enable transmitter
        pub dmat: bool,
        /// DMA enable receiver
        pub dmar: bool,
        /// Smartcard mode enable
        pub scen: bool,
        /// Smartcard NACK enable
        pub nack: bool,
        /// Half-duplex selection
        pub hdsel: bool,
        /// IrDA low-power
        pub irlp: bool,
        /// IrDA mode enable
        pub iren: bool,
        /// Error interrupt enable
        pub eie: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            ctsie: ((value >> 10) & 0b1) > 0,
            ctse: ((value >> 9) & 0b1) > 0,
            rtse: ((value >> 8) & 0b1) > 0,
            dmat: ((value >> 7) & 0b1) > 0,
            dmar: ((value >> 6) & 0b1) > 0,
            scen: ((value >> 5) & 0b1) > 0,
            nack: ((value >> 4) & 0b1) > 0,
            hdsel: ((value >> 3) & 0b1) > 0,
            irlp: ((value >> 2) & 0b1) > 0,
            iren: ((value >> 1) & 0b1) > 0,
            eie: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            ctsie: ((value >> 10) & 0b1) > 0,
            ctse: ((value >> 9) & 0b1) > 0,
            rtse: ((value >> 8) & 0b1) > 0,
            dmat: ((value >> 7) & 0b1) > 0,
            dmar: ((value >> 6) & 0b1) > 0,
            scen: ((value >> 5) & 0b1) > 0,
            nack: ((value >> 4) & 0b1) > 0,
            hdsel: ((value >> 3) & 0b1) > 0,
            irlp: ((value >> 2) & 0b1) > 0,
            iren: ((value >> 1) & 0b1) > 0,
            eie: ((value >> 0) & 0b1) > 0,
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
                | ((self.ctsie as u32) << 10)
                | ((self.ctse as u32) << 9)
                | ((self.rtse as u32) << 8)
                | ((self.dmat as u32) << 7)
                | ((self.dmar as u32) << 6)
                | ((self.scen as u32) << 5)
                | ((self.nack as u32) << 4)
                | ((self.hdsel as u32) << 3)
                | ((self.irlp as u32) << 2)
                | ((self.iren as u32) << 1)
                | ((self.eie as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Guard time and prescaler register
pub mod gtpr {
    pub const OFFSET: u32 = 0x18;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Guard time value
        pub gt: u8,
        /// Prescaler value
        pub psc: u8,
    }
    pub struct Cache {
        /// Guard time value
        pub gt: u8,
        /// Prescaler value
        pub psc: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            gt: ((value >> 8) & 0b11111111) as u8,
            psc: ((value >> 0) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            gt: ((value >> 8) & 0b11111111) as u8,
            psc: ((value >> 0) & 0b11111111) as u8,
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
                | ((self.gt as u32) << 8)
                | ((self.psc as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// USART2 global interrupt
pub const INTERRUPT_USART2: u32 = 38;
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema" derivedFrom="USART1">
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0x40004400</baseAddress>
  <description>Universal synchronous asynchronous receiver
      transmitter</description>
  <groupName>USART</groupName>
  <interrupt>
    <description>USART2 global interrupt</description>
    <name>USART2</name>
    <value>38</value>
  </interrupt>
  <name>USART2</name>
  <registers>
    <register>
      <addressOffset>0x0</addressOffset>
      <description>Status register</description>
      <displayName>SR</displayName>
      <fields>
        <field>
          <access>read-write</access>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>CTS flag</description>
          <name>CTS</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>LIN break detection flag</description>
          <name>LBD</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Transmit data register
              empty</description>
          <name>TXE</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Transmission complete</description>
          <name>TC</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Read data register not
              empty</description>
          <name>RXNE</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>IDLE line detected</description>
          <name>IDLE</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Overrun error</description>
          <name>ORE</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Noise error flag</description>
          <name>NE</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Framing error</description>
          <name>FE</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Parity error</description>
          <name>PE</name>
        </field>
      </fields>
      <name>SR</name>
      <resetValue>0x00C0</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x4</addressOffset>
      <description>Data register</description>
      <displayName>DR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>9</bitWidth>
          <description>Data value</description>
          <name>DR</name>
        </field>
      </fields>
      <name>DR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x8</addressOffset>
      <description>Baud rate register</description>
      <displayName>BRR</displayName>
      <fields>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>12</bitWidth>
          <description>mantissa of USARTDIV</description>
          <name>DIV_Mantissa</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>fraction of USARTDIV</description>
          <name>DIV_Fraction</name>
        </field>
      </fields>
      <name>BRR</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xC</addressOffset>
      <description>Control register 1</description>
      <displayName>CR1</displayName>
      <fields>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>USART enable</description>
          <name>UE</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Word length</description>
          <name>M</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Wakeup method</description>
          <name>WAKE</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Parity control enable</description>
          <name>PCE</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Parity selection</description>
          <name>PS</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>PE interrupt enable</description>
          <name>PEIE</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TXE interrupt enable</description>
          <name>TXEIE</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Transmission complete interrupt
              enable</description>
          <name>TCIE</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RXNE interrupt enable</description>
          <name>RXNEIE</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>IDLE interrupt enable</description>
          <name>IDLEIE</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Transmitter enable</description>
          <name>TE</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Receiver enable</description>
          <name>RE</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Receiver wakeup</description>
          <name>RWU</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Send break</description>
          <name>SBK</name>
        </field>
      </fields>
      <name>CR1</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x10</addressOffset>
      <description>Control register 2</description>
      <displayName>CR2</displayName>
      <fields>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>LIN mode enable</description>
          <name>LINEN</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <description>STOP bits</description>
          <name>STOP</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Clock enable</description>
          <name>CLKEN</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Clock polarity</description>
          <name>CPOL</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Clock phase</description>
          <name>CPHA</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Last bit clock pulse</description>
          <name>LBCL</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>LIN break detection interrupt
              enable</description>
          <name>LBDIE</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>lin break detection length</description>
          <name>LBDL</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Address of the USART node</description>
          <name>ADD</name>
        </field>
      </fields>
      <name>CR2</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x14</addressOffset>
      <description>Control register 3</description>
      <displayName>CR3</displayName>
      <fields>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>CTS interrupt enable</description>
          <name>CTSIE</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>CTS enable</description>
          <name>CTSE</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RTS enable</description>
          <name>RTSE</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DMA enable transmitter</description>
          <name>DMAT</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DMA enable receiver</description>
          <name>DMAR</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Smartcard mode enable</description>
          <name>SCEN</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Smartcard NACK enable</description>
          <name>NACK</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Half-duplex selection</description>
          <name>HDSEL</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>IrDA low-power</description>
          <name>IRLP</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>IrDA mode enable</description>
          <name>IREN</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Error interrupt enable</description>
          <name>EIE</name>
        </field>
      </fields>
      <name>CR3</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x18</addressOffset>
      <description>Guard time and prescaler
          register</description>
      <displayName>GTPR</displayName>
      <fields>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>Guard time value</description>
          <name>GT</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>Prescaler value</description>
          <name>PSC</name>
        </field>
      </fields>
      <name>GTPR</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
