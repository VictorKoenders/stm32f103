pub const ADDRESS: u32 = 0x40006400;
/// CAN_MCR
pub mod can_mcr {
    pub const OFFSET: u32 = 0x0;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// DBF
        pub dbf: bool,
        /// RESET
        pub reset: bool,
        /// TTCM
        pub ttcm: bool,
        /// ABOM
        pub abom: bool,
        /// AWUM
        pub awum: bool,
        /// NART
        pub nart: bool,
        /// RFLM
        pub rflm: bool,
        /// TXFP
        pub txfp: bool,
        /// SLEEP
        pub sleep: bool,
        /// INRQ
        pub inrq: bool,
    }
    pub struct Cache {
        /// DBF
        pub dbf: bool,
        /// RESET
        pub reset: bool,
        /// TTCM
        pub ttcm: bool,
        /// ABOM
        pub abom: bool,
        /// AWUM
        pub awum: bool,
        /// NART
        pub nart: bool,
        /// RFLM
        pub rflm: bool,
        /// TXFP
        pub txfp: bool,
        /// SLEEP
        pub sleep: bool,
        /// INRQ
        pub inrq: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            dbf: ((value >> 16) & 0b1) > 0,
            reset: ((value >> 15) & 0b1) > 0,
            ttcm: ((value >> 7) & 0b1) > 0,
            abom: ((value >> 6) & 0b1) > 0,
            awum: ((value >> 5) & 0b1) > 0,
            nart: ((value >> 4) & 0b1) > 0,
            rflm: ((value >> 3) & 0b1) > 0,
            txfp: ((value >> 2) & 0b1) > 0,
            sleep: ((value >> 1) & 0b1) > 0,
            inrq: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            dbf: ((value >> 16) & 0b1) > 0,
            reset: ((value >> 15) & 0b1) > 0,
            ttcm: ((value >> 7) & 0b1) > 0,
            abom: ((value >> 6) & 0b1) > 0,
            awum: ((value >> 5) & 0b1) > 0,
            nart: ((value >> 4) & 0b1) > 0,
            rflm: ((value >> 3) & 0b1) > 0,
            txfp: ((value >> 2) & 0b1) > 0,
            sleep: ((value >> 1) & 0b1) > 0,
            inrq: ((value >> 0) & 0b1) > 0,
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
                | ((self.dbf as u32) << 16)
                | ((self.reset as u32) << 15)
                | ((self.ttcm as u32) << 7)
                | ((self.abom as u32) << 6)
                | ((self.awum as u32) << 5)
                | ((self.nart as u32) << 4)
                | ((self.rflm as u32) << 3)
                | ((self.txfp as u32) << 2)
                | ((self.sleep as u32) << 1)
                | ((self.inrq as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// CAN_MSR
pub mod can_msr {
    pub const OFFSET: u32 = 0x4;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// RX
    /// Access: read-only, Width: 1, Offset: 11
    /// Get RX
    pub fn rx() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 11);
        value > 0
    }
    /// SAMP
    /// Access: read-only, Width: 1, Offset: 10
    /// Get SAMP
    pub fn samp() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 10);
        value > 0
    }
    /// RXM
    /// Access: read-only, Width: 1, Offset: 9
    /// Get RXM
    pub fn rxm() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 9);
        value > 0
    }
    /// TXM
    /// Access: read-only, Width: 1, Offset: 8
    /// Get TXM
    pub fn txm() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 8);
        value > 0
    }
    /// SLAKI
    /// Access: read-write, Width: 1, Offset: 4
    /// Set SLAKI
    pub fn set_slaki(value: bool) {
        let value = value as u32;
        let value = value << 4;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get SLAKI
    pub fn get_slaki() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 4);
        value > 0
    }
    /// WKUI
    /// Access: read-write, Width: 1, Offset: 3
    /// Set WKUI
    pub fn set_wkui(value: bool) {
        let value = value as u32;
        let value = value << 3;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get WKUI
    pub fn get_wkui() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 3);
        value > 0
    }
    /// ERRI
    /// Access: read-write, Width: 1, Offset: 2
    /// Set ERRI
    pub fn set_erri(value: bool) {
        let value = value as u32;
        let value = value << 2;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get ERRI
    pub fn get_erri() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 2);
        value > 0
    }
    /// SLAK
    /// Access: read-only, Width: 1, Offset: 1
    /// Get SLAK
    pub fn slak() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 1);
        value > 0
    }
    /// INAK
    /// Access: read-only, Width: 1, Offset: 0
    /// Get INAK
    pub fn inak() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
}
/// CAN_TSR
pub mod can_tsr {
    pub const OFFSET: u32 = 0x8;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// Lowest priority flag for mailbox 2
    /// Access: read-only, Width: 1, Offset: 31
    /// Get Lowest priority flag for mailbox 2
    pub fn low2() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 31);
        value > 0
    }
    /// Lowest priority flag for mailbox 1
    /// Access: read-only, Width: 1, Offset: 30
    /// Get Lowest priority flag for mailbox 1
    pub fn low1() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 30);
        value > 0
    }
    /// Lowest priority flag for mailbox 0
    /// Access: read-only, Width: 1, Offset: 29
    /// Get Lowest priority flag for mailbox 0
    pub fn low0() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 29);
        value > 0
    }
    /// Lowest priority flag for mailbox 2
    /// Access: read-only, Width: 1, Offset: 28
    /// Get Lowest priority flag for mailbox 2
    pub fn tme2() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 28);
        value > 0
    }
    /// Lowest priority flag for mailbox 1
    /// Access: read-only, Width: 1, Offset: 27
    /// Get Lowest priority flag for mailbox 1
    pub fn tme1() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 27);
        value > 0
    }
    /// Lowest priority flag for mailbox 0
    /// Access: read-only, Width: 1, Offset: 26
    /// Get Lowest priority flag for mailbox 0
    pub fn tme0() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 26);
        value > 0
    }
    /// CODE
    /// Access: read-only, Width: 2, Offset: 24
    /// Get CODE
    pub fn code() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b11 << 24);
        value as u8
    }
    /// ABRQ2
    /// Access: read-write, Width: 1, Offset: 23
    /// Set ABRQ2
    pub fn set_abrq2(value: bool) {
        let value = value as u32;
        let value = value << 23;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get ABRQ2
    pub fn get_abrq2() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 23);
        value > 0
    }
    /// TERR2
    /// Access: read-write, Width: 1, Offset: 19
    /// Set TERR2
    pub fn set_terr2(value: bool) {
        let value = value as u32;
        let value = value << 19;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get TERR2
    pub fn get_terr2() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 19);
        value > 0
    }
    /// ALST2
    /// Access: read-write, Width: 1, Offset: 18
    /// Set ALST2
    pub fn set_alst2(value: bool) {
        let value = value as u32;
        let value = value << 18;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get ALST2
    pub fn get_alst2() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 18);
        value > 0
    }
    /// TXOK2
    /// Access: read-write, Width: 1, Offset: 17
    /// Set TXOK2
    pub fn set_txok2(value: bool) {
        let value = value as u32;
        let value = value << 17;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get TXOK2
    pub fn get_txok2() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 17);
        value > 0
    }
    /// RQCP2
    /// Access: read-write, Width: 1, Offset: 16
    /// Set RQCP2
    pub fn set_rqcp2(value: bool) {
        let value = value as u32;
        let value = value << 16;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get RQCP2
    pub fn get_rqcp2() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 16);
        value > 0
    }
    /// ABRQ1
    /// Access: read-write, Width: 1, Offset: 15
    /// Set ABRQ1
    pub fn set_abrq1(value: bool) {
        let value = value as u32;
        let value = value << 15;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get ABRQ1
    pub fn get_abrq1() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 15);
        value > 0
    }
    /// TERR1
    /// Access: read-write, Width: 1, Offset: 11
    /// Set TERR1
    pub fn set_terr1(value: bool) {
        let value = value as u32;
        let value = value << 11;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get TERR1
    pub fn get_terr1() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 11);
        value > 0
    }
    /// ALST1
    /// Access: read-write, Width: 1, Offset: 10
    /// Set ALST1
    pub fn set_alst1(value: bool) {
        let value = value as u32;
        let value = value << 10;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get ALST1
    pub fn get_alst1() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 10);
        value > 0
    }
    /// TXOK1
    /// Access: read-write, Width: 1, Offset: 9
    /// Set TXOK1
    pub fn set_txok1(value: bool) {
        let value = value as u32;
        let value = value << 9;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get TXOK1
    pub fn get_txok1() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 9);
        value > 0
    }
    /// RQCP1
    /// Access: read-write, Width: 1, Offset: 8
    /// Set RQCP1
    pub fn set_rqcp1(value: bool) {
        let value = value as u32;
        let value = value << 8;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get RQCP1
    pub fn get_rqcp1() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 8);
        value > 0
    }
    /// ABRQ0
    /// Access: read-write, Width: 1, Offset: 7
    /// Set ABRQ0
    pub fn set_abrq0(value: bool) {
        let value = value as u32;
        let value = value << 7;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get ABRQ0
    pub fn get_abrq0() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 7);
        value > 0
    }
    /// TERR0
    /// Access: read-write, Width: 1, Offset: 3
    /// Set TERR0
    pub fn set_terr0(value: bool) {
        let value = value as u32;
        let value = value << 3;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get TERR0
    pub fn get_terr0() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 3);
        value > 0
    }
    /// ALST0
    /// Access: read-write, Width: 1, Offset: 2
    /// Set ALST0
    pub fn set_alst0(value: bool) {
        let value = value as u32;
        let value = value << 2;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get ALST0
    pub fn get_alst0() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 2);
        value > 0
    }
    /// TXOK0
    /// Access: read-write, Width: 1, Offset: 1
    /// Set TXOK0
    pub fn set_txok0(value: bool) {
        let value = value as u32;
        let value = value << 1;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get TXOK0
    pub fn get_txok0() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 1);
        value > 0
    }
    /// RQCP0
    /// Access: read-write, Width: 1, Offset: 0
    /// Set RQCP0
    pub fn set_rqcp0(value: bool) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get RQCP0
    pub fn get_rqcp0() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
}
/// CAN_RF0R
pub mod can_rf0r {
    pub const OFFSET: u32 = 0xC;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// RFOM0
    /// Access: read-write, Width: 1, Offset: 5
    /// Set RFOM0
    pub fn set_rfom0(value: bool) {
        let value = value as u32;
        let value = value << 5;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get RFOM0
    pub fn get_rfom0() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 5);
        value > 0
    }
    /// FOVR0
    /// Access: read-write, Width: 1, Offset: 4
    /// Set FOVR0
    pub fn set_fovr0(value: bool) {
        let value = value as u32;
        let value = value << 4;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get FOVR0
    pub fn get_fovr0() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 4);
        value > 0
    }
    /// FULL0
    /// Access: read-write, Width: 1, Offset: 3
    /// Set FULL0
    pub fn set_full0(value: bool) {
        let value = value as u32;
        let value = value << 3;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get FULL0
    pub fn get_full0() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 3);
        value > 0
    }
    /// FMP0
    /// Access: read-only, Width: 2, Offset: 0
    /// Get FMP0
    pub fn fmp0() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b11 << 0);
        value as u8
    }
}
/// CAN_RF1R
pub mod can_rf1r {
    pub const OFFSET: u32 = 0x10;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// RFOM1
    /// Access: read-write, Width: 1, Offset: 5
    /// Set RFOM1
    pub fn set_rfom1(value: bool) {
        let value = value as u32;
        let value = value << 5;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get RFOM1
    pub fn get_rfom1() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 5);
        value > 0
    }
    /// FOVR1
    /// Access: read-write, Width: 1, Offset: 4
    /// Set FOVR1
    pub fn set_fovr1(value: bool) {
        let value = value as u32;
        let value = value << 4;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get FOVR1
    pub fn get_fovr1() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 4);
        value > 0
    }
    /// FULL1
    /// Access: read-write, Width: 1, Offset: 3
    /// Set FULL1
    pub fn set_full1(value: bool) {
        let value = value as u32;
        let value = value << 3;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get FULL1
    pub fn get_full1() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 3);
        value > 0
    }
    /// FMP1
    /// Access: read-only, Width: 2, Offset: 0
    /// Get FMP1
    pub fn fmp1() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b11 << 0);
        value as u8
    }
}
/// CAN_IER
pub mod can_ier {
    pub const OFFSET: u32 = 0x14;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// SLKIE
        pub slkie: bool,
        /// WKUIE
        pub wkuie: bool,
        /// ERRIE
        pub errie: bool,
        /// LECIE
        pub lecie: bool,
        /// BOFIE
        pub bofie: bool,
        /// EPVIE
        pub epvie: bool,
        /// EWGIE
        pub ewgie: bool,
        /// FOVIE1
        pub fovie1: bool,
        /// FFIE1
        pub ffie1: bool,
        /// FMPIE1
        pub fmpie1: bool,
        /// FOVIE0
        pub fovie0: bool,
        /// FFIE0
        pub ffie0: bool,
        /// FMPIE0
        pub fmpie0: bool,
        /// TMEIE
        pub tmeie: bool,
    }
    pub struct Cache {
        /// SLKIE
        pub slkie: bool,
        /// WKUIE
        pub wkuie: bool,
        /// ERRIE
        pub errie: bool,
        /// LECIE
        pub lecie: bool,
        /// BOFIE
        pub bofie: bool,
        /// EPVIE
        pub epvie: bool,
        /// EWGIE
        pub ewgie: bool,
        /// FOVIE1
        pub fovie1: bool,
        /// FFIE1
        pub ffie1: bool,
        /// FMPIE1
        pub fmpie1: bool,
        /// FOVIE0
        pub fovie0: bool,
        /// FFIE0
        pub ffie0: bool,
        /// FMPIE0
        pub fmpie0: bool,
        /// TMEIE
        pub tmeie: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            slkie: ((value >> 17) & 0b1) > 0,
            wkuie: ((value >> 16) & 0b1) > 0,
            errie: ((value >> 15) & 0b1) > 0,
            lecie: ((value >> 11) & 0b1) > 0,
            bofie: ((value >> 10) & 0b1) > 0,
            epvie: ((value >> 9) & 0b1) > 0,
            ewgie: ((value >> 8) & 0b1) > 0,
            fovie1: ((value >> 6) & 0b1) > 0,
            ffie1: ((value >> 5) & 0b1) > 0,
            fmpie1: ((value >> 4) & 0b1) > 0,
            fovie0: ((value >> 3) & 0b1) > 0,
            ffie0: ((value >> 2) & 0b1) > 0,
            fmpie0: ((value >> 1) & 0b1) > 0,
            tmeie: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            slkie: ((value >> 17) & 0b1) > 0,
            wkuie: ((value >> 16) & 0b1) > 0,
            errie: ((value >> 15) & 0b1) > 0,
            lecie: ((value >> 11) & 0b1) > 0,
            bofie: ((value >> 10) & 0b1) > 0,
            epvie: ((value >> 9) & 0b1) > 0,
            ewgie: ((value >> 8) & 0b1) > 0,
            fovie1: ((value >> 6) & 0b1) > 0,
            ffie1: ((value >> 5) & 0b1) > 0,
            fmpie1: ((value >> 4) & 0b1) > 0,
            fovie0: ((value >> 3) & 0b1) > 0,
            ffie0: ((value >> 2) & 0b1) > 0,
            fmpie0: ((value >> 1) & 0b1) > 0,
            tmeie: ((value >> 0) & 0b1) > 0,
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
                | ((self.slkie as u32) << 17)
                | ((self.wkuie as u32) << 16)
                | ((self.errie as u32) << 15)
                | ((self.lecie as u32) << 11)
                | ((self.bofie as u32) << 10)
                | ((self.epvie as u32) << 9)
                | ((self.ewgie as u32) << 8)
                | ((self.fovie1 as u32) << 6)
                | ((self.ffie1 as u32) << 5)
                | ((self.fmpie1 as u32) << 4)
                | ((self.fovie0 as u32) << 3)
                | ((self.ffie0 as u32) << 2)
                | ((self.fmpie0 as u32) << 1)
                | ((self.tmeie as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// CAN_ESR
pub mod can_esr {
    pub const OFFSET: u32 = 0x18;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// REC
    /// Access: read-only, Width: 8, Offset: 24
    /// Get REC
    pub fn rec() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b11111111 << 24);
        value as u8
    }
    /// TEC
    /// Access: read-only, Width: 8, Offset: 16
    /// Get TEC
    pub fn tec() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b11111111 << 16);
        value as u8
    }
    /// LEC
    /// Access: read-write, Width: 3, Offset: 4
    /// Set LEC
    pub fn set_lec(value: u8) {
        debug_assert!(value <= 0b111, "set_lec out of range");
        let value = value as u32;
        let value = value << 4;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get LEC
    pub fn get_lec() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b111 << 4);
        value as u8
    }
    /// BOFF
    /// Access: read-only, Width: 1, Offset: 2
    /// Get BOFF
    pub fn boff() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 2);
        value > 0
    }
    /// EPVF
    /// Access: read-only, Width: 1, Offset: 1
    /// Get EPVF
    pub fn epvf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 1);
        value > 0
    }
    /// EWGF
    /// Access: read-only, Width: 1, Offset: 0
    /// Get EWGF
    pub fn ewgf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
}
/// CAN_BTR
pub mod can_btr {
    pub const OFFSET: u32 = 0x1C;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// SILM
        pub silm: bool,
        /// LBKM
        pub lbkm: bool,
        /// SJW
        pub sjw: bool,
        /// TS2
        pub ts2: bool,
        /// TS1
        pub ts1: bool,
        /// BRP
        pub brp: bool,
    }
    pub struct Cache {
        /// SILM
        pub silm: bool,
        /// LBKM
        pub lbkm: bool,
        /// SJW
        pub sjw: bool,
        /// TS2
        pub ts2: bool,
        /// TS1
        pub ts1: bool,
        /// BRP
        pub brp: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            silm: ((value >> 31) & 0b1) > 0,
            lbkm: ((value >> 30) & 0b1) > 0,
            sjw: ((value >> 24) & 0b1) > 0,
            ts2: ((value >> 20) & 0b1) > 0,
            ts1: ((value >> 16) & 0b1) > 0,
            brp: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            silm: ((value >> 31) & 0b1) > 0,
            lbkm: ((value >> 30) & 0b1) > 0,
            sjw: ((value >> 24) & 0b1) > 0,
            ts2: ((value >> 20) & 0b1) > 0,
            ts1: ((value >> 16) & 0b1) > 0,
            brp: ((value >> 0) & 0b1) > 0,
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
                | ((self.silm as u32) << 31)
                | ((self.lbkm as u32) << 30)
                | ((self.sjw as u32) << 24)
                | ((self.ts2 as u32) << 20)
                | ((self.ts1 as u32) << 16)
                | ((self.brp as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// CAN_TI0R
pub mod can_ti0r {
    pub const OFFSET: u32 = 0x180;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// STID
        pub stid: u16,
        /// EXID
        pub exid: u16,
        /// IDE
        pub ide: u16,
        /// RTR
        pub rtr: u16,
        /// TXRQ
        pub txrq: u16,
    }
    pub struct Cache {
        /// STID
        pub stid: u16,
        /// EXID
        pub exid: u16,
        /// IDE
        pub ide: u16,
        /// RTR
        pub rtr: u16,
        /// TXRQ
        pub txrq: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            stid: ((value >> 21) & 0b11111111111) as u16,
            exid: ((value >> 3) & 0b11111111111) as u16,
            ide: ((value >> 2) & 0b11111111111) as u16,
            rtr: ((value >> 1) & 0b11111111111) as u16,
            txrq: ((value >> 0) & 0b11111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            stid: ((value >> 21) & 0b11111111111) as u16,
            exid: ((value >> 3) & 0b11111111111) as u16,
            ide: ((value >> 2) & 0b11111111111) as u16,
            rtr: ((value >> 1) & 0b11111111111) as u16,
            txrq: ((value >> 0) & 0b11111111111) as u16,
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
                | ((self.stid as u32) << 21)
                | ((self.exid as u32) << 3)
                | ((self.ide as u32) << 2)
                | ((self.rtr as u32) << 1)
                | ((self.txrq as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// CAN_TDT0R
pub mod can_tdt0r {
    pub const OFFSET: u32 = 0x184;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// TIME
        pub time: u16,
        /// TGT
        pub tgt: u16,
        /// DLC
        pub dlc: u16,
    }
    pub struct Cache {
        /// TIME
        pub time: u16,
        /// TGT
        pub tgt: u16,
        /// DLC
        pub dlc: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            time: ((value >> 16) & 0b1111111111111111) as u16,
            tgt: ((value >> 8) & 0b1111111111111111) as u16,
            dlc: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            time: ((value >> 16) & 0b1111111111111111) as u16,
            tgt: ((value >> 8) & 0b1111111111111111) as u16,
            dlc: ((value >> 0) & 0b1111111111111111) as u16,
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
                | ((self.time as u32) << 16)
                | ((self.tgt as u32) << 8)
                | ((self.dlc as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// CAN_TDL0R
pub mod can_tdl0r {
    pub const OFFSET: u32 = 0x188;
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
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
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
                | ((self.0[1] as u32) << 8)
                | ((self.0[2] as u32) << 16)
                | ((self.0[3] as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// CAN_TDH0R
pub mod can_tdh0r {
    pub const OFFSET: u32 = 0x18C;
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
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
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
                | ((self.0[1] as u32) << 8)
                | ((self.0[2] as u32) << 16)
                | ((self.0[3] as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// CAN_TI1R
pub mod can_ti1r {
    pub const OFFSET: u32 = 0x190;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// STID
        pub stid: u16,
        /// EXID
        pub exid: u16,
        /// IDE
        pub ide: u16,
        /// RTR
        pub rtr: u16,
        /// TXRQ
        pub txrq: u16,
    }
    pub struct Cache {
        /// STID
        pub stid: u16,
        /// EXID
        pub exid: u16,
        /// IDE
        pub ide: u16,
        /// RTR
        pub rtr: u16,
        /// TXRQ
        pub txrq: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            stid: ((value >> 21) & 0b11111111111) as u16,
            exid: ((value >> 3) & 0b11111111111) as u16,
            ide: ((value >> 2) & 0b11111111111) as u16,
            rtr: ((value >> 1) & 0b11111111111) as u16,
            txrq: ((value >> 0) & 0b11111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            stid: ((value >> 21) & 0b11111111111) as u16,
            exid: ((value >> 3) & 0b11111111111) as u16,
            ide: ((value >> 2) & 0b11111111111) as u16,
            rtr: ((value >> 1) & 0b11111111111) as u16,
            txrq: ((value >> 0) & 0b11111111111) as u16,
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
                | ((self.stid as u32) << 21)
                | ((self.exid as u32) << 3)
                | ((self.ide as u32) << 2)
                | ((self.rtr as u32) << 1)
                | ((self.txrq as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// CAN_TDT1R
pub mod can_tdt1r {
    pub const OFFSET: u32 = 0x194;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// TIME
        pub time: u16,
        /// TGT
        pub tgt: u16,
        /// DLC
        pub dlc: u16,
    }
    pub struct Cache {
        /// TIME
        pub time: u16,
        /// TGT
        pub tgt: u16,
        /// DLC
        pub dlc: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            time: ((value >> 16) & 0b1111111111111111) as u16,
            tgt: ((value >> 8) & 0b1111111111111111) as u16,
            dlc: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            time: ((value >> 16) & 0b1111111111111111) as u16,
            tgt: ((value >> 8) & 0b1111111111111111) as u16,
            dlc: ((value >> 0) & 0b1111111111111111) as u16,
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
                | ((self.time as u32) << 16)
                | ((self.tgt as u32) << 8)
                | ((self.dlc as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// CAN_TDL1R
pub mod can_tdl1r {
    pub const OFFSET: u32 = 0x198;
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
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
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
                | ((self.0[1] as u32) << 8)
                | ((self.0[2] as u32) << 16)
                | ((self.0[3] as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// CAN_TDH1R
pub mod can_tdh1r {
    pub const OFFSET: u32 = 0x19C;
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
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
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
                | ((self.0[1] as u32) << 8)
                | ((self.0[2] as u32) << 16)
                | ((self.0[3] as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// CAN_TI2R
pub mod can_ti2r {
    pub const OFFSET: u32 = 0x1A0;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// STID
        pub stid: u16,
        /// EXID
        pub exid: u16,
        /// IDE
        pub ide: u16,
        /// RTR
        pub rtr: u16,
        /// TXRQ
        pub txrq: u16,
    }
    pub struct Cache {
        /// STID
        pub stid: u16,
        /// EXID
        pub exid: u16,
        /// IDE
        pub ide: u16,
        /// RTR
        pub rtr: u16,
        /// TXRQ
        pub txrq: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            stid: ((value >> 21) & 0b11111111111) as u16,
            exid: ((value >> 3) & 0b11111111111) as u16,
            ide: ((value >> 2) & 0b11111111111) as u16,
            rtr: ((value >> 1) & 0b11111111111) as u16,
            txrq: ((value >> 0) & 0b11111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            stid: ((value >> 21) & 0b11111111111) as u16,
            exid: ((value >> 3) & 0b11111111111) as u16,
            ide: ((value >> 2) & 0b11111111111) as u16,
            rtr: ((value >> 1) & 0b11111111111) as u16,
            txrq: ((value >> 0) & 0b11111111111) as u16,
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
                | ((self.stid as u32) << 21)
                | ((self.exid as u32) << 3)
                | ((self.ide as u32) << 2)
                | ((self.rtr as u32) << 1)
                | ((self.txrq as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// CAN_TDT2R
pub mod can_tdt2r {
    pub const OFFSET: u32 = 0x1A4;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// TIME
        pub time: u16,
        /// TGT
        pub tgt: u16,
        /// DLC
        pub dlc: u16,
    }
    pub struct Cache {
        /// TIME
        pub time: u16,
        /// TGT
        pub tgt: u16,
        /// DLC
        pub dlc: u16,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            time: ((value >> 16) & 0b1111111111111111) as u16,
            tgt: ((value >> 8) & 0b1111111111111111) as u16,
            dlc: ((value >> 0) & 0b1111111111111111) as u16,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            time: ((value >> 16) & 0b1111111111111111) as u16,
            tgt: ((value >> 8) & 0b1111111111111111) as u16,
            dlc: ((value >> 0) & 0b1111111111111111) as u16,
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
                | ((self.time as u32) << 16)
                | ((self.tgt as u32) << 8)
                | ((self.dlc as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// CAN_TDL2R
pub mod can_tdl2r {
    pub const OFFSET: u32 = 0x1A8;
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
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
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
                | ((self.0[1] as u32) << 8)
                | ((self.0[2] as u32) << 16)
                | ((self.0[3] as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// CAN_TDH2R
pub mod can_tdh2r {
    pub const OFFSET: u32 = 0x1AC;
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
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b11111111) as u8,
            ((value >> 8) & 0b11111111) as u8,
            ((value >> 16) & 0b11111111) as u8,
            ((value >> 24) & 0b11111111) as u8,
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
                | ((self.0[1] as u32) << 8)
                | ((self.0[2] as u32) << 16)
                | ((self.0[3] as u32) << 24)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// CAN_RI0R
pub mod can_ri0r {
    pub const OFFSET: u32 = 0x1B0;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// STID
    /// Access: read-only, Width: 11, Offset: 21
    /// Get STID
    pub fn stid() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b11111111111 << 21);
        value as u16
    }
    /// EXID
    /// Access: read-only, Width: 18, Offset: 3
    /// Get EXID
    pub fn exid() -> u32 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b111111111111111111 << 3);
        value as u32
    }
    /// IDE
    /// Access: read-only, Width: 1, Offset: 2
    /// Get IDE
    pub fn ide() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 2);
        value > 0
    }
    /// RTR
    /// Access: read-only, Width: 1, Offset: 1
    /// Get RTR
    pub fn rtr() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 1);
        value > 0
    }
}
/// CAN_RDT0R
pub mod can_rdt0r {
    pub const OFFSET: u32 = 0x1B4;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// TIME
    /// Access: read-only, Width: 16, Offset: 16
    /// Get TIME
    pub fn time() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1111111111111111 << 16);
        value as u16
    }
    /// FMI
    /// Access: read-only, Width: 8, Offset: 8
    /// Get FMI
    pub fn fmi() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b11111111 << 8);
        value as u8
    }
    /// DLC
    /// Access: read-only, Width: 4, Offset: 0
    /// Get DLC
    pub fn dlc() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1111 << 0);
        value as u8
    }
}
/// CAN_RDL0R
pub mod can_rdl0r {
    pub const OFFSET: u32 = 0x1B8;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// Get DATA3
    pub fn data(index: u8) -> u8 {
        debug_assert!(index < 4, "data out of range");
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b11111111 << (0 + index * 8));
        value as u8
    }
}
/// CAN_RDH0R
pub mod can_rdh0r {
    pub const OFFSET: u32 = 0x1BC;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// Get DATA7
    pub fn data(index: u8) -> u8 {
        debug_assert!(index < 4, "data out of range");
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b11111111 << (0 + index * 8));
        value as u8
    }
}
/// CAN_RI1R
pub mod can_ri1r {
    pub const OFFSET: u32 = 0x1C0;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// STID
    /// Access: read-only, Width: 11, Offset: 21
    /// Get STID
    pub fn stid() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b11111111111 << 21);
        value as u16
    }
    /// EXID
    /// Access: read-only, Width: 18, Offset: 3
    /// Get EXID
    pub fn exid() -> u32 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b111111111111111111 << 3);
        value as u32
    }
    /// IDE
    /// Access: read-only, Width: 1, Offset: 2
    /// Get IDE
    pub fn ide() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 2);
        value > 0
    }
    /// RTR
    /// Access: read-only, Width: 1, Offset: 1
    /// Get RTR
    pub fn rtr() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 1);
        value > 0
    }
}
/// CAN_RDT1R
pub mod can_rdt1r {
    pub const OFFSET: u32 = 0x1C4;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// TIME
    /// Access: read-only, Width: 16, Offset: 16
    /// Get TIME
    pub fn time() -> u16 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1111111111111111 << 16);
        value as u16
    }
    /// FMI
    /// Access: read-only, Width: 8, Offset: 8
    /// Get FMI
    pub fn fmi() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b11111111 << 8);
        value as u8
    }
    /// DLC
    /// Access: read-only, Width: 4, Offset: 0
    /// Get DLC
    pub fn dlc() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1111 << 0);
        value as u8
    }
}
/// CAN_RDL1R
pub mod can_rdl1r {
    pub const OFFSET: u32 = 0x1C8;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// Get DATA3
    pub fn data(index: u8) -> u8 {
        debug_assert!(index < 4, "data out of range");
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b11111111 << (0 + index * 8));
        value as u8
    }
}
/// CAN_RDH1R
pub mod can_rdh1r {
    pub const OFFSET: u32 = 0x1CC;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// Get DATA7
    pub fn data(index: u8) -> u8 {
        debug_assert!(index < 4, "data out of range");
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b11111111 << (0 + index * 8));
        value as u8
    }
}
/// CAN_FMR
pub mod can_fmr {
    pub const OFFSET: u32 = 0x200;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// FINIT
        pub finit: bool,
    }
    pub struct Cache {
        /// FINIT
        pub finit: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            finit: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            finit: ((value >> 0) & 0b1) > 0,
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
                | ((self.finit as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// CAN_FM1R
pub mod can_fm1r {
    pub const OFFSET: u32 = 0x204;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([bool;14]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([bool;14]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
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
                | ((self.0[1] as u32) << 1)
                | ((self.0[2] as u32) << 2)
                | ((self.0[3] as u32) << 3)
                | ((self.0[4] as u32) << 4)
                | ((self.0[5] as u32) << 5)
                | ((self.0[6] as u32) << 6)
                | ((self.0[7] as u32) << 7)
                | ((self.0[8] as u32) << 8)
                | ((self.0[9] as u32) << 9)
                | ((self.0[10] as u32) << 10)
                | ((self.0[11] as u32) << 11)
                | ((self.0[12] as u32) << 12)
                | ((self.0[13] as u32) << 13)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// CAN_FS1R
pub mod can_fs1r {
    pub const OFFSET: u32 = 0x20C;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([bool;14]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([bool;14]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
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
                | ((self.0[1] as u32) << 1)
                | ((self.0[2] as u32) << 2)
                | ((self.0[3] as u32) << 3)
                | ((self.0[4] as u32) << 4)
                | ((self.0[5] as u32) << 5)
                | ((self.0[6] as u32) << 6)
                | ((self.0[7] as u32) << 7)
                | ((self.0[8] as u32) << 8)
                | ((self.0[9] as u32) << 9)
                | ((self.0[10] as u32) << 10)
                | ((self.0[11] as u32) << 11)
                | ((self.0[12] as u32) << 12)
                | ((self.0[13] as u32) << 13)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// CAN_FFA1R
pub mod can_ffa1r {
    pub const OFFSET: u32 = 0x214;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([bool;14]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([bool;14]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
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
                | ((self.0[1] as u32) << 1)
                | ((self.0[2] as u32) << 2)
                | ((self.0[3] as u32) << 3)
                | ((self.0[4] as u32) << 4)
                | ((self.0[5] as u32) << 5)
                | ((self.0[6] as u32) << 6)
                | ((self.0[7] as u32) << 7)
                | ((self.0[8] as u32) << 8)
                | ((self.0[9] as u32) << 9)
                | ((self.0[10] as u32) << 10)
                | ((self.0[11] as u32) << 11)
                | ((self.0[12] as u32) << 12)
                | ((self.0[13] as u32) << 13)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// CAN_FA1R
pub mod can_fa1r {
    pub const OFFSET: u32 = 0x21C;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([bool;14]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([bool;14]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
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
                | ((self.0[1] as u32) << 1)
                | ((self.0[2] as u32) << 2)
                | ((self.0[3] as u32) << 3)
                | ((self.0[4] as u32) << 4)
                | ((self.0[5] as u32) << 5)
                | ((self.0[6] as u32) << 6)
                | ((self.0[7] as u32) << 7)
                | ((self.0[8] as u32) << 8)
                | ((self.0[9] as u32) << 9)
                | ((self.0[10] as u32) << 10)
                | ((self.0[11] as u32) << 11)
                | ((self.0[12] as u32) << 12)
                | ((self.0[13] as u32) << 13)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Filter bank 0 register 1
pub mod f0r1 {
    pub const OFFSET: u32 = 0x240;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([bool;32]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([bool;32]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
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
                | ((self.0[1] as u32) << 1)
                | ((self.0[2] as u32) << 2)
                | ((self.0[3] as u32) << 3)
                | ((self.0[4] as u32) << 4)
                | ((self.0[5] as u32) << 5)
                | ((self.0[6] as u32) << 6)
                | ((self.0[7] as u32) << 7)
                | ((self.0[8] as u32) << 8)
                | ((self.0[9] as u32) << 9)
                | ((self.0[10] as u32) << 10)
                | ((self.0[11] as u32) << 11)
                | ((self.0[12] as u32) << 12)
                | ((self.0[13] as u32) << 13)
                | ((self.0[14] as u32) << 14)
                | ((self.0[15] as u32) << 15)
                | ((self.0[16] as u32) << 16)
                | ((self.0[17] as u32) << 17)
                | ((self.0[18] as u32) << 18)
                | ((self.0[19] as u32) << 19)
                | ((self.0[20] as u32) << 20)
                | ((self.0[21] as u32) << 21)
                | ((self.0[22] as u32) << 22)
                | ((self.0[23] as u32) << 23)
                | ((self.0[24] as u32) << 24)
                | ((self.0[25] as u32) << 25)
                | ((self.0[26] as u32) << 26)
                | ((self.0[27] as u32) << 27)
                | ((self.0[28] as u32) << 28)
                | ((self.0[29] as u32) << 29)
                | ((self.0[30] as u32) << 30)
                | ((self.0[31] as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Filter bank 0 register 2
pub mod f0r2 {
    pub const OFFSET: u32 = 0x244;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([bool;32]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([bool;32]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
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
                | ((self.0[1] as u32) << 1)
                | ((self.0[2] as u32) << 2)
                | ((self.0[3] as u32) << 3)
                | ((self.0[4] as u32) << 4)
                | ((self.0[5] as u32) << 5)
                | ((self.0[6] as u32) << 6)
                | ((self.0[7] as u32) << 7)
                | ((self.0[8] as u32) << 8)
                | ((self.0[9] as u32) << 9)
                | ((self.0[10] as u32) << 10)
                | ((self.0[11] as u32) << 11)
                | ((self.0[12] as u32) << 12)
                | ((self.0[13] as u32) << 13)
                | ((self.0[14] as u32) << 14)
                | ((self.0[15] as u32) << 15)
                | ((self.0[16] as u32) << 16)
                | ((self.0[17] as u32) << 17)
                | ((self.0[18] as u32) << 18)
                | ((self.0[19] as u32) << 19)
                | ((self.0[20] as u32) << 20)
                | ((self.0[21] as u32) << 21)
                | ((self.0[22] as u32) << 22)
                | ((self.0[23] as u32) << 23)
                | ((self.0[24] as u32) << 24)
                | ((self.0[25] as u32) << 25)
                | ((self.0[26] as u32) << 26)
                | ((self.0[27] as u32) << 27)
                | ((self.0[28] as u32) << 28)
                | ((self.0[29] as u32) << 29)
                | ((self.0[30] as u32) << 30)
                | ((self.0[31] as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Filter bank 1 register 1
pub mod f1r1 {
    pub const OFFSET: u32 = 0x248;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([bool;32]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([bool;32]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
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
                | ((self.0[1] as u32) << 1)
                | ((self.0[2] as u32) << 2)
                | ((self.0[3] as u32) << 3)
                | ((self.0[4] as u32) << 4)
                | ((self.0[5] as u32) << 5)
                | ((self.0[6] as u32) << 6)
                | ((self.0[7] as u32) << 7)
                | ((self.0[8] as u32) << 8)
                | ((self.0[9] as u32) << 9)
                | ((self.0[10] as u32) << 10)
                | ((self.0[11] as u32) << 11)
                | ((self.0[12] as u32) << 12)
                | ((self.0[13] as u32) << 13)
                | ((self.0[14] as u32) << 14)
                | ((self.0[15] as u32) << 15)
                | ((self.0[16] as u32) << 16)
                | ((self.0[17] as u32) << 17)
                | ((self.0[18] as u32) << 18)
                | ((self.0[19] as u32) << 19)
                | ((self.0[20] as u32) << 20)
                | ((self.0[21] as u32) << 21)
                | ((self.0[22] as u32) << 22)
                | ((self.0[23] as u32) << 23)
                | ((self.0[24] as u32) << 24)
                | ((self.0[25] as u32) << 25)
                | ((self.0[26] as u32) << 26)
                | ((self.0[27] as u32) << 27)
                | ((self.0[28] as u32) << 28)
                | ((self.0[29] as u32) << 29)
                | ((self.0[30] as u32) << 30)
                | ((self.0[31] as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Filter bank 1 register 2
pub mod f1r2 {
    pub const OFFSET: u32 = 0x24C;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([bool;32]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([bool;32]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
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
                | ((self.0[1] as u32) << 1)
                | ((self.0[2] as u32) << 2)
                | ((self.0[3] as u32) << 3)
                | ((self.0[4] as u32) << 4)
                | ((self.0[5] as u32) << 5)
                | ((self.0[6] as u32) << 6)
                | ((self.0[7] as u32) << 7)
                | ((self.0[8] as u32) << 8)
                | ((self.0[9] as u32) << 9)
                | ((self.0[10] as u32) << 10)
                | ((self.0[11] as u32) << 11)
                | ((self.0[12] as u32) << 12)
                | ((self.0[13] as u32) << 13)
                | ((self.0[14] as u32) << 14)
                | ((self.0[15] as u32) << 15)
                | ((self.0[16] as u32) << 16)
                | ((self.0[17] as u32) << 17)
                | ((self.0[18] as u32) << 18)
                | ((self.0[19] as u32) << 19)
                | ((self.0[20] as u32) << 20)
                | ((self.0[21] as u32) << 21)
                | ((self.0[22] as u32) << 22)
                | ((self.0[23] as u32) << 23)
                | ((self.0[24] as u32) << 24)
                | ((self.0[25] as u32) << 25)
                | ((self.0[26] as u32) << 26)
                | ((self.0[27] as u32) << 27)
                | ((self.0[28] as u32) << 28)
                | ((self.0[29] as u32) << 29)
                | ((self.0[30] as u32) << 30)
                | ((self.0[31] as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Filter bank 2 register 1
pub mod f2r1 {
    pub const OFFSET: u32 = 0x250;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([bool;32]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([bool;32]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
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
                | ((self.0[1] as u32) << 1)
                | ((self.0[2] as u32) << 2)
                | ((self.0[3] as u32) << 3)
                | ((self.0[4] as u32) << 4)
                | ((self.0[5] as u32) << 5)
                | ((self.0[6] as u32) << 6)
                | ((self.0[7] as u32) << 7)
                | ((self.0[8] as u32) << 8)
                | ((self.0[9] as u32) << 9)
                | ((self.0[10] as u32) << 10)
                | ((self.0[11] as u32) << 11)
                | ((self.0[12] as u32) << 12)
                | ((self.0[13] as u32) << 13)
                | ((self.0[14] as u32) << 14)
                | ((self.0[15] as u32) << 15)
                | ((self.0[16] as u32) << 16)
                | ((self.0[17] as u32) << 17)
                | ((self.0[18] as u32) << 18)
                | ((self.0[19] as u32) << 19)
                | ((self.0[20] as u32) << 20)
                | ((self.0[21] as u32) << 21)
                | ((self.0[22] as u32) << 22)
                | ((self.0[23] as u32) << 23)
                | ((self.0[24] as u32) << 24)
                | ((self.0[25] as u32) << 25)
                | ((self.0[26] as u32) << 26)
                | ((self.0[27] as u32) << 27)
                | ((self.0[28] as u32) << 28)
                | ((self.0[29] as u32) << 29)
                | ((self.0[30] as u32) << 30)
                | ((self.0[31] as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Filter bank 2 register 2
pub mod f2r2 {
    pub const OFFSET: u32 = 0x254;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([bool;32]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([bool;32]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
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
                | ((self.0[1] as u32) << 1)
                | ((self.0[2] as u32) << 2)
                | ((self.0[3] as u32) << 3)
                | ((self.0[4] as u32) << 4)
                | ((self.0[5] as u32) << 5)
                | ((self.0[6] as u32) << 6)
                | ((self.0[7] as u32) << 7)
                | ((self.0[8] as u32) << 8)
                | ((self.0[9] as u32) << 9)
                | ((self.0[10] as u32) << 10)
                | ((self.0[11] as u32) << 11)
                | ((self.0[12] as u32) << 12)
                | ((self.0[13] as u32) << 13)
                | ((self.0[14] as u32) << 14)
                | ((self.0[15] as u32) << 15)
                | ((self.0[16] as u32) << 16)
                | ((self.0[17] as u32) << 17)
                | ((self.0[18] as u32) << 18)
                | ((self.0[19] as u32) << 19)
                | ((self.0[20] as u32) << 20)
                | ((self.0[21] as u32) << 21)
                | ((self.0[22] as u32) << 22)
                | ((self.0[23] as u32) << 23)
                | ((self.0[24] as u32) << 24)
                | ((self.0[25] as u32) << 25)
                | ((self.0[26] as u32) << 26)
                | ((self.0[27] as u32) << 27)
                | ((self.0[28] as u32) << 28)
                | ((self.0[29] as u32) << 29)
                | ((self.0[30] as u32) << 30)
                | ((self.0[31] as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Filter bank 3 register 1
pub mod f3r1 {
    pub const OFFSET: u32 = 0x258;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([bool;32]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([bool;32]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
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
                | ((self.0[1] as u32) << 1)
                | ((self.0[2] as u32) << 2)
                | ((self.0[3] as u32) << 3)
                | ((self.0[4] as u32) << 4)
                | ((self.0[5] as u32) << 5)
                | ((self.0[6] as u32) << 6)
                | ((self.0[7] as u32) << 7)
                | ((self.0[8] as u32) << 8)
                | ((self.0[9] as u32) << 9)
                | ((self.0[10] as u32) << 10)
                | ((self.0[11] as u32) << 11)
                | ((self.0[12] as u32) << 12)
                | ((self.0[13] as u32) << 13)
                | ((self.0[14] as u32) << 14)
                | ((self.0[15] as u32) << 15)
                | ((self.0[16] as u32) << 16)
                | ((self.0[17] as u32) << 17)
                | ((self.0[18] as u32) << 18)
                | ((self.0[19] as u32) << 19)
                | ((self.0[20] as u32) << 20)
                | ((self.0[21] as u32) << 21)
                | ((self.0[22] as u32) << 22)
                | ((self.0[23] as u32) << 23)
                | ((self.0[24] as u32) << 24)
                | ((self.0[25] as u32) << 25)
                | ((self.0[26] as u32) << 26)
                | ((self.0[27] as u32) << 27)
                | ((self.0[28] as u32) << 28)
                | ((self.0[29] as u32) << 29)
                | ((self.0[30] as u32) << 30)
                | ((self.0[31] as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Filter bank 3 register 2
pub mod f3r2 {
    pub const OFFSET: u32 = 0x25C;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([bool;32]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([bool;32]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
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
                | ((self.0[1] as u32) << 1)
                | ((self.0[2] as u32) << 2)
                | ((self.0[3] as u32) << 3)
                | ((self.0[4] as u32) << 4)
                | ((self.0[5] as u32) << 5)
                | ((self.0[6] as u32) << 6)
                | ((self.0[7] as u32) << 7)
                | ((self.0[8] as u32) << 8)
                | ((self.0[9] as u32) << 9)
                | ((self.0[10] as u32) << 10)
                | ((self.0[11] as u32) << 11)
                | ((self.0[12] as u32) << 12)
                | ((self.0[13] as u32) << 13)
                | ((self.0[14] as u32) << 14)
                | ((self.0[15] as u32) << 15)
                | ((self.0[16] as u32) << 16)
                | ((self.0[17] as u32) << 17)
                | ((self.0[18] as u32) << 18)
                | ((self.0[19] as u32) << 19)
                | ((self.0[20] as u32) << 20)
                | ((self.0[21] as u32) << 21)
                | ((self.0[22] as u32) << 22)
                | ((self.0[23] as u32) << 23)
                | ((self.0[24] as u32) << 24)
                | ((self.0[25] as u32) << 25)
                | ((self.0[26] as u32) << 26)
                | ((self.0[27] as u32) << 27)
                | ((self.0[28] as u32) << 28)
                | ((self.0[29] as u32) << 29)
                | ((self.0[30] as u32) << 30)
                | ((self.0[31] as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Filter bank 4 register 1
pub mod f4r1 {
    pub const OFFSET: u32 = 0x260;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([bool;32]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([bool;32]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
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
                | ((self.0[1] as u32) << 1)
                | ((self.0[2] as u32) << 2)
                | ((self.0[3] as u32) << 3)
                | ((self.0[4] as u32) << 4)
                | ((self.0[5] as u32) << 5)
                | ((self.0[6] as u32) << 6)
                | ((self.0[7] as u32) << 7)
                | ((self.0[8] as u32) << 8)
                | ((self.0[9] as u32) << 9)
                | ((self.0[10] as u32) << 10)
                | ((self.0[11] as u32) << 11)
                | ((self.0[12] as u32) << 12)
                | ((self.0[13] as u32) << 13)
                | ((self.0[14] as u32) << 14)
                | ((self.0[15] as u32) << 15)
                | ((self.0[16] as u32) << 16)
                | ((self.0[17] as u32) << 17)
                | ((self.0[18] as u32) << 18)
                | ((self.0[19] as u32) << 19)
                | ((self.0[20] as u32) << 20)
                | ((self.0[21] as u32) << 21)
                | ((self.0[22] as u32) << 22)
                | ((self.0[23] as u32) << 23)
                | ((self.0[24] as u32) << 24)
                | ((self.0[25] as u32) << 25)
                | ((self.0[26] as u32) << 26)
                | ((self.0[27] as u32) << 27)
                | ((self.0[28] as u32) << 28)
                | ((self.0[29] as u32) << 29)
                | ((self.0[30] as u32) << 30)
                | ((self.0[31] as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Filter bank 4 register 2
pub mod f4r2 {
    pub const OFFSET: u32 = 0x264;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([bool;32]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([bool;32]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
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
                | ((self.0[1] as u32) << 1)
                | ((self.0[2] as u32) << 2)
                | ((self.0[3] as u32) << 3)
                | ((self.0[4] as u32) << 4)
                | ((self.0[5] as u32) << 5)
                | ((self.0[6] as u32) << 6)
                | ((self.0[7] as u32) << 7)
                | ((self.0[8] as u32) << 8)
                | ((self.0[9] as u32) << 9)
                | ((self.0[10] as u32) << 10)
                | ((self.0[11] as u32) << 11)
                | ((self.0[12] as u32) << 12)
                | ((self.0[13] as u32) << 13)
                | ((self.0[14] as u32) << 14)
                | ((self.0[15] as u32) << 15)
                | ((self.0[16] as u32) << 16)
                | ((self.0[17] as u32) << 17)
                | ((self.0[18] as u32) << 18)
                | ((self.0[19] as u32) << 19)
                | ((self.0[20] as u32) << 20)
                | ((self.0[21] as u32) << 21)
                | ((self.0[22] as u32) << 22)
                | ((self.0[23] as u32) << 23)
                | ((self.0[24] as u32) << 24)
                | ((self.0[25] as u32) << 25)
                | ((self.0[26] as u32) << 26)
                | ((self.0[27] as u32) << 27)
                | ((self.0[28] as u32) << 28)
                | ((self.0[29] as u32) << 29)
                | ((self.0[30] as u32) << 30)
                | ((self.0[31] as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Filter bank 5 register 1
pub mod f5r1 {
    pub const OFFSET: u32 = 0x268;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([bool;32]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([bool;32]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
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
                | ((self.0[1] as u32) << 1)
                | ((self.0[2] as u32) << 2)
                | ((self.0[3] as u32) << 3)
                | ((self.0[4] as u32) << 4)
                | ((self.0[5] as u32) << 5)
                | ((self.0[6] as u32) << 6)
                | ((self.0[7] as u32) << 7)
                | ((self.0[8] as u32) << 8)
                | ((self.0[9] as u32) << 9)
                | ((self.0[10] as u32) << 10)
                | ((self.0[11] as u32) << 11)
                | ((self.0[12] as u32) << 12)
                | ((self.0[13] as u32) << 13)
                | ((self.0[14] as u32) << 14)
                | ((self.0[15] as u32) << 15)
                | ((self.0[16] as u32) << 16)
                | ((self.0[17] as u32) << 17)
                | ((self.0[18] as u32) << 18)
                | ((self.0[19] as u32) << 19)
                | ((self.0[20] as u32) << 20)
                | ((self.0[21] as u32) << 21)
                | ((self.0[22] as u32) << 22)
                | ((self.0[23] as u32) << 23)
                | ((self.0[24] as u32) << 24)
                | ((self.0[25] as u32) << 25)
                | ((self.0[26] as u32) << 26)
                | ((self.0[27] as u32) << 27)
                | ((self.0[28] as u32) << 28)
                | ((self.0[29] as u32) << 29)
                | ((self.0[30] as u32) << 30)
                | ((self.0[31] as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Filter bank 5 register 2
pub mod f5r2 {
    pub const OFFSET: u32 = 0x26C;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([bool;32]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([bool;32]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
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
                | ((self.0[1] as u32) << 1)
                | ((self.0[2] as u32) << 2)
                | ((self.0[3] as u32) << 3)
                | ((self.0[4] as u32) << 4)
                | ((self.0[5] as u32) << 5)
                | ((self.0[6] as u32) << 6)
                | ((self.0[7] as u32) << 7)
                | ((self.0[8] as u32) << 8)
                | ((self.0[9] as u32) << 9)
                | ((self.0[10] as u32) << 10)
                | ((self.0[11] as u32) << 11)
                | ((self.0[12] as u32) << 12)
                | ((self.0[13] as u32) << 13)
                | ((self.0[14] as u32) << 14)
                | ((self.0[15] as u32) << 15)
                | ((self.0[16] as u32) << 16)
                | ((self.0[17] as u32) << 17)
                | ((self.0[18] as u32) << 18)
                | ((self.0[19] as u32) << 19)
                | ((self.0[20] as u32) << 20)
                | ((self.0[21] as u32) << 21)
                | ((self.0[22] as u32) << 22)
                | ((self.0[23] as u32) << 23)
                | ((self.0[24] as u32) << 24)
                | ((self.0[25] as u32) << 25)
                | ((self.0[26] as u32) << 26)
                | ((self.0[27] as u32) << 27)
                | ((self.0[28] as u32) << 28)
                | ((self.0[29] as u32) << 29)
                | ((self.0[30] as u32) << 30)
                | ((self.0[31] as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Filter bank 6 register 1
pub mod f6r1 {
    pub const OFFSET: u32 = 0x270;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([bool;32]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([bool;32]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
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
                | ((self.0[1] as u32) << 1)
                | ((self.0[2] as u32) << 2)
                | ((self.0[3] as u32) << 3)
                | ((self.0[4] as u32) << 4)
                | ((self.0[5] as u32) << 5)
                | ((self.0[6] as u32) << 6)
                | ((self.0[7] as u32) << 7)
                | ((self.0[8] as u32) << 8)
                | ((self.0[9] as u32) << 9)
                | ((self.0[10] as u32) << 10)
                | ((self.0[11] as u32) << 11)
                | ((self.0[12] as u32) << 12)
                | ((self.0[13] as u32) << 13)
                | ((self.0[14] as u32) << 14)
                | ((self.0[15] as u32) << 15)
                | ((self.0[16] as u32) << 16)
                | ((self.0[17] as u32) << 17)
                | ((self.0[18] as u32) << 18)
                | ((self.0[19] as u32) << 19)
                | ((self.0[20] as u32) << 20)
                | ((self.0[21] as u32) << 21)
                | ((self.0[22] as u32) << 22)
                | ((self.0[23] as u32) << 23)
                | ((self.0[24] as u32) << 24)
                | ((self.0[25] as u32) << 25)
                | ((self.0[26] as u32) << 26)
                | ((self.0[27] as u32) << 27)
                | ((self.0[28] as u32) << 28)
                | ((self.0[29] as u32) << 29)
                | ((self.0[30] as u32) << 30)
                | ((self.0[31] as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Filter bank 6 register 2
pub mod f6r2 {
    pub const OFFSET: u32 = 0x274;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([bool;32]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([bool;32]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
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
                | ((self.0[1] as u32) << 1)
                | ((self.0[2] as u32) << 2)
                | ((self.0[3] as u32) << 3)
                | ((self.0[4] as u32) << 4)
                | ((self.0[5] as u32) << 5)
                | ((self.0[6] as u32) << 6)
                | ((self.0[7] as u32) << 7)
                | ((self.0[8] as u32) << 8)
                | ((self.0[9] as u32) << 9)
                | ((self.0[10] as u32) << 10)
                | ((self.0[11] as u32) << 11)
                | ((self.0[12] as u32) << 12)
                | ((self.0[13] as u32) << 13)
                | ((self.0[14] as u32) << 14)
                | ((self.0[15] as u32) << 15)
                | ((self.0[16] as u32) << 16)
                | ((self.0[17] as u32) << 17)
                | ((self.0[18] as u32) << 18)
                | ((self.0[19] as u32) << 19)
                | ((self.0[20] as u32) << 20)
                | ((self.0[21] as u32) << 21)
                | ((self.0[22] as u32) << 22)
                | ((self.0[23] as u32) << 23)
                | ((self.0[24] as u32) << 24)
                | ((self.0[25] as u32) << 25)
                | ((self.0[26] as u32) << 26)
                | ((self.0[27] as u32) << 27)
                | ((self.0[28] as u32) << 28)
                | ((self.0[29] as u32) << 29)
                | ((self.0[30] as u32) << 30)
                | ((self.0[31] as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Filter bank 7 register 1
pub mod f7r1 {
    pub const OFFSET: u32 = 0x278;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([bool;32]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([bool;32]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
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
                | ((self.0[1] as u32) << 1)
                | ((self.0[2] as u32) << 2)
                | ((self.0[3] as u32) << 3)
                | ((self.0[4] as u32) << 4)
                | ((self.0[5] as u32) << 5)
                | ((self.0[6] as u32) << 6)
                | ((self.0[7] as u32) << 7)
                | ((self.0[8] as u32) << 8)
                | ((self.0[9] as u32) << 9)
                | ((self.0[10] as u32) << 10)
                | ((self.0[11] as u32) << 11)
                | ((self.0[12] as u32) << 12)
                | ((self.0[13] as u32) << 13)
                | ((self.0[14] as u32) << 14)
                | ((self.0[15] as u32) << 15)
                | ((self.0[16] as u32) << 16)
                | ((self.0[17] as u32) << 17)
                | ((self.0[18] as u32) << 18)
                | ((self.0[19] as u32) << 19)
                | ((self.0[20] as u32) << 20)
                | ((self.0[21] as u32) << 21)
                | ((self.0[22] as u32) << 22)
                | ((self.0[23] as u32) << 23)
                | ((self.0[24] as u32) << 24)
                | ((self.0[25] as u32) << 25)
                | ((self.0[26] as u32) << 26)
                | ((self.0[27] as u32) << 27)
                | ((self.0[28] as u32) << 28)
                | ((self.0[29] as u32) << 29)
                | ((self.0[30] as u32) << 30)
                | ((self.0[31] as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Filter bank 7 register 2
pub mod f7r2 {
    pub const OFFSET: u32 = 0x27C;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([bool;32]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([bool;32]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
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
                | ((self.0[1] as u32) << 1)
                | ((self.0[2] as u32) << 2)
                | ((self.0[3] as u32) << 3)
                | ((self.0[4] as u32) << 4)
                | ((self.0[5] as u32) << 5)
                | ((self.0[6] as u32) << 6)
                | ((self.0[7] as u32) << 7)
                | ((self.0[8] as u32) << 8)
                | ((self.0[9] as u32) << 9)
                | ((self.0[10] as u32) << 10)
                | ((self.0[11] as u32) << 11)
                | ((self.0[12] as u32) << 12)
                | ((self.0[13] as u32) << 13)
                | ((self.0[14] as u32) << 14)
                | ((self.0[15] as u32) << 15)
                | ((self.0[16] as u32) << 16)
                | ((self.0[17] as u32) << 17)
                | ((self.0[18] as u32) << 18)
                | ((self.0[19] as u32) << 19)
                | ((self.0[20] as u32) << 20)
                | ((self.0[21] as u32) << 21)
                | ((self.0[22] as u32) << 22)
                | ((self.0[23] as u32) << 23)
                | ((self.0[24] as u32) << 24)
                | ((self.0[25] as u32) << 25)
                | ((self.0[26] as u32) << 26)
                | ((self.0[27] as u32) << 27)
                | ((self.0[28] as u32) << 28)
                | ((self.0[29] as u32) << 29)
                | ((self.0[30] as u32) << 30)
                | ((self.0[31] as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Filter bank 8 register 1
pub mod f8r1 {
    pub const OFFSET: u32 = 0x280;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([bool;32]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([bool;32]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
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
                | ((self.0[1] as u32) << 1)
                | ((self.0[2] as u32) << 2)
                | ((self.0[3] as u32) << 3)
                | ((self.0[4] as u32) << 4)
                | ((self.0[5] as u32) << 5)
                | ((self.0[6] as u32) << 6)
                | ((self.0[7] as u32) << 7)
                | ((self.0[8] as u32) << 8)
                | ((self.0[9] as u32) << 9)
                | ((self.0[10] as u32) << 10)
                | ((self.0[11] as u32) << 11)
                | ((self.0[12] as u32) << 12)
                | ((self.0[13] as u32) << 13)
                | ((self.0[14] as u32) << 14)
                | ((self.0[15] as u32) << 15)
                | ((self.0[16] as u32) << 16)
                | ((self.0[17] as u32) << 17)
                | ((self.0[18] as u32) << 18)
                | ((self.0[19] as u32) << 19)
                | ((self.0[20] as u32) << 20)
                | ((self.0[21] as u32) << 21)
                | ((self.0[22] as u32) << 22)
                | ((self.0[23] as u32) << 23)
                | ((self.0[24] as u32) << 24)
                | ((self.0[25] as u32) << 25)
                | ((self.0[26] as u32) << 26)
                | ((self.0[27] as u32) << 27)
                | ((self.0[28] as u32) << 28)
                | ((self.0[29] as u32) << 29)
                | ((self.0[30] as u32) << 30)
                | ((self.0[31] as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Filter bank 8 register 2
pub mod f8r2 {
    pub const OFFSET: u32 = 0x284;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([bool;32]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([bool;32]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
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
                | ((self.0[1] as u32) << 1)
                | ((self.0[2] as u32) << 2)
                | ((self.0[3] as u32) << 3)
                | ((self.0[4] as u32) << 4)
                | ((self.0[5] as u32) << 5)
                | ((self.0[6] as u32) << 6)
                | ((self.0[7] as u32) << 7)
                | ((self.0[8] as u32) << 8)
                | ((self.0[9] as u32) << 9)
                | ((self.0[10] as u32) << 10)
                | ((self.0[11] as u32) << 11)
                | ((self.0[12] as u32) << 12)
                | ((self.0[13] as u32) << 13)
                | ((self.0[14] as u32) << 14)
                | ((self.0[15] as u32) << 15)
                | ((self.0[16] as u32) << 16)
                | ((self.0[17] as u32) << 17)
                | ((self.0[18] as u32) << 18)
                | ((self.0[19] as u32) << 19)
                | ((self.0[20] as u32) << 20)
                | ((self.0[21] as u32) << 21)
                | ((self.0[22] as u32) << 22)
                | ((self.0[23] as u32) << 23)
                | ((self.0[24] as u32) << 24)
                | ((self.0[25] as u32) << 25)
                | ((self.0[26] as u32) << 26)
                | ((self.0[27] as u32) << 27)
                | ((self.0[28] as u32) << 28)
                | ((self.0[29] as u32) << 29)
                | ((self.0[30] as u32) << 30)
                | ((self.0[31] as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Filter bank 9 register 1
pub mod f9r1 {
    pub const OFFSET: u32 = 0x288;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([bool;32]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([bool;32]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
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
                | ((self.0[1] as u32) << 1)
                | ((self.0[2] as u32) << 2)
                | ((self.0[3] as u32) << 3)
                | ((self.0[4] as u32) << 4)
                | ((self.0[5] as u32) << 5)
                | ((self.0[6] as u32) << 6)
                | ((self.0[7] as u32) << 7)
                | ((self.0[8] as u32) << 8)
                | ((self.0[9] as u32) << 9)
                | ((self.0[10] as u32) << 10)
                | ((self.0[11] as u32) << 11)
                | ((self.0[12] as u32) << 12)
                | ((self.0[13] as u32) << 13)
                | ((self.0[14] as u32) << 14)
                | ((self.0[15] as u32) << 15)
                | ((self.0[16] as u32) << 16)
                | ((self.0[17] as u32) << 17)
                | ((self.0[18] as u32) << 18)
                | ((self.0[19] as u32) << 19)
                | ((self.0[20] as u32) << 20)
                | ((self.0[21] as u32) << 21)
                | ((self.0[22] as u32) << 22)
                | ((self.0[23] as u32) << 23)
                | ((self.0[24] as u32) << 24)
                | ((self.0[25] as u32) << 25)
                | ((self.0[26] as u32) << 26)
                | ((self.0[27] as u32) << 27)
                | ((self.0[28] as u32) << 28)
                | ((self.0[29] as u32) << 29)
                | ((self.0[30] as u32) << 30)
                | ((self.0[31] as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Filter bank 9 register 2
pub mod f9r2 {
    pub const OFFSET: u32 = 0x28C;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([bool;32]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([bool;32]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
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
                | ((self.0[1] as u32) << 1)
                | ((self.0[2] as u32) << 2)
                | ((self.0[3] as u32) << 3)
                | ((self.0[4] as u32) << 4)
                | ((self.0[5] as u32) << 5)
                | ((self.0[6] as u32) << 6)
                | ((self.0[7] as u32) << 7)
                | ((self.0[8] as u32) << 8)
                | ((self.0[9] as u32) << 9)
                | ((self.0[10] as u32) << 10)
                | ((self.0[11] as u32) << 11)
                | ((self.0[12] as u32) << 12)
                | ((self.0[13] as u32) << 13)
                | ((self.0[14] as u32) << 14)
                | ((self.0[15] as u32) << 15)
                | ((self.0[16] as u32) << 16)
                | ((self.0[17] as u32) << 17)
                | ((self.0[18] as u32) << 18)
                | ((self.0[19] as u32) << 19)
                | ((self.0[20] as u32) << 20)
                | ((self.0[21] as u32) << 21)
                | ((self.0[22] as u32) << 22)
                | ((self.0[23] as u32) << 23)
                | ((self.0[24] as u32) << 24)
                | ((self.0[25] as u32) << 25)
                | ((self.0[26] as u32) << 26)
                | ((self.0[27] as u32) << 27)
                | ((self.0[28] as u32) << 28)
                | ((self.0[29] as u32) << 29)
                | ((self.0[30] as u32) << 30)
                | ((self.0[31] as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Filter bank 10 register 1
pub mod f10r1 {
    pub const OFFSET: u32 = 0x290;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([bool;32]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([bool;32]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
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
                | ((self.0[1] as u32) << 1)
                | ((self.0[2] as u32) << 2)
                | ((self.0[3] as u32) << 3)
                | ((self.0[4] as u32) << 4)
                | ((self.0[5] as u32) << 5)
                | ((self.0[6] as u32) << 6)
                | ((self.0[7] as u32) << 7)
                | ((self.0[8] as u32) << 8)
                | ((self.0[9] as u32) << 9)
                | ((self.0[10] as u32) << 10)
                | ((self.0[11] as u32) << 11)
                | ((self.0[12] as u32) << 12)
                | ((self.0[13] as u32) << 13)
                | ((self.0[14] as u32) << 14)
                | ((self.0[15] as u32) << 15)
                | ((self.0[16] as u32) << 16)
                | ((self.0[17] as u32) << 17)
                | ((self.0[18] as u32) << 18)
                | ((self.0[19] as u32) << 19)
                | ((self.0[20] as u32) << 20)
                | ((self.0[21] as u32) << 21)
                | ((self.0[22] as u32) << 22)
                | ((self.0[23] as u32) << 23)
                | ((self.0[24] as u32) << 24)
                | ((self.0[25] as u32) << 25)
                | ((self.0[26] as u32) << 26)
                | ((self.0[27] as u32) << 27)
                | ((self.0[28] as u32) << 28)
                | ((self.0[29] as u32) << 29)
                | ((self.0[30] as u32) << 30)
                | ((self.0[31] as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Filter bank 10 register 2
pub mod f10r2 {
    pub const OFFSET: u32 = 0x294;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([bool;32]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([bool;32]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
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
                | ((self.0[1] as u32) << 1)
                | ((self.0[2] as u32) << 2)
                | ((self.0[3] as u32) << 3)
                | ((self.0[4] as u32) << 4)
                | ((self.0[5] as u32) << 5)
                | ((self.0[6] as u32) << 6)
                | ((self.0[7] as u32) << 7)
                | ((self.0[8] as u32) << 8)
                | ((self.0[9] as u32) << 9)
                | ((self.0[10] as u32) << 10)
                | ((self.0[11] as u32) << 11)
                | ((self.0[12] as u32) << 12)
                | ((self.0[13] as u32) << 13)
                | ((self.0[14] as u32) << 14)
                | ((self.0[15] as u32) << 15)
                | ((self.0[16] as u32) << 16)
                | ((self.0[17] as u32) << 17)
                | ((self.0[18] as u32) << 18)
                | ((self.0[19] as u32) << 19)
                | ((self.0[20] as u32) << 20)
                | ((self.0[21] as u32) << 21)
                | ((self.0[22] as u32) << 22)
                | ((self.0[23] as u32) << 23)
                | ((self.0[24] as u32) << 24)
                | ((self.0[25] as u32) << 25)
                | ((self.0[26] as u32) << 26)
                | ((self.0[27] as u32) << 27)
                | ((self.0[28] as u32) << 28)
                | ((self.0[29] as u32) << 29)
                | ((self.0[30] as u32) << 30)
                | ((self.0[31] as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Filter bank 11 register 1
pub mod f11r1 {
    pub const OFFSET: u32 = 0x298;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([bool;32]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([bool;32]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
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
                | ((self.0[1] as u32) << 1)
                | ((self.0[2] as u32) << 2)
                | ((self.0[3] as u32) << 3)
                | ((self.0[4] as u32) << 4)
                | ((self.0[5] as u32) << 5)
                | ((self.0[6] as u32) << 6)
                | ((self.0[7] as u32) << 7)
                | ((self.0[8] as u32) << 8)
                | ((self.0[9] as u32) << 9)
                | ((self.0[10] as u32) << 10)
                | ((self.0[11] as u32) << 11)
                | ((self.0[12] as u32) << 12)
                | ((self.0[13] as u32) << 13)
                | ((self.0[14] as u32) << 14)
                | ((self.0[15] as u32) << 15)
                | ((self.0[16] as u32) << 16)
                | ((self.0[17] as u32) << 17)
                | ((self.0[18] as u32) << 18)
                | ((self.0[19] as u32) << 19)
                | ((self.0[20] as u32) << 20)
                | ((self.0[21] as u32) << 21)
                | ((self.0[22] as u32) << 22)
                | ((self.0[23] as u32) << 23)
                | ((self.0[24] as u32) << 24)
                | ((self.0[25] as u32) << 25)
                | ((self.0[26] as u32) << 26)
                | ((self.0[27] as u32) << 27)
                | ((self.0[28] as u32) << 28)
                | ((self.0[29] as u32) << 29)
                | ((self.0[30] as u32) << 30)
                | ((self.0[31] as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Filter bank 11 register 2
pub mod f11r2 {
    pub const OFFSET: u32 = 0x29C;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([bool;32]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([bool;32]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
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
                | ((self.0[1] as u32) << 1)
                | ((self.0[2] as u32) << 2)
                | ((self.0[3] as u32) << 3)
                | ((self.0[4] as u32) << 4)
                | ((self.0[5] as u32) << 5)
                | ((self.0[6] as u32) << 6)
                | ((self.0[7] as u32) << 7)
                | ((self.0[8] as u32) << 8)
                | ((self.0[9] as u32) << 9)
                | ((self.0[10] as u32) << 10)
                | ((self.0[11] as u32) << 11)
                | ((self.0[12] as u32) << 12)
                | ((self.0[13] as u32) << 13)
                | ((self.0[14] as u32) << 14)
                | ((self.0[15] as u32) << 15)
                | ((self.0[16] as u32) << 16)
                | ((self.0[17] as u32) << 17)
                | ((self.0[18] as u32) << 18)
                | ((self.0[19] as u32) << 19)
                | ((self.0[20] as u32) << 20)
                | ((self.0[21] as u32) << 21)
                | ((self.0[22] as u32) << 22)
                | ((self.0[23] as u32) << 23)
                | ((self.0[24] as u32) << 24)
                | ((self.0[25] as u32) << 25)
                | ((self.0[26] as u32) << 26)
                | ((self.0[27] as u32) << 27)
                | ((self.0[28] as u32) << 28)
                | ((self.0[29] as u32) << 29)
                | ((self.0[30] as u32) << 30)
                | ((self.0[31] as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Filter bank 4 register 1
pub mod f12r1 {
    pub const OFFSET: u32 = 0x2A0;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([bool;32]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([bool;32]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
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
                | ((self.0[1] as u32) << 1)
                | ((self.0[2] as u32) << 2)
                | ((self.0[3] as u32) << 3)
                | ((self.0[4] as u32) << 4)
                | ((self.0[5] as u32) << 5)
                | ((self.0[6] as u32) << 6)
                | ((self.0[7] as u32) << 7)
                | ((self.0[8] as u32) << 8)
                | ((self.0[9] as u32) << 9)
                | ((self.0[10] as u32) << 10)
                | ((self.0[11] as u32) << 11)
                | ((self.0[12] as u32) << 12)
                | ((self.0[13] as u32) << 13)
                | ((self.0[14] as u32) << 14)
                | ((self.0[15] as u32) << 15)
                | ((self.0[16] as u32) << 16)
                | ((self.0[17] as u32) << 17)
                | ((self.0[18] as u32) << 18)
                | ((self.0[19] as u32) << 19)
                | ((self.0[20] as u32) << 20)
                | ((self.0[21] as u32) << 21)
                | ((self.0[22] as u32) << 22)
                | ((self.0[23] as u32) << 23)
                | ((self.0[24] as u32) << 24)
                | ((self.0[25] as u32) << 25)
                | ((self.0[26] as u32) << 26)
                | ((self.0[27] as u32) << 27)
                | ((self.0[28] as u32) << 28)
                | ((self.0[29] as u32) << 29)
                | ((self.0[30] as u32) << 30)
                | ((self.0[31] as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Filter bank 12 register 2
pub mod f12r2 {
    pub const OFFSET: u32 = 0x2A4;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([bool;32]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([bool;32]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
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
                | ((self.0[1] as u32) << 1)
                | ((self.0[2] as u32) << 2)
                | ((self.0[3] as u32) << 3)
                | ((self.0[4] as u32) << 4)
                | ((self.0[5] as u32) << 5)
                | ((self.0[6] as u32) << 6)
                | ((self.0[7] as u32) << 7)
                | ((self.0[8] as u32) << 8)
                | ((self.0[9] as u32) << 9)
                | ((self.0[10] as u32) << 10)
                | ((self.0[11] as u32) << 11)
                | ((self.0[12] as u32) << 12)
                | ((self.0[13] as u32) << 13)
                | ((self.0[14] as u32) << 14)
                | ((self.0[15] as u32) << 15)
                | ((self.0[16] as u32) << 16)
                | ((self.0[17] as u32) << 17)
                | ((self.0[18] as u32) << 18)
                | ((self.0[19] as u32) << 19)
                | ((self.0[20] as u32) << 20)
                | ((self.0[21] as u32) << 21)
                | ((self.0[22] as u32) << 22)
                | ((self.0[23] as u32) << 23)
                | ((self.0[24] as u32) << 24)
                | ((self.0[25] as u32) << 25)
                | ((self.0[26] as u32) << 26)
                | ((self.0[27] as u32) << 27)
                | ((self.0[28] as u32) << 28)
                | ((self.0[29] as u32) << 29)
                | ((self.0[30] as u32) << 30)
                | ((self.0[31] as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Filter bank 13 register 1
pub mod f13r1 {
    pub const OFFSET: u32 = 0x2A8;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([bool;32]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([bool;32]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
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
                | ((self.0[1] as u32) << 1)
                | ((self.0[2] as u32) << 2)
                | ((self.0[3] as u32) << 3)
                | ((self.0[4] as u32) << 4)
                | ((self.0[5] as u32) << 5)
                | ((self.0[6] as u32) << 6)
                | ((self.0[7] as u32) << 7)
                | ((self.0[8] as u32) << 8)
                | ((self.0[9] as u32) << 9)
                | ((self.0[10] as u32) << 10)
                | ((self.0[11] as u32) << 11)
                | ((self.0[12] as u32) << 12)
                | ((self.0[13] as u32) << 13)
                | ((self.0[14] as u32) << 14)
                | ((self.0[15] as u32) << 15)
                | ((self.0[16] as u32) << 16)
                | ((self.0[17] as u32) << 17)
                | ((self.0[18] as u32) << 18)
                | ((self.0[19] as u32) << 19)
                | ((self.0[20] as u32) << 20)
                | ((self.0[21] as u32) << 21)
                | ((self.0[22] as u32) << 22)
                | ((self.0[23] as u32) << 23)
                | ((self.0[24] as u32) << 24)
                | ((self.0[25] as u32) << 25)
                | ((self.0[26] as u32) << 26)
                | ((self.0[27] as u32) << 27)
                | ((self.0[28] as u32) << 28)
                | ((self.0[29] as u32) << 29)
                | ((self.0[30] as u32) << 30)
                | ((self.0[31] as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Filter bank 13 register 2
pub mod f13r2 {
    pub const OFFSET: u32 = 0x2AC;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache([bool;32]);
    impl ::core::ops::Index<u8> for ReadonlyCache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for ReadonlyCache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub struct Cache([bool;32]);
    impl ::core::ops::Index<u8> for Cache {
        type Output = bool;
        fn index(&self, index: u8) -> &bool {
            self.0.index(index as usize)
        }
    }
    impl ::core::ops::IndexMut<u8> for Cache {
        fn index_mut(&mut self, index: u8) -> &mut bool {
            self.0.index_mut(index as usize)
        }
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
        ])
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache([
            ((value >> 0) & 0b1) > 0,
            ((value >> 1) & 0b1) > 0,
            ((value >> 2) & 0b1) > 0,
            ((value >> 3) & 0b1) > 0,
            ((value >> 4) & 0b1) > 0,
            ((value >> 5) & 0b1) > 0,
            ((value >> 6) & 0b1) > 0,
            ((value >> 7) & 0b1) > 0,
            ((value >> 8) & 0b1) > 0,
            ((value >> 9) & 0b1) > 0,
            ((value >> 10) & 0b1) > 0,
            ((value >> 11) & 0b1) > 0,
            ((value >> 12) & 0b1) > 0,
            ((value >> 13) & 0b1) > 0,
            ((value >> 14) & 0b1) > 0,
            ((value >> 15) & 0b1) > 0,
            ((value >> 16) & 0b1) > 0,
            ((value >> 17) & 0b1) > 0,
            ((value >> 18) & 0b1) > 0,
            ((value >> 19) & 0b1) > 0,
            ((value >> 20) & 0b1) > 0,
            ((value >> 21) & 0b1) > 0,
            ((value >> 22) & 0b1) > 0,
            ((value >> 23) & 0b1) > 0,
            ((value >> 24) & 0b1) > 0,
            ((value >> 25) & 0b1) > 0,
            ((value >> 26) & 0b1) > 0,
            ((value >> 27) & 0b1) > 0,
            ((value >> 28) & 0b1) > 0,
            ((value >> 29) & 0b1) > 0,
            ((value >> 30) & 0b1) > 0,
            ((value >> 31) & 0b1) > 0,
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
                | ((self.0[1] as u32) << 1)
                | ((self.0[2] as u32) << 2)
                | ((self.0[3] as u32) << 3)
                | ((self.0[4] as u32) << 4)
                | ((self.0[5] as u32) << 5)
                | ((self.0[6] as u32) << 6)
                | ((self.0[7] as u32) << 7)
                | ((self.0[8] as u32) << 8)
                | ((self.0[9] as u32) << 9)
                | ((self.0[10] as u32) << 10)
                | ((self.0[11] as u32) << 11)
                | ((self.0[12] as u32) << 12)
                | ((self.0[13] as u32) << 13)
                | ((self.0[14] as u32) << 14)
                | ((self.0[15] as u32) << 15)
                | ((self.0[16] as u32) << 16)
                | ((self.0[17] as u32) << 17)
                | ((self.0[18] as u32) << 18)
                | ((self.0[19] as u32) << 19)
                | ((self.0[20] as u32) << 20)
                | ((self.0[21] as u32) << 21)
                | ((self.0[22] as u32) << 22)
                | ((self.0[23] as u32) << 23)
                | ((self.0[24] as u32) << 24)
                | ((self.0[25] as u32) << 25)
                | ((self.0[26] as u32) << 26)
                | ((self.0[27] as u32) << 27)
                | ((self.0[28] as u32) << 28)
                | ((self.0[29] as u32) << 29)
                | ((self.0[30] as u32) << 30)
                | ((self.0[31] as u32) << 31)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// CAN1 TX interrupts
pub const INTERRUPT_CAN1_TX: u32 = 19;
/// CAN1 RX0 interrupts
pub const INTERRUPT_CAN1_RX0: u32 = 20;
/// CAN1 RX1 interrupt
pub const INTERRUPT_CAN1_RX1: u32 = 21;
/// CAN1 SCE interrupt
pub const INTERRUPT_CAN1_SCE: u32 = 22;
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0x40006400</baseAddress>
  <description>Controller area network</description>
  <groupName>CAN</groupName>
  <interrupt>
    <description>CAN1 TX interrupts</description>
    <name>CAN1_TX</name>
    <value>19</value>
  </interrupt>
  <interrupt>
    <description>CAN1 RX0 interrupts</description>
    <name>CAN1_RX0</name>
    <value>20</value>
  </interrupt>
  <interrupt>
    <description>CAN1 RX1 interrupt</description>
    <name>CAN1_RX1</name>
    <value>21</value>
  </interrupt>
  <interrupt>
    <description>CAN1 SCE interrupt</description>
    <name>CAN1_SCE</name>
    <value>22</value>
  </interrupt>
  <name>CAN</name>
  <registers>
    <register>
      <access>read-write</access>
      <addressOffset>0x0</addressOffset>
      <description>CAN_MCR</description>
      <displayName>CAN_MCR</displayName>
      <fields>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DBF</description>
          <name>DBF</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RESET</description>
          <name>RESET</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TTCM</description>
          <name>TTCM</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ABOM</description>
          <name>ABOM</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>AWUM</description>
          <name>AWUM</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>NART</description>
          <name>NART</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RFLM</description>
          <name>RFLM</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TXFP</description>
          <name>TXFP</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>SLEEP</description>
          <name>SLEEP</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>INRQ</description>
          <name>INRQ</name>
        </field>
      </fields>
      <name>CAN_MCR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0x4</addressOffset>
      <description>CAN_MSR</description>
      <displayName>CAN_MSR</displayName>
      <fields>
        <field>
          <access>read-only</access>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RX</description>
          <name>RX</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>SAMP</description>
          <name>SAMP</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RXM</description>
          <name>RXM</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TXM</description>
          <name>TXM</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>SLAKI</description>
          <name>SLAKI</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>WKUI</description>
          <name>WKUI</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ERRI</description>
          <name>ERRI</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>SLAK</description>
          <name>SLAK</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>INAK</description>
          <name>INAK</name>
        </field>
      </fields>
      <name>CAN_MSR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0x8</addressOffset>
      <description>CAN_TSR</description>
      <displayName>CAN_TSR</displayName>
      <fields>
        <field>
          <access>read-only</access>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Lowest priority flag for mailbox
              2</description>
          <name>LOW2</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Lowest priority flag for mailbox
              1</description>
          <name>LOW1</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Lowest priority flag for mailbox
              0</description>
          <name>LOW0</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Lowest priority flag for mailbox
              2</description>
          <name>TME2</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Lowest priority flag for mailbox
              1</description>
          <name>TME1</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Lowest priority flag for mailbox
              0</description>
          <name>TME0</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>24</bitOffset>
          <bitWidth>2</bitWidth>
          <description>CODE</description>
          <name>CODE</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ABRQ2</description>
          <name>ABRQ2</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TERR2</description>
          <name>TERR2</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ALST2</description>
          <name>ALST2</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TXOK2</description>
          <name>TXOK2</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RQCP2</description>
          <name>RQCP2</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ABRQ1</description>
          <name>ABRQ1</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TERR1</description>
          <name>TERR1</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ALST1</description>
          <name>ALST1</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TXOK1</description>
          <name>TXOK1</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RQCP1</description>
          <name>RQCP1</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ABRQ0</description>
          <name>ABRQ0</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TERR0</description>
          <name>TERR0</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ALST0</description>
          <name>ALST0</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TXOK0</description>
          <name>TXOK0</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RQCP0</description>
          <name>RQCP0</name>
        </field>
      </fields>
      <name>CAN_TSR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0xC</addressOffset>
      <description>CAN_RF0R</description>
      <displayName>CAN_RF0R</displayName>
      <fields>
        <field>
          <access>read-write</access>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RFOM0</description>
          <name>RFOM0</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>FOVR0</description>
          <name>FOVR0</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>FULL0</description>
          <name>FULL0</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>0</bitOffset>
          <bitWidth>2</bitWidth>
          <description>FMP0</description>
          <name>FMP0</name>
        </field>
      </fields>
      <name>CAN_RF0R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0x10</addressOffset>
      <description>CAN_RF1R</description>
      <displayName>CAN_RF1R</displayName>
      <fields>
        <field>
          <access>read-write</access>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RFOM1</description>
          <name>RFOM1</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>FOVR1</description>
          <name>FOVR1</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>FULL1</description>
          <name>FULL1</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>0</bitOffset>
          <bitWidth>2</bitWidth>
          <description>FMP1</description>
          <name>FMP1</name>
        </field>
      </fields>
      <name>CAN_RF1R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x14</addressOffset>
      <description>CAN_IER</description>
      <displayName>CAN_IER</displayName>
      <fields>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>SLKIE</description>
          <name>SLKIE</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>WKUIE</description>
          <name>WKUIE</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ERRIE</description>
          <name>ERRIE</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>LECIE</description>
          <name>LECIE</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>BOFIE</description>
          <name>BOFIE</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>EPVIE</description>
          <name>EPVIE</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>EWGIE</description>
          <name>EWGIE</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>FOVIE1</description>
          <name>FOVIE1</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>FFIE1</description>
          <name>FFIE1</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>FMPIE1</description>
          <name>FMPIE1</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>FOVIE0</description>
          <name>FOVIE0</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>FFIE0</description>
          <name>FFIE0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>FMPIE0</description>
          <name>FMPIE0</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TMEIE</description>
          <name>TMEIE</name>
        </field>
      </fields>
      <name>CAN_IER</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0x18</addressOffset>
      <description>CAN_ESR</description>
      <displayName>CAN_ESR</displayName>
      <fields>
        <field>
          <access>read-only</access>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>REC</description>
          <name>REC</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>TEC</description>
          <name>TEC</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>4</bitOffset>
          <bitWidth>3</bitWidth>
          <description>LEC</description>
          <name>LEC</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>BOFF</description>
          <name>BOFF</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>EPVF</description>
          <name>EPVF</name>
        </field>
        <field>
          <access>read-only</access>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>EWGF</description>
          <name>EWGF</name>
        </field>
      </fields>
      <name>CAN_ESR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x1C</addressOffset>
      <description>CAN_BTR</description>
      <displayName>CAN_BTR</displayName>
      <fields>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>SILM</description>
          <name>SILM</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>LBKM</description>
          <name>LBKM</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>2</bitWidth>
          <description>SJW</description>
          <name>SJW</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>3</bitWidth>
          <description>TS2</description>
          <name>TS2</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>4</bitWidth>
          <description>TS1</description>
          <name>TS1</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>10</bitWidth>
          <description>BRP</description>
          <name>BRP</name>
        </field>
      </fields>
      <name>CAN_BTR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x180</addressOffset>
      <description>CAN_TI0R</description>
      <displayName>CAN_TI0R</displayName>
      <fields>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>11</bitWidth>
          <description>STID</description>
          <name>STID</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>18</bitWidth>
          <description>EXID</description>
          <name>EXID</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>IDE</description>
          <name>IDE</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RTR</description>
          <name>RTR</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TXRQ</description>
          <name>TXRQ</name>
        </field>
      </fields>
      <name>CAN_TI0R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x184</addressOffset>
      <description>CAN_TDT0R</description>
      <displayName>CAN_TDT0R</displayName>
      <fields>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>16</bitWidth>
          <description>TIME</description>
          <name>TIME</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TGT</description>
          <name>TGT</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>DLC</description>
          <name>DLC</name>
        </field>
      </fields>
      <name>CAN_TDT0R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x188</addressOffset>
      <description>CAN_TDL0R</description>
      <displayName>CAN_TDL0R</displayName>
      <fields>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA3</description>
          <name>DATA3</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA2</description>
          <name>DATA2</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA1</description>
          <name>DATA1</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA0</description>
          <name>DATA0</name>
        </field>
      </fields>
      <name>CAN_TDL0R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x18C</addressOffset>
      <description>CAN_TDH0R</description>
      <displayName>CAN_TDH0R</displayName>
      <fields>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA7</description>
          <name>DATA7</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA6</description>
          <name>DATA6</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA5</description>
          <name>DATA5</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA4</description>
          <name>DATA4</name>
        </field>
      </fields>
      <name>CAN_TDH0R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x190</addressOffset>
      <description>CAN_TI1R</description>
      <displayName>CAN_TI1R</displayName>
      <fields>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>11</bitWidth>
          <description>STID</description>
          <name>STID</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>18</bitWidth>
          <description>EXID</description>
          <name>EXID</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>IDE</description>
          <name>IDE</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RTR</description>
          <name>RTR</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TXRQ</description>
          <name>TXRQ</name>
        </field>
      </fields>
      <name>CAN_TI1R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x194</addressOffset>
      <description>CAN_TDT1R</description>
      <displayName>CAN_TDT1R</displayName>
      <fields>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>16</bitWidth>
          <description>TIME</description>
          <name>TIME</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TGT</description>
          <name>TGT</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>DLC</description>
          <name>DLC</name>
        </field>
      </fields>
      <name>CAN_TDT1R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x198</addressOffset>
      <description>CAN_TDL1R</description>
      <displayName>CAN_TDL1R</displayName>
      <fields>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA3</description>
          <name>DATA3</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA2</description>
          <name>DATA2</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA1</description>
          <name>DATA1</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA0</description>
          <name>DATA0</name>
        </field>
      </fields>
      <name>CAN_TDL1R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x19C</addressOffset>
      <description>CAN_TDH1R</description>
      <displayName>CAN_TDH1R</displayName>
      <fields>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA7</description>
          <name>DATA7</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA6</description>
          <name>DATA6</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA5</description>
          <name>DATA5</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA4</description>
          <name>DATA4</name>
        </field>
      </fields>
      <name>CAN_TDH1R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x1A0</addressOffset>
      <description>CAN_TI2R</description>
      <displayName>CAN_TI2R</displayName>
      <fields>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>11</bitWidth>
          <description>STID</description>
          <name>STID</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>18</bitWidth>
          <description>EXID</description>
          <name>EXID</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>IDE</description>
          <name>IDE</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RTR</description>
          <name>RTR</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TXRQ</description>
          <name>TXRQ</name>
        </field>
      </fields>
      <name>CAN_TI2R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x1A4</addressOffset>
      <description>CAN_TDT2R</description>
      <displayName>CAN_TDT2R</displayName>
      <fields>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>16</bitWidth>
          <description>TIME</description>
          <name>TIME</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TGT</description>
          <name>TGT</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>DLC</description>
          <name>DLC</name>
        </field>
      </fields>
      <name>CAN_TDT2R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x1A8</addressOffset>
      <description>CAN_TDL2R</description>
      <displayName>CAN_TDL2R</displayName>
      <fields>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA3</description>
          <name>DATA3</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA2</description>
          <name>DATA2</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA1</description>
          <name>DATA1</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA0</description>
          <name>DATA0</name>
        </field>
      </fields>
      <name>CAN_TDL2R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x1AC</addressOffset>
      <description>CAN_TDH2R</description>
      <displayName>CAN_TDH2R</displayName>
      <fields>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA7</description>
          <name>DATA7</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA6</description>
          <name>DATA6</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA5</description>
          <name>DATA5</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA4</description>
          <name>DATA4</name>
        </field>
      </fields>
      <name>CAN_TDH2R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x1B0</addressOffset>
      <description>CAN_RI0R</description>
      <displayName>CAN_RI0R</displayName>
      <fields>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>11</bitWidth>
          <description>STID</description>
          <name>STID</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>18</bitWidth>
          <description>EXID</description>
          <name>EXID</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>IDE</description>
          <name>IDE</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RTR</description>
          <name>RTR</name>
        </field>
      </fields>
      <name>CAN_RI0R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x1B4</addressOffset>
      <description>CAN_RDT0R</description>
      <displayName>CAN_RDT0R</displayName>
      <fields>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>16</bitWidth>
          <description>TIME</description>
          <name>TIME</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>FMI</description>
          <name>FMI</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>DLC</description>
          <name>DLC</name>
        </field>
      </fields>
      <name>CAN_RDT0R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x1B8</addressOffset>
      <description>CAN_RDL0R</description>
      <displayName>CAN_RDL0R</displayName>
      <fields>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA3</description>
          <name>DATA3</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA2</description>
          <name>DATA2</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA1</description>
          <name>DATA1</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA0</description>
          <name>DATA0</name>
        </field>
      </fields>
      <name>CAN_RDL0R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x1BC</addressOffset>
      <description>CAN_RDH0R</description>
      <displayName>CAN_RDH0R</displayName>
      <fields>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA7</description>
          <name>DATA7</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA6</description>
          <name>DATA6</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA5</description>
          <name>DATA5</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA4</description>
          <name>DATA4</name>
        </field>
      </fields>
      <name>CAN_RDH0R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x1C0</addressOffset>
      <description>CAN_RI1R</description>
      <displayName>CAN_RI1R</displayName>
      <fields>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>11</bitWidth>
          <description>STID</description>
          <name>STID</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>18</bitWidth>
          <description>EXID</description>
          <name>EXID</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>IDE</description>
          <name>IDE</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RTR</description>
          <name>RTR</name>
        </field>
      </fields>
      <name>CAN_RI1R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x1C4</addressOffset>
      <description>CAN_RDT1R</description>
      <displayName>CAN_RDT1R</displayName>
      <fields>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>16</bitWidth>
          <description>TIME</description>
          <name>TIME</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>FMI</description>
          <name>FMI</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>DLC</description>
          <name>DLC</name>
        </field>
      </fields>
      <name>CAN_RDT1R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x1C8</addressOffset>
      <description>CAN_RDL1R</description>
      <displayName>CAN_RDL1R</displayName>
      <fields>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA3</description>
          <name>DATA3</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA2</description>
          <name>DATA2</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA1</description>
          <name>DATA1</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA0</description>
          <name>DATA0</name>
        </field>
      </fields>
      <name>CAN_RDL1R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x1CC</addressOffset>
      <description>CAN_RDH1R</description>
      <displayName>CAN_RDH1R</displayName>
      <fields>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA7</description>
          <name>DATA7</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA6</description>
          <name>DATA6</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA5</description>
          <name>DATA5</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATA4</description>
          <name>DATA4</name>
        </field>
      </fields>
      <name>CAN_RDH1R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x200</addressOffset>
      <description>CAN_FMR</description>
      <displayName>CAN_FMR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>FINIT</description>
          <name>FINIT</name>
        </field>
      </fields>
      <name>CAN_FMR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x204</addressOffset>
      <description>CAN_FM1R</description>
      <displayName>CAN_FM1R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter mode</description>
          <name>FBM13</name>
        </field>
      </fields>
      <name>CAN_FM1R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x20C</addressOffset>
      <description>CAN_FS1R</description>
      <displayName>CAN_FS1R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter scale configuration</description>
          <name>FSC13</name>
        </field>
      </fields>
      <name>CAN_FS1R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x214</addressOffset>
      <description>CAN_FFA1R</description>
      <displayName>CAN_FFA1R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter FIFO assignment for filter
              0</description>
          <name>FFA0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter FIFO assignment for filter
              1</description>
          <name>FFA1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter FIFO assignment for filter
              2</description>
          <name>FFA2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter FIFO assignment for filter
              3</description>
          <name>FFA3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter FIFO assignment for filter
              4</description>
          <name>FFA4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter FIFO assignment for filter
              5</description>
          <name>FFA5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter FIFO assignment for filter
              6</description>
          <name>FFA6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter FIFO assignment for filter
              7</description>
          <name>FFA7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter FIFO assignment for filter
              8</description>
          <name>FFA8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter FIFO assignment for filter
              9</description>
          <name>FFA9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter FIFO assignment for filter
              10</description>
          <name>FFA10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter FIFO assignment for filter
              11</description>
          <name>FFA11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter FIFO assignment for filter
              12</description>
          <name>FFA12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter FIFO assignment for filter
              13</description>
          <name>FFA13</name>
        </field>
      </fields>
      <name>CAN_FFA1R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x21C</addressOffset>
      <description>CAN_FA1R</description>
      <displayName>CAN_FA1R</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter active</description>
          <name>FACT13</name>
        </field>
      </fields>
      <name>CAN_FA1R</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x240</addressOffset>
      <description>Filter bank 0 register 1</description>
      <displayName>F0R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F0R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x244</addressOffset>
      <description>Filter bank 0 register 2</description>
      <displayName>F0R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F0R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x248</addressOffset>
      <description>Filter bank 1 register 1</description>
      <displayName>F1R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F1R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x24C</addressOffset>
      <description>Filter bank 1 register 2</description>
      <displayName>F1R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F1R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x250</addressOffset>
      <description>Filter bank 2 register 1</description>
      <displayName>F2R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F2R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x254</addressOffset>
      <description>Filter bank 2 register 2</description>
      <displayName>F2R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F2R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x258</addressOffset>
      <description>Filter bank 3 register 1</description>
      <displayName>F3R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F3R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x25C</addressOffset>
      <description>Filter bank 3 register 2</description>
      <displayName>F3R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F3R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x260</addressOffset>
      <description>Filter bank 4 register 1</description>
      <displayName>F4R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F4R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x264</addressOffset>
      <description>Filter bank 4 register 2</description>
      <displayName>F4R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F4R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x268</addressOffset>
      <description>Filter bank 5 register 1</description>
      <displayName>F5R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F5R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x26C</addressOffset>
      <description>Filter bank 5 register 2</description>
      <displayName>F5R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F5R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x270</addressOffset>
      <description>Filter bank 6 register 1</description>
      <displayName>F6R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F6R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x274</addressOffset>
      <description>Filter bank 6 register 2</description>
      <displayName>F6R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F6R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x278</addressOffset>
      <description>Filter bank 7 register 1</description>
      <displayName>F7R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F7R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x27C</addressOffset>
      <description>Filter bank 7 register 2</description>
      <displayName>F7R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F7R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x280</addressOffset>
      <description>Filter bank 8 register 1</description>
      <displayName>F8R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F8R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x284</addressOffset>
      <description>Filter bank 8 register 2</description>
      <displayName>F8R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F8R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x288</addressOffset>
      <description>Filter bank 9 register 1</description>
      <displayName>F9R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F9R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x28C</addressOffset>
      <description>Filter bank 9 register 2</description>
      <displayName>F9R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F9R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x290</addressOffset>
      <description>Filter bank 10 register 1</description>
      <displayName>F10R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F10R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x294</addressOffset>
      <description>Filter bank 10 register 2</description>
      <displayName>F10R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F10R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x298</addressOffset>
      <description>Filter bank 11 register 1</description>
      <displayName>F11R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F11R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x29C</addressOffset>
      <description>Filter bank 11 register 2</description>
      <displayName>F11R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F11R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x2A0</addressOffset>
      <description>Filter bank 4 register 1</description>
      <displayName>F12R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F12R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x2A4</addressOffset>
      <description>Filter bank 12 register 2</description>
      <displayName>F12R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F12R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x2A8</addressOffset>
      <description>Filter bank 13 register 1</description>
      <displayName>F13R1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F13R1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x2AC</addressOffset>
      <description>Filter bank 13 register 2</description>
      <displayName>F13R2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB0</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB1</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB2</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB3</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB4</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB5</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB6</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB7</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB8</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB9</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB10</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB11</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB12</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB13</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB14</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB15</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB16</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB17</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB18</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB19</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB20</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB21</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB22</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB23</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB24</name>
        </field>
        <field>
          <bitOffset>25</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB25</name>
        </field>
        <field>
          <bitOffset>26</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB26</name>
        </field>
        <field>
          <bitOffset>27</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB27</name>
        </field>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB28</name>
        </field>
        <field>
          <bitOffset>29</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB29</name>
        </field>
        <field>
          <bitOffset>30</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB30</name>
        </field>
        <field>
          <bitOffset>31</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Filter bits</description>
          <name>FB31</name>
        </field>
      </fields>
      <name>F13R2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
