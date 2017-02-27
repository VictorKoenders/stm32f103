// SDIO
// Secure digital input/output interface
pub const ADDRESS: u32 = 0x40018000;
pub mod POWER {
}
pub mod CLKCR {
}
pub mod ARG {
}
pub mod CMD {
}
pub mod RESPCMD {
}
pub mod RESPI1 {
}
pub mod RESP2 {
}
pub mod RESP3 {
}
pub mod RESP4 {
}
pub mod DTIMER {
}
pub mod DLEN {
}
pub mod DCTRL {
}
pub mod DCOUNT {
}
pub mod STA {
}
pub mod ICR {
}
pub mod MASK {
}
pub mod FIFOCNT {
}
pub mod FIFO {
}
/*
Peripheral {
    name: "SDIO",
    group_name: Some(
        "SDIO"
    ),
    description: Some(
        "Secure digital input/output\r\n      interface"
    ),
    base_address: 1073840128,
    interrupt: Some(
        Interrupt {
            name: "SDIO",
            description: Some(
                "SDIO global interrupt"
            ),
            value: 49
        }
    ),
    registers: Some(
        [
            Single(
                RegisterInfo {
                    name: "POWER",
                    description: "Bits 1:0 = PWRCTRL: Power supply control\r\n          bits",
                    address_offset: 0,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        0
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "PWRCTRL",
                                description: Some(
                                    "PWRCTRL"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            }
                        ]
                    )
                }
            ),
            Single(
                RegisterInfo {
                    name: "CLKCR",
                    description: "SDI clock control register\r\n          (SDIO_CLKCR)",
                    address_offset: 4,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        0
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "CLKDIV",
                                description: Some(
                                    "Clock divide factor"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CLKEN",
                                description: Some(
                                    "Clock enable bit"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PWRSAV",
                                description: Some(
                                    "Power saving configuration\r\n              bit"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BYPASS",
                                description: Some(
                                    "Clock divider bypass enable\r\n              bit"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "WIDBUS",
                                description: Some(
                                    "Wide bus mode enable bit"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "NEGEDGE",
                                description: Some(
                                    "SDIO_CK dephasing selection\r\n              bit"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "HWFC_EN",
                                description: Some(
                                    "HW Flow Control enable"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            }
                        ]
                    )
                }
            ),
            Single(
                RegisterInfo {
                    name: "ARG",
                    description: "Bits 31:0 = : Command argument",
                    address_offset: 8,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        0
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "CMDARG",
                                description: Some(
                                    "Command argument"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 32
                                },
                                access: None,
                                enumerated_values: []
                            }
                        ]
                    )
                }
            ),
            Single(
                RegisterInfo {
                    name: "CMD",
                    description: "SDIO command register\r\n          (SDIO_CMD)",
                    address_offset: 12,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        0
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "CMDINDEX",
                                description: Some(
                                    "CMDINDEX"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 6
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "WAITRESP",
                                description: Some(
                                    "WAITRESP"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "WAITINT",
                                description: Some(
                                    "WAITINT"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "WAITPEND",
                                description: Some(
                                    "WAITPEND"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CPSMEN",
                                description: Some(
                                    "CPSMEN"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SDIOSuspend",
                                description: Some(
                                    "SDIOSuspend"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ENCMDcompl",
                                description: Some(
                                    "ENCMDcompl"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "nIEN",
                                description: Some(
                                    "nIEN"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CE_ATACMD",
                                description: Some(
                                    "CE_ATACMD"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            }
                        ]
                    )
                }
            ),
            Single(
                RegisterInfo {
                    name: "RESPCMD",
                    description: "SDIO command register",
                    address_offset: 16,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadOnly
                    ),
                    reset_value: Some(
                        0
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "RESPCMD",
                                description: Some(
                                    "RESPCMD"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 6
                                },
                                access: None,
                                enumerated_values: []
                            }
                        ]
                    )
                }
            ),
            Single(
                RegisterInfo {
                    name: "RESPI1",
                    description: "Bits 31:0 = CARDSTATUS1",
                    address_offset: 20,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadOnly
                    ),
                    reset_value: Some(
                        0
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "CARDSTATUS1",
                                description: Some(
                                    "CARDSTATUS1"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 32
                                },
                                access: None,
                                enumerated_values: []
                            }
                        ]
                    )
                }
            ),
            Single(
                RegisterInfo {
                    name: "RESP2",
                    description: "Bits 31:0 = CARDSTATUS2",
                    address_offset: 24,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadOnly
                    ),
                    reset_value: Some(
                        0
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "CARDSTATUS2",
                                description: Some(
                                    "CARDSTATUS2"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 32
                                },
                                access: None,
                                enumerated_values: []
                            }
                        ]
                    )
                }
            ),
            Single(
                RegisterInfo {
                    name: "RESP3",
                    description: "Bits 31:0 = CARDSTATUS3",
                    address_offset: 28,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadOnly
                    ),
                    reset_value: Some(
                        0
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "CARDSTATUS3",
                                description: Some(
                                    "CARDSTATUS3"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 32
                                },
                                access: None,
                                enumerated_values: []
                            }
                        ]
                    )
                }
            ),
            Single(
                RegisterInfo {
                    name: "RESP4",
                    description: "Bits 31:0 = CARDSTATUS4",
                    address_offset: 32,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadOnly
                    ),
                    reset_value: Some(
                        0
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "CARDSTATUS4",
                                description: Some(
                                    "CARDSTATUS4"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 32
                                },
                                access: None,
                                enumerated_values: []
                            }
                        ]
                    )
                }
            ),
            Single(
                RegisterInfo {
                    name: "DTIMER",
                    description: "Bits 31:0 = DATATIME: Data timeout\r\n          period",
                    address_offset: 36,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        0
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "DATATIME",
                                description: Some(
                                    "Data timeout period"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 32
                                },
                                access: None,
                                enumerated_values: []
                            }
                        ]
                    )
                }
            ),
            Single(
                RegisterInfo {
                    name: "DLEN",
                    description: "Bits 24:0 = DATALENGTH: Data length\r\n          value",
                    address_offset: 40,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        0
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "DATALENGTH",
                                description: Some(
                                    "Data length value"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 25
                                },
                                access: None,
                                enumerated_values: []
                            }
                        ]
                    )
                }
            ),
            Single(
                RegisterInfo {
                    name: "DCTRL",
                    description: "SDIO data control register\r\n          (SDIO_DCTRL)",
                    address_offset: 44,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        0
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "DTEN",
                                description: Some(
                                    "DTEN"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DTDIR",
                                description: Some(
                                    "DTDIR"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DTMODE",
                                description: Some(
                                    "DTMODE"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DMAEN",
                                description: Some(
                                    "DMAEN"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DBLOCKSIZE",
                                description: Some(
                                    "DBLOCKSIZE"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PWSTART",
                                description: Some(
                                    "PWSTART"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PWSTOP",
                                description: Some(
                                    "PWSTOP"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "RWMOD",
                                description: Some(
                                    "RWMOD"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SDIOEN",
                                description: Some(
                                    "SDIOEN"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            }
                        ]
                    )
                }
            ),
            Single(
                RegisterInfo {
                    name: "DCOUNT",
                    description: "Bits 24:0 = DATACOUNT: Data count\r\n          value",
                    address_offset: 48,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadOnly
                    ),
                    reset_value: Some(
                        0
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "DATACOUNT",
                                description: Some(
                                    "Data count value"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 25
                                },
                                access: None,
                                enumerated_values: []
                            }
                        ]
                    )
                }
            ),
            Single(
                RegisterInfo {
                    name: "STA",
                    description: "SDIO status register\r\n          (SDIO_STA)",
                    address_offset: 52,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadOnly
                    ),
                    reset_value: Some(
                        0
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "CCRCFAIL",
                                description: Some(
                                    "CCRCFAIL"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DCRCFAIL",
                                description: Some(
                                    "DCRCFAIL"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CTIMEOUT",
                                description: Some(
                                    "CTIMEOUT"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DTIMEOUT",
                                description: Some(
                                    "DTIMEOUT"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TXUNDERR",
                                description: Some(
                                    "TXUNDERR"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "RXOVERR",
                                description: Some(
                                    "RXOVERR"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CMDREND",
                                description: Some(
                                    "CMDREND"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CMDSENT",
                                description: Some(
                                    "CMDSENT"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATAEND",
                                description: Some(
                                    "DATAEND"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "STBITERR",
                                description: Some(
                                    "STBITERR"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DBCKEND",
                                description: Some(
                                    "DBCKEND"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CMDACT",
                                description: Some(
                                    "CMDACT"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TXACT",
                                description: Some(
                                    "TXACT"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "RXACT",
                                description: Some(
                                    "RXACT"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TXFIFOHE",
                                description: Some(
                                    "TXFIFOHE"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "RXFIFOHF",
                                description: Some(
                                    "RXFIFOHF"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TXFIFOF",
                                description: Some(
                                    "TXFIFOF"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "RXFIFOF",
                                description: Some(
                                    "RXFIFOF"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TXFIFOE",
                                description: Some(
                                    "TXFIFOE"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "RXFIFOE",
                                description: Some(
                                    "RXFIFOE"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TXDAVL",
                                description: Some(
                                    "TXDAVL"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "RXDAVL",
                                description: Some(
                                    "RXDAVL"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SDIOIT",
                                description: Some(
                                    "SDIOIT"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CEATAEND",
                                description: Some(
                                    "CEATAEND"
                                ),
                                bit_range: BitRange {
                                    offset: 23,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            }
                        ]
                    )
                }
            ),
            Single(
                RegisterInfo {
                    name: "ICR",
                    description: "SDIO interrupt clear register\r\n          (SDIO_ICR)",
                    address_offset: 56,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        0
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "CCRCFAILC",
                                description: Some(
                                    "CCRCFAILC"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DCRCFAILC",
                                description: Some(
                                    "DCRCFAILC"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CTIMEOUTC",
                                description: Some(
                                    "CTIMEOUTC"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DTIMEOUTC",
                                description: Some(
                                    "DTIMEOUTC"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TXUNDERRC",
                                description: Some(
                                    "TXUNDERRC"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "RXOVERRC",
                                description: Some(
                                    "RXOVERRC"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CMDRENDC",
                                description: Some(
                                    "CMDRENDC"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CMDSENTC",
                                description: Some(
                                    "CMDSENTC"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATAENDC",
                                description: Some(
                                    "DATAENDC"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "STBITERRC",
                                description: Some(
                                    "STBITERRC"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DBCKENDC",
                                description: Some(
                                    "DBCKENDC"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SDIOITC",
                                description: Some(
                                    "SDIOITC"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CEATAENDC",
                                description: Some(
                                    "CEATAENDC"
                                ),
                                bit_range: BitRange {
                                    offset: 23,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            }
                        ]
                    )
                }
            ),
            Single(
                RegisterInfo {
                    name: "MASK",
                    description: "SDIO mask register (SDIO_MASK)",
                    address_offset: 60,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        0
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "CCRCFAILIE",
                                description: Some(
                                    "CCRCFAILIE"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DCRCFAILIE",
                                description: Some(
                                    "DCRCFAILIE"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CTIMEOUTIE",
                                description: Some(
                                    "CTIMEOUTIE"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DTIMEOUTIE",
                                description: Some(
                                    "DTIMEOUTIE"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TXUNDERRIE",
                                description: Some(
                                    "TXUNDERRIE"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "RXOVERRIE",
                                description: Some(
                                    "RXOVERRIE"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CMDRENDIE",
                                description: Some(
                                    "CMDRENDIE"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CMDSENTIE",
                                description: Some(
                                    "CMDSENTIE"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATAENDIE",
                                description: Some(
                                    "DATAENDIE"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "STBITERRIE",
                                description: Some(
                                    "STBITERRIE"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DBACKENDIE",
                                description: Some(
                                    "DBACKENDIE"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CMDACTIE",
                                description: Some(
                                    "CMDACTIE"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TXACTIE",
                                description: Some(
                                    "TXACTIE"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "RXACTIE",
                                description: Some(
                                    "RXACTIE"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TXFIFOHEIE",
                                description: Some(
                                    "TXFIFOHEIE"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "RXFIFOHFIE",
                                description: Some(
                                    "RXFIFOHFIE"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TXFIFOFIE",
                                description: Some(
                                    "TXFIFOFIE"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "RXFIFOFIE",
                                description: Some(
                                    "RXFIFOFIE"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TXFIFOEIE",
                                description: Some(
                                    "TXFIFOEIE"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "RXFIFOEIE",
                                description: Some(
                                    "RXFIFOEIE"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TXDAVLIE",
                                description: Some(
                                    "TXDAVLIE"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "RXDAVLIE",
                                description: Some(
                                    "RXDAVLIE"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SDIOITIE",
                                description: Some(
                                    "SDIOITIE"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CEATENDIE",
                                description: Some(
                                    "CEATENDIE"
                                ),
                                bit_range: BitRange {
                                    offset: 23,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            }
                        ]
                    )
                }
            ),
            Single(
                RegisterInfo {
                    name: "FIFOCNT",
                    description: "Bits 23:0 = FIFOCOUNT: Remaining number of\r\n          words to be written to or read from the\r\n          FIFO",
                    address_offset: 72,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadOnly
                    ),
                    reset_value: Some(
                        0
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "FIF0COUNT",
                                description: Some(
                                    "FIF0COUNT"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 24
                                },
                                access: None,
                                enumerated_values: []
                            }
                        ]
                    )
                }
            ),
            Single(
                RegisterInfo {
                    name: "FIFO",
                    description: "bits 31:0 = FIFOData: Receive and transmit\r\n          FIFO data",
                    address_offset: 128,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        0
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "FIFOData",
                                description: Some(
                                    "FIFOData"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 32
                                },
                                access: None,
                                enumerated_values: []
                            }
                        ]
                    )
                }
            )
        ]
    ),
    derived_from: None
}
*/
