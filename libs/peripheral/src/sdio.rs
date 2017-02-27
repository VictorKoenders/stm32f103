pub const ADDRESS: u32 = 0x40018000;
/// Bits 1:0 = PWRCTRL: Power supply control bits
pub mod power {
    pub const OFFSET: u32 = 0x0;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// PWRCTRL
        pub pwrctrl: u8,
    }
    pub struct Cache {
        /// PWRCTRL
        pub pwrctrl: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            pwrctrl: ((value >> 0) & 0b11) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            pwrctrl: ((value >> 0) & 0b11) as u8,
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
                | ((self.pwrctrl as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// SDI clock control register (SDIO_CLKCR)
pub mod clkcr {
    pub const OFFSET: u32 = 0x4;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Clock divide factor
        pub clkdiv: u8,
        /// Clock enable bit
        pub clken: u8,
        /// Power saving configuration bit
        pub pwrsav: u8,
        /// Clock divider bypass enable bit
        pub bypass: u8,
        /// Wide bus mode enable bit
        pub widbus: u8,
        /// SDIO_CK dephasing selection bit
        pub negedge: u8,
        /// HW Flow Control enable
        pub hwfc_en: u8,
    }
    pub struct Cache {
        /// Clock divide factor
        pub clkdiv: u8,
        /// Clock enable bit
        pub clken: u8,
        /// Power saving configuration bit
        pub pwrsav: u8,
        /// Clock divider bypass enable bit
        pub bypass: u8,
        /// Wide bus mode enable bit
        pub widbus: u8,
        /// SDIO_CK dephasing selection bit
        pub negedge: u8,
        /// HW Flow Control enable
        pub hwfc_en: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            clkdiv: ((value >> 0) & 0b11111111) as u8,
            clken: ((value >> 8) & 0b11111111) as u8,
            pwrsav: ((value >> 9) & 0b11111111) as u8,
            bypass: ((value >> 10) & 0b11111111) as u8,
            widbus: ((value >> 11) & 0b11111111) as u8,
            negedge: ((value >> 13) & 0b11111111) as u8,
            hwfc_en: ((value >> 14) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            clkdiv: ((value >> 0) & 0b11111111) as u8,
            clken: ((value >> 8) & 0b11111111) as u8,
            pwrsav: ((value >> 9) & 0b11111111) as u8,
            bypass: ((value >> 10) & 0b11111111) as u8,
            widbus: ((value >> 11) & 0b11111111) as u8,
            negedge: ((value >> 13) & 0b11111111) as u8,
            hwfc_en: ((value >> 14) & 0b11111111) as u8,
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
                | ((self.clkdiv as u32) << 0)
                | ((self.clken as u32) << 8)
                | ((self.pwrsav as u32) << 9)
                | ((self.bypass as u32) << 10)
                | ((self.widbus as u32) << 11)
                | ((self.negedge as u32) << 13)
                | ((self.hwfc_en as u32) << 14)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Bits 31:0 = : Command argument
pub mod arg {
    pub const OFFSET: u32 = 0x8;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Command argument
        pub cmdarg: u32,
    }
    pub struct Cache {
        /// Command argument
        pub cmdarg: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            cmdarg: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            cmdarg: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.cmdarg as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// SDIO command register (SDIO_CMD)
pub mod cmd {
    pub const OFFSET: u32 = 0xC;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// CMDINDEX
        pub cmdindex: u8,
        /// WAITRESP
        pub waitresp: u8,
        /// WAITINT
        pub waitint: u8,
        /// WAITPEND
        pub waitpend: u8,
        /// CPSMEN
        pub cpsmen: u8,
        /// SDIOSuspend
        pub sdiosuspend: u8,
        /// ENCMDcompl
        pub encmdcompl: u8,
        /// nIEN
        pub nien: u8,
        /// CE_ATACMD
        pub ce_atacmd: u8,
    }
    pub struct Cache {
        /// CMDINDEX
        pub cmdindex: u8,
        /// WAITRESP
        pub waitresp: u8,
        /// WAITINT
        pub waitint: u8,
        /// WAITPEND
        pub waitpend: u8,
        /// CPSMEN
        pub cpsmen: u8,
        /// SDIOSuspend
        pub sdiosuspend: u8,
        /// ENCMDcompl
        pub encmdcompl: u8,
        /// nIEN
        pub nien: u8,
        /// CE_ATACMD
        pub ce_atacmd: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            cmdindex: ((value >> 0) & 0b111111) as u8,
            waitresp: ((value >> 6) & 0b111111) as u8,
            waitint: ((value >> 8) & 0b111111) as u8,
            waitpend: ((value >> 9) & 0b111111) as u8,
            cpsmen: ((value >> 10) & 0b111111) as u8,
            sdiosuspend: ((value >> 11) & 0b111111) as u8,
            encmdcompl: ((value >> 12) & 0b111111) as u8,
            nien: ((value >> 13) & 0b111111) as u8,
            ce_atacmd: ((value >> 14) & 0b111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            cmdindex: ((value >> 0) & 0b111111) as u8,
            waitresp: ((value >> 6) & 0b111111) as u8,
            waitint: ((value >> 8) & 0b111111) as u8,
            waitpend: ((value >> 9) & 0b111111) as u8,
            cpsmen: ((value >> 10) & 0b111111) as u8,
            sdiosuspend: ((value >> 11) & 0b111111) as u8,
            encmdcompl: ((value >> 12) & 0b111111) as u8,
            nien: ((value >> 13) & 0b111111) as u8,
            ce_atacmd: ((value >> 14) & 0b111111) as u8,
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
                | ((self.cmdindex as u32) << 0)
                | ((self.waitresp as u32) << 6)
                | ((self.waitint as u32) << 8)
                | ((self.waitpend as u32) << 9)
                | ((self.cpsmen as u32) << 10)
                | ((self.sdiosuspend as u32) << 11)
                | ((self.encmdcompl as u32) << 12)
                | ((self.nien as u32) << 13)
                | ((self.ce_atacmd as u32) << 14)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// SDIO command register
pub mod respcmd {
    pub const OFFSET: u32 = 0x10;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// RESPCMD
    /// Access: read-only, Width: 6, Offset: 0
    /// Get RESPCMD
    pub fn respcmd() -> u8 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b111111 << 0);
        value as u8
    }
}
/// Bits 31:0 = CARDSTATUS1
pub mod respi1 {
    pub const OFFSET: u32 = 0x14;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// CARDSTATUS1
    /// Access: read-only, Width: 32, Offset: 0
    /// Get CARDSTATUS1
    pub fn cardstatus1() -> u32 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b11111111111111111111111111111111 << 0);
        value as u32
    }
}
/// Bits 31:0 = CARDSTATUS2
pub mod resp2 {
    pub const OFFSET: u32 = 0x18;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// CARDSTATUS2
    /// Access: read-only, Width: 32, Offset: 0
    /// Get CARDSTATUS2
    pub fn cardstatus2() -> u32 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b11111111111111111111111111111111 << 0);
        value as u32
    }
}
/// Bits 31:0 = CARDSTATUS3
pub mod resp3 {
    pub const OFFSET: u32 = 0x1C;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// CARDSTATUS3
    /// Access: read-only, Width: 32, Offset: 0
    /// Get CARDSTATUS3
    pub fn cardstatus3() -> u32 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b11111111111111111111111111111111 << 0);
        value as u32
    }
}
/// Bits 31:0 = CARDSTATUS4
pub mod resp4 {
    pub const OFFSET: u32 = 0x20;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// CARDSTATUS4
    /// Access: read-only, Width: 32, Offset: 0
    /// Get CARDSTATUS4
    pub fn cardstatus4() -> u32 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b11111111111111111111111111111111 << 0);
        value as u32
    }
}
/// Bits 31:0 = DATATIME: Data timeout period
pub mod dtimer {
    pub const OFFSET: u32 = 0x24;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Data timeout period
        pub datatime: u32,
    }
    pub struct Cache {
        /// Data timeout period
        pub datatime: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            datatime: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            datatime: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.datatime as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Bits 24:0 = DATALENGTH: Data length value
pub mod dlen {
    pub const OFFSET: u32 = 0x28;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Data length value
        pub datalength: u32,
    }
    pub struct Cache {
        /// Data length value
        pub datalength: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            datalength: ((value >> 0) & 0b1111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            datalength: ((value >> 0) & 0b1111111111111111111111111) as u32,
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
                | ((self.datalength as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// SDIO data control register (SDIO_DCTRL)
pub mod dctrl {
    pub const OFFSET: u32 = 0x2C;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// DTEN
        pub dten: bool,
        /// DTDIR
        pub dtdir: bool,
        /// DTMODE
        pub dtmode: bool,
        /// DMAEN
        pub dmaen: bool,
        /// DBLOCKSIZE
        pub dblocksize: bool,
        /// PWSTART
        pub pwstart: bool,
        /// PWSTOP
        pub pwstop: bool,
        /// RWMOD
        pub rwmod: bool,
        /// SDIOEN
        pub sdioen: bool,
    }
    pub struct Cache {
        /// DTEN
        pub dten: bool,
        /// DTDIR
        pub dtdir: bool,
        /// DTMODE
        pub dtmode: bool,
        /// DMAEN
        pub dmaen: bool,
        /// DBLOCKSIZE
        pub dblocksize: bool,
        /// PWSTART
        pub pwstart: bool,
        /// PWSTOP
        pub pwstop: bool,
        /// RWMOD
        pub rwmod: bool,
        /// SDIOEN
        pub sdioen: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            dten: ((value >> 0) & 0b1) > 0,
            dtdir: ((value >> 1) & 0b1) > 0,
            dtmode: ((value >> 2) & 0b1) > 0,
            dmaen: ((value >> 3) & 0b1) > 0,
            dblocksize: ((value >> 4) & 0b1) > 0,
            pwstart: ((value >> 8) & 0b1) > 0,
            pwstop: ((value >> 9) & 0b1) > 0,
            rwmod: ((value >> 10) & 0b1) > 0,
            sdioen: ((value >> 11) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            dten: ((value >> 0) & 0b1) > 0,
            dtdir: ((value >> 1) & 0b1) > 0,
            dtmode: ((value >> 2) & 0b1) > 0,
            dmaen: ((value >> 3) & 0b1) > 0,
            dblocksize: ((value >> 4) & 0b1) > 0,
            pwstart: ((value >> 8) & 0b1) > 0,
            pwstop: ((value >> 9) & 0b1) > 0,
            rwmod: ((value >> 10) & 0b1) > 0,
            sdioen: ((value >> 11) & 0b1) > 0,
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
                | ((self.dten as u32) << 0)
                | ((self.dtdir as u32) << 1)
                | ((self.dtmode as u32) << 2)
                | ((self.dmaen as u32) << 3)
                | ((self.dblocksize as u32) << 4)
                | ((self.pwstart as u32) << 8)
                | ((self.pwstop as u32) << 9)
                | ((self.rwmod as u32) << 10)
                | ((self.sdioen as u32) << 11)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Bits 24:0 = DATACOUNT: Data count value
pub mod dcount {
    pub const OFFSET: u32 = 0x30;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// Data count value
    /// Access: read-only, Width: 25, Offset: 0
    /// Get Data count value
    pub fn datacount() -> u32 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1111111111111111111111111 << 0);
        value as u32
    }
}
/// SDIO status register (SDIO_STA)
pub mod sta {
    pub const OFFSET: u32 = 0x34;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// CCRCFAIL
    /// Access: read-only, Width: 1, Offset: 0
    /// Get CCRCFAIL
    pub fn ccrcfail() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
    /// DCRCFAIL
    /// Access: read-only, Width: 1, Offset: 1
    /// Get DCRCFAIL
    pub fn dcrcfail() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 1);
        value > 0
    }
    /// CTIMEOUT
    /// Access: read-only, Width: 1, Offset: 2
    /// Get CTIMEOUT
    pub fn ctimeout() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 2);
        value > 0
    }
    /// DTIMEOUT
    /// Access: read-only, Width: 1, Offset: 3
    /// Get DTIMEOUT
    pub fn dtimeout() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 3);
        value > 0
    }
    /// TXUNDERR
    /// Access: read-only, Width: 1, Offset: 4
    /// Get TXUNDERR
    pub fn txunderr() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 4);
        value > 0
    }
    /// RXOVERR
    /// Access: read-only, Width: 1, Offset: 5
    /// Get RXOVERR
    pub fn rxoverr() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 5);
        value > 0
    }
    /// CMDREND
    /// Access: read-only, Width: 1, Offset: 6
    /// Get CMDREND
    pub fn cmdrend() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 6);
        value > 0
    }
    /// CMDSENT
    /// Access: read-only, Width: 1, Offset: 7
    /// Get CMDSENT
    pub fn cmdsent() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 7);
        value > 0
    }
    /// DATAEND
    /// Access: read-only, Width: 1, Offset: 8
    /// Get DATAEND
    pub fn dataend() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 8);
        value > 0
    }
    /// STBITERR
    /// Access: read-only, Width: 1, Offset: 9
    /// Get STBITERR
    pub fn stbiterr() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 9);
        value > 0
    }
    /// DBCKEND
    /// Access: read-only, Width: 1, Offset: 10
    /// Get DBCKEND
    pub fn dbckend() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 10);
        value > 0
    }
    /// CMDACT
    /// Access: read-only, Width: 1, Offset: 11
    /// Get CMDACT
    pub fn cmdact() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 11);
        value > 0
    }
    /// TXACT
    /// Access: read-only, Width: 1, Offset: 12
    /// Get TXACT
    pub fn txact() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 12);
        value > 0
    }
    /// RXACT
    /// Access: read-only, Width: 1, Offset: 13
    /// Get RXACT
    pub fn rxact() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 13);
        value > 0
    }
    /// TXFIFOHE
    /// Access: read-only, Width: 1, Offset: 14
    /// Get TXFIFOHE
    pub fn txfifohe() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 14);
        value > 0
    }
    /// RXFIFOHF
    /// Access: read-only, Width: 1, Offset: 15
    /// Get RXFIFOHF
    pub fn rxfifohf() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 15);
        value > 0
    }
    /// TXFIFOF
    /// Access: read-only, Width: 1, Offset: 16
    /// Get TXFIFOF
    pub fn txfifof() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 16);
        value > 0
    }
    /// RXFIFOF
    /// Access: read-only, Width: 1, Offset: 17
    /// Get RXFIFOF
    pub fn rxfifof() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 17);
        value > 0
    }
    /// TXFIFOE
    /// Access: read-only, Width: 1, Offset: 18
    /// Get TXFIFOE
    pub fn txfifoe() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 18);
        value > 0
    }
    /// RXFIFOE
    /// Access: read-only, Width: 1, Offset: 19
    /// Get RXFIFOE
    pub fn rxfifoe() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 19);
        value > 0
    }
    /// TXDAVL
    /// Access: read-only, Width: 1, Offset: 20
    /// Get TXDAVL
    pub fn txdavl() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 20);
        value > 0
    }
    /// RXDAVL
    /// Access: read-only, Width: 1, Offset: 21
    /// Get RXDAVL
    pub fn rxdavl() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 21);
        value > 0
    }
    /// SDIOIT
    /// Access: read-only, Width: 1, Offset: 22
    /// Get SDIOIT
    pub fn sdioit() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 22);
        value > 0
    }
    /// CEATAEND
    /// Access: read-only, Width: 1, Offset: 23
    /// Get CEATAEND
    pub fn ceataend() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 23);
        value > 0
    }
}
/// SDIO interrupt clear register (SDIO_ICR)
pub mod icr {
    pub const OFFSET: u32 = 0x38;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// CCRCFAILC
        pub ccrcfailc: bool,
        /// DCRCFAILC
        pub dcrcfailc: bool,
        /// CTIMEOUTC
        pub ctimeoutc: bool,
        /// DTIMEOUTC
        pub dtimeoutc: bool,
        /// TXUNDERRC
        pub txunderrc: bool,
        /// RXOVERRC
        pub rxoverrc: bool,
        /// CMDRENDC
        pub cmdrendc: bool,
        /// CMDSENTC
        pub cmdsentc: bool,
        /// DATAENDC
        pub dataendc: bool,
        /// STBITERRC
        pub stbiterrc: bool,
        /// DBCKENDC
        pub dbckendc: bool,
        /// SDIOITC
        pub sdioitc: bool,
        /// CEATAENDC
        pub ceataendc: bool,
    }
    pub struct Cache {
        /// CCRCFAILC
        pub ccrcfailc: bool,
        /// DCRCFAILC
        pub dcrcfailc: bool,
        /// CTIMEOUTC
        pub ctimeoutc: bool,
        /// DTIMEOUTC
        pub dtimeoutc: bool,
        /// TXUNDERRC
        pub txunderrc: bool,
        /// RXOVERRC
        pub rxoverrc: bool,
        /// CMDRENDC
        pub cmdrendc: bool,
        /// CMDSENTC
        pub cmdsentc: bool,
        /// DATAENDC
        pub dataendc: bool,
        /// STBITERRC
        pub stbiterrc: bool,
        /// DBCKENDC
        pub dbckendc: bool,
        /// SDIOITC
        pub sdioitc: bool,
        /// CEATAENDC
        pub ceataendc: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            ccrcfailc: ((value >> 0) & 0b1) > 0,
            dcrcfailc: ((value >> 1) & 0b1) > 0,
            ctimeoutc: ((value >> 2) & 0b1) > 0,
            dtimeoutc: ((value >> 3) & 0b1) > 0,
            txunderrc: ((value >> 4) & 0b1) > 0,
            rxoverrc: ((value >> 5) & 0b1) > 0,
            cmdrendc: ((value >> 6) & 0b1) > 0,
            cmdsentc: ((value >> 7) & 0b1) > 0,
            dataendc: ((value >> 8) & 0b1) > 0,
            stbiterrc: ((value >> 9) & 0b1) > 0,
            dbckendc: ((value >> 10) & 0b1) > 0,
            sdioitc: ((value >> 22) & 0b1) > 0,
            ceataendc: ((value >> 23) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            ccrcfailc: ((value >> 0) & 0b1) > 0,
            dcrcfailc: ((value >> 1) & 0b1) > 0,
            ctimeoutc: ((value >> 2) & 0b1) > 0,
            dtimeoutc: ((value >> 3) & 0b1) > 0,
            txunderrc: ((value >> 4) & 0b1) > 0,
            rxoverrc: ((value >> 5) & 0b1) > 0,
            cmdrendc: ((value >> 6) & 0b1) > 0,
            cmdsentc: ((value >> 7) & 0b1) > 0,
            dataendc: ((value >> 8) & 0b1) > 0,
            stbiterrc: ((value >> 9) & 0b1) > 0,
            dbckendc: ((value >> 10) & 0b1) > 0,
            sdioitc: ((value >> 22) & 0b1) > 0,
            ceataendc: ((value >> 23) & 0b1) > 0,
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
                | ((self.ccrcfailc as u32) << 0)
                | ((self.dcrcfailc as u32) << 1)
                | ((self.ctimeoutc as u32) << 2)
                | ((self.dtimeoutc as u32) << 3)
                | ((self.txunderrc as u32) << 4)
                | ((self.rxoverrc as u32) << 5)
                | ((self.cmdrendc as u32) << 6)
                | ((self.cmdsentc as u32) << 7)
                | ((self.dataendc as u32) << 8)
                | ((self.stbiterrc as u32) << 9)
                | ((self.dbckendc as u32) << 10)
                | ((self.sdioitc as u32) << 22)
                | ((self.ceataendc as u32) << 23)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// SDIO mask register (SDIO_MASK)
pub mod mask {
    pub const OFFSET: u32 = 0x3C;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// CCRCFAILIE
        pub ccrcfailie: bool,
        /// DCRCFAILIE
        pub dcrcfailie: bool,
        /// CTIMEOUTIE
        pub ctimeoutie: bool,
        /// DTIMEOUTIE
        pub dtimeoutie: bool,
        /// TXUNDERRIE
        pub txunderrie: bool,
        /// RXOVERRIE
        pub rxoverrie: bool,
        /// CMDRENDIE
        pub cmdrendie: bool,
        /// CMDSENTIE
        pub cmdsentie: bool,
        /// DATAENDIE
        pub dataendie: bool,
        /// STBITERRIE
        pub stbiterrie: bool,
        /// DBACKENDIE
        pub dbackendie: bool,
        /// CMDACTIE
        pub cmdactie: bool,
        /// TXACTIE
        pub txactie: bool,
        /// RXACTIE
        pub rxactie: bool,
        /// TXFIFOHEIE
        pub txfifoheie: bool,
        /// RXFIFOHFIE
        pub rxfifohfie: bool,
        /// TXFIFOFIE
        pub txfifofie: bool,
        /// RXFIFOFIE
        pub rxfifofie: bool,
        /// TXFIFOEIE
        pub txfifoeie: bool,
        /// RXFIFOEIE
        pub rxfifoeie: bool,
        /// TXDAVLIE
        pub txdavlie: bool,
        /// RXDAVLIE
        pub rxdavlie: bool,
        /// SDIOITIE
        pub sdioitie: bool,
        /// CEATENDIE
        pub ceatendie: bool,
    }
    pub struct Cache {
        /// CCRCFAILIE
        pub ccrcfailie: bool,
        /// DCRCFAILIE
        pub dcrcfailie: bool,
        /// CTIMEOUTIE
        pub ctimeoutie: bool,
        /// DTIMEOUTIE
        pub dtimeoutie: bool,
        /// TXUNDERRIE
        pub txunderrie: bool,
        /// RXOVERRIE
        pub rxoverrie: bool,
        /// CMDRENDIE
        pub cmdrendie: bool,
        /// CMDSENTIE
        pub cmdsentie: bool,
        /// DATAENDIE
        pub dataendie: bool,
        /// STBITERRIE
        pub stbiterrie: bool,
        /// DBACKENDIE
        pub dbackendie: bool,
        /// CMDACTIE
        pub cmdactie: bool,
        /// TXACTIE
        pub txactie: bool,
        /// RXACTIE
        pub rxactie: bool,
        /// TXFIFOHEIE
        pub txfifoheie: bool,
        /// RXFIFOHFIE
        pub rxfifohfie: bool,
        /// TXFIFOFIE
        pub txfifofie: bool,
        /// RXFIFOFIE
        pub rxfifofie: bool,
        /// TXFIFOEIE
        pub txfifoeie: bool,
        /// RXFIFOEIE
        pub rxfifoeie: bool,
        /// TXDAVLIE
        pub txdavlie: bool,
        /// RXDAVLIE
        pub rxdavlie: bool,
        /// SDIOITIE
        pub sdioitie: bool,
        /// CEATENDIE
        pub ceatendie: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            ccrcfailie: ((value >> 0) & 0b1) > 0,
            dcrcfailie: ((value >> 1) & 0b1) > 0,
            ctimeoutie: ((value >> 2) & 0b1) > 0,
            dtimeoutie: ((value >> 3) & 0b1) > 0,
            txunderrie: ((value >> 4) & 0b1) > 0,
            rxoverrie: ((value >> 5) & 0b1) > 0,
            cmdrendie: ((value >> 6) & 0b1) > 0,
            cmdsentie: ((value >> 7) & 0b1) > 0,
            dataendie: ((value >> 8) & 0b1) > 0,
            stbiterrie: ((value >> 9) & 0b1) > 0,
            dbackendie: ((value >> 10) & 0b1) > 0,
            cmdactie: ((value >> 11) & 0b1) > 0,
            txactie: ((value >> 12) & 0b1) > 0,
            rxactie: ((value >> 13) & 0b1) > 0,
            txfifoheie: ((value >> 14) & 0b1) > 0,
            rxfifohfie: ((value >> 15) & 0b1) > 0,
            txfifofie: ((value >> 16) & 0b1) > 0,
            rxfifofie: ((value >> 17) & 0b1) > 0,
            txfifoeie: ((value >> 18) & 0b1) > 0,
            rxfifoeie: ((value >> 19) & 0b1) > 0,
            txdavlie: ((value >> 20) & 0b1) > 0,
            rxdavlie: ((value >> 21) & 0b1) > 0,
            sdioitie: ((value >> 22) & 0b1) > 0,
            ceatendie: ((value >> 23) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            ccrcfailie: ((value >> 0) & 0b1) > 0,
            dcrcfailie: ((value >> 1) & 0b1) > 0,
            ctimeoutie: ((value >> 2) & 0b1) > 0,
            dtimeoutie: ((value >> 3) & 0b1) > 0,
            txunderrie: ((value >> 4) & 0b1) > 0,
            rxoverrie: ((value >> 5) & 0b1) > 0,
            cmdrendie: ((value >> 6) & 0b1) > 0,
            cmdsentie: ((value >> 7) & 0b1) > 0,
            dataendie: ((value >> 8) & 0b1) > 0,
            stbiterrie: ((value >> 9) & 0b1) > 0,
            dbackendie: ((value >> 10) & 0b1) > 0,
            cmdactie: ((value >> 11) & 0b1) > 0,
            txactie: ((value >> 12) & 0b1) > 0,
            rxactie: ((value >> 13) & 0b1) > 0,
            txfifoheie: ((value >> 14) & 0b1) > 0,
            rxfifohfie: ((value >> 15) & 0b1) > 0,
            txfifofie: ((value >> 16) & 0b1) > 0,
            rxfifofie: ((value >> 17) & 0b1) > 0,
            txfifoeie: ((value >> 18) & 0b1) > 0,
            rxfifoeie: ((value >> 19) & 0b1) > 0,
            txdavlie: ((value >> 20) & 0b1) > 0,
            rxdavlie: ((value >> 21) & 0b1) > 0,
            sdioitie: ((value >> 22) & 0b1) > 0,
            ceatendie: ((value >> 23) & 0b1) > 0,
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
                | ((self.ccrcfailie as u32) << 0)
                | ((self.dcrcfailie as u32) << 1)
                | ((self.ctimeoutie as u32) << 2)
                | ((self.dtimeoutie as u32) << 3)
                | ((self.txunderrie as u32) << 4)
                | ((self.rxoverrie as u32) << 5)
                | ((self.cmdrendie as u32) << 6)
                | ((self.cmdsentie as u32) << 7)
                | ((self.dataendie as u32) << 8)
                | ((self.stbiterrie as u32) << 9)
                | ((self.dbackendie as u32) << 10)
                | ((self.cmdactie as u32) << 11)
                | ((self.txactie as u32) << 12)
                | ((self.rxactie as u32) << 13)
                | ((self.txfifoheie as u32) << 14)
                | ((self.rxfifohfie as u32) << 15)
                | ((self.txfifofie as u32) << 16)
                | ((self.rxfifofie as u32) << 17)
                | ((self.txfifoeie as u32) << 18)
                | ((self.rxfifoeie as u32) << 19)
                | ((self.txdavlie as u32) << 20)
                | ((self.rxdavlie as u32) << 21)
                | ((self.sdioitie as u32) << 22)
                | ((self.ceatendie as u32) << 23)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Bits 23:0 = FIFOCOUNT: Remaining number of words to be written to or read from the FIFO
pub mod fifocnt {
    pub const OFFSET: u32 = 0x48;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// FIF0COUNT
    /// Access: read-only, Width: 24, Offset: 0
    /// Get FIF0COUNT
    pub fn fif0count() -> u32 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b111111111111111111111111 << 0);
        value as u32
    }
}
/// bits 31:0 = FIFOData: Receive and transmit FIFO data
pub mod fifo {
    pub const OFFSET: u32 = 0x80;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// FIFOData
        pub fifodata: u32,
    }
    pub struct Cache {
        /// FIFOData
        pub fifodata: u32,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            fifodata: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            fifodata: ((value >> 0) & 0b11111111111111111111111111111111) as u32,
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
                | ((self.fifodata as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// SDIO global interrupt
pub const INTERRUPT_SDIO: u32 = 49;
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <addressBlock>
    <offset>0x0</offset>
    <size>0x400</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0x40018000</baseAddress>
  <description>Secure digital input/output
      interface</description>
  <groupName>SDIO</groupName>
  <interrupt>
    <description>SDIO global interrupt</description>
    <name>SDIO</name>
    <value>49</value>
  </interrupt>
  <name>SDIO</name>
  <registers>
    <register>
      <access>read-write</access>
      <addressOffset>0x0</addressOffset>
      <description>Bits 1:0 = PWRCTRL: Power supply control
          bits</description>
      <displayName>POWER</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>2</bitWidth>
          <description>PWRCTRL</description>
          <name>PWRCTRL</name>
        </field>
      </fields>
      <name>POWER</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x4</addressOffset>
      <description>SDI clock control register
          (SDIO_CLKCR)</description>
      <displayName>CLKCR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>Clock divide factor</description>
          <name>CLKDIV</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Clock enable bit</description>
          <name>CLKEN</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Power saving configuration
              bit</description>
          <name>PWRSAV</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>Clock divider bypass enable
              bit</description>
          <name>BYPASS</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>2</bitWidth>
          <description>Wide bus mode enable bit</description>
          <name>WIDBUS</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>SDIO_CK dephasing selection
              bit</description>
          <name>NEGEDGE</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>HW Flow Control enable</description>
          <name>HWFC_EN</name>
        </field>
      </fields>
      <name>CLKCR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x8</addressOffset>
      <description>Bits 31:0 = : Command argument</description>
      <displayName>ARG</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>Command argument</description>
          <name>CMDARG</name>
        </field>
      </fields>
      <name>ARG</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xC</addressOffset>
      <description>SDIO command register
          (SDIO_CMD)</description>
      <displayName>CMD</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>6</bitWidth>
          <description>CMDINDEX</description>
          <name>CMDINDEX</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>2</bitWidth>
          <description>WAITRESP</description>
          <name>WAITRESP</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>WAITINT</description>
          <name>WAITINT</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>WAITPEND</description>
          <name>WAITPEND</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>CPSMEN</description>
          <name>CPSMEN</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>SDIOSuspend</description>
          <name>SDIOSuspend</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ENCMDcompl</description>
          <name>ENCMDcompl</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>nIEN</description>
          <name>nIEN</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>CE_ATACMD</description>
          <name>CE_ATACMD</name>
        </field>
      </fields>
      <name>CMD</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x10</addressOffset>
      <description>SDIO command register</description>
      <displayName>RESPCMD</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>6</bitWidth>
          <description>RESPCMD</description>
          <name>RESPCMD</name>
        </field>
      </fields>
      <name>RESPCMD</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x14</addressOffset>
      <description>Bits 31:0 = CARDSTATUS1</description>
      <displayName>RESPI1</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>CARDSTATUS1</description>
          <name>CARDSTATUS1</name>
        </field>
      </fields>
      <name>RESPI1</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x18</addressOffset>
      <description>Bits 31:0 = CARDSTATUS2</description>
      <displayName>RESP2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>CARDSTATUS2</description>
          <name>CARDSTATUS2</name>
        </field>
      </fields>
      <name>RESP2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x1C</addressOffset>
      <description>Bits 31:0 = CARDSTATUS3</description>
      <displayName>RESP3</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>CARDSTATUS3</description>
          <name>CARDSTATUS3</name>
        </field>
      </fields>
      <name>RESP3</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x20</addressOffset>
      <description>Bits 31:0 = CARDSTATUS4</description>
      <displayName>RESP4</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>CARDSTATUS4</description>
          <name>CARDSTATUS4</name>
        </field>
      </fields>
      <name>RESP4</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x24</addressOffset>
      <description>Bits 31:0 = DATATIME: Data timeout
          period</description>
      <displayName>DTIMER</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>Data timeout period</description>
          <name>DATATIME</name>
        </field>
      </fields>
      <name>DTIMER</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x28</addressOffset>
      <description>Bits 24:0 = DATALENGTH: Data length
          value</description>
      <displayName>DLEN</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>25</bitWidth>
          <description>Data length value</description>
          <name>DATALENGTH</name>
        </field>
      </fields>
      <name>DLEN</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x2C</addressOffset>
      <description>SDIO data control register
          (SDIO_DCTRL)</description>
      <displayName>DCTRL</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DTEN</description>
          <name>DTEN</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DTDIR</description>
          <name>DTDIR</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DTMODE</description>
          <name>DTMODE</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DMAEN</description>
          <name>DMAEN</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>4</bitWidth>
          <description>DBLOCKSIZE</description>
          <name>DBLOCKSIZE</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>PWSTART</description>
          <name>PWSTART</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>PWSTOP</description>
          <name>PWSTOP</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RWMOD</description>
          <name>RWMOD</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>SDIOEN</description>
          <name>SDIOEN</name>
        </field>
      </fields>
      <name>DCTRL</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x30</addressOffset>
      <description>Bits 24:0 = DATACOUNT: Data count
          value</description>
      <displayName>DCOUNT</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>25</bitWidth>
          <description>Data count value</description>
          <name>DATACOUNT</name>
        </field>
      </fields>
      <name>DCOUNT</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x34</addressOffset>
      <description>SDIO status register
          (SDIO_STA)</description>
      <displayName>STA</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>CCRCFAIL</description>
          <name>CCRCFAIL</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DCRCFAIL</description>
          <name>DCRCFAIL</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>CTIMEOUT</description>
          <name>CTIMEOUT</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DTIMEOUT</description>
          <name>DTIMEOUT</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TXUNDERR</description>
          <name>TXUNDERR</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RXOVERR</description>
          <name>RXOVERR</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>CMDREND</description>
          <name>CMDREND</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>CMDSENT</description>
          <name>CMDSENT</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DATAEND</description>
          <name>DATAEND</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>STBITERR</description>
          <name>STBITERR</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DBCKEND</description>
          <name>DBCKEND</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>CMDACT</description>
          <name>CMDACT</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TXACT</description>
          <name>TXACT</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RXACT</description>
          <name>RXACT</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TXFIFOHE</description>
          <name>TXFIFOHE</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RXFIFOHF</description>
          <name>RXFIFOHF</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TXFIFOF</description>
          <name>TXFIFOF</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RXFIFOF</description>
          <name>RXFIFOF</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TXFIFOE</description>
          <name>TXFIFOE</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RXFIFOE</description>
          <name>RXFIFOE</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TXDAVL</description>
          <name>TXDAVL</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RXDAVL</description>
          <name>RXDAVL</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>SDIOIT</description>
          <name>SDIOIT</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>CEATAEND</description>
          <name>CEATAEND</name>
        </field>
      </fields>
      <name>STA</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x38</addressOffset>
      <description>SDIO interrupt clear register
          (SDIO_ICR)</description>
      <displayName>ICR</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>CCRCFAILC</description>
          <name>CCRCFAILC</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DCRCFAILC</description>
          <name>DCRCFAILC</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>CTIMEOUTC</description>
          <name>CTIMEOUTC</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DTIMEOUTC</description>
          <name>DTIMEOUTC</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TXUNDERRC</description>
          <name>TXUNDERRC</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RXOVERRC</description>
          <name>RXOVERRC</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>CMDRENDC</description>
          <name>CMDRENDC</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>CMDSENTC</description>
          <name>CMDSENTC</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DATAENDC</description>
          <name>DATAENDC</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>STBITERRC</description>
          <name>STBITERRC</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DBCKENDC</description>
          <name>DBCKENDC</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>SDIOITC</description>
          <name>SDIOITC</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>CEATAENDC</description>
          <name>CEATAENDC</name>
        </field>
      </fields>
      <name>ICR</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x3C</addressOffset>
      <description>SDIO mask register (SDIO_MASK)</description>
      <displayName>MASK</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>CCRCFAILIE</description>
          <name>CCRCFAILIE</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DCRCFAILIE</description>
          <name>DCRCFAILIE</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>CTIMEOUTIE</description>
          <name>CTIMEOUTIE</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DTIMEOUTIE</description>
          <name>DTIMEOUTIE</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TXUNDERRIE</description>
          <name>TXUNDERRIE</name>
        </field>
        <field>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RXOVERRIE</description>
          <name>RXOVERRIE</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>CMDRENDIE</description>
          <name>CMDRENDIE</name>
        </field>
        <field>
          <bitOffset>7</bitOffset>
          <bitWidth>1</bitWidth>
          <description>CMDSENTIE</description>
          <name>CMDSENTIE</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DATAENDIE</description>
          <name>DATAENDIE</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>STBITERRIE</description>
          <name>STBITERRIE</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>DBACKENDIE</description>
          <name>DBACKENDIE</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>CMDACTIE</description>
          <name>CMDACTIE</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TXACTIE</description>
          <name>TXACTIE</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RXACTIE</description>
          <name>RXACTIE</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TXFIFOHEIE</description>
          <name>TXFIFOHEIE</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RXFIFOHFIE</description>
          <name>RXFIFOHFIE</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TXFIFOFIE</description>
          <name>TXFIFOFIE</name>
        </field>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RXFIFOFIE</description>
          <name>RXFIFOFIE</name>
        </field>
        <field>
          <bitOffset>18</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TXFIFOEIE</description>
          <name>TXFIFOEIE</name>
        </field>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RXFIFOEIE</description>
          <name>RXFIFOEIE</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>1</bitWidth>
          <description>TXDAVLIE</description>
          <name>TXDAVLIE</name>
        </field>
        <field>
          <bitOffset>21</bitOffset>
          <bitWidth>1</bitWidth>
          <description>RXDAVLIE</description>
          <name>RXDAVLIE</name>
        </field>
        <field>
          <bitOffset>22</bitOffset>
          <bitWidth>1</bitWidth>
          <description>SDIOITIE</description>
          <name>SDIOITIE</name>
        </field>
        <field>
          <bitOffset>23</bitOffset>
          <bitWidth>1</bitWidth>
          <description>CEATENDIE</description>
          <name>CEATENDIE</name>
        </field>
      </fields>
      <name>MASK</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x48</addressOffset>
      <description>Bits 23:0 = FIFOCOUNT: Remaining number of
          words to be written to or read from the
          FIFO</description>
      <displayName>FIFOCNT</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>24</bitWidth>
          <description>FIF0COUNT</description>
          <name>FIF0COUNT</name>
        </field>
      </fields>
      <name>FIFOCNT</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x80</addressOffset>
      <description>bits 31:0 = FIFOData: Receive and transmit
          FIFO data</description>
      <displayName>FIFO</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>FIFOData</description>
          <name>FIFOData</name>
        </field>
      </fields>
      <name>FIFO</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
