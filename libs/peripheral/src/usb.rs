pub const ADDRESS: u32 = 0x40005C00;
/// endpoint 0 register
pub mod ep0r {
    pub const OFFSET: u32 = 0x0;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Endpoint address
        pub ea: u8,
        /// Status bits, for transmission transfers
        pub stat_tx: u8,
        /// Data Toggle, for transmission transfers
        pub dtog_tx: u8,
        /// Correct Transfer for transmission
        pub ctr_tx: u8,
        /// Endpoint kind
        pub ep_kind: u8,
        /// Endpoint type
        pub ep_type: u8,
        /// Setup transaction completed
        pub setup: u8,
        /// Status bits, for reception transfers
        pub stat_rx: u8,
        /// Data Toggle, for reception transfers
        pub dtog_rx: u8,
        /// Correct transfer for reception
        pub ctr_rx: u8,
    }
    pub struct Cache {
        /// Endpoint address
        pub ea: u8,
        /// Status bits, for transmission transfers
        pub stat_tx: u8,
        /// Data Toggle, for transmission transfers
        pub dtog_tx: u8,
        /// Correct Transfer for transmission
        pub ctr_tx: u8,
        /// Endpoint kind
        pub ep_kind: u8,
        /// Endpoint type
        pub ep_type: u8,
        /// Setup transaction completed
        pub setup: u8,
        /// Status bits, for reception transfers
        pub stat_rx: u8,
        /// Data Toggle, for reception transfers
        pub dtog_rx: u8,
        /// Correct transfer for reception
        pub ctr_rx: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            ea: ((value >> 0) & 0b1111) as u8,
            stat_tx: ((value >> 4) & 0b1111) as u8,
            dtog_tx: ((value >> 6) & 0b1111) as u8,
            ctr_tx: ((value >> 7) & 0b1111) as u8,
            ep_kind: ((value >> 8) & 0b1111) as u8,
            ep_type: ((value >> 9) & 0b1111) as u8,
            setup: ((value >> 11) & 0b1111) as u8,
            stat_rx: ((value >> 12) & 0b1111) as u8,
            dtog_rx: ((value >> 14) & 0b1111) as u8,
            ctr_rx: ((value >> 15) & 0b1111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            ea: ((value >> 0) & 0b1111) as u8,
            stat_tx: ((value >> 4) & 0b1111) as u8,
            dtog_tx: ((value >> 6) & 0b1111) as u8,
            ctr_tx: ((value >> 7) & 0b1111) as u8,
            ep_kind: ((value >> 8) & 0b1111) as u8,
            ep_type: ((value >> 9) & 0b1111) as u8,
            setup: ((value >> 11) & 0b1111) as u8,
            stat_rx: ((value >> 12) & 0b1111) as u8,
            dtog_rx: ((value >> 14) & 0b1111) as u8,
            ctr_rx: ((value >> 15) & 0b1111) as u8,
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
                | ((self.ea as u32) << 0)
                | ((self.stat_tx as u32) << 4)
                | ((self.dtog_tx as u32) << 6)
                | ((self.ctr_tx as u32) << 7)
                | ((self.ep_kind as u32) << 8)
                | ((self.ep_type as u32) << 9)
                | ((self.setup as u32) << 11)
                | ((self.stat_rx as u32) << 12)
                | ((self.dtog_rx as u32) << 14)
                | ((self.ctr_rx as u32) << 15)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// endpoint 1 register
pub mod ep1r {
    pub const OFFSET: u32 = 0x4;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Endpoint address
        pub ea: u8,
        /// Status bits, for transmission transfers
        pub stat_tx: u8,
        /// Data Toggle, for transmission transfers
        pub dtog_tx: u8,
        /// Correct Transfer for transmission
        pub ctr_tx: u8,
        /// Endpoint kind
        pub ep_kind: u8,
        /// Endpoint type
        pub ep_type: u8,
        /// Setup transaction completed
        pub setup: u8,
        /// Status bits, for reception transfers
        pub stat_rx: u8,
        /// Data Toggle, for reception transfers
        pub dtog_rx: u8,
        /// Correct transfer for reception
        pub ctr_rx: u8,
    }
    pub struct Cache {
        /// Endpoint address
        pub ea: u8,
        /// Status bits, for transmission transfers
        pub stat_tx: u8,
        /// Data Toggle, for transmission transfers
        pub dtog_tx: u8,
        /// Correct Transfer for transmission
        pub ctr_tx: u8,
        /// Endpoint kind
        pub ep_kind: u8,
        /// Endpoint type
        pub ep_type: u8,
        /// Setup transaction completed
        pub setup: u8,
        /// Status bits, for reception transfers
        pub stat_rx: u8,
        /// Data Toggle, for reception transfers
        pub dtog_rx: u8,
        /// Correct transfer for reception
        pub ctr_rx: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            ea: ((value >> 0) & 0b1111) as u8,
            stat_tx: ((value >> 4) & 0b1111) as u8,
            dtog_tx: ((value >> 6) & 0b1111) as u8,
            ctr_tx: ((value >> 7) & 0b1111) as u8,
            ep_kind: ((value >> 8) & 0b1111) as u8,
            ep_type: ((value >> 9) & 0b1111) as u8,
            setup: ((value >> 11) & 0b1111) as u8,
            stat_rx: ((value >> 12) & 0b1111) as u8,
            dtog_rx: ((value >> 14) & 0b1111) as u8,
            ctr_rx: ((value >> 15) & 0b1111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            ea: ((value >> 0) & 0b1111) as u8,
            stat_tx: ((value >> 4) & 0b1111) as u8,
            dtog_tx: ((value >> 6) & 0b1111) as u8,
            ctr_tx: ((value >> 7) & 0b1111) as u8,
            ep_kind: ((value >> 8) & 0b1111) as u8,
            ep_type: ((value >> 9) & 0b1111) as u8,
            setup: ((value >> 11) & 0b1111) as u8,
            stat_rx: ((value >> 12) & 0b1111) as u8,
            dtog_rx: ((value >> 14) & 0b1111) as u8,
            ctr_rx: ((value >> 15) & 0b1111) as u8,
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
                | ((self.ea as u32) << 0)
                | ((self.stat_tx as u32) << 4)
                | ((self.dtog_tx as u32) << 6)
                | ((self.ctr_tx as u32) << 7)
                | ((self.ep_kind as u32) << 8)
                | ((self.ep_type as u32) << 9)
                | ((self.setup as u32) << 11)
                | ((self.stat_rx as u32) << 12)
                | ((self.dtog_rx as u32) << 14)
                | ((self.ctr_rx as u32) << 15)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// endpoint 2 register
pub mod ep2r {
    pub const OFFSET: u32 = 0x8;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Endpoint address
        pub ea: u8,
        /// Status bits, for transmission transfers
        pub stat_tx: u8,
        /// Data Toggle, for transmission transfers
        pub dtog_tx: u8,
        /// Correct Transfer for transmission
        pub ctr_tx: u8,
        /// Endpoint kind
        pub ep_kind: u8,
        /// Endpoint type
        pub ep_type: u8,
        /// Setup transaction completed
        pub setup: u8,
        /// Status bits, for reception transfers
        pub stat_rx: u8,
        /// Data Toggle, for reception transfers
        pub dtog_rx: u8,
        /// Correct transfer for reception
        pub ctr_rx: u8,
    }
    pub struct Cache {
        /// Endpoint address
        pub ea: u8,
        /// Status bits, for transmission transfers
        pub stat_tx: u8,
        /// Data Toggle, for transmission transfers
        pub dtog_tx: u8,
        /// Correct Transfer for transmission
        pub ctr_tx: u8,
        /// Endpoint kind
        pub ep_kind: u8,
        /// Endpoint type
        pub ep_type: u8,
        /// Setup transaction completed
        pub setup: u8,
        /// Status bits, for reception transfers
        pub stat_rx: u8,
        /// Data Toggle, for reception transfers
        pub dtog_rx: u8,
        /// Correct transfer for reception
        pub ctr_rx: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            ea: ((value >> 0) & 0b1111) as u8,
            stat_tx: ((value >> 4) & 0b1111) as u8,
            dtog_tx: ((value >> 6) & 0b1111) as u8,
            ctr_tx: ((value >> 7) & 0b1111) as u8,
            ep_kind: ((value >> 8) & 0b1111) as u8,
            ep_type: ((value >> 9) & 0b1111) as u8,
            setup: ((value >> 11) & 0b1111) as u8,
            stat_rx: ((value >> 12) & 0b1111) as u8,
            dtog_rx: ((value >> 14) & 0b1111) as u8,
            ctr_rx: ((value >> 15) & 0b1111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            ea: ((value >> 0) & 0b1111) as u8,
            stat_tx: ((value >> 4) & 0b1111) as u8,
            dtog_tx: ((value >> 6) & 0b1111) as u8,
            ctr_tx: ((value >> 7) & 0b1111) as u8,
            ep_kind: ((value >> 8) & 0b1111) as u8,
            ep_type: ((value >> 9) & 0b1111) as u8,
            setup: ((value >> 11) & 0b1111) as u8,
            stat_rx: ((value >> 12) & 0b1111) as u8,
            dtog_rx: ((value >> 14) & 0b1111) as u8,
            ctr_rx: ((value >> 15) & 0b1111) as u8,
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
                | ((self.ea as u32) << 0)
                | ((self.stat_tx as u32) << 4)
                | ((self.dtog_tx as u32) << 6)
                | ((self.ctr_tx as u32) << 7)
                | ((self.ep_kind as u32) << 8)
                | ((self.ep_type as u32) << 9)
                | ((self.setup as u32) << 11)
                | ((self.stat_rx as u32) << 12)
                | ((self.dtog_rx as u32) << 14)
                | ((self.ctr_rx as u32) << 15)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// endpoint 3 register
pub mod ep3r {
    pub const OFFSET: u32 = 0xC;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Endpoint address
        pub ea: u8,
        /// Status bits, for transmission transfers
        pub stat_tx: u8,
        /// Data Toggle, for transmission transfers
        pub dtog_tx: u8,
        /// Correct Transfer for transmission
        pub ctr_tx: u8,
        /// Endpoint kind
        pub ep_kind: u8,
        /// Endpoint type
        pub ep_type: u8,
        /// Setup transaction completed
        pub setup: u8,
        /// Status bits, for reception transfers
        pub stat_rx: u8,
        /// Data Toggle, for reception transfers
        pub dtog_rx: u8,
        /// Correct transfer for reception
        pub ctr_rx: u8,
    }
    pub struct Cache {
        /// Endpoint address
        pub ea: u8,
        /// Status bits, for transmission transfers
        pub stat_tx: u8,
        /// Data Toggle, for transmission transfers
        pub dtog_tx: u8,
        /// Correct Transfer for transmission
        pub ctr_tx: u8,
        /// Endpoint kind
        pub ep_kind: u8,
        /// Endpoint type
        pub ep_type: u8,
        /// Setup transaction completed
        pub setup: u8,
        /// Status bits, for reception transfers
        pub stat_rx: u8,
        /// Data Toggle, for reception transfers
        pub dtog_rx: u8,
        /// Correct transfer for reception
        pub ctr_rx: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            ea: ((value >> 0) & 0b1111) as u8,
            stat_tx: ((value >> 4) & 0b1111) as u8,
            dtog_tx: ((value >> 6) & 0b1111) as u8,
            ctr_tx: ((value >> 7) & 0b1111) as u8,
            ep_kind: ((value >> 8) & 0b1111) as u8,
            ep_type: ((value >> 9) & 0b1111) as u8,
            setup: ((value >> 11) & 0b1111) as u8,
            stat_rx: ((value >> 12) & 0b1111) as u8,
            dtog_rx: ((value >> 14) & 0b1111) as u8,
            ctr_rx: ((value >> 15) & 0b1111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            ea: ((value >> 0) & 0b1111) as u8,
            stat_tx: ((value >> 4) & 0b1111) as u8,
            dtog_tx: ((value >> 6) & 0b1111) as u8,
            ctr_tx: ((value >> 7) & 0b1111) as u8,
            ep_kind: ((value >> 8) & 0b1111) as u8,
            ep_type: ((value >> 9) & 0b1111) as u8,
            setup: ((value >> 11) & 0b1111) as u8,
            stat_rx: ((value >> 12) & 0b1111) as u8,
            dtog_rx: ((value >> 14) & 0b1111) as u8,
            ctr_rx: ((value >> 15) & 0b1111) as u8,
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
                | ((self.ea as u32) << 0)
                | ((self.stat_tx as u32) << 4)
                | ((self.dtog_tx as u32) << 6)
                | ((self.ctr_tx as u32) << 7)
                | ((self.ep_kind as u32) << 8)
                | ((self.ep_type as u32) << 9)
                | ((self.setup as u32) << 11)
                | ((self.stat_rx as u32) << 12)
                | ((self.dtog_rx as u32) << 14)
                | ((self.ctr_rx as u32) << 15)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// endpoint 4 register
pub mod ep4r {
    pub const OFFSET: u32 = 0x10;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Endpoint address
        pub ea: u8,
        /// Status bits, for transmission transfers
        pub stat_tx: u8,
        /// Data Toggle, for transmission transfers
        pub dtog_tx: u8,
        /// Correct Transfer for transmission
        pub ctr_tx: u8,
        /// Endpoint kind
        pub ep_kind: u8,
        /// Endpoint type
        pub ep_type: u8,
        /// Setup transaction completed
        pub setup: u8,
        /// Status bits, for reception transfers
        pub stat_rx: u8,
        /// Data Toggle, for reception transfers
        pub dtog_rx: u8,
        /// Correct transfer for reception
        pub ctr_rx: u8,
    }
    pub struct Cache {
        /// Endpoint address
        pub ea: u8,
        /// Status bits, for transmission transfers
        pub stat_tx: u8,
        /// Data Toggle, for transmission transfers
        pub dtog_tx: u8,
        /// Correct Transfer for transmission
        pub ctr_tx: u8,
        /// Endpoint kind
        pub ep_kind: u8,
        /// Endpoint type
        pub ep_type: u8,
        /// Setup transaction completed
        pub setup: u8,
        /// Status bits, for reception transfers
        pub stat_rx: u8,
        /// Data Toggle, for reception transfers
        pub dtog_rx: u8,
        /// Correct transfer for reception
        pub ctr_rx: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            ea: ((value >> 0) & 0b1111) as u8,
            stat_tx: ((value >> 4) & 0b1111) as u8,
            dtog_tx: ((value >> 6) & 0b1111) as u8,
            ctr_tx: ((value >> 7) & 0b1111) as u8,
            ep_kind: ((value >> 8) & 0b1111) as u8,
            ep_type: ((value >> 9) & 0b1111) as u8,
            setup: ((value >> 11) & 0b1111) as u8,
            stat_rx: ((value >> 12) & 0b1111) as u8,
            dtog_rx: ((value >> 14) & 0b1111) as u8,
            ctr_rx: ((value >> 15) & 0b1111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            ea: ((value >> 0) & 0b1111) as u8,
            stat_tx: ((value >> 4) & 0b1111) as u8,
            dtog_tx: ((value >> 6) & 0b1111) as u8,
            ctr_tx: ((value >> 7) & 0b1111) as u8,
            ep_kind: ((value >> 8) & 0b1111) as u8,
            ep_type: ((value >> 9) & 0b1111) as u8,
            setup: ((value >> 11) & 0b1111) as u8,
            stat_rx: ((value >> 12) & 0b1111) as u8,
            dtog_rx: ((value >> 14) & 0b1111) as u8,
            ctr_rx: ((value >> 15) & 0b1111) as u8,
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
                | ((self.ea as u32) << 0)
                | ((self.stat_tx as u32) << 4)
                | ((self.dtog_tx as u32) << 6)
                | ((self.ctr_tx as u32) << 7)
                | ((self.ep_kind as u32) << 8)
                | ((self.ep_type as u32) << 9)
                | ((self.setup as u32) << 11)
                | ((self.stat_rx as u32) << 12)
                | ((self.dtog_rx as u32) << 14)
                | ((self.ctr_rx as u32) << 15)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// endpoint 5 register
pub mod ep5r {
    pub const OFFSET: u32 = 0x14;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Endpoint address
        pub ea: u8,
        /// Status bits, for transmission transfers
        pub stat_tx: u8,
        /// Data Toggle, for transmission transfers
        pub dtog_tx: u8,
        /// Correct Transfer for transmission
        pub ctr_tx: u8,
        /// Endpoint kind
        pub ep_kind: u8,
        /// Endpoint type
        pub ep_type: u8,
        /// Setup transaction completed
        pub setup: u8,
        /// Status bits, for reception transfers
        pub stat_rx: u8,
        /// Data Toggle, for reception transfers
        pub dtog_rx: u8,
        /// Correct transfer for reception
        pub ctr_rx: u8,
    }
    pub struct Cache {
        /// Endpoint address
        pub ea: u8,
        /// Status bits, for transmission transfers
        pub stat_tx: u8,
        /// Data Toggle, for transmission transfers
        pub dtog_tx: u8,
        /// Correct Transfer for transmission
        pub ctr_tx: u8,
        /// Endpoint kind
        pub ep_kind: u8,
        /// Endpoint type
        pub ep_type: u8,
        /// Setup transaction completed
        pub setup: u8,
        /// Status bits, for reception transfers
        pub stat_rx: u8,
        /// Data Toggle, for reception transfers
        pub dtog_rx: u8,
        /// Correct transfer for reception
        pub ctr_rx: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            ea: ((value >> 0) & 0b1111) as u8,
            stat_tx: ((value >> 4) & 0b1111) as u8,
            dtog_tx: ((value >> 6) & 0b1111) as u8,
            ctr_tx: ((value >> 7) & 0b1111) as u8,
            ep_kind: ((value >> 8) & 0b1111) as u8,
            ep_type: ((value >> 9) & 0b1111) as u8,
            setup: ((value >> 11) & 0b1111) as u8,
            stat_rx: ((value >> 12) & 0b1111) as u8,
            dtog_rx: ((value >> 14) & 0b1111) as u8,
            ctr_rx: ((value >> 15) & 0b1111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            ea: ((value >> 0) & 0b1111) as u8,
            stat_tx: ((value >> 4) & 0b1111) as u8,
            dtog_tx: ((value >> 6) & 0b1111) as u8,
            ctr_tx: ((value >> 7) & 0b1111) as u8,
            ep_kind: ((value >> 8) & 0b1111) as u8,
            ep_type: ((value >> 9) & 0b1111) as u8,
            setup: ((value >> 11) & 0b1111) as u8,
            stat_rx: ((value >> 12) & 0b1111) as u8,
            dtog_rx: ((value >> 14) & 0b1111) as u8,
            ctr_rx: ((value >> 15) & 0b1111) as u8,
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
                | ((self.ea as u32) << 0)
                | ((self.stat_tx as u32) << 4)
                | ((self.dtog_tx as u32) << 6)
                | ((self.ctr_tx as u32) << 7)
                | ((self.ep_kind as u32) << 8)
                | ((self.ep_type as u32) << 9)
                | ((self.setup as u32) << 11)
                | ((self.stat_rx as u32) << 12)
                | ((self.dtog_rx as u32) << 14)
                | ((self.ctr_rx as u32) << 15)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// endpoint 6 register
pub mod ep6r {
    pub const OFFSET: u32 = 0x18;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Endpoint address
        pub ea: u8,
        /// Status bits, for transmission transfers
        pub stat_tx: u8,
        /// Data Toggle, for transmission transfers
        pub dtog_tx: u8,
        /// Correct Transfer for transmission
        pub ctr_tx: u8,
        /// Endpoint kind
        pub ep_kind: u8,
        /// Endpoint type
        pub ep_type: u8,
        /// Setup transaction completed
        pub setup: u8,
        /// Status bits, for reception transfers
        pub stat_rx: u8,
        /// Data Toggle, for reception transfers
        pub dtog_rx: u8,
        /// Correct transfer for reception
        pub ctr_rx: u8,
    }
    pub struct Cache {
        /// Endpoint address
        pub ea: u8,
        /// Status bits, for transmission transfers
        pub stat_tx: u8,
        /// Data Toggle, for transmission transfers
        pub dtog_tx: u8,
        /// Correct Transfer for transmission
        pub ctr_tx: u8,
        /// Endpoint kind
        pub ep_kind: u8,
        /// Endpoint type
        pub ep_type: u8,
        /// Setup transaction completed
        pub setup: u8,
        /// Status bits, for reception transfers
        pub stat_rx: u8,
        /// Data Toggle, for reception transfers
        pub dtog_rx: u8,
        /// Correct transfer for reception
        pub ctr_rx: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            ea: ((value >> 0) & 0b1111) as u8,
            stat_tx: ((value >> 4) & 0b1111) as u8,
            dtog_tx: ((value >> 6) & 0b1111) as u8,
            ctr_tx: ((value >> 7) & 0b1111) as u8,
            ep_kind: ((value >> 8) & 0b1111) as u8,
            ep_type: ((value >> 9) & 0b1111) as u8,
            setup: ((value >> 11) & 0b1111) as u8,
            stat_rx: ((value >> 12) & 0b1111) as u8,
            dtog_rx: ((value >> 14) & 0b1111) as u8,
            ctr_rx: ((value >> 15) & 0b1111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            ea: ((value >> 0) & 0b1111) as u8,
            stat_tx: ((value >> 4) & 0b1111) as u8,
            dtog_tx: ((value >> 6) & 0b1111) as u8,
            ctr_tx: ((value >> 7) & 0b1111) as u8,
            ep_kind: ((value >> 8) & 0b1111) as u8,
            ep_type: ((value >> 9) & 0b1111) as u8,
            setup: ((value >> 11) & 0b1111) as u8,
            stat_rx: ((value >> 12) & 0b1111) as u8,
            dtog_rx: ((value >> 14) & 0b1111) as u8,
            ctr_rx: ((value >> 15) & 0b1111) as u8,
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
                | ((self.ea as u32) << 0)
                | ((self.stat_tx as u32) << 4)
                | ((self.dtog_tx as u32) << 6)
                | ((self.ctr_tx as u32) << 7)
                | ((self.ep_kind as u32) << 8)
                | ((self.ep_type as u32) << 9)
                | ((self.setup as u32) << 11)
                | ((self.stat_rx as u32) << 12)
                | ((self.dtog_rx as u32) << 14)
                | ((self.ctr_rx as u32) << 15)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// endpoint 7 register
pub mod ep7r {
    pub const OFFSET: u32 = 0x1C;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Endpoint address
        pub ea: u8,
        /// Status bits, for transmission transfers
        pub stat_tx: u8,
        /// Data Toggle, for transmission transfers
        pub dtog_tx: u8,
        /// Correct Transfer for transmission
        pub ctr_tx: u8,
        /// Endpoint kind
        pub ep_kind: u8,
        /// Endpoint type
        pub ep_type: u8,
        /// Setup transaction completed
        pub setup: u8,
        /// Status bits, for reception transfers
        pub stat_rx: u8,
        /// Data Toggle, for reception transfers
        pub dtog_rx: u8,
        /// Correct transfer for reception
        pub ctr_rx: u8,
    }
    pub struct Cache {
        /// Endpoint address
        pub ea: u8,
        /// Status bits, for transmission transfers
        pub stat_tx: u8,
        /// Data Toggle, for transmission transfers
        pub dtog_tx: u8,
        /// Correct Transfer for transmission
        pub ctr_tx: u8,
        /// Endpoint kind
        pub ep_kind: u8,
        /// Endpoint type
        pub ep_type: u8,
        /// Setup transaction completed
        pub setup: u8,
        /// Status bits, for reception transfers
        pub stat_rx: u8,
        /// Data Toggle, for reception transfers
        pub dtog_rx: u8,
        /// Correct transfer for reception
        pub ctr_rx: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            ea: ((value >> 0) & 0b1111) as u8,
            stat_tx: ((value >> 4) & 0b1111) as u8,
            dtog_tx: ((value >> 6) & 0b1111) as u8,
            ctr_tx: ((value >> 7) & 0b1111) as u8,
            ep_kind: ((value >> 8) & 0b1111) as u8,
            ep_type: ((value >> 9) & 0b1111) as u8,
            setup: ((value >> 11) & 0b1111) as u8,
            stat_rx: ((value >> 12) & 0b1111) as u8,
            dtog_rx: ((value >> 14) & 0b1111) as u8,
            ctr_rx: ((value >> 15) & 0b1111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            ea: ((value >> 0) & 0b1111) as u8,
            stat_tx: ((value >> 4) & 0b1111) as u8,
            dtog_tx: ((value >> 6) & 0b1111) as u8,
            ctr_tx: ((value >> 7) & 0b1111) as u8,
            ep_kind: ((value >> 8) & 0b1111) as u8,
            ep_type: ((value >> 9) & 0b1111) as u8,
            setup: ((value >> 11) & 0b1111) as u8,
            stat_rx: ((value >> 12) & 0b1111) as u8,
            dtog_rx: ((value >> 14) & 0b1111) as u8,
            ctr_rx: ((value >> 15) & 0b1111) as u8,
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
                | ((self.ea as u32) << 0)
                | ((self.stat_tx as u32) << 4)
                | ((self.dtog_tx as u32) << 6)
                | ((self.ctr_tx as u32) << 7)
                | ((self.ep_kind as u32) << 8)
                | ((self.ep_type as u32) << 9)
                | ((self.setup as u32) << 11)
                | ((self.stat_rx as u32) << 12)
                | ((self.dtog_rx as u32) << 14)
                | ((self.ctr_rx as u32) << 15)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// control register
pub mod cntr {
    pub const OFFSET: u32 = 0x40;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Force USB Reset
        pub fres: bool,
        /// Power down
        pub pdwn: bool,
        /// Low-power mode
        pub lpmode: bool,
        /// Force suspend
        pub fsusp: bool,
        /// Resume request
        pub resume: bool,
        /// Expected start of frame interrupt mask
        pub esofm: bool,
        /// Start of frame interrupt mask
        pub sofm: bool,
        /// USB reset interrupt mask
        pub resetm: bool,
        /// Suspend mode interrupt mask
        pub suspm: bool,
        /// Wakeup interrupt mask
        pub wkupm: bool,
        /// Error interrupt mask
        pub errm: bool,
        /// Packet memory area over / underrun interrupt mask
        pub pmaovrm: bool,
        /// Correct transfer interrupt mask
        pub ctrm: bool,
    }
    pub struct Cache {
        /// Force USB Reset
        pub fres: bool,
        /// Power down
        pub pdwn: bool,
        /// Low-power mode
        pub lpmode: bool,
        /// Force suspend
        pub fsusp: bool,
        /// Resume request
        pub resume: bool,
        /// Expected start of frame interrupt mask
        pub esofm: bool,
        /// Start of frame interrupt mask
        pub sofm: bool,
        /// USB reset interrupt mask
        pub resetm: bool,
        /// Suspend mode interrupt mask
        pub suspm: bool,
        /// Wakeup interrupt mask
        pub wkupm: bool,
        /// Error interrupt mask
        pub errm: bool,
        /// Packet memory area over / underrun interrupt mask
        pub pmaovrm: bool,
        /// Correct transfer interrupt mask
        pub ctrm: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            fres: ((value >> 0) & 0b1) > 0,
            pdwn: ((value >> 1) & 0b1) > 0,
            lpmode: ((value >> 2) & 0b1) > 0,
            fsusp: ((value >> 3) & 0b1) > 0,
            resume: ((value >> 4) & 0b1) > 0,
            esofm: ((value >> 8) & 0b1) > 0,
            sofm: ((value >> 9) & 0b1) > 0,
            resetm: ((value >> 10) & 0b1) > 0,
            suspm: ((value >> 11) & 0b1) > 0,
            wkupm: ((value >> 12) & 0b1) > 0,
            errm: ((value >> 13) & 0b1) > 0,
            pmaovrm: ((value >> 14) & 0b1) > 0,
            ctrm: ((value >> 15) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            fres: ((value >> 0) & 0b1) > 0,
            pdwn: ((value >> 1) & 0b1) > 0,
            lpmode: ((value >> 2) & 0b1) > 0,
            fsusp: ((value >> 3) & 0b1) > 0,
            resume: ((value >> 4) & 0b1) > 0,
            esofm: ((value >> 8) & 0b1) > 0,
            sofm: ((value >> 9) & 0b1) > 0,
            resetm: ((value >> 10) & 0b1) > 0,
            suspm: ((value >> 11) & 0b1) > 0,
            wkupm: ((value >> 12) & 0b1) > 0,
            errm: ((value >> 13) & 0b1) > 0,
            pmaovrm: ((value >> 14) & 0b1) > 0,
            ctrm: ((value >> 15) & 0b1) > 0,
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
                | ((self.fres as u32) << 0)
                | ((self.pdwn as u32) << 1)
                | ((self.lpmode as u32) << 2)
                | ((self.fsusp as u32) << 3)
                | ((self.resume as u32) << 4)
                | ((self.esofm as u32) << 8)
                | ((self.sofm as u32) << 9)
                | ((self.resetm as u32) << 10)
                | ((self.suspm as u32) << 11)
                | ((self.wkupm as u32) << 12)
                | ((self.errm as u32) << 13)
                | ((self.pmaovrm as u32) << 14)
                | ((self.ctrm as u32) << 15)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// interrupt status register
pub mod istr {
    pub const OFFSET: u32 = 0x44;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Endpoint Identifier
        pub ep_id: u8,
        /// Direction of transaction
        pub dir: u8,
        /// Expected start frame
        pub esof: u8,
        /// start of frame
        pub sof: u8,
        /// reset request
        pub reset: u8,
        /// Suspend mode request
        pub susp: u8,
        /// Wakeup
        pub wkup: u8,
        /// Error
        pub err: u8,
        /// Packet memory area over / underrun
        pub pmaovr: u8,
        /// Correct transfer
        pub ctr: u8,
    }
    pub struct Cache {
        /// Endpoint Identifier
        pub ep_id: u8,
        /// Direction of transaction
        pub dir: u8,
        /// Expected start frame
        pub esof: u8,
        /// start of frame
        pub sof: u8,
        /// reset request
        pub reset: u8,
        /// Suspend mode request
        pub susp: u8,
        /// Wakeup
        pub wkup: u8,
        /// Error
        pub err: u8,
        /// Packet memory area over / underrun
        pub pmaovr: u8,
        /// Correct transfer
        pub ctr: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            ep_id: ((value >> 0) & 0b1111) as u8,
            dir: ((value >> 4) & 0b1111) as u8,
            esof: ((value >> 8) & 0b1111) as u8,
            sof: ((value >> 9) & 0b1111) as u8,
            reset: ((value >> 10) & 0b1111) as u8,
            susp: ((value >> 11) & 0b1111) as u8,
            wkup: ((value >> 12) & 0b1111) as u8,
            err: ((value >> 13) & 0b1111) as u8,
            pmaovr: ((value >> 14) & 0b1111) as u8,
            ctr: ((value >> 15) & 0b1111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            ep_id: ((value >> 0) & 0b1111) as u8,
            dir: ((value >> 4) & 0b1111) as u8,
            esof: ((value >> 8) & 0b1111) as u8,
            sof: ((value >> 9) & 0b1111) as u8,
            reset: ((value >> 10) & 0b1111) as u8,
            susp: ((value >> 11) & 0b1111) as u8,
            wkup: ((value >> 12) & 0b1111) as u8,
            err: ((value >> 13) & 0b1111) as u8,
            pmaovr: ((value >> 14) & 0b1111) as u8,
            ctr: ((value >> 15) & 0b1111) as u8,
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
                | ((self.ep_id as u32) << 0)
                | ((self.dir as u32) << 4)
                | ((self.esof as u32) << 8)
                | ((self.sof as u32) << 9)
                | ((self.reset as u32) << 10)
                | ((self.susp as u32) << 11)
                | ((self.wkup as u32) << 12)
                | ((self.err as u32) << 13)
                | ((self.pmaovr as u32) << 14)
                | ((self.ctr as u32) << 15)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// frame number register
pub mod fnr {
    pub const OFFSET: u32 = 0x48;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// Frame number
    /// Access: read-only, Width: 11, Offset: 0
    /// Get Frame number
    pub fn fnr_fn() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b11111111111 << 0);
        value as u16
    }
    /// Lost SOF
    /// Access: read-only, Width: 2, Offset: 11
    /// Get Lost SOF
    pub fn lsof() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b11 << 11);
        value as u8
    }
    /// Locked
    /// Access: read-only, Width: 1, Offset: 13
    /// Get Locked
    pub fn lck() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 13);
        value > 0
    }
    /// Receive data - line status
    /// Access: read-only, Width: 1, Offset: 14
    /// Get Receive data - line status
    pub fn rxdm() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 14);
        value > 0
    }
    /// Receive data + line status
    /// Access: read-only, Width: 1, Offset: 15
    /// Get Receive data + line status
    pub fn rxdp() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 15);
        value > 0
    }
}
/// device address
pub mod daddr {
    pub const OFFSET: u32 = 0x4C;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Device address
        pub add: u8,
        /// Enable function
        pub ef: u8,
    }
    pub struct Cache {
        /// Device address
        pub add: u8,
        /// Enable function
        pub ef: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            add: ((value >> 0) & 0b1111111) as u8,
            ef: ((value >> 7) & 0b1111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            add: ((value >> 0) & 0b1111111) as u8,
            ef: ((value >> 7) & 0b1111111) as u8,
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
                | ((self.ef as u32) << 7)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Buffer table address
pub mod btable {
    pub const OFFSET: u32 = 0x50;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Buffer table
        pub btable: u16,
    }
    pub struct Cache {
        /// Buffer table
        pub btable: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            btable: ((value >> 3) & 0b1111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            btable: ((value >> 3) & 0b1111111111111) as u16,
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
                | ((self.btable as u32) << 3)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// USB Device FS Wakeup through EXTI line interrupt
pub const INTERRUPT_USB_FS_WKUP: u32 = 42;
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0x40005C00</baseAddress>
  <description>Universal serial bus full-speed device
      interface</description>
  <groupName>USB</groupName>
  <interrupt>
    <description>USB Device FS Wakeup through EXTI line
        interrupt</description>
    <name>USB_FS_WKUP</name>
    <value>42</value>
  </interrupt>
  <name>USB</name>
  <registers>
    <register>
      <access>read-write</access>
      <addressOffset>0x0</addressOffset>
      <description>endpoint 0 register</description>
      <displayName>EP0R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Endpoint address</description>
          <name>EA</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Status bits, for transmission
              transfers</description>
          <name>STAT_TX</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Data Toggle, for transmission
              transfers</description>
          <name>DTOG_TX</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Correct Transfer for
              transmission</description>
          <name>CTR_TX</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Endpoint kind</description>
          <name>EP_KIND</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Endpoint type</description>
          <name>EP_TYPE</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Setup transaction
              completed</description>
          <name>SETUP</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Status bits, for reception
              transfers</description>
          <name>STAT_RX</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Data Toggle, for reception
              transfers</description>
          <name>DTOG_RX</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Correct transfer for
              reception</description>
          <name>CTR_RX</name>
        </field>
      </fields>
      <name>EP0R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x4</addressOffset>
      <description>endpoint 1 register</description>
      <displayName>EP1R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Endpoint address</description>
          <name>EA</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Status bits, for transmission
              transfers</description>
          <name>STAT_TX</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Data Toggle, for transmission
              transfers</description>
          <name>DTOG_TX</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Correct Transfer for
              transmission</description>
          <name>CTR_TX</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Endpoint kind</description>
          <name>EP_KIND</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Endpoint type</description>
          <name>EP_TYPE</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Setup transaction
              completed</description>
          <name>SETUP</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Status bits, for reception
              transfers</description>
          <name>STAT_RX</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Data Toggle, for reception
              transfers</description>
          <name>DTOG_RX</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Correct transfer for
              reception</description>
          <name>CTR_RX</name>
        </field>
      </fields>
      <name>EP1R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x8</addressOffset>
      <description>endpoint 2 register</description>
      <displayName>EP2R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Endpoint address</description>
          <name>EA</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Status bits, for transmission
              transfers</description>
          <name>STAT_TX</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Data Toggle, for transmission
              transfers</description>
          <name>DTOG_TX</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Correct Transfer for
              transmission</description>
          <name>CTR_TX</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Endpoint kind</description>
          <name>EP_KIND</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Endpoint type</description>
          <name>EP_TYPE</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Setup transaction
              completed</description>
          <name>SETUP</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Status bits, for reception
              transfers</description>
          <name>STAT_RX</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Data Toggle, for reception
              transfers</description>
          <name>DTOG_RX</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Correct transfer for
              reception</description>
          <name>CTR_RX</name>
        </field>
      </fields>
      <name>EP2R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xC</addressOffset>
      <description>endpoint 3 register</description>
      <displayName>EP3R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Endpoint address</description>
          <name>EA</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Status bits, for transmission
              transfers</description>
          <name>STAT_TX</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Data Toggle, for transmission
              transfers</description>
          <name>DTOG_TX</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Correct Transfer for
              transmission</description>
          <name>CTR_TX</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Endpoint kind</description>
          <name>EP_KIND</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Endpoint type</description>
          <name>EP_TYPE</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Setup transaction
              completed</description>
          <name>SETUP</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Status bits, for reception
              transfers</description>
          <name>STAT_RX</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Data Toggle, for reception
              transfers</description>
          <name>DTOG_RX</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Correct transfer for
              reception</description>
          <name>CTR_RX</name>
        </field>
      </fields>
      <name>EP3R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x10</addressOffset>
      <description>endpoint 4 register</description>
      <displayName>EP4R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Endpoint address</description>
          <name>EA</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Status bits, for transmission
              transfers</description>
          <name>STAT_TX</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Data Toggle, for transmission
              transfers</description>
          <name>DTOG_TX</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Correct Transfer for
              transmission</description>
          <name>CTR_TX</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Endpoint kind</description>
          <name>EP_KIND</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Endpoint type</description>
          <name>EP_TYPE</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Setup transaction
              completed</description>
          <name>SETUP</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Status bits, for reception
              transfers</description>
          <name>STAT_RX</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Data Toggle, for reception
              transfers</description>
          <name>DTOG_RX</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Correct transfer for
              reception</description>
          <name>CTR_RX</name>
        </field>
      </fields>
      <name>EP4R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x14</addressOffset>
      <description>endpoint 5 register</description>
      <displayName>EP5R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Endpoint address</description>
          <name>EA</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Status bits, for transmission
              transfers</description>
          <name>STAT_TX</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Data Toggle, for transmission
              transfers</description>
          <name>DTOG_TX</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Correct Transfer for
              transmission</description>
          <name>CTR_TX</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Endpoint kind</description>
          <name>EP_KIND</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Endpoint type</description>
          <name>EP_TYPE</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Setup transaction
              completed</description>
          <name>SETUP</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Status bits, for reception
              transfers</description>
          <name>STAT_RX</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Data Toggle, for reception
              transfers</description>
          <name>DTOG_RX</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Correct transfer for
              reception</description>
          <name>CTR_RX</name>
        </field>
      </fields>
      <name>EP5R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x18</addressOffset>
      <description>endpoint 6 register</description>
      <displayName>EP6R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Endpoint address</description>
          <name>EA</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Status bits, for transmission
              transfers</description>
          <name>STAT_TX</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Data Toggle, for transmission
              transfers</description>
          <name>DTOG_TX</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Correct Transfer for
              transmission</description>
          <name>CTR_TX</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Endpoint kind</description>
          <name>EP_KIND</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Endpoint type</description>
          <name>EP_TYPE</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Setup transaction
              completed</description>
          <name>SETUP</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Status bits, for reception
              transfers</description>
          <name>STAT_RX</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Data Toggle, for reception
              transfers</description>
          <name>DTOG_RX</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Correct transfer for
              reception</description>
          <name>CTR_RX</name>
        </field>
      </fields>
      <name>EP6R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x1C</addressOffset>
      <description>endpoint 7 register</description>
      <displayName>EP7R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Endpoint address</description>
          <name>EA</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Status bits, for transmission
              transfers</description>
          <name>STAT_TX</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Data Toggle, for transmission
              transfers</description>
          <name>DTOG_TX</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Correct Transfer for
              transmission</description>
          <name>CTR_TX</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Endpoint kind</description>
          <name>EP_KIND</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Endpoint type</description>
          <name>EP_TYPE</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Setup transaction
              completed</description>
          <name>SETUP</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Status bits, for reception
              transfers</description>
          <name>STAT_RX</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Data Toggle, for reception
              transfers</description>
          <name>DTOG_RX</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Correct transfer for
              reception</description>
          <name>CTR_RX</name>
        </field>
      </fields>
      <name>EP7R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x40</addressOffset>
      <description>control register</description>
      <displayName>USB_CNTR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Force USB Reset</description>
          <name>FRES</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Power down</description>
          <name>PDWN</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Low-power mode</description>
          <name>LPMODE</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Force suspend</description>
          <name>FSUSP</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Resume request</description>
          <name>RESUME</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Expected start of frame interrupt
              mask</description>
          <name>ESOFM</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Start of frame interrupt
              mask</description>
          <name>SOFM</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>USB reset interrupt mask</description>
          <name>RESETM</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Suspend mode interrupt
              mask</description>
          <name>SUSPM</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Wakeup interrupt mask</description>
          <name>WKUPM</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Error interrupt mask</description>
          <name>ERRM</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Packet memory area over / underrun
              interrupt mask</description>
          <name>PMAOVRM</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Correct transfer interrupt
              mask</description>
          <name>CTRM</name>
        </field>
      </fields>
      <name>CNTR</name>
      <resetValue>0x00000003</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x44</addressOffset>
      <description>interrupt status register</description>
      <displayName>ISTR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>Endpoint Identifier</description>
          <name>EP_ID</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Direction of transaction</description>
          <name>DIR</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Expected start frame</description>
          <name>ESOF</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>start of frame</description>
          <name>SOF</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>reset request</description>
          <name>RESET</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Suspend mode request</description>
          <name>SUSP</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Wakeup</description>
          <name>WKUP</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Error</description>
          <name>ERR</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Packet memory area over /
              underrun</description>
          <name>PMAOVR</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Correct transfer</description>
          <name>CTR</name>
        </field>
      </fields>
      <name>ISTR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x48</addressOffset>
      <description>frame number register</description>
      <displayName>FNR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>11</bitWidth>
          <description>Frame number</description>
          <name>FN</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Lost SOF</description>
          <name>LSOF</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Locked</description>
          <name>LCK</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Receive data - line status</description>
          <name>RXDM</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Receive data + line status</description>
          <name>RXDP</name>
        </field>
      </fields>
      <name>FNR</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x4C</addressOffset>
      <description>device address</description>
      <displayName>DADDR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>7</bitWidth>
          <description>Device address</description>
          <name>ADD</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Enable function</description>
          <name>EF</name>
        </field>
      </fields>
      <name>DADDR</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x50</addressOffset>
      <description>Buffer table address</description>
      <displayName>BTABLE</displayName>
      <fields>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>13</bitWidth>
          <description>Buffer table</description>
          <name>BTABLE</name>
        </field>
      </fields>
      <name>BTABLE</name>
      <resetValue>0x0000</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
