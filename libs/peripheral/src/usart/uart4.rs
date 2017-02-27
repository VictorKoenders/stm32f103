pub const ADDRESS: u32 = 0x40004C00;
/// UART4_SR
pub mod sr {
    pub const OFFSET: u32 = 0x0;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// Parity error
    /// Access: read-only, Width: 1, Offset: 0
    /// Get Parity error
    pub fn pe() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 0);
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
    /// Noise error flag
    /// Access: read-only, Width: 1, Offset: 2
    /// Get Noise error flag
    pub fn ne() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 2);
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
    /// IDLE line detected
    /// Access: read-only, Width: 1, Offset: 4
    /// Get IDLE line detected
    pub fn idle() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 4);
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
    /// Transmit data register empty
    /// Access: read-only, Width: 1, Offset: 7
    /// Get Transmit data register empty
    pub fn txe() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 7);
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
}
/// UART4_DR
pub mod dr {
    pub const OFFSET: u32 = 0x4;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// DR
        pub dr: u16,
    }
    pub struct Cache {
        /// DR
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
/// UART4_BRR
pub mod brr {
    pub const OFFSET: u32 = 0x8;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// DIV_Fraction
        pub div_fraction: u8,
        /// DIV_Mantissa
        pub div_mantissa: u8,
    }
    pub struct Cache {
        /// DIV_Fraction
        pub div_fraction: u8,
        /// DIV_Mantissa
        pub div_mantissa: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            div_fraction: ((value >> 0) & 0b1111) as u8,
            div_mantissa: ((value >> 4) & 0b1111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            div_fraction: ((value >> 0) & 0b1111) as u8,
            div_mantissa: ((value >> 4) & 0b1111) as u8,
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
                | ((self.div_fraction as u32) << 0)
                | ((self.div_mantissa as u32) << 4)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// UART4_CR1
pub mod cr1 {
    pub const OFFSET: u32 = 0xC;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Send break
        pub sbk: bool,
        /// Receiver wakeup
        pub rwu: bool,
        /// Receiver enable
        pub re: bool,
        /// Transmitter enable
        pub te: bool,
        /// IDLE interrupt enable
        pub idleie: bool,
        /// RXNE interrupt enable
        pub rxneie: bool,
        /// Transmission complete interrupt enable
        pub tcie: bool,
        /// TXE interrupt enable
        pub txeie: bool,
        /// PE interrupt enable
        pub peie: bool,
        /// Parity selection
        pub ps: bool,
        /// Parity control enable
        pub pce: bool,
        /// Wakeup method
        pub wake: bool,
        /// Word length
        pub m: bool,
        /// USART enable
        pub ue: bool,
    }
    pub struct Cache {
        /// Send break
        pub sbk: bool,
        /// Receiver wakeup
        pub rwu: bool,
        /// Receiver enable
        pub re: bool,
        /// Transmitter enable
        pub te: bool,
        /// IDLE interrupt enable
        pub idleie: bool,
        /// RXNE interrupt enable
        pub rxneie: bool,
        /// Transmission complete interrupt enable
        pub tcie: bool,
        /// TXE interrupt enable
        pub txeie: bool,
        /// PE interrupt enable
        pub peie: bool,
        /// Parity selection
        pub ps: bool,
        /// Parity control enable
        pub pce: bool,
        /// Wakeup method
        pub wake: bool,
        /// Word length
        pub m: bool,
        /// USART enable
        pub ue: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            sbk: ((value >> 0) & 0b1) > 0,
            rwu: ((value >> 1) & 0b1) > 0,
            re: ((value >> 2) & 0b1) > 0,
            te: ((value >> 3) & 0b1) > 0,
            idleie: ((value >> 4) & 0b1) > 0,
            rxneie: ((value >> 5) & 0b1) > 0,
            tcie: ((value >> 6) & 0b1) > 0,
            txeie: ((value >> 7) & 0b1) > 0,
            peie: ((value >> 8) & 0b1) > 0,
            ps: ((value >> 9) & 0b1) > 0,
            pce: ((value >> 10) & 0b1) > 0,
            wake: ((value >> 11) & 0b1) > 0,
            m: ((value >> 12) & 0b1) > 0,
            ue: ((value >> 13) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            sbk: ((value >> 0) & 0b1) > 0,
            rwu: ((value >> 1) & 0b1) > 0,
            re: ((value >> 2) & 0b1) > 0,
            te: ((value >> 3) & 0b1) > 0,
            idleie: ((value >> 4) & 0b1) > 0,
            rxneie: ((value >> 5) & 0b1) > 0,
            tcie: ((value >> 6) & 0b1) > 0,
            txeie: ((value >> 7) & 0b1) > 0,
            peie: ((value >> 8) & 0b1) > 0,
            ps: ((value >> 9) & 0b1) > 0,
            pce: ((value >> 10) & 0b1) > 0,
            wake: ((value >> 11) & 0b1) > 0,
            m: ((value >> 12) & 0b1) > 0,
            ue: ((value >> 13) & 0b1) > 0,
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
                | ((self.sbk as u32) << 0)
                | ((self.rwu as u32) << 1)
                | ((self.re as u32) << 2)
                | ((self.te as u32) << 3)
                | ((self.idleie as u32) << 4)
                | ((self.rxneie as u32) << 5)
                | ((self.tcie as u32) << 6)
                | ((self.txeie as u32) << 7)
                | ((self.peie as u32) << 8)
                | ((self.ps as u32) << 9)
                | ((self.pce as u32) << 10)
                | ((self.wake as u32) << 11)
                | ((self.m as u32) << 12)
                | ((self.ue as u32) << 13)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// UART4_CR2
pub mod cr2 {
    pub const OFFSET: u32 = 0x10;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Address of the USART node
        pub add: u8,
        /// lin break detection length
        pub lbdl: u8,
        /// LIN break detection interrupt enable
        pub lbdie: u8,
        /// STOP bits
        pub stop: u8,
        /// LIN mode enable
        pub linen: u8,
    }
    pub struct Cache {
        /// Address of the USART node
        pub add: u8,
        /// lin break detection length
        pub lbdl: u8,
        /// LIN break detection interrupt enable
        pub lbdie: u8,
        /// STOP bits
        pub stop: u8,
        /// LIN mode enable
        pub linen: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            add: ((value >> 0) & 0b1111) as u8,
            lbdl: ((value >> 5) & 0b1111) as u8,
            lbdie: ((value >> 6) & 0b1111) as u8,
            stop: ((value >> 12) & 0b1111) as u8,
            linen: ((value >> 14) & 0b1111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            add: ((value >> 0) & 0b1111) as u8,
            lbdl: ((value >> 5) & 0b1111) as u8,
            lbdie: ((value >> 6) & 0b1111) as u8,
            stop: ((value >> 12) & 0b1111) as u8,
            linen: ((value >> 14) & 0b1111) as u8,
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
                | ((self.add as u32) << 0)
                | ((self.lbdl as u32) << 5)
                | ((self.lbdie as u32) << 6)
                | ((self.stop as u32) << 12)
                | ((self.linen as u32) << 14)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// UART4_CR3
pub mod cr3 {
    pub const OFFSET: u32 = 0x14;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Error interrupt enable
        pub eie: bool,
        /// IrDA mode enable
        pub iren: bool,
        /// IrDA low-power
        pub irlp: bool,
        /// Half-duplex selection
        pub hdsel: bool,
        /// DMA enable receiver
        pub dmar: bool,
        /// DMA enable transmitter
        pub dmat: bool,
    }
    pub struct Cache {
        /// Error interrupt enable
        pub eie: bool,
        /// IrDA mode enable
        pub iren: bool,
        /// IrDA low-power
        pub irlp: bool,
        /// Half-duplex selection
        pub hdsel: bool,
        /// DMA enable receiver
        pub dmar: bool,
        /// DMA enable transmitter
        pub dmat: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            eie: ((value >> 0) & 0b1) > 0,
            iren: ((value >> 1) & 0b1) > 0,
            irlp: ((value >> 2) & 0b1) > 0,
            hdsel: ((value >> 3) & 0b1) > 0,
            dmar: ((value >> 6) & 0b1) > 0,
            dmat: ((value >> 7) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            eie: ((value >> 0) & 0b1) > 0,
            iren: ((value >> 1) & 0b1) > 0,
            irlp: ((value >> 2) & 0b1) > 0,
            hdsel: ((value >> 3) & 0b1) > 0,
            dmar: ((value >> 6) & 0b1) > 0,
            dmat: ((value >> 7) & 0b1) > 0,
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
                | ((self.eie as u32) << 0)
                | ((self.iren as u32) << 1)
                | ((self.irlp as u32) << 2)
                | ((self.hdsel as u32) << 3)
                | ((self.dmar as u32) << 6)
                | ((self.dmat as u32) << 7)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// UART4 global interrupt
pub const INTERRUPT_UART4: u32 = 52;
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0x40004C00</baseAddress>
  <description>Universal asynchronous receiver
      transmitter</description>
  <groupName>USART</groupName>
  <interrupt>
    <description>UART4 global interrupt</description>
    <name>UART4</name>
    <value>52</value>
  </interrupt>
  <name>UART4</name>
  <registers>
    <register>
      <addressOffset>0x0</addressOffset>
      <description>UART4_SR</description>
      <displayName>SR</displayName>
      <fields>
        <field>
          <access>read-only</access>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Parity error</description>
          <name>PE</name>
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
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Noise error flag</description>
          <name>NE</name>
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
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>IDLE line detected</description>
          <name>IDLE</name>
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
          <access>read-write</access>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Transmission complete</description>
          <name>TC</name>
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
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>LIN break detection flag</description>
          <name>LBD</name>
        </field>
      </fields>
      <name>SR</name>
      <resetValue>0x0</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x4</addressOffset>
      <description>UART4_DR</description>
      <displayName>DR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>9</bitWidth>
          <description>DR</description>
          <name>DR</name>
        </field>
      </fields>
      <name>DR</name>
      <resetValue>0x0</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x8</addressOffset>
      <description>UART4_BRR</description>
      <displayName>BRR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>DIV_Fraction</description>
          <name>DIV_Fraction</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>12</bitWidth>
          <description>DIV_Mantissa</description>
          <name>DIV_Mantissa</name>
        </field>
      </fields>
      <name>BRR</name>
      <resetValue>0x0</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xC</addressOffset>
      <description>UART4_CR1</description>
      <displayName>CR1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Send break</description>
          <name>SBK</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Receiver wakeup</description>
          <name>RWU</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Receiver enable</description>
          <name>RE</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Transmitter enable</description>
          <name>TE</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>IDLE interrupt enable</description>
          <name>IDLEIE</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RXNE interrupt enable</description>
          <name>RXNEIE</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Transmission complete interrupt
              enable</description>
          <name>TCIE</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TXE interrupt enable</description>
          <name>TXEIE</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>PE interrupt enable</description>
          <name>PEIE</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Parity selection</description>
          <name>PS</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Parity control enable</description>
          <name>PCE</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Wakeup method</description>
          <name>WAKE</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Word length</description>
          <name>M</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>USART enable</description>
          <name>UE</name>
        </field>
      </fields>
      <name>CR1</name>
      <resetValue>0x0</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x10</addressOffset>
      <description>UART4_CR2</description>
      <displayName>CR2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Address of the USART node</description>
          <name>ADD</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>lin break detection length</description>
          <name>LBDL</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>LIN break detection interrupt
              enable</description>
          <name>LBDIE</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <description>STOP bits</description>
          <name>STOP</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>LIN mode enable</description>
          <name>LINEN</name>
        </field>
      </fields>
      <name>CR2</name>
      <resetValue>0x0</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x14</addressOffset>
      <description>UART4_CR3</description>
      <displayName>CR3</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Error interrupt enable</description>
          <name>EIE</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>IrDA mode enable</description>
          <name>IREN</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>IrDA low-power</description>
          <name>IRLP</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Half-duplex selection</description>
          <name>HDSEL</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DMA enable receiver</description>
          <name>DMAR</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DMA enable transmitter</description>
          <name>DMAT</name>
        </field>
      </fields>
      <name>CR3</name>
      <resetValue>0x0</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
