pub const ADDRESS: u32 = 0xE0042000;
/// DBGMCU_IDCODE
pub mod idcode {
    pub const OFFSET: u32 = 0x0;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// DEV_ID
    /// Access: read-only, Width: 12, Offset: 0
    /// Get DEV_ID
    pub fn dev_id() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b111111111111 << 0);
        value as u16
    }
    /// REV_ID
    /// Access: read-only, Width: 16, Offset: 16
    /// Get REV_ID
    pub fn rev_id() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1111111111111111 << 16);
        value as u16
    }
}
/// DBGMCU_CR
pub mod cr {
    pub const OFFSET: u32 = 0x4;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// DBG_SLEEP
        pub dbg_sleep: bool,
        /// DBG_STOP
        pub dbg_stop: bool,
        /// DBG_STANDBY
        pub dbg_standby: bool,
        /// TRACE_IOEN
        pub trace_ioen: bool,
        /// TRACE_MODE
        pub trace_mode: bool,
        /// DBG_IWDG_STOP
        pub dbg_iwdg_stop: bool,
        /// DBG_WWDG_STOP
        pub dbg_wwdg_stop: bool,
        /// DBG_TIM1_STOP
        pub dbg_tim1_stop: bool,
        /// DBG_TIM2_STOP
        pub dbg_tim2_stop: bool,
        /// DBG_TIM3_STOP
        pub dbg_tim3_stop: bool,
        /// DBG_TIM4_STOP
        pub dbg_tim4_stop: bool,
        /// DBG_CAN1_STOP
        pub dbg_can1_stop: bool,
        /// DBG_I2C1_SMBUS_TIMEOUT
        pub dbg_i2c1_smbus_timeout: bool,
        /// DBG_I2C2_SMBUS_TIMEOUT
        pub dbg_i2c2_smbus_timeout: bool,
        /// DBG_TIM8_STOP
        pub dbg_tim8_stop: bool,
        /// DBG_TIM5_STOP
        pub dbg_tim5_stop: bool,
        /// DBG_TIM6_STOP
        pub dbg_tim6_stop: bool,
        /// DBG_TIM7_STOP
        pub dbg_tim7_stop: bool,
        /// DBG_CAN2_STOP
        pub dbg_can2_stop: bool,
    }
    pub struct Cache {
        /// DBG_SLEEP
        pub dbg_sleep: bool,
        /// DBG_STOP
        pub dbg_stop: bool,
        /// DBG_STANDBY
        pub dbg_standby: bool,
        /// TRACE_IOEN
        pub trace_ioen: bool,
        /// TRACE_MODE
        pub trace_mode: bool,
        /// DBG_IWDG_STOP
        pub dbg_iwdg_stop: bool,
        /// DBG_WWDG_STOP
        pub dbg_wwdg_stop: bool,
        /// DBG_TIM1_STOP
        pub dbg_tim1_stop: bool,
        /// DBG_TIM2_STOP
        pub dbg_tim2_stop: bool,
        /// DBG_TIM3_STOP
        pub dbg_tim3_stop: bool,
        /// DBG_TIM4_STOP
        pub dbg_tim4_stop: bool,
        /// DBG_CAN1_STOP
        pub dbg_can1_stop: bool,
        /// DBG_I2C1_SMBUS_TIMEOUT
        pub dbg_i2c1_smbus_timeout: bool,
        /// DBG_I2C2_SMBUS_TIMEOUT
        pub dbg_i2c2_smbus_timeout: bool,
        /// DBG_TIM8_STOP
        pub dbg_tim8_stop: bool,
        /// DBG_TIM5_STOP
        pub dbg_tim5_stop: bool,
        /// DBG_TIM6_STOP
        pub dbg_tim6_stop: bool,
        /// DBG_TIM7_STOP
        pub dbg_tim7_stop: bool,
        /// DBG_CAN2_STOP
        pub dbg_can2_stop: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            dbg_sleep: ((value >> 0) & 0b1) > 0,
            dbg_stop: ((value >> 1) & 0b1) > 0,
            dbg_standby: ((value >> 2) & 0b1) > 0,
            trace_ioen: ((value >> 5) & 0b1) > 0,
            trace_mode: ((value >> 6) & 0b1) > 0,
            dbg_iwdg_stop: ((value >> 8) & 0b1) > 0,
            dbg_wwdg_stop: ((value >> 9) & 0b1) > 0,
            dbg_tim1_stop: ((value >> 10) & 0b1) > 0,
            dbg_tim2_stop: ((value >> 11) & 0b1) > 0,
            dbg_tim3_stop: ((value >> 12) & 0b1) > 0,
            dbg_tim4_stop: ((value >> 13) & 0b1) > 0,
            dbg_can1_stop: ((value >> 14) & 0b1) > 0,
            dbg_i2c1_smbus_timeout: ((value >> 15) & 0b1) > 0,
            dbg_i2c2_smbus_timeout: ((value >> 16) & 0b1) > 0,
            dbg_tim8_stop: ((value >> 17) & 0b1) > 0,
            dbg_tim5_stop: ((value >> 18) & 0b1) > 0,
            dbg_tim6_stop: ((value >> 19) & 0b1) > 0,
            dbg_tim7_stop: ((value >> 20) & 0b1) > 0,
            dbg_can2_stop: ((value >> 21) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            dbg_sleep: ((value >> 0) & 0b1) > 0,
            dbg_stop: ((value >> 1) & 0b1) > 0,
            dbg_standby: ((value >> 2) & 0b1) > 0,
            trace_ioen: ((value >> 5) & 0b1) > 0,
            trace_mode: ((value >> 6) & 0b1) > 0,
            dbg_iwdg_stop: ((value >> 8) & 0b1) > 0,
            dbg_wwdg_stop: ((value >> 9) & 0b1) > 0,
            dbg_tim1_stop: ((value >> 10) & 0b1) > 0,
            dbg_tim2_stop: ((value >> 11) & 0b1) > 0,
            dbg_tim3_stop: ((value >> 12) & 0b1) > 0,
            dbg_tim4_stop: ((value >> 13) & 0b1) > 0,
            dbg_can1_stop: ((value >> 14) & 0b1) > 0,
            dbg_i2c1_smbus_timeout: ((value >> 15) & 0b1) > 0,
            dbg_i2c2_smbus_timeout: ((value >> 16) & 0b1) > 0,
            dbg_tim8_stop: ((value >> 17) & 0b1) > 0,
            dbg_tim5_stop: ((value >> 18) & 0b1) > 0,
            dbg_tim6_stop: ((value >> 19) & 0b1) > 0,
            dbg_tim7_stop: ((value >> 20) & 0b1) > 0,
            dbg_can2_stop: ((value >> 21) & 0b1) > 0,
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
                | ((self.dbg_sleep as u32) << 0)
                | ((self.dbg_stop as u32) << 1)
                | ((self.dbg_standby as u32) << 2)
                | ((self.trace_ioen as u32) << 5)
                | ((self.trace_mode as u32) << 6)
                | ((self.dbg_iwdg_stop as u32) << 8)
                | ((self.dbg_wwdg_stop as u32) << 9)
                | ((self.dbg_tim1_stop as u32) << 10)
                | ((self.dbg_tim2_stop as u32) << 11)
                | ((self.dbg_tim3_stop as u32) << 12)
                | ((self.dbg_tim4_stop as u32) << 13)
                | ((self.dbg_can1_stop as u32) << 14)
                | ((self.dbg_i2c1_smbus_timeout as u32) << 15)
                | ((self.dbg_i2c2_smbus_timeout as u32) << 16)
                | ((self.dbg_tim8_stop as u32) << 17)
                | ((self.dbg_tim5_stop as u32) << 18)
                | ((self.dbg_tim6_stop as u32) << 19)
                | ((self.dbg_tim7_stop as u32) << 20)
                | ((self.dbg_can2_stop as u32) << 21)
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
  <baseAddress>0xE0042000</baseAddress>
  <description>Debug support</description>
  <groupName>DBG</groupName>
  <name>DBG</name>
  <registers>
    <register>
      <access>read-only</access>
      <addressOffset>0x0</addressOffset>
      <description>DBGMCU_IDCODE</description>
      <displayName>IDCODE</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>12</bitWidth>
          <description>DEV_ID</description>
          <name>DEV_ID</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>16</bitWidth>
          <description>REV_ID</description>
          <name>REV_ID</name>
        </field>
      </fields>
      <name>IDCODE</name>
      <resetValue>0x0</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x4</addressOffset>
      <description>DBGMCU_CR</description>
      <displayName>CR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DBG_SLEEP</description>
          <name>DBG_SLEEP</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DBG_STOP</description>
          <name>DBG_STOP</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DBG_STANDBY</description>
          <name>DBG_STANDBY</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TRACE_IOEN</description>
          <name>TRACE_IOEN</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>2</bitWidth>
          <description>TRACE_MODE</description>
          <name>TRACE_MODE</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DBG_IWDG_STOP</description>
          <name>DBG_IWDG_STOP</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DBG_WWDG_STOP</description>
          <name>DBG_WWDG_STOP</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DBG_TIM1_STOP</description>
          <name>DBG_TIM1_STOP</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DBG_TIM2_STOP</description>
          <name>DBG_TIM2_STOP</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DBG_TIM3_STOP</description>
          <name>DBG_TIM3_STOP</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DBG_TIM4_STOP</description>
          <name>DBG_TIM4_STOP</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DBG_CAN1_STOP</description>
          <name>DBG_CAN1_STOP</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DBG_I2C1_SMBUS_TIMEOUT</description>
          <name>DBG_I2C1_SMBUS_TIMEOUT</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DBG_I2C2_SMBUS_TIMEOUT</description>
          <name>DBG_I2C2_SMBUS_TIMEOUT</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DBG_TIM8_STOP</description>
          <name>DBG_TIM8_STOP</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DBG_TIM5_STOP</description>
          <name>DBG_TIM5_STOP</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DBG_TIM6_STOP</description>
          <name>DBG_TIM6_STOP</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DBG_TIM7_STOP</description>
          <name>DBG_TIM7_STOP</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DBG_CAN2_STOP</description>
          <name>DBG_CAN2_STOP</name>
        </field>
      </fields>
      <name>CR</name>
      <resetValue>0x0</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
