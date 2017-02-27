pub const ADDRESS: u32 = 0x40010000;
/// Event Control Register (AFIO_EVCR)
pub mod evcr {
    pub const OFFSET: u32 = 0x0;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Pin selection
        pub pin: u8,
        /// Port selection
        pub port: u8,
        /// Event Output Enable
        pub evoe: u8,
    }
    pub struct Cache {
        /// Pin selection
        pub pin: u8,
        /// Port selection
        pub port: u8,
        /// Event Output Enable
        pub evoe: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            pin: ((value >> 0) & 0b1111) as u8,
            port: ((value >> 4) & 0b1111) as u8,
            evoe: ((value >> 7) & 0b1111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            pin: ((value >> 0) & 0b1111) as u8,
            port: ((value >> 4) & 0b1111) as u8,
            evoe: ((value >> 7) & 0b1111) as u8,
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
                | ((self.pin as u32) << 0)
                | ((self.port as u32) << 4)
                | ((self.evoe as u32) << 7)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// AF remap and debug I/O configuration register (AFIO_MAPR)
pub mod mapr {
    pub const OFFSET: u32 = 0x4;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// SPI1 remapping
    /// Access: read-write, Width: 1, Offset: 0
    /// Set SPI1 remapping
    pub fn set_spi1_remap(value: bool) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get SPI1 remapping
    pub fn get_spi1_remap() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
    /// I2C1 remapping
    /// Access: read-write, Width: 1, Offset: 1
    /// Set I2C1 remapping
    pub fn set_i2c1_remap(value: bool) {
        let value = value as u32;
        let value = value << 1;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get I2C1 remapping
    pub fn get_i2c1_remap() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 1);
        value > 0
    }
    /// USART1 remapping
    /// Access: read-write, Width: 1, Offset: 2
    /// Set USART1 remapping
    pub fn set_usart1_remap(value: bool) {
        let value = value as u32;
        let value = value << 2;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get USART1 remapping
    pub fn get_usart1_remap() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 2);
        value > 0
    }
    /// USART2 remapping
    /// Access: read-write, Width: 1, Offset: 3
    /// Set USART2 remapping
    pub fn set_usart2_remap(value: bool) {
        let value = value as u32;
        let value = value << 3;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get USART2 remapping
    pub fn get_usart2_remap() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 3);
        value > 0
    }
    /// USART3 remapping
    /// Access: read-write, Width: 2, Offset: 4
    /// Set USART3 remapping
    pub fn set_usart3_remap(value: u8) {
        debug_assert!(value <= 0b11, "set_usart3_remap out of range");
        let value = value as u32;
        let value = value << 4;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get USART3 remapping
    pub fn get_usart3_remap() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b11 << 4);
        value as u8
    }
    /// TIM1 remapping
    /// Access: read-write, Width: 2, Offset: 6
    /// Set TIM1 remapping
    pub fn set_tim1_remap(value: u8) {
        debug_assert!(value <= 0b11, "set_tim1_remap out of range");
        let value = value as u32;
        let value = value << 6;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get TIM1 remapping
    pub fn get_tim1_remap() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b11 << 6);
        value as u8
    }
    /// TIM2 remapping
    /// Access: read-write, Width: 2, Offset: 8
    /// Set TIM2 remapping
    pub fn set_tim2_remap(value: u8) {
        debug_assert!(value <= 0b11, "set_tim2_remap out of range");
        let value = value as u32;
        let value = value << 8;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get TIM2 remapping
    pub fn get_tim2_remap() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b11 << 8);
        value as u8
    }
    /// TIM3 remapping
    /// Access: read-write, Width: 2, Offset: 10
    /// Set TIM3 remapping
    pub fn set_tim3_remap(value: u8) {
        debug_assert!(value <= 0b11, "set_tim3_remap out of range");
        let value = value as u32;
        let value = value << 10;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get TIM3 remapping
    pub fn get_tim3_remap() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b11 << 10);
        value as u8
    }
    /// TIM4 remapping
    /// Access: read-write, Width: 1, Offset: 12
    /// Set TIM4 remapping
    pub fn set_tim4_remap(value: bool) {
        let value = value as u32;
        let value = value << 12;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get TIM4 remapping
    pub fn get_tim4_remap() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 12);
        value > 0
    }
    /// CAN1 remapping
    /// Access: read-write, Width: 2, Offset: 13
    /// Set CAN1 remapping
    pub fn set_can_remap(value: u8) {
        debug_assert!(value <= 0b11, "set_can_remap out of range");
        let value = value as u32;
        let value = value << 13;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get CAN1 remapping
    pub fn get_can_remap() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b11 << 13);
        value as u8
    }
    /// Port D0/Port D1 mapping on OSCIN/OSCOUT
    /// Access: read-write, Width: 1, Offset: 15
    /// Set Port D0/Port D1 mapping on OSCIN/OSCOUT
    pub fn set_pd01_remap(value: bool) {
        let value = value as u32;
        let value = value << 15;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get Port D0/Port D1 mapping on OSCIN/OSCOUT
    pub fn get_pd01_remap() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 15);
        value > 0
    }
    /// Set and cleared by software
    /// Access: read-write, Width: 1, Offset: 16
    /// Set Set and cleared by software
    pub fn set_tim5ch4_iremap(value: bool) {
        let value = value as u32;
        let value = value << 16;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get Set and cleared by software
    pub fn get_tim5ch4_iremap() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 16);
        value > 0
    }
    /// ADC 1 External trigger injected conversion remapping
    /// Access: read-write, Width: 1, Offset: 17
    /// Set ADC 1 External trigger injected conversion remapping
    pub fn set_adc1_etrginj_remap(value: bool) {
        let value = value as u32;
        let value = value << 17;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get ADC 1 External trigger injected conversion remapping
    pub fn get_adc1_etrginj_remap() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 17);
        value > 0
    }
    /// ADC 1 external trigger regular conversion remapping
    /// Access: read-write, Width: 1, Offset: 18
    /// Set ADC 1 external trigger regular conversion remapping
    pub fn set_adc1_etrgreg_remap(value: bool) {
        let value = value as u32;
        let value = value << 18;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get ADC 1 external trigger regular conversion remapping
    pub fn get_adc1_etrgreg_remap() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 18);
        value > 0
    }
    /// ADC 2 external trigger injected conversion remapping
    /// Access: read-write, Width: 1, Offset: 19
    /// Set ADC 2 external trigger injected conversion remapping
    pub fn set_adc2_etrginj_remap(value: bool) {
        let value = value as u32;
        let value = value << 19;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get ADC 2 external trigger injected conversion remapping
    pub fn get_adc2_etrginj_remap() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 19);
        value > 0
    }
    /// ADC 2 external trigger regular conversion remapping
    /// Access: read-write, Width: 1, Offset: 20
    /// Set ADC 2 external trigger regular conversion remapping
    pub fn set_adc2_etrgreg_remap(value: bool) {
        let value = value as u32;
        let value = value << 20;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get ADC 2 external trigger regular conversion remapping
    pub fn get_adc2_etrgreg_remap() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 20);
        value > 0
    }
    /// Serial wire JTAG configuration
    /// Access: write-only, Width: 3, Offset: 24
    /// Set Serial wire JTAG configuration
    pub fn swj_cfg(value: u8) {
        debug_assert!(value <= 0b111, "swj_cfg out of range");
        let value = value as u32;
        let value = value << 24;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
}
/// External interrupt configuration register 1 (AFIO_EXTICR1)
pub mod exticr1 {
    pub const OFFSET: u32 = 0x8;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([u8;4]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = u8;
        fn index(&self, index: u8) -> &u8 {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut u8 {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([u8;4]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = u8;
        fn index(&self, index: u8) -> &u8 {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut u8 {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1111) as u8,
            ((value >> 4) & 0b1111) as u8,
            ((value >> 8) & 0b1111) as u8,
            ((value >> 12) & 0b1111) as u8,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1111) as u8,
            ((value >> 4) & 0b1111) as u8,
            ((value >> 8) & 0b1111) as u8,
            ((value >> 12) & 0b1111) as u8,
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
                | ((self.0[1] as u32) << 4)
                | ((self.0[2] as u32) << 8)
                | ((self.0[3] as u32) << 12)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// External interrupt configuration register 2 (AFIO_EXTICR2)
pub mod exticr2 {
    pub const OFFSET: u32 = 0xC;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([u8;4]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = u8;
        fn index(&self, index: u8) -> &u8 {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut u8 {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([u8;4]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = u8;
        fn index(&self, index: u8) -> &u8 {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut u8 {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1111) as u8,
            ((value >> 4) & 0b1111) as u8,
            ((value >> 8) & 0b1111) as u8,
            ((value >> 12) & 0b1111) as u8,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1111) as u8,
            ((value >> 4) & 0b1111) as u8,
            ((value >> 8) & 0b1111) as u8,
            ((value >> 12) & 0b1111) as u8,
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
                | ((self.0[1] as u32) << 4)
                | ((self.0[2] as u32) << 8)
                | ((self.0[3] as u32) << 12)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// External interrupt configuration register 3 (AFIO_EXTICR3)
pub mod exticr3 {
    pub const OFFSET: u32 = 0x10;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([u8;4]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = u8;
        fn index(&self, index: u8) -> &u8 {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut u8 {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([u8;4]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = u8;
        fn index(&self, index: u8) -> &u8 {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut u8 {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1111) as u8,
            ((value >> 4) & 0b1111) as u8,
            ((value >> 8) & 0b1111) as u8,
            ((value >> 12) & 0b1111) as u8,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1111) as u8,
            ((value >> 4) & 0b1111) as u8,
            ((value >> 8) & 0b1111) as u8,
            ((value >> 12) & 0b1111) as u8,
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
                | ((self.0[1] as u32) << 4)
                | ((self.0[2] as u32) << 8)
                | ((self.0[3] as u32) << 12)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// External interrupt configuration register 4 (AFIO_EXTICR4)
pub mod exticr4 {
    pub const OFFSET: u32 = 0x14;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([u8;4]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = u8;
        fn index(&self, index: u8) -> &u8 {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut u8 {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([u8;4]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = u8;
        fn index(&self, index: u8) -> &u8 {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut u8 {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1111) as u8,
            ((value >> 4) & 0b1111) as u8,
            ((value >> 8) & 0b1111) as u8,
            ((value >> 12) & 0b1111) as u8,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1111) as u8,
            ((value >> 4) & 0b1111) as u8,
            ((value >> 8) & 0b1111) as u8,
            ((value >> 12) & 0b1111) as u8,
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
                | ((self.0[1] as u32) << 4)
                | ((self.0[2] as u32) << 8)
                | ((self.0[3] as u32) << 12)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// AF remap and debug I/O configuration register
pub mod mapr2 {
    pub const OFFSET: u32 = 0x1C;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// TIM9 remapping
        pub tim9_remap: bool,
        /// TIM10 remapping
        pub tim10_remap: bool,
        /// TIM11 remapping
        pub tim11_remap: bool,
        /// TIM13 remapping
        pub tim13_remap: bool,
        /// TIM14 remapping
        pub tim14_remap: bool,
        /// NADV connect/disconnect
        pub fsmc_nadv: bool,
    }
    pub struct Cache {
        /// TIM9 remapping
        pub tim9_remap: bool,
        /// TIM10 remapping
        pub tim10_remap: bool,
        /// TIM11 remapping
        pub tim11_remap: bool,
        /// TIM13 remapping
        pub tim13_remap: bool,
        /// TIM14 remapping
        pub tim14_remap: bool,
        /// NADV connect/disconnect
        pub fsmc_nadv: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            tim9_remap: ((value >> 5) & 0b1) > 0,
            tim10_remap: ((value >> 6) & 0b1) > 0,
            tim11_remap: ((value >> 7) & 0b1) > 0,
            tim13_remap: ((value >> 8) & 0b1) > 0,
            tim14_remap: ((value >> 9) & 0b1) > 0,
            fsmc_nadv: ((value >> 10) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            tim9_remap: ((value >> 5) & 0b1) > 0,
            tim10_remap: ((value >> 6) & 0b1) > 0,
            tim11_remap: ((value >> 7) & 0b1) > 0,
            tim13_remap: ((value >> 8) & 0b1) > 0,
            tim14_remap: ((value >> 9) & 0b1) > 0,
            fsmc_nadv: ((value >> 10) & 0b1) > 0,
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
                | ((self.tim9_remap as u32) << 5)
                | ((self.tim10_remap as u32) << 6)
                | ((self.tim11_remap as u32) << 7)
                | ((self.tim13_remap as u32) << 8)
                | ((self.tim14_remap as u32) << 9)
                | ((self.fsmc_nadv as u32) << 10)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
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
  <baseAddress>0x40010000</baseAddress>
  <description>Alternate function I/O</description>
  <groupName>AFIO</groupName>
  <name>AFIO</name>
  <registers>
    <register>
      <access>read-write</access>
      <addressOffset>0x0</addressOffset>
      <description>Event Control Register
          (AFIO_EVCR)</description>
      <displayName>EVCR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Pin selection</description>
          <name>PIN</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Port selection</description>
          <name>PORT</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Event Output Enable</description>
          <name>EVOE</name>
        </field>
      </fields>
      <name>EVCR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0x4</addressOffset>
      <description>AF remap and debug I/O configuration
          register (AFIO_MAPR)</description>
      <displayName>MAPR</displayName>
      <fields>
        <field>
          <access>read-write</access>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>SPI1 remapping</description>
          <name>SPI1_REMAP</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>I2C1 remapping</description>
          <name>I2C1_REMAP</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>USART1 remapping</description>
          <name>USART1_REMAP</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>USART2 remapping</description>
          <name>USART2_REMAP</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
          <description>USART3 remapping</description>
          <name>USART3_REMAP</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>6</bitOffset>
          <bitWidth>2</bitWidth>
          <description>TIM1 remapping</description>
          <name>TIM1_REMAP</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>8</bitOffset>
          <bitWidth>2</bitWidth>
          <description>TIM2 remapping</description>
          <name>TIM2_REMAP</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>10</bitOffset>
          <bitWidth>2</bitWidth>
          <description>TIM3 remapping</description>
          <name>TIM3_REMAP</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TIM4 remapping</description>
          <name>TIM4_REMAP</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>13</bitOffset>
          <bitWidth>2</bitWidth>
          <description>CAN1 remapping</description>
          <name>CAN_REMAP</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Port D0/Port D1 mapping on
              OSCIN/OSCOUT</description>
          <name>PD01_REMAP</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Set and cleared by
              software</description>
          <name>TIM5CH4_IREMAP</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ADC 1 External trigger injected
              conversion remapping</description>
          <name>ADC1_ETRGINJ_REMAP</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ADC 1 external trigger regular
              conversion remapping</description>
          <name>ADC1_ETRGREG_REMAP</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ADC 2 external trigger injected
              conversion remapping</description>
          <name>ADC2_ETRGINJ_REMAP</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ADC 2 external trigger regular
              conversion remapping</description>
          <name>ADC2_ETRGREG_REMAP</name>
        </field>
        <field>
          <access>write-only</access>
          <bitOffset>24</bitOffset>
          <bitWidth>3</bitWidth>
          <description>Serial wire JTAG
              configuration</description>
          <name>SWJ_CFG</name>
        </field>
      </fields>
      <name>MAPR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x8</addressOffset>
      <description>External interrupt configuration register 1
          (AFIO_EXTICR1)</description>
      <displayName>EXTICR1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>EXTI0 configuration</description>
          <name>EXTI0</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>4</bitWidth>
          <description>EXTI1 configuration</description>
          <name>EXTI1</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>4</bitWidth>
          <description>EXTI2 configuration</description>
          <name>EXTI2</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>4</bitWidth>
          <description>EXTI3 configuration</description>
          <name>EXTI3</name>
        </field>
      </fields>
      <name>EXTICR1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xC</addressOffset>
      <description>External interrupt configuration register 2
          (AFIO_EXTICR2)</description>
      <displayName>EXTICR2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>EXTI4 configuration</description>
          <name>EXTI4</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>4</bitWidth>
          <description>EXTI5 configuration</description>
          <name>EXTI5</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>4</bitWidth>
          <description>EXTI6 configuration</description>
          <name>EXTI6</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>4</bitWidth>
          <description>EXTI7 configuration</description>
          <name>EXTI7</name>
        </field>
      </fields>
      <name>EXTICR2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x10</addressOffset>
      <description>External interrupt configuration register 3
          (AFIO_EXTICR3)</description>
      <displayName>EXTICR3</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>EXTI8 configuration</description>
          <name>EXTI8</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>4</bitWidth>
          <description>EXTI9 configuration</description>
          <name>EXTI9</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>4</bitWidth>
          <description>EXTI10 configuration</description>
          <name>EXTI10</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>4</bitWidth>
          <description>EXTI11 configuration</description>
          <name>EXTI11</name>
        </field>
      </fields>
      <name>EXTICR3</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x14</addressOffset>
      <description>External interrupt configuration register 4
          (AFIO_EXTICR4)</description>
      <displayName>EXTICR4</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>EXTI12 configuration</description>
          <name>EXTI12</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>4</bitWidth>
          <description>EXTI13 configuration</description>
          <name>EXTI13</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>4</bitWidth>
          <description>EXTI14 configuration</description>
          <name>EXTI14</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>4</bitWidth>
          <description>EXTI15 configuration</description>
          <name>EXTI15</name>
        </field>
      </fields>
      <name>EXTICR4</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x1C</addressOffset>
      <description>AF remap and debug I/O configuration
          register</description>
      <displayName>MAPR2</displayName>
      <fields>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TIM9 remapping</description>
          <name>TIM9_REMAP</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TIM10 remapping</description>
          <name>TIM10_REMAP</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TIM11 remapping</description>
          <name>TIM11_REMAP</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TIM13 remapping</description>
          <name>TIM13_REMAP</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TIM14 remapping</description>
          <name>TIM14_REMAP</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>NADV connect/disconnect</description>
          <name>FSMC_NADV</name>
        </field>
      </fields>
      <name>MAPR2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
