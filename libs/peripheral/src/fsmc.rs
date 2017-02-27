pub const ADDRESS: u32 = 0xA0000000;
/// SRAM/NOR-Flash chip-select control register 1
pub mod bcr1 {
    pub const OFFSET: u32 = 0x0;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// CBURSTRW
        pub cburstrw: bool,
        /// ASYNCWAIT
        pub asyncwait: bool,
        /// EXTMOD
        pub extmod: bool,
        /// WAITEN
        pub waiten: bool,
        /// WREN
        pub wren: bool,
        /// WAITCFG
        pub waitcfg: bool,
        /// WAITPOL
        pub waitpol: bool,
        /// BURSTEN
        pub bursten: bool,
        /// FACCEN
        pub faccen: bool,
        /// MWID
        pub mwid: bool,
        /// MTYP
        pub mtyp: bool,
        /// MUXEN
        pub muxen: bool,
        /// MBKEN
        pub mbken: bool,
    }
    pub struct Cache {
        /// CBURSTRW
        pub cburstrw: bool,
        /// ASYNCWAIT
        pub asyncwait: bool,
        /// EXTMOD
        pub extmod: bool,
        /// WAITEN
        pub waiten: bool,
        /// WREN
        pub wren: bool,
        /// WAITCFG
        pub waitcfg: bool,
        /// WAITPOL
        pub waitpol: bool,
        /// BURSTEN
        pub bursten: bool,
        /// FACCEN
        pub faccen: bool,
        /// MWID
        pub mwid: bool,
        /// MTYP
        pub mtyp: bool,
        /// MUXEN
        pub muxen: bool,
        /// MBKEN
        pub mbken: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            cburstrw: ((value >> 19) & 0b1) > 0,
            asyncwait: ((value >> 15) & 0b1) > 0,
            extmod: ((value >> 14) & 0b1) > 0,
            waiten: ((value >> 13) & 0b1) > 0,
            wren: ((value >> 12) & 0b1) > 0,
            waitcfg: ((value >> 11) & 0b1) > 0,
            waitpol: ((value >> 9) & 0b1) > 0,
            bursten: ((value >> 8) & 0b1) > 0,
            faccen: ((value >> 6) & 0b1) > 0,
            mwid: ((value >> 4) & 0b1) > 0,
            mtyp: ((value >> 2) & 0b1) > 0,
            muxen: ((value >> 1) & 0b1) > 0,
            mbken: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            cburstrw: ((value >> 19) & 0b1) > 0,
            asyncwait: ((value >> 15) & 0b1) > 0,
            extmod: ((value >> 14) & 0b1) > 0,
            waiten: ((value >> 13) & 0b1) > 0,
            wren: ((value >> 12) & 0b1) > 0,
            waitcfg: ((value >> 11) & 0b1) > 0,
            waitpol: ((value >> 9) & 0b1) > 0,
            bursten: ((value >> 8) & 0b1) > 0,
            faccen: ((value >> 6) & 0b1) > 0,
            mwid: ((value >> 4) & 0b1) > 0,
            mtyp: ((value >> 2) & 0b1) > 0,
            muxen: ((value >> 1) & 0b1) > 0,
            mbken: ((value >> 0) & 0b1) > 0,
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
                | ((self.cburstrw as u32) << 19)
                | ((self.asyncwait as u32) << 15)
                | ((self.extmod as u32) << 14)
                | ((self.waiten as u32) << 13)
                | ((self.wren as u32) << 12)
                | ((self.waitcfg as u32) << 11)
                | ((self.waitpol as u32) << 9)
                | ((self.bursten as u32) << 8)
                | ((self.faccen as u32) << 6)
                | ((self.mwid as u32) << 4)
                | ((self.mtyp as u32) << 2)
                | ((self.muxen as u32) << 1)
                | ((self.mbken as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// SRAM/NOR-Flash chip-select timing register 1
pub mod btr1 {
    pub const OFFSET: u32 = 0x4;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// ACCMOD
        pub accmod: u8,
        /// DATLAT
        pub datlat: u8,
        /// CLKDIV
        pub clkdiv: u8,
        /// BUSTURN
        pub busturn: u8,
        /// DATAST
        pub datast: u8,
        /// ADDHLD
        pub addhld: u8,
        /// ADDSET
        pub addset: u8,
    }
    pub struct Cache {
        /// ACCMOD
        pub accmod: u8,
        /// DATLAT
        pub datlat: u8,
        /// CLKDIV
        pub clkdiv: u8,
        /// BUSTURN
        pub busturn: u8,
        /// DATAST
        pub datast: u8,
        /// ADDHLD
        pub addhld: u8,
        /// ADDSET
        pub addset: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            accmod: ((value >> 28) & 0b11) as u8,
            datlat: ((value >> 24) & 0b11) as u8,
            clkdiv: ((value >> 20) & 0b11) as u8,
            busturn: ((value >> 16) & 0b11) as u8,
            datast: ((value >> 8) & 0b11) as u8,
            addhld: ((value >> 4) & 0b11) as u8,
            addset: ((value >> 0) & 0b11) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            accmod: ((value >> 28) & 0b11) as u8,
            datlat: ((value >> 24) & 0b11) as u8,
            clkdiv: ((value >> 20) & 0b11) as u8,
            busturn: ((value >> 16) & 0b11) as u8,
            datast: ((value >> 8) & 0b11) as u8,
            addhld: ((value >> 4) & 0b11) as u8,
            addset: ((value >> 0) & 0b11) as u8,
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
                | ((self.accmod as u32) << 28)
                | ((self.datlat as u32) << 24)
                | ((self.clkdiv as u32) << 20)
                | ((self.busturn as u32) << 16)
                | ((self.datast as u32) << 8)
                | ((self.addhld as u32) << 4)
                | ((self.addset as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// SRAM/NOR-Flash chip-select control register 2
pub mod bcr2 {
    pub const OFFSET: u32 = 0x8;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// CBURSTRW
        pub cburstrw: bool,
        /// ASYNCWAIT
        pub asyncwait: bool,
        /// EXTMOD
        pub extmod: bool,
        /// WAITEN
        pub waiten: bool,
        /// WREN
        pub wren: bool,
        /// WAITCFG
        pub waitcfg: bool,
        /// WRAPMOD
        pub wrapmod: bool,
        /// WAITPOL
        pub waitpol: bool,
        /// BURSTEN
        pub bursten: bool,
        /// FACCEN
        pub faccen: bool,
        /// MWID
        pub mwid: bool,
        /// MTYP
        pub mtyp: bool,
        /// MUXEN
        pub muxen: bool,
        /// MBKEN
        pub mbken: bool,
    }
    pub struct Cache {
        /// CBURSTRW
        pub cburstrw: bool,
        /// ASYNCWAIT
        pub asyncwait: bool,
        /// EXTMOD
        pub extmod: bool,
        /// WAITEN
        pub waiten: bool,
        /// WREN
        pub wren: bool,
        /// WAITCFG
        pub waitcfg: bool,
        /// WRAPMOD
        pub wrapmod: bool,
        /// WAITPOL
        pub waitpol: bool,
        /// BURSTEN
        pub bursten: bool,
        /// FACCEN
        pub faccen: bool,
        /// MWID
        pub mwid: bool,
        /// MTYP
        pub mtyp: bool,
        /// MUXEN
        pub muxen: bool,
        /// MBKEN
        pub mbken: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            cburstrw: ((value >> 19) & 0b1) > 0,
            asyncwait: ((value >> 15) & 0b1) > 0,
            extmod: ((value >> 14) & 0b1) > 0,
            waiten: ((value >> 13) & 0b1) > 0,
            wren: ((value >> 12) & 0b1) > 0,
            waitcfg: ((value >> 11) & 0b1) > 0,
            wrapmod: ((value >> 10) & 0b1) > 0,
            waitpol: ((value >> 9) & 0b1) > 0,
            bursten: ((value >> 8) & 0b1) > 0,
            faccen: ((value >> 6) & 0b1) > 0,
            mwid: ((value >> 4) & 0b1) > 0,
            mtyp: ((value >> 2) & 0b1) > 0,
            muxen: ((value >> 1) & 0b1) > 0,
            mbken: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            cburstrw: ((value >> 19) & 0b1) > 0,
            asyncwait: ((value >> 15) & 0b1) > 0,
            extmod: ((value >> 14) & 0b1) > 0,
            waiten: ((value >> 13) & 0b1) > 0,
            wren: ((value >> 12) & 0b1) > 0,
            waitcfg: ((value >> 11) & 0b1) > 0,
            wrapmod: ((value >> 10) & 0b1) > 0,
            waitpol: ((value >> 9) & 0b1) > 0,
            bursten: ((value >> 8) & 0b1) > 0,
            faccen: ((value >> 6) & 0b1) > 0,
            mwid: ((value >> 4) & 0b1) > 0,
            mtyp: ((value >> 2) & 0b1) > 0,
            muxen: ((value >> 1) & 0b1) > 0,
            mbken: ((value >> 0) & 0b1) > 0,
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
                | ((self.cburstrw as u32) << 19)
                | ((self.asyncwait as u32) << 15)
                | ((self.extmod as u32) << 14)
                | ((self.waiten as u32) << 13)
                | ((self.wren as u32) << 12)
                | ((self.waitcfg as u32) << 11)
                | ((self.wrapmod as u32) << 10)
                | ((self.waitpol as u32) << 9)
                | ((self.bursten as u32) << 8)
                | ((self.faccen as u32) << 6)
                | ((self.mwid as u32) << 4)
                | ((self.mtyp as u32) << 2)
                | ((self.muxen as u32) << 1)
                | ((self.mbken as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// SRAM/NOR-Flash chip-select timing register 2
pub mod btr2 {
    pub const OFFSET: u32 = 0xC;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// ACCMOD
        pub accmod: u8,
        /// DATLAT
        pub datlat: u8,
        /// CLKDIV
        pub clkdiv: u8,
        /// BUSTURN
        pub busturn: u8,
        /// DATAST
        pub datast: u8,
        /// ADDHLD
        pub addhld: u8,
        /// ADDSET
        pub addset: u8,
    }
    pub struct Cache {
        /// ACCMOD
        pub accmod: u8,
        /// DATLAT
        pub datlat: u8,
        /// CLKDIV
        pub clkdiv: u8,
        /// BUSTURN
        pub busturn: u8,
        /// DATAST
        pub datast: u8,
        /// ADDHLD
        pub addhld: u8,
        /// ADDSET
        pub addset: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            accmod: ((value >> 28) & 0b11) as u8,
            datlat: ((value >> 24) & 0b11) as u8,
            clkdiv: ((value >> 20) & 0b11) as u8,
            busturn: ((value >> 16) & 0b11) as u8,
            datast: ((value >> 8) & 0b11) as u8,
            addhld: ((value >> 4) & 0b11) as u8,
            addset: ((value >> 0) & 0b11) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            accmod: ((value >> 28) & 0b11) as u8,
            datlat: ((value >> 24) & 0b11) as u8,
            clkdiv: ((value >> 20) & 0b11) as u8,
            busturn: ((value >> 16) & 0b11) as u8,
            datast: ((value >> 8) & 0b11) as u8,
            addhld: ((value >> 4) & 0b11) as u8,
            addset: ((value >> 0) & 0b11) as u8,
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
                | ((self.accmod as u32) << 28)
                | ((self.datlat as u32) << 24)
                | ((self.clkdiv as u32) << 20)
                | ((self.busturn as u32) << 16)
                | ((self.datast as u32) << 8)
                | ((self.addhld as u32) << 4)
                | ((self.addset as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// SRAM/NOR-Flash chip-select control register 3
pub mod bcr3 {
    pub const OFFSET: u32 = 0x10;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// CBURSTRW
        pub cburstrw: bool,
        /// ASYNCWAIT
        pub asyncwait: bool,
        /// EXTMOD
        pub extmod: bool,
        /// WAITEN
        pub waiten: bool,
        /// WREN
        pub wren: bool,
        /// WAITCFG
        pub waitcfg: bool,
        /// WRAPMOD
        pub wrapmod: bool,
        /// WAITPOL
        pub waitpol: bool,
        /// BURSTEN
        pub bursten: bool,
        /// FACCEN
        pub faccen: bool,
        /// MWID
        pub mwid: bool,
        /// MTYP
        pub mtyp: bool,
        /// MUXEN
        pub muxen: bool,
        /// MBKEN
        pub mbken: bool,
    }
    pub struct Cache {
        /// CBURSTRW
        pub cburstrw: bool,
        /// ASYNCWAIT
        pub asyncwait: bool,
        /// EXTMOD
        pub extmod: bool,
        /// WAITEN
        pub waiten: bool,
        /// WREN
        pub wren: bool,
        /// WAITCFG
        pub waitcfg: bool,
        /// WRAPMOD
        pub wrapmod: bool,
        /// WAITPOL
        pub waitpol: bool,
        /// BURSTEN
        pub bursten: bool,
        /// FACCEN
        pub faccen: bool,
        /// MWID
        pub mwid: bool,
        /// MTYP
        pub mtyp: bool,
        /// MUXEN
        pub muxen: bool,
        /// MBKEN
        pub mbken: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            cburstrw: ((value >> 19) & 0b1) > 0,
            asyncwait: ((value >> 15) & 0b1) > 0,
            extmod: ((value >> 14) & 0b1) > 0,
            waiten: ((value >> 13) & 0b1) > 0,
            wren: ((value >> 12) & 0b1) > 0,
            waitcfg: ((value >> 11) & 0b1) > 0,
            wrapmod: ((value >> 10) & 0b1) > 0,
            waitpol: ((value >> 9) & 0b1) > 0,
            bursten: ((value >> 8) & 0b1) > 0,
            faccen: ((value >> 6) & 0b1) > 0,
            mwid: ((value >> 4) & 0b1) > 0,
            mtyp: ((value >> 2) & 0b1) > 0,
            muxen: ((value >> 1) & 0b1) > 0,
            mbken: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            cburstrw: ((value >> 19) & 0b1) > 0,
            asyncwait: ((value >> 15) & 0b1) > 0,
            extmod: ((value >> 14) & 0b1) > 0,
            waiten: ((value >> 13) & 0b1) > 0,
            wren: ((value >> 12) & 0b1) > 0,
            waitcfg: ((value >> 11) & 0b1) > 0,
            wrapmod: ((value >> 10) & 0b1) > 0,
            waitpol: ((value >> 9) & 0b1) > 0,
            bursten: ((value >> 8) & 0b1) > 0,
            faccen: ((value >> 6) & 0b1) > 0,
            mwid: ((value >> 4) & 0b1) > 0,
            mtyp: ((value >> 2) & 0b1) > 0,
            muxen: ((value >> 1) & 0b1) > 0,
            mbken: ((value >> 0) & 0b1) > 0,
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
                | ((self.cburstrw as u32) << 19)
                | ((self.asyncwait as u32) << 15)
                | ((self.extmod as u32) << 14)
                | ((self.waiten as u32) << 13)
                | ((self.wren as u32) << 12)
                | ((self.waitcfg as u32) << 11)
                | ((self.wrapmod as u32) << 10)
                | ((self.waitpol as u32) << 9)
                | ((self.bursten as u32) << 8)
                | ((self.faccen as u32) << 6)
                | ((self.mwid as u32) << 4)
                | ((self.mtyp as u32) << 2)
                | ((self.muxen as u32) << 1)
                | ((self.mbken as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// SRAM/NOR-Flash chip-select timing register 3
pub mod btr3 {
    pub const OFFSET: u32 = 0x14;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// ACCMOD
        pub accmod: u8,
        /// DATLAT
        pub datlat: u8,
        /// CLKDIV
        pub clkdiv: u8,
        /// BUSTURN
        pub busturn: u8,
        /// DATAST
        pub datast: u8,
        /// ADDHLD
        pub addhld: u8,
        /// ADDSET
        pub addset: u8,
    }
    pub struct Cache {
        /// ACCMOD
        pub accmod: u8,
        /// DATLAT
        pub datlat: u8,
        /// CLKDIV
        pub clkdiv: u8,
        /// BUSTURN
        pub busturn: u8,
        /// DATAST
        pub datast: u8,
        /// ADDHLD
        pub addhld: u8,
        /// ADDSET
        pub addset: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            accmod: ((value >> 28) & 0b11) as u8,
            datlat: ((value >> 24) & 0b11) as u8,
            clkdiv: ((value >> 20) & 0b11) as u8,
            busturn: ((value >> 16) & 0b11) as u8,
            datast: ((value >> 8) & 0b11) as u8,
            addhld: ((value >> 4) & 0b11) as u8,
            addset: ((value >> 0) & 0b11) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            accmod: ((value >> 28) & 0b11) as u8,
            datlat: ((value >> 24) & 0b11) as u8,
            clkdiv: ((value >> 20) & 0b11) as u8,
            busturn: ((value >> 16) & 0b11) as u8,
            datast: ((value >> 8) & 0b11) as u8,
            addhld: ((value >> 4) & 0b11) as u8,
            addset: ((value >> 0) & 0b11) as u8,
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
                | ((self.accmod as u32) << 28)
                | ((self.datlat as u32) << 24)
                | ((self.clkdiv as u32) << 20)
                | ((self.busturn as u32) << 16)
                | ((self.datast as u32) << 8)
                | ((self.addhld as u32) << 4)
                | ((self.addset as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// SRAM/NOR-Flash chip-select control register 4
pub mod bcr4 {
    pub const OFFSET: u32 = 0x18;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// CBURSTRW
        pub cburstrw: bool,
        /// ASYNCWAIT
        pub asyncwait: bool,
        /// EXTMOD
        pub extmod: bool,
        /// WAITEN
        pub waiten: bool,
        /// WREN
        pub wren: bool,
        /// WAITCFG
        pub waitcfg: bool,
        /// WRAPMOD
        pub wrapmod: bool,
        /// WAITPOL
        pub waitpol: bool,
        /// BURSTEN
        pub bursten: bool,
        /// FACCEN
        pub faccen: bool,
        /// MWID
        pub mwid: bool,
        /// MTYP
        pub mtyp: bool,
        /// MUXEN
        pub muxen: bool,
        /// MBKEN
        pub mbken: bool,
    }
    pub struct Cache {
        /// CBURSTRW
        pub cburstrw: bool,
        /// ASYNCWAIT
        pub asyncwait: bool,
        /// EXTMOD
        pub extmod: bool,
        /// WAITEN
        pub waiten: bool,
        /// WREN
        pub wren: bool,
        /// WAITCFG
        pub waitcfg: bool,
        /// WRAPMOD
        pub wrapmod: bool,
        /// WAITPOL
        pub waitpol: bool,
        /// BURSTEN
        pub bursten: bool,
        /// FACCEN
        pub faccen: bool,
        /// MWID
        pub mwid: bool,
        /// MTYP
        pub mtyp: bool,
        /// MUXEN
        pub muxen: bool,
        /// MBKEN
        pub mbken: bool,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            cburstrw: ((value >> 19) & 0b1) > 0,
            asyncwait: ((value >> 15) & 0b1) > 0,
            extmod: ((value >> 14) & 0b1) > 0,
            waiten: ((value >> 13) & 0b1) > 0,
            wren: ((value >> 12) & 0b1) > 0,
            waitcfg: ((value >> 11) & 0b1) > 0,
            wrapmod: ((value >> 10) & 0b1) > 0,
            waitpol: ((value >> 9) & 0b1) > 0,
            bursten: ((value >> 8) & 0b1) > 0,
            faccen: ((value >> 6) & 0b1) > 0,
            mwid: ((value >> 4) & 0b1) > 0,
            mtyp: ((value >> 2) & 0b1) > 0,
            muxen: ((value >> 1) & 0b1) > 0,
            mbken: ((value >> 0) & 0b1) > 0,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            cburstrw: ((value >> 19) & 0b1) > 0,
            asyncwait: ((value >> 15) & 0b1) > 0,
            extmod: ((value >> 14) & 0b1) > 0,
            waiten: ((value >> 13) & 0b1) > 0,
            wren: ((value >> 12) & 0b1) > 0,
            waitcfg: ((value >> 11) & 0b1) > 0,
            wrapmod: ((value >> 10) & 0b1) > 0,
            waitpol: ((value >> 9) & 0b1) > 0,
            bursten: ((value >> 8) & 0b1) > 0,
            faccen: ((value >> 6) & 0b1) > 0,
            mwid: ((value >> 4) & 0b1) > 0,
            mtyp: ((value >> 2) & 0b1) > 0,
            muxen: ((value >> 1) & 0b1) > 0,
            mbken: ((value >> 0) & 0b1) > 0,
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
                | ((self.cburstrw as u32) << 19)
                | ((self.asyncwait as u32) << 15)
                | ((self.extmod as u32) << 14)
                | ((self.waiten as u32) << 13)
                | ((self.wren as u32) << 12)
                | ((self.waitcfg as u32) << 11)
                | ((self.wrapmod as u32) << 10)
                | ((self.waitpol as u32) << 9)
                | ((self.bursten as u32) << 8)
                | ((self.faccen as u32) << 6)
                | ((self.mwid as u32) << 4)
                | ((self.mtyp as u32) << 2)
                | ((self.muxen as u32) << 1)
                | ((self.mbken as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// SRAM/NOR-Flash chip-select timing register 4
pub mod btr4 {
    pub const OFFSET: u32 = 0x1C;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// ACCMOD
        pub accmod: u8,
        /// DATLAT
        pub datlat: u8,
        /// CLKDIV
        pub clkdiv: u8,
        /// BUSTURN
        pub busturn: u8,
        /// DATAST
        pub datast: u8,
        /// ADDHLD
        pub addhld: u8,
        /// ADDSET
        pub addset: u8,
    }
    pub struct Cache {
        /// ACCMOD
        pub accmod: u8,
        /// DATLAT
        pub datlat: u8,
        /// CLKDIV
        pub clkdiv: u8,
        /// BUSTURN
        pub busturn: u8,
        /// DATAST
        pub datast: u8,
        /// ADDHLD
        pub addhld: u8,
        /// ADDSET
        pub addset: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            accmod: ((value >> 28) & 0b11) as u8,
            datlat: ((value >> 24) & 0b11) as u8,
            clkdiv: ((value >> 20) & 0b11) as u8,
            busturn: ((value >> 16) & 0b11) as u8,
            datast: ((value >> 8) & 0b11) as u8,
            addhld: ((value >> 4) & 0b11) as u8,
            addset: ((value >> 0) & 0b11) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            accmod: ((value >> 28) & 0b11) as u8,
            datlat: ((value >> 24) & 0b11) as u8,
            clkdiv: ((value >> 20) & 0b11) as u8,
            busturn: ((value >> 16) & 0b11) as u8,
            datast: ((value >> 8) & 0b11) as u8,
            addhld: ((value >> 4) & 0b11) as u8,
            addset: ((value >> 0) & 0b11) as u8,
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
                | ((self.accmod as u32) << 28)
                | ((self.datlat as u32) << 24)
                | ((self.clkdiv as u32) << 20)
                | ((self.busturn as u32) << 16)
                | ((self.datast as u32) << 8)
                | ((self.addhld as u32) << 4)
                | ((self.addset as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// PC Card/NAND Flash control register 2
pub mod pcr2 {
    pub const OFFSET: u32 = 0x60;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// ECCPS
        pub eccps: u8,
        /// TAR
        pub tar: u8,
        /// TCLR
        pub tclr: u8,
        /// ECCEN
        pub eccen: u8,
        /// PWID
        pub pwid: u8,
        /// PTYP
        pub ptyp: u8,
        /// PBKEN
        pub pbken: u8,
        /// PWAITEN
        pub pwaiten: u8,
    }
    pub struct Cache {
        /// ECCPS
        pub eccps: u8,
        /// TAR
        pub tar: u8,
        /// TCLR
        pub tclr: u8,
        /// ECCEN
        pub eccen: u8,
        /// PWID
        pub pwid: u8,
        /// PTYP
        pub ptyp: u8,
        /// PBKEN
        pub pbken: u8,
        /// PWAITEN
        pub pwaiten: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            eccps: ((value >> 17) & 0b111) as u8,
            tar: ((value >> 13) & 0b111) as u8,
            tclr: ((value >> 9) & 0b111) as u8,
            eccen: ((value >> 6) & 0b111) as u8,
            pwid: ((value >> 4) & 0b111) as u8,
            ptyp: ((value >> 3) & 0b111) as u8,
            pbken: ((value >> 2) & 0b111) as u8,
            pwaiten: ((value >> 1) & 0b111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            eccps: ((value >> 17) & 0b111) as u8,
            tar: ((value >> 13) & 0b111) as u8,
            tclr: ((value >> 9) & 0b111) as u8,
            eccen: ((value >> 6) & 0b111) as u8,
            pwid: ((value >> 4) & 0b111) as u8,
            ptyp: ((value >> 3) & 0b111) as u8,
            pbken: ((value >> 2) & 0b111) as u8,
            pwaiten: ((value >> 1) & 0b111) as u8,
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
                | ((self.eccps as u32) << 17)
                | ((self.tar as u32) << 13)
                | ((self.tclr as u32) << 9)
                | ((self.eccen as u32) << 6)
                | ((self.pwid as u32) << 4)
                | ((self.ptyp as u32) << 3)
                | ((self.pbken as u32) << 2)
                | ((self.pwaiten as u32) << 1)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// FIFO status and interrupt register 2
pub mod sr2 {
    pub const OFFSET: u32 = 0x64;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// FEMPT
    /// Access: read-only, Width: 1, Offset: 6
    /// Get FEMPT
    pub fn fempt() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 6);
        value > 0
    }
    /// IFEN
    /// Access: read-write, Width: 1, Offset: 5
    /// Set IFEN
    pub fn set_ifen(value: bool) {
        let value = value as u32;
        let value = value << 5;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get IFEN
    pub fn get_ifen() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 5);
        value > 0
    }
    /// ILEN
    /// Access: read-write, Width: 1, Offset: 4
    /// Set ILEN
    pub fn set_ilen(value: bool) {
        let value = value as u32;
        let value = value << 4;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get ILEN
    pub fn get_ilen() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 4);
        value > 0
    }
    /// IREN
    /// Access: read-write, Width: 1, Offset: 3
    /// Set IREN
    pub fn set_iren(value: bool) {
        let value = value as u32;
        let value = value << 3;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get IREN
    pub fn get_iren() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 3);
        value > 0
    }
    /// IFS
    /// Access: read-write, Width: 1, Offset: 2
    /// Set IFS
    pub fn set_ifs(value: bool) {
        let value = value as u32;
        let value = value << 2;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get IFS
    pub fn get_ifs() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 2);
        value > 0
    }
    /// ILS
    /// Access: read-write, Width: 1, Offset: 1
    /// Set ILS
    pub fn set_ils(value: bool) {
        let value = value as u32;
        let value = value << 1;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get ILS
    pub fn get_ils() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 1);
        value > 0
    }
    /// IRS
    /// Access: read-write, Width: 1, Offset: 0
    /// Set IRS
    pub fn set_irs(value: bool) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get IRS
    pub fn get_irs() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
}
/// Common memory space timing register 2
pub mod pmem2 {
    pub const OFFSET: u32 = 0x68;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// MEMHIZx
        pub memhizx: u8,
        /// MEMHOLDx
        pub memholdx: u8,
        /// MEMWAITx
        pub memwaitx: u8,
        /// MEMSETx
        pub memsetx: u8,
    }
    pub struct Cache {
        /// MEMHIZx
        pub memhizx: u8,
        /// MEMHOLDx
        pub memholdx: u8,
        /// MEMWAITx
        pub memwaitx: u8,
        /// MEMSETx
        pub memsetx: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            memhizx: ((value >> 24) & 0b11111111) as u8,
            memholdx: ((value >> 16) & 0b11111111) as u8,
            memwaitx: ((value >> 8) & 0b11111111) as u8,
            memsetx: ((value >> 0) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            memhizx: ((value >> 24) & 0b11111111) as u8,
            memholdx: ((value >> 16) & 0b11111111) as u8,
            memwaitx: ((value >> 8) & 0b11111111) as u8,
            memsetx: ((value >> 0) & 0b11111111) as u8,
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
                | ((self.memhizx as u32) << 24)
                | ((self.memholdx as u32) << 16)
                | ((self.memwaitx as u32) << 8)
                | ((self.memsetx as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Attribute memory space timing register 2
pub mod patt2 {
    pub const OFFSET: u32 = 0x6C;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// Attribute memory x databus HiZ time
        pub atthizx: u8,
        /// Attribute memory x hold time
        pub attholdx: u8,
        /// Attribute memory x wait time
        pub attwaitx: u8,
        /// Attribute memory x setup time
        pub attsetx: u8,
    }
    pub struct Cache {
        /// Attribute memory x databus HiZ time
        pub atthizx: u8,
        /// Attribute memory x hold time
        pub attholdx: u8,
        /// Attribute memory x wait time
        pub attwaitx: u8,
        /// Attribute memory x setup time
        pub attsetx: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            atthizx: ((value >> 24) & 0b11111111) as u8,
            attholdx: ((value >> 16) & 0b11111111) as u8,
            attwaitx: ((value >> 8) & 0b11111111) as u8,
            attsetx: ((value >> 0) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            atthizx: ((value >> 24) & 0b11111111) as u8,
            attholdx: ((value >> 16) & 0b11111111) as u8,
            attwaitx: ((value >> 8) & 0b11111111) as u8,
            attsetx: ((value >> 0) & 0b11111111) as u8,
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
                | ((self.atthizx as u32) << 24)
                | ((self.attholdx as u32) << 16)
                | ((self.attwaitx as u32) << 8)
                | ((self.attsetx as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// ECC result register 2
pub mod eccr2 {
    pub const OFFSET: u32 = 0x74;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// ECC result
    /// Access: read-only, Width: 32, Offset: 0
    /// Get ECC result
    pub fn eccx() -> u32 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b11111111111111111111111111111111 << 0);
        value as u32
    }
}
/// PC Card/NAND Flash control register 3
pub mod pcr3 {
    pub const OFFSET: u32 = 0x80;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// ECCPS
        pub eccps: u8,
        /// TAR
        pub tar: u8,
        /// TCLR
        pub tclr: u8,
        /// ECCEN
        pub eccen: u8,
        /// PWID
        pub pwid: u8,
        /// PTYP
        pub ptyp: u8,
        /// PBKEN
        pub pbken: u8,
        /// PWAITEN
        pub pwaiten: u8,
    }
    pub struct Cache {
        /// ECCPS
        pub eccps: u8,
        /// TAR
        pub tar: u8,
        /// TCLR
        pub tclr: u8,
        /// ECCEN
        pub eccen: u8,
        /// PWID
        pub pwid: u8,
        /// PTYP
        pub ptyp: u8,
        /// PBKEN
        pub pbken: u8,
        /// PWAITEN
        pub pwaiten: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            eccps: ((value >> 17) & 0b111) as u8,
            tar: ((value >> 13) & 0b111) as u8,
            tclr: ((value >> 9) & 0b111) as u8,
            eccen: ((value >> 6) & 0b111) as u8,
            pwid: ((value >> 4) & 0b111) as u8,
            ptyp: ((value >> 3) & 0b111) as u8,
            pbken: ((value >> 2) & 0b111) as u8,
            pwaiten: ((value >> 1) & 0b111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            eccps: ((value >> 17) & 0b111) as u8,
            tar: ((value >> 13) & 0b111) as u8,
            tclr: ((value >> 9) & 0b111) as u8,
            eccen: ((value >> 6) & 0b111) as u8,
            pwid: ((value >> 4) & 0b111) as u8,
            ptyp: ((value >> 3) & 0b111) as u8,
            pbken: ((value >> 2) & 0b111) as u8,
            pwaiten: ((value >> 1) & 0b111) as u8,
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
                | ((self.eccps as u32) << 17)
                | ((self.tar as u32) << 13)
                | ((self.tclr as u32) << 9)
                | ((self.eccen as u32) << 6)
                | ((self.pwid as u32) << 4)
                | ((self.ptyp as u32) << 3)
                | ((self.pbken as u32) << 2)
                | ((self.pwaiten as u32) << 1)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// FIFO status and interrupt register 3
pub mod sr3 {
    pub const OFFSET: u32 = 0x84;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// FEMPT
    /// Access: read-only, Width: 1, Offset: 6
    /// Get FEMPT
    pub fn fempt() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 6);
        value > 0
    }
    /// IFEN
    /// Access: read-write, Width: 1, Offset: 5
    /// Set IFEN
    pub fn set_ifen(value: bool) {
        let value = value as u32;
        let value = value << 5;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get IFEN
    pub fn get_ifen() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 5);
        value > 0
    }
    /// ILEN
    /// Access: read-write, Width: 1, Offset: 4
    /// Set ILEN
    pub fn set_ilen(value: bool) {
        let value = value as u32;
        let value = value << 4;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get ILEN
    pub fn get_ilen() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 4);
        value > 0
    }
    /// IREN
    /// Access: read-write, Width: 1, Offset: 3
    /// Set IREN
    pub fn set_iren(value: bool) {
        let value = value as u32;
        let value = value << 3;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get IREN
    pub fn get_iren() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 3);
        value > 0
    }
    /// IFS
    /// Access: read-write, Width: 1, Offset: 2
    /// Set IFS
    pub fn set_ifs(value: bool) {
        let value = value as u32;
        let value = value << 2;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get IFS
    pub fn get_ifs() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 2);
        value > 0
    }
    /// ILS
    /// Access: read-write, Width: 1, Offset: 1
    /// Set ILS
    pub fn set_ils(value: bool) {
        let value = value as u32;
        let value = value << 1;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get ILS
    pub fn get_ils() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 1);
        value > 0
    }
    /// IRS
    /// Access: read-write, Width: 1, Offset: 0
    /// Set IRS
    pub fn set_irs(value: bool) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get IRS
    pub fn get_irs() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
}
/// Common memory space timing register 3
pub mod pmem3 {
    pub const OFFSET: u32 = 0x88;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// MEMHIZx
        pub memhizx: u8,
        /// MEMHOLDx
        pub memholdx: u8,
        /// MEMWAITx
        pub memwaitx: u8,
        /// MEMSETx
        pub memsetx: u8,
    }
    pub struct Cache {
        /// MEMHIZx
        pub memhizx: u8,
        /// MEMHOLDx
        pub memholdx: u8,
        /// MEMWAITx
        pub memwaitx: u8,
        /// MEMSETx
        pub memsetx: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            memhizx: ((value >> 24) & 0b11111111) as u8,
            memholdx: ((value >> 16) & 0b11111111) as u8,
            memwaitx: ((value >> 8) & 0b11111111) as u8,
            memsetx: ((value >> 0) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            memhizx: ((value >> 24) & 0b11111111) as u8,
            memholdx: ((value >> 16) & 0b11111111) as u8,
            memwaitx: ((value >> 8) & 0b11111111) as u8,
            memsetx: ((value >> 0) & 0b11111111) as u8,
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
                | ((self.memhizx as u32) << 24)
                | ((self.memholdx as u32) << 16)
                | ((self.memwaitx as u32) << 8)
                | ((self.memsetx as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Attribute memory space timing register 3
pub mod patt3 {
    pub const OFFSET: u32 = 0x8C;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// ATTHIZx
        pub atthizx: u8,
        /// ATTHOLDx
        pub attholdx: u8,
        /// ATTWAITx
        pub attwaitx: u8,
        /// ATTSETx
        pub attsetx: u8,
    }
    pub struct Cache {
        /// ATTHIZx
        pub atthizx: u8,
        /// ATTHOLDx
        pub attholdx: u8,
        /// ATTWAITx
        pub attwaitx: u8,
        /// ATTSETx
        pub attsetx: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            atthizx: ((value >> 24) & 0b11111111) as u8,
            attholdx: ((value >> 16) & 0b11111111) as u8,
            attwaitx: ((value >> 8) & 0b11111111) as u8,
            attsetx: ((value >> 0) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            atthizx: ((value >> 24) & 0b11111111) as u8,
            attholdx: ((value >> 16) & 0b11111111) as u8,
            attwaitx: ((value >> 8) & 0b11111111) as u8,
            attsetx: ((value >> 0) & 0b11111111) as u8,
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
                | ((self.atthizx as u32) << 24)
                | ((self.attholdx as u32) << 16)
                | ((self.attwaitx as u32) << 8)
                | ((self.attsetx as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// ECC result register 3
pub mod eccr3 {
    pub const OFFSET: u32 = 0x94;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// ECCx
    /// Access: read-only, Width: 32, Offset: 0
    /// Get ECCx
    pub fn eccx() -> u32 {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b11111111111111111111111111111111 << 0);
        value as u32
    }
}
/// PC Card/NAND Flash control register 4
pub mod pcr4 {
    pub const OFFSET: u32 = 0xA0;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// ECCPS
        pub eccps: u8,
        /// TAR
        pub tar: u8,
        /// TCLR
        pub tclr: u8,
        /// ECCEN
        pub eccen: u8,
        /// PWID
        pub pwid: u8,
        /// PTYP
        pub ptyp: u8,
        /// PBKEN
        pub pbken: u8,
        /// PWAITEN
        pub pwaiten: u8,
    }
    pub struct Cache {
        /// ECCPS
        pub eccps: u8,
        /// TAR
        pub tar: u8,
        /// TCLR
        pub tclr: u8,
        /// ECCEN
        pub eccen: u8,
        /// PWID
        pub pwid: u8,
        /// PTYP
        pub ptyp: u8,
        /// PBKEN
        pub pbken: u8,
        /// PWAITEN
        pub pwaiten: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            eccps: ((value >> 17) & 0b111) as u8,
            tar: ((value >> 13) & 0b111) as u8,
            tclr: ((value >> 9) & 0b111) as u8,
            eccen: ((value >> 6) & 0b111) as u8,
            pwid: ((value >> 4) & 0b111) as u8,
            ptyp: ((value >> 3) & 0b111) as u8,
            pbken: ((value >> 2) & 0b111) as u8,
            pwaiten: ((value >> 1) & 0b111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            eccps: ((value >> 17) & 0b111) as u8,
            tar: ((value >> 13) & 0b111) as u8,
            tclr: ((value >> 9) & 0b111) as u8,
            eccen: ((value >> 6) & 0b111) as u8,
            pwid: ((value >> 4) & 0b111) as u8,
            ptyp: ((value >> 3) & 0b111) as u8,
            pbken: ((value >> 2) & 0b111) as u8,
            pwaiten: ((value >> 1) & 0b111) as u8,
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
                | ((self.eccps as u32) << 17)
                | ((self.tar as u32) << 13)
                | ((self.tclr as u32) << 9)
                | ((self.eccen as u32) << 6)
                | ((self.pwid as u32) << 4)
                | ((self.ptyp as u32) << 3)
                | ((self.pbken as u32) << 2)
                | ((self.pwaiten as u32) << 1)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// FIFO status and interrupt register 4
pub mod sr4 {
    pub const OFFSET: u32 = 0xA4;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    /// FEMPT
    /// Access: read-only, Width: 1, Offset: 6
    /// Get FEMPT
    pub fn fempt() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 6);
        value > 0
    }
    /// IFEN
    /// Access: read-write, Width: 1, Offset: 5
    /// Set IFEN
    pub fn set_ifen(value: bool) {
        let value = value as u32;
        let value = value << 5;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get IFEN
    pub fn get_ifen() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 5);
        value > 0
    }
    /// ILEN
    /// Access: read-write, Width: 1, Offset: 4
    /// Set ILEN
    pub fn set_ilen(value: bool) {
        let value = value as u32;
        let value = value << 4;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get ILEN
    pub fn get_ilen() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 4);
        value > 0
    }
    /// IREN
    /// Access: read-write, Width: 1, Offset: 3
    /// Set IREN
    pub fn set_iren(value: bool) {
        let value = value as u32;
        let value = value << 3;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get IREN
    pub fn get_iren() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 3);
        value > 0
    }
    /// IFS
    /// Access: read-write, Width: 1, Offset: 2
    /// Set IFS
    pub fn set_ifs(value: bool) {
        let value = value as u32;
        let value = value << 2;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get IFS
    pub fn get_ifs() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 2);
        value > 0
    }
    /// ILS
    /// Access: read-write, Width: 1, Offset: 1
    /// Set ILS
    pub fn set_ils(value: bool) {
        let value = value as u32;
        let value = value << 1;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get ILS
    pub fn get_ils() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 1);
        value > 0
    }
    /// IRS
    /// Access: read-write, Width: 1, Offset: 0
    /// Set IRS
    pub fn set_irs(value: bool) {
        let value = value as u32;
        unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
    }
    /// Get IRS
    pub fn get_irs() -> bool {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        let value = value & (0b1 << 0);
        value > 0
    }
}
/// Common memory space timing register 4
pub mod pmem4 {
    pub const OFFSET: u32 = 0xA8;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// MEMHIZx
        pub memhizx: u8,
        /// MEMHOLDx
        pub memholdx: u8,
        /// MEMWAITx
        pub memwaitx: u8,
        /// MEMSETx
        pub memsetx: u8,
    }
    pub struct Cache {
        /// MEMHIZx
        pub memhizx: u8,
        /// MEMHOLDx
        pub memholdx: u8,
        /// MEMWAITx
        pub memwaitx: u8,
        /// MEMSETx
        pub memsetx: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            memhizx: ((value >> 24) & 0b11111111) as u8,
            memholdx: ((value >> 16) & 0b11111111) as u8,
            memwaitx: ((value >> 8) & 0b11111111) as u8,
            memsetx: ((value >> 0) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            memhizx: ((value >> 24) & 0b11111111) as u8,
            memholdx: ((value >> 16) & 0b11111111) as u8,
            memwaitx: ((value >> 8) & 0b11111111) as u8,
            memsetx: ((value >> 0) & 0b11111111) as u8,
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
                | ((self.memhizx as u32) << 24)
                | ((self.memholdx as u32) << 16)
                | ((self.memwaitx as u32) << 8)
                | ((self.memsetx as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// Attribute memory space timing register 4
pub mod patt4 {
    pub const OFFSET: u32 = 0xAC;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// ATTHIZx
        pub atthizx: u8,
        /// ATTHOLDx
        pub attholdx: u8,
        /// ATTWAITx
        pub attwaitx: u8,
        /// ATTSETx
        pub attsetx: u8,
    }
    pub struct Cache {
        /// ATTHIZx
        pub atthizx: u8,
        /// ATTHOLDx
        pub attholdx: u8,
        /// ATTWAITx
        pub attwaitx: u8,
        /// ATTSETx
        pub attsetx: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            atthizx: ((value >> 24) & 0b11111111) as u8,
            attholdx: ((value >> 16) & 0b11111111) as u8,
            attwaitx: ((value >> 8) & 0b11111111) as u8,
            attsetx: ((value >> 0) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            atthizx: ((value >> 24) & 0b11111111) as u8,
            attholdx: ((value >> 16) & 0b11111111) as u8,
            attwaitx: ((value >> 8) & 0b11111111) as u8,
            attsetx: ((value >> 0) & 0b11111111) as u8,
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
                | ((self.atthizx as u32) << 24)
                | ((self.attholdx as u32) << 16)
                | ((self.attwaitx as u32) << 8)
                | ((self.attsetx as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// I/O space timing register 4
pub mod pio4 {
    pub const OFFSET: u32 = 0xB0;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// IOHIZx
        pub iohizx: u8,
        /// IOHOLDx
        pub ioholdx: u8,
        /// IOWAITx
        pub iowaitx: u8,
        /// IOSETx
        pub iosetx: u8,
    }
    pub struct Cache {
        /// IOHIZx
        pub iohizx: u8,
        /// IOHOLDx
        pub ioholdx: u8,
        /// IOWAITx
        pub iowaitx: u8,
        /// IOSETx
        pub iosetx: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            iohizx: ((value >> 24) & 0b11111111) as u8,
            ioholdx: ((value >> 16) & 0b11111111) as u8,
            iowaitx: ((value >> 8) & 0b11111111) as u8,
            iosetx: ((value >> 0) & 0b11111111) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            iohizx: ((value >> 24) & 0b11111111) as u8,
            ioholdx: ((value >> 16) & 0b11111111) as u8,
            iowaitx: ((value >> 8) & 0b11111111) as u8,
            iosetx: ((value >> 0) & 0b11111111) as u8,
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
                | ((self.iohizx as u32) << 24)
                | ((self.ioholdx as u32) << 16)
                | ((self.iowaitx as u32) << 8)
                | ((self.iosetx as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// SRAM/NOR-Flash write timing registers 1
pub mod bwtr1 {
    pub const OFFSET: u32 = 0x104;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// ACCMOD
        pub accmod: u8,
        /// DATLAT
        pub datlat: u8,
        /// CLKDIV
        pub clkdiv: u8,
        /// DATAST
        pub datast: u8,
        /// ADDHLD
        pub addhld: u8,
        /// ADDSET
        pub addset: u8,
    }
    pub struct Cache {
        /// ACCMOD
        pub accmod: u8,
        /// DATLAT
        pub datlat: u8,
        /// CLKDIV
        pub clkdiv: u8,
        /// DATAST
        pub datast: u8,
        /// ADDHLD
        pub addhld: u8,
        /// ADDSET
        pub addset: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            accmod: ((value >> 28) & 0b11) as u8,
            datlat: ((value >> 24) & 0b11) as u8,
            clkdiv: ((value >> 20) & 0b11) as u8,
            datast: ((value >> 8) & 0b11) as u8,
            addhld: ((value >> 4) & 0b11) as u8,
            addset: ((value >> 0) & 0b11) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            accmod: ((value >> 28) & 0b11) as u8,
            datlat: ((value >> 24) & 0b11) as u8,
            clkdiv: ((value >> 20) & 0b11) as u8,
            datast: ((value >> 8) & 0b11) as u8,
            addhld: ((value >> 4) & 0b11) as u8,
            addset: ((value >> 0) & 0b11) as u8,
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
                | ((self.accmod as u32) << 28)
                | ((self.datlat as u32) << 24)
                | ((self.clkdiv as u32) << 20)
                | ((self.datast as u32) << 8)
                | ((self.addhld as u32) << 4)
                | ((self.addset as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// SRAM/NOR-Flash write timing registers 2
pub mod bwtr2 {
    pub const OFFSET: u32 = 0x10C;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// ACCMOD
        pub accmod: u8,
        /// DATLAT
        pub datlat: u8,
        /// CLKDIV
        pub clkdiv: u8,
        /// DATAST
        pub datast: u8,
        /// ADDHLD
        pub addhld: u8,
        /// ADDSET
        pub addset: u8,
    }
    pub struct Cache {
        /// ACCMOD
        pub accmod: u8,
        /// DATLAT
        pub datlat: u8,
        /// CLKDIV
        pub clkdiv: u8,
        /// DATAST
        pub datast: u8,
        /// ADDHLD
        pub addhld: u8,
        /// ADDSET
        pub addset: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            accmod: ((value >> 28) & 0b11) as u8,
            datlat: ((value >> 24) & 0b11) as u8,
            clkdiv: ((value >> 20) & 0b11) as u8,
            datast: ((value >> 8) & 0b11) as u8,
            addhld: ((value >> 4) & 0b11) as u8,
            addset: ((value >> 0) & 0b11) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            accmod: ((value >> 28) & 0b11) as u8,
            datlat: ((value >> 24) & 0b11) as u8,
            clkdiv: ((value >> 20) & 0b11) as u8,
            datast: ((value >> 8) & 0b11) as u8,
            addhld: ((value >> 4) & 0b11) as u8,
            addset: ((value >> 0) & 0b11) as u8,
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
                | ((self.accmod as u32) << 28)
                | ((self.datlat as u32) << 24)
                | ((self.clkdiv as u32) << 20)
                | ((self.datast as u32) << 8)
                | ((self.addhld as u32) << 4)
                | ((self.addset as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// SRAM/NOR-Flash write timing registers 3
pub mod bwtr3 {
    pub const OFFSET: u32 = 0x114;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// ACCMOD
        pub accmod: u8,
        /// DATLAT
        pub datlat: u8,
        /// CLKDIV
        pub clkdiv: u8,
        /// DATAST
        pub datast: u8,
        /// ADDHLD
        pub addhld: u8,
        /// ADDSET
        pub addset: u8,
    }
    pub struct Cache {
        /// ACCMOD
        pub accmod: u8,
        /// DATLAT
        pub datlat: u8,
        /// CLKDIV
        pub clkdiv: u8,
        /// DATAST
        pub datast: u8,
        /// ADDHLD
        pub addhld: u8,
        /// ADDSET
        pub addset: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            accmod: ((value >> 28) & 0b11) as u8,
            datlat: ((value >> 24) & 0b11) as u8,
            clkdiv: ((value >> 20) & 0b11) as u8,
            datast: ((value >> 8) & 0b11) as u8,
            addhld: ((value >> 4) & 0b11) as u8,
            addset: ((value >> 0) & 0b11) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            accmod: ((value >> 28) & 0b11) as u8,
            datlat: ((value >> 24) & 0b11) as u8,
            clkdiv: ((value >> 20) & 0b11) as u8,
            datast: ((value >> 8) & 0b11) as u8,
            addhld: ((value >> 4) & 0b11) as u8,
            addset: ((value >> 0) & 0b11) as u8,
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
                | ((self.accmod as u32) << 28)
                | ((self.datlat as u32) << 24)
                | ((self.clkdiv as u32) << 20)
                | ((self.datast as u32) << 8)
                | ((self.addhld as u32) << 4)
                | ((self.addset as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// SRAM/NOR-Flash write timing registers 4
pub mod bwtr4 {
    pub const OFFSET: u32 = 0x11C;
    pub const REGISTER_ADDRESS: u32 = super::ADDRESS + OFFSET;
    pub struct ReadonlyCache {
        /// ACCMOD
        pub accmod: u8,
        /// DATLAT
        pub datlat: u8,
        /// CLKDIV
        pub clkdiv: u8,
        /// DATAST
        pub datast: u8,
        /// ADDHLD
        pub addhld: u8,
        /// ADDSET
        pub addset: u8,
    }
    pub struct Cache {
        /// ACCMOD
        pub accmod: u8,
        /// DATLAT
        pub datlat: u8,
        /// CLKDIV
        pub clkdiv: u8,
        /// DATAST
        pub datast: u8,
        /// ADDHLD
        pub addhld: u8,
        /// ADDSET
        pub addset: u8,
    }
    pub fn load() -> ReadonlyCache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        ReadonlyCache {
            accmod: ((value >> 28) & 0b11) as u8,
            datlat: ((value >> 24) & 0b11) as u8,
            clkdiv: ((value >> 20) & 0b11) as u8,
            datast: ((value >> 8) & 0b11) as u8,
            addhld: ((value >> 4) & 0b11) as u8,
            addset: ((value >> 0) & 0b11) as u8,
        }
    }
    pub fn modify() -> Cache {
        let value = unsafe { ::core::ptr::read_volatile(REGISTER_ADDRESS as *mut u32) };
        Cache {
            accmod: ((value >> 28) & 0b11) as u8,
            datlat: ((value >> 24) & 0b11) as u8,
            clkdiv: ((value >> 20) & 0b11) as u8,
            datast: ((value >> 8) & 0b11) as u8,
            addhld: ((value >> 4) & 0b11) as u8,
            addset: ((value >> 0) & 0b11) as u8,
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
                | ((self.accmod as u32) << 28)
                | ((self.datlat as u32) << 24)
                | ((self.clkdiv as u32) << 20)
                | ((self.datast as u32) << 8)
                | ((self.addhld as u32) << 4)
                | ((self.addset as u32) << 0)
            ;
            unsafe { ::core::ptr::write_volatile(REGISTER_ADDRESS as *mut u32, value) };
        }
    }
}
/// FSMC global interrupt
pub const INTERRUPT_FSMC: u32 = 48;
/*
<?xml version="1.0"?>
<peripheral xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <addressBlock>
    <offset>0x0</offset>
    <size>0x1000</size>
    <usage>registers</usage>
  </addressBlock>
  <baseAddress>0xA0000000</baseAddress>
  <description>Flexible static memory controller</description>
  <groupName>FSMC</groupName>
  <interrupt>
    <description>FSMC global interrupt</description>
    <name>FSMC</name>
    <value>48</value>
  </interrupt>
  <name>FSMC</name>
  <registers>
    <register>
      <access>read-write</access>
      <addressOffset>0x0</addressOffset>
      <description>SRAM/NOR-Flash chip-select control register
          1</description>
      <displayName>BCR1</displayName>
      <fields>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>CBURSTRW</description>
          <name>CBURSTRW</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ASYNCWAIT</description>
          <name>ASYNCWAIT</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>EXTMOD</description>
          <name>EXTMOD</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>WAITEN</description>
          <name>WAITEN</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>WREN</description>
          <name>WREN</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>WAITCFG</description>
          <name>WAITCFG</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>WAITPOL</description>
          <name>WAITPOL</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>BURSTEN</description>
          <name>BURSTEN</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>FACCEN</description>
          <name>FACCEN</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
          <description>MWID</description>
          <name>MWID</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
          <description>MTYP</description>
          <name>MTYP</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>MUXEN</description>
          <name>MUXEN</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>MBKEN</description>
          <name>MBKEN</name>
        </field>
      </fields>
      <name>BCR1</name>
      <resetValue>0x000030D0</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x4</addressOffset>
      <description>SRAM/NOR-Flash chip-select timing register
          1</description>
      <displayName>BTR1</displayName>
      <fields>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>2</bitWidth>
          <description>ACCMOD</description>
          <name>ACCMOD</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>4</bitWidth>
          <description>DATLAT</description>
          <name>DATLAT</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>4</bitWidth>
          <description>CLKDIV</description>
          <name>CLKDIV</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>4</bitWidth>
          <description>BUSTURN</description>
          <name>BUSTURN</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATAST</description>
          <name>DATAST</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>4</bitWidth>
          <description>ADDHLD</description>
          <name>ADDHLD</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>ADDSET</description>
          <name>ADDSET</name>
        </field>
      </fields>
      <name>BTR1</name>
      <resetValue>0xFFFFFFFF</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x8</addressOffset>
      <description>SRAM/NOR-Flash chip-select control register
          2</description>
      <displayName>BCR2</displayName>
      <fields>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>CBURSTRW</description>
          <name>CBURSTRW</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ASYNCWAIT</description>
          <name>ASYNCWAIT</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>EXTMOD</description>
          <name>EXTMOD</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>WAITEN</description>
          <name>WAITEN</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>WREN</description>
          <name>WREN</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>WAITCFG</description>
          <name>WAITCFG</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>WRAPMOD</description>
          <name>WRAPMOD</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>WAITPOL</description>
          <name>WAITPOL</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>BURSTEN</description>
          <name>BURSTEN</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>FACCEN</description>
          <name>FACCEN</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
          <description>MWID</description>
          <name>MWID</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
          <description>MTYP</description>
          <name>MTYP</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>MUXEN</description>
          <name>MUXEN</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>MBKEN</description>
          <name>MBKEN</name>
        </field>
      </fields>
      <name>BCR2</name>
      <resetValue>0x000030D0</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xC</addressOffset>
      <description>SRAM/NOR-Flash chip-select timing register
          2</description>
      <displayName>BTR2</displayName>
      <fields>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>2</bitWidth>
          <description>ACCMOD</description>
          <name>ACCMOD</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>4</bitWidth>
          <description>DATLAT</description>
          <name>DATLAT</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>4</bitWidth>
          <description>CLKDIV</description>
          <name>CLKDIV</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>4</bitWidth>
          <description>BUSTURN</description>
          <name>BUSTURN</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATAST</description>
          <name>DATAST</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>4</bitWidth>
          <description>ADDHLD</description>
          <name>ADDHLD</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>ADDSET</description>
          <name>ADDSET</name>
        </field>
      </fields>
      <name>BTR2</name>
      <resetValue>0xFFFFFFFF</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x10</addressOffset>
      <description>SRAM/NOR-Flash chip-select control register
          3</description>
      <displayName>BCR3</displayName>
      <fields>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>CBURSTRW</description>
          <name>CBURSTRW</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ASYNCWAIT</description>
          <name>ASYNCWAIT</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>EXTMOD</description>
          <name>EXTMOD</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>WAITEN</description>
          <name>WAITEN</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>WREN</description>
          <name>WREN</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>WAITCFG</description>
          <name>WAITCFG</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>WRAPMOD</description>
          <name>WRAPMOD</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>WAITPOL</description>
          <name>WAITPOL</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>BURSTEN</description>
          <name>BURSTEN</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>FACCEN</description>
          <name>FACCEN</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
          <description>MWID</description>
          <name>MWID</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
          <description>MTYP</description>
          <name>MTYP</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>MUXEN</description>
          <name>MUXEN</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>MBKEN</description>
          <name>MBKEN</name>
        </field>
      </fields>
      <name>BCR3</name>
      <resetValue>0x000030D0</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x14</addressOffset>
      <description>SRAM/NOR-Flash chip-select timing register
          3</description>
      <displayName>BTR3</displayName>
      <fields>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>2</bitWidth>
          <description>ACCMOD</description>
          <name>ACCMOD</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>4</bitWidth>
          <description>DATLAT</description>
          <name>DATLAT</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>4</bitWidth>
          <description>CLKDIV</description>
          <name>CLKDIV</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>4</bitWidth>
          <description>BUSTURN</description>
          <name>BUSTURN</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATAST</description>
          <name>DATAST</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>4</bitWidth>
          <description>ADDHLD</description>
          <name>ADDHLD</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>ADDSET</description>
          <name>ADDSET</name>
        </field>
      </fields>
      <name>BTR3</name>
      <resetValue>0xFFFFFFFF</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x18</addressOffset>
      <description>SRAM/NOR-Flash chip-select control register
          4</description>
      <displayName>BCR4</displayName>
      <fields>
        <field>
          <bitOffset>19</bitOffset>
          <bitWidth>1</bitWidth>
          <description>CBURSTRW</description>
          <name>CBURSTRW</name>
        </field>
        <field>
          <bitOffset>15</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ASYNCWAIT</description>
          <name>ASYNCWAIT</name>
        </field>
        <field>
          <bitOffset>14</bitOffset>
          <bitWidth>1</bitWidth>
          <description>EXTMOD</description>
          <name>EXTMOD</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>1</bitWidth>
          <description>WAITEN</description>
          <name>WAITEN</name>
        </field>
        <field>
          <bitOffset>12</bitOffset>
          <bitWidth>1</bitWidth>
          <description>WREN</description>
          <name>WREN</name>
        </field>
        <field>
          <bitOffset>11</bitOffset>
          <bitWidth>1</bitWidth>
          <description>WAITCFG</description>
          <name>WAITCFG</name>
        </field>
        <field>
          <bitOffset>10</bitOffset>
          <bitWidth>1</bitWidth>
          <description>WRAPMOD</description>
          <name>WRAPMOD</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>1</bitWidth>
          <description>WAITPOL</description>
          <name>WAITPOL</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>1</bitWidth>
          <description>BURSTEN</description>
          <name>BURSTEN</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>FACCEN</description>
          <name>FACCEN</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
          <description>MWID</description>
          <name>MWID</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>2</bitWidth>
          <description>MTYP</description>
          <name>MTYP</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>MUXEN</description>
          <name>MUXEN</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>MBKEN</description>
          <name>MBKEN</name>
        </field>
      </fields>
      <name>BCR4</name>
      <resetValue>0x000030D0</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x1C</addressOffset>
      <description>SRAM/NOR-Flash chip-select timing register
          4</description>
      <displayName>BTR4</displayName>
      <fields>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>2</bitWidth>
          <description>ACCMOD</description>
          <name>ACCMOD</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>4</bitWidth>
          <description>DATLAT</description>
          <name>DATLAT</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>4</bitWidth>
          <description>CLKDIV</description>
          <name>CLKDIV</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>4</bitWidth>
          <description>BUSTURN</description>
          <name>BUSTURN</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATAST</description>
          <name>DATAST</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>4</bitWidth>
          <description>ADDHLD</description>
          <name>ADDHLD</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>ADDSET</description>
          <name>ADDSET</name>
        </field>
      </fields>
      <name>BTR4</name>
      <resetValue>0xFFFFFFFF</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x60</addressOffset>
      <description>PC Card/NAND Flash control register
          2</description>
      <displayName>PCR2</displayName>
      <fields>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>3</bitWidth>
          <description>ECCPS</description>
          <name>ECCPS</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>4</bitWidth>
          <description>TAR</description>
          <name>TAR</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>4</bitWidth>
          <description>TCLR</description>
          <name>TCLR</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ECCEN</description>
          <name>ECCEN</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
          <description>PWID</description>
          <name>PWID</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>PTYP</description>
          <name>PTYP</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>PBKEN</description>
          <name>PBKEN</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>PWAITEN</description>
          <name>PWAITEN</name>
        </field>
      </fields>
      <name>PCR2</name>
      <resetValue>0x00000018</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0x64</addressOffset>
      <description>FIFO status and interrupt register
          2</description>
      <displayName>SR2</displayName>
      <fields>
        <field>
          <access>read-only</access>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>FEMPT</description>
          <name>FEMPT</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>IFEN</description>
          <name>IFEN</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ILEN</description>
          <name>ILEN</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>IREN</description>
          <name>IREN</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>IFS</description>
          <name>IFS</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ILS</description>
          <name>ILS</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>IRS</description>
          <name>IRS</name>
        </field>
      </fields>
      <name>SR2</name>
      <resetValue>0x00000040</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x68</addressOffset>
      <description>Common memory space timing register
          2</description>
      <displayName>PMEM2</displayName>
      <fields>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>MEMHIZx</description>
          <name>MEMHIZx</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>MEMHOLDx</description>
          <name>MEMHOLDx</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>MEMWAITx</description>
          <name>MEMWAITx</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>MEMSETx</description>
          <name>MEMSETx</name>
        </field>
      </fields>
      <name>PMEM2</name>
      <resetValue>0xFCFCFCFC</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x6C</addressOffset>
      <description>Attribute memory space timing register
          2</description>
      <displayName>PATT2</displayName>
      <fields>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>Attribute memory x databus HiZ
              time</description>
          <name>ATTHIZx</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>Attribute memory x hold
              time</description>
          <name>ATTHOLDx</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>Attribute memory x wait
              time</description>
          <name>ATTWAITx</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>Attribute memory x setup
              time</description>
          <name>ATTSETx</name>
        </field>
      </fields>
      <name>PATT2</name>
      <resetValue>0xFCFCFCFC</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x74</addressOffset>
      <description>ECC result register 2</description>
      <displayName>ECCR2</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>ECC result</description>
          <name>ECCx</name>
        </field>
      </fields>
      <name>ECCR2</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x80</addressOffset>
      <description>PC Card/NAND Flash control register
          3</description>
      <displayName>PCR3</displayName>
      <fields>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>3</bitWidth>
          <description>ECCPS</description>
          <name>ECCPS</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>4</bitWidth>
          <description>TAR</description>
          <name>TAR</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>4</bitWidth>
          <description>TCLR</description>
          <name>TCLR</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ECCEN</description>
          <name>ECCEN</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
          <description>PWID</description>
          <name>PWID</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>PTYP</description>
          <name>PTYP</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>PBKEN</description>
          <name>PBKEN</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>PWAITEN</description>
          <name>PWAITEN</name>
        </field>
      </fields>
      <name>PCR3</name>
      <resetValue>0x00000018</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0x84</addressOffset>
      <description>FIFO status and interrupt register
          3</description>
      <displayName>SR3</displayName>
      <fields>
        <field>
          <access>read-only</access>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>FEMPT</description>
          <name>FEMPT</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>IFEN</description>
          <name>IFEN</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ILEN</description>
          <name>ILEN</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>IREN</description>
          <name>IREN</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>IFS</description>
          <name>IFS</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ILS</description>
          <name>ILS</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>IRS</description>
          <name>IRS</name>
        </field>
      </fields>
      <name>SR3</name>
      <resetValue>0x00000040</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x88</addressOffset>
      <description>Common memory space timing register
          3</description>
      <displayName>PMEM3</displayName>
      <fields>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>MEMHIZx</description>
          <name>MEMHIZx</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>MEMHOLDx</description>
          <name>MEMHOLDx</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>MEMWAITx</description>
          <name>MEMWAITx</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>MEMSETx</description>
          <name>MEMSETx</name>
        </field>
      </fields>
      <name>PMEM3</name>
      <resetValue>0xFCFCFCFC</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x8C</addressOffset>
      <description>Attribute memory space timing register
          3</description>
      <displayName>PATT3</displayName>
      <fields>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>ATTHIZx</description>
          <name>ATTHIZx</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>ATTHOLDx</description>
          <name>ATTHOLDx</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>ATTWAITx</description>
          <name>ATTWAITx</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>ATTSETx</description>
          <name>ATTSETx</name>
        </field>
      </fields>
      <name>PATT3</name>
      <resetValue>0xFCFCFCFC</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-only</access>
      <addressOffset>0x94</addressOffset>
      <description>ECC result register 3</description>
      <displayName>ECCR3</displayName>
      <fields>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>32</bitWidth>
          <description>ECCx</description>
          <name>ECCx</name>
        </field>
      </fields>
      <name>ECCR3</name>
      <resetValue>0x00000000</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xA0</addressOffset>
      <description>PC Card/NAND Flash control register
          4</description>
      <displayName>PCR4</displayName>
      <fields>
        <field>
          <bitOffset>17</bitOffset>
          <bitWidth>3</bitWidth>
          <description>ECCPS</description>
          <name>ECCPS</name>
        </field>
        <field>
          <bitOffset>13</bitOffset>
          <bitWidth>4</bitWidth>
          <description>TAR</description>
          <name>TAR</name>
        </field>
        <field>
          <bitOffset>9</bitOffset>
          <bitWidth>4</bitWidth>
          <description>TCLR</description>
          <name>TCLR</name>
        </field>
        <field>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ECCEN</description>
          <name>ECCEN</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>2</bitWidth>
          <description>PWID</description>
          <name>PWID</name>
        </field>
        <field>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>PTYP</description>
          <name>PTYP</name>
        </field>
        <field>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>PBKEN</description>
          <name>PBKEN</name>
        </field>
        <field>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>PWAITEN</description>
          <name>PWAITEN</name>
        </field>
      </fields>
      <name>PCR4</name>
      <resetValue>0x00000018</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <addressOffset>0xA4</addressOffset>
      <description>FIFO status and interrupt register
          4</description>
      <displayName>SR4</displayName>
      <fields>
        <field>
          <access>read-only</access>
          <bitOffset>6</bitOffset>
          <bitWidth>1</bitWidth>
          <description>FEMPT</description>
          <name>FEMPT</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>5</bitOffset>
          <bitWidth>1</bitWidth>
          <description>IFEN</description>
          <name>IFEN</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>4</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ILEN</description>
          <name>ILEN</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>3</bitOffset>
          <bitWidth>1</bitWidth>
          <description>IREN</description>
          <name>IREN</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>2</bitOffset>
          <bitWidth>1</bitWidth>
          <description>IFS</description>
          <name>IFS</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>1</bitOffset>
          <bitWidth>1</bitWidth>
          <description>ILS</description>
          <name>ILS</name>
        </field>
        <field>
          <access>read-write</access>
          <bitOffset>0</bitOffset>
          <bitWidth>1</bitWidth>
          <description>IRS</description>
          <name>IRS</name>
        </field>
      </fields>
      <name>SR4</name>
      <resetValue>0x00000040</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xA8</addressOffset>
      <description>Common memory space timing register
          4</description>
      <displayName>PMEM4</displayName>
      <fields>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>MEMHIZx</description>
          <name>MEMHIZx</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>MEMHOLDx</description>
          <name>MEMHOLDx</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>MEMWAITx</description>
          <name>MEMWAITx</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>MEMSETx</description>
          <name>MEMSETx</name>
        </field>
      </fields>
      <name>PMEM4</name>
      <resetValue>0xFCFCFCFC</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xAC</addressOffset>
      <description>Attribute memory space timing register
          4</description>
      <displayName>PATT4</displayName>
      <fields>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>ATTHIZx</description>
          <name>ATTHIZx</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>ATTHOLDx</description>
          <name>ATTHOLDx</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>ATTWAITx</description>
          <name>ATTWAITx</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>ATTSETx</description>
          <name>ATTSETx</name>
        </field>
      </fields>
      <name>PATT4</name>
      <resetValue>0xFCFCFCFC</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0xB0</addressOffset>
      <description>I/O space timing register 4</description>
      <displayName>PIO4</displayName>
      <fields>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IOHIZx</description>
          <name>IOHIZx</name>
        </field>
        <field>
          <bitOffset>16</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IOHOLDx</description>
          <name>IOHOLDx</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IOWAITx</description>
          <name>IOWAITx</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>8</bitWidth>
          <description>IOSETx</description>
          <name>IOSETx</name>
        </field>
      </fields>
      <name>PIO4</name>
      <resetValue>0xFCFCFCFC</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x104</addressOffset>
      <description>SRAM/NOR-Flash write timing registers
          1</description>
      <displayName>BWTR1</displayName>
      <fields>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>2</bitWidth>
          <description>ACCMOD</description>
          <name>ACCMOD</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>4</bitWidth>
          <description>DATLAT</description>
          <name>DATLAT</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>4</bitWidth>
          <description>CLKDIV</description>
          <name>CLKDIV</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATAST</description>
          <name>DATAST</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>4</bitWidth>
          <description>ADDHLD</description>
          <name>ADDHLD</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>ADDSET</description>
          <name>ADDSET</name>
        </field>
      </fields>
      <name>BWTR1</name>
      <resetValue>0x0FFFFFFF</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x10C</addressOffset>
      <description>SRAM/NOR-Flash write timing registers
          2</description>
      <displayName>BWTR2</displayName>
      <fields>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>2</bitWidth>
          <description>ACCMOD</description>
          <name>ACCMOD</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>4</bitWidth>
          <description>DATLAT</description>
          <name>DATLAT</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>4</bitWidth>
          <description>CLKDIV</description>
          <name>CLKDIV</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATAST</description>
          <name>DATAST</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>4</bitWidth>
          <description>ADDHLD</description>
          <name>ADDHLD</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>ADDSET</description>
          <name>ADDSET</name>
        </field>
      </fields>
      <name>BWTR2</name>
      <resetValue>0x0FFFFFFF</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x114</addressOffset>
      <description>SRAM/NOR-Flash write timing registers
          3</description>
      <displayName>BWTR3</displayName>
      <fields>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>2</bitWidth>
          <description>ACCMOD</description>
          <name>ACCMOD</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>4</bitWidth>
          <description>DATLAT</description>
          <name>DATLAT</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>4</bitWidth>
          <description>CLKDIV</description>
          <name>CLKDIV</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATAST</description>
          <name>DATAST</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>4</bitWidth>
          <description>ADDHLD</description>
          <name>ADDHLD</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>ADDSET</description>
          <name>ADDSET</name>
        </field>
      </fields>
      <name>BWTR3</name>
      <resetValue>0x0FFFFFFF</resetValue>
      <size>0x20</size>
    </register>
    <register>
      <access>read-write</access>
      <addressOffset>0x11C</addressOffset>
      <description>SRAM/NOR-Flash write timing registers
          4</description>
      <displayName>BWTR4</displayName>
      <fields>
        <field>
          <bitOffset>28</bitOffset>
          <bitWidth>2</bitWidth>
          <description>ACCMOD</description>
          <name>ACCMOD</name>
        </field>
        <field>
          <bitOffset>24</bitOffset>
          <bitWidth>4</bitWidth>
          <description>DATLAT</description>
          <name>DATLAT</name>
        </field>
        <field>
          <bitOffset>20</bitOffset>
          <bitWidth>4</bitWidth>
          <description>CLKDIV</description>
          <name>CLKDIV</name>
        </field>
        <field>
          <bitOffset>8</bitOffset>
          <bitWidth>8</bitWidth>
          <description>DATAST</description>
          <name>DATAST</name>
        </field>
        <field>
          <bitOffset>4</bitOffset>
          <bitWidth>4</bitWidth>
          <description>ADDHLD</description>
          <name>ADDHLD</name>
        </field>
        <field>
          <bitOffset>0</bitOffset>
          <bitWidth>4</bitWidth>
          <description>ADDSET</description>
          <name>ADDSET</name>
        </field>
      </fields>
      <name>BWTR4</name>
      <resetValue>0x0FFFFFFF</resetValue>
      <size>0x20</size>
    </register>
  </registers>
</peripheral>
*/
