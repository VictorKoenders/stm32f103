// CAN
// Controller area network
pub const ADDRESS: u32 = 0x40006400;
pub mod CAN_MCR {
}
pub mod CAN_MSR {
}
pub mod CAN_TSR {
}
pub mod CAN_RF0R {
}
pub mod CAN_RF1R {
}
pub mod CAN_IER {
}
pub mod CAN_ESR {
}
pub mod CAN_BTR {
}
pub mod CAN_TI0R {
}
pub mod CAN_TDT0R {
}
pub mod CAN_TDL0R {
}
pub mod CAN_TDH0R {
}
pub mod CAN_TI1R {
}
pub mod CAN_TDT1R {
}
pub mod CAN_TDL1R {
}
pub mod CAN_TDH1R {
}
pub mod CAN_TI2R {
}
pub mod CAN_TDT2R {
}
pub mod CAN_TDL2R {
}
pub mod CAN_TDH2R {
}
pub mod CAN_RI0R {
}
pub mod CAN_RDT0R {
}
pub mod CAN_RDL0R {
}
pub mod CAN_RDH0R {
}
pub mod CAN_RI1R {
}
pub mod CAN_RDT1R {
}
pub mod CAN_RDL1R {
}
pub mod CAN_RDH1R {
}
pub mod CAN_FMR {
}
pub mod CAN_FM1R {
}
pub mod CAN_FS1R {
}
pub mod CAN_FFA1R {
}
pub mod CAN_FA1R {
}
pub mod F0R1 {
}
pub mod F0R2 {
}
pub mod F1R1 {
}
pub mod F1R2 {
}
pub mod F2R1 {
}
pub mod F2R2 {
}
pub mod F3R1 {
}
pub mod F3R2 {
}
pub mod F4R1 {
}
pub mod F4R2 {
}
pub mod F5R1 {
}
pub mod F5R2 {
}
pub mod F6R1 {
}
pub mod F6R2 {
}
pub mod F7R1 {
}
pub mod F7R2 {
}
pub mod F8R1 {
}
pub mod F8R2 {
}
pub mod F9R1 {
}
pub mod F9R2 {
}
pub mod F10R1 {
}
pub mod F10R2 {
}
pub mod F11R1 {
}
pub mod F11R2 {
}
pub mod F12R1 {
}
pub mod F12R2 {
}
pub mod F13R1 {
}
pub mod F13R2 {
}
/*
Peripheral {
    name: "CAN",
    group_name: Some(
        "CAN"
    ),
    description: Some(
        "Controller area network"
    ),
    base_address: 1073767424,
    interrupt: Some(
        Interrupt {
            name: "CAN1_TX",
            description: Some(
                "CAN1 TX interrupts"
            ),
            value: 19
        }
    ),
    registers: Some(
        [
            Single(
                RegisterInfo {
                    name: "CAN_MCR",
                    description: "CAN_MCR",
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
                                name: "DBF",
                                description: Some(
                                    "DBF"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "RESET",
                                description: Some(
                                    "RESET"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TTCM",
                                description: Some(
                                    "TTCM"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ABOM",
                                description: Some(
                                    "ABOM"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "AWUM",
                                description: Some(
                                    "AWUM"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "NART",
                                description: Some(
                                    "NART"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "RFLM",
                                description: Some(
                                    "RFLM"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TXFP",
                                description: Some(
                                    "TXFP"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SLEEP",
                                description: Some(
                                    "SLEEP"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "INRQ",
                                description: Some(
                                    "INRQ"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
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
                    name: "CAN_MSR",
                    description: "CAN_MSR",
                    address_offset: 4,
                    size: Some(
                        32
                    ),
                    access: None,
                    reset_value: Some(
                        0
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "RX",
                                description: Some(
                                    "RX"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: Some(
                                    ReadOnly
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "SAMP",
                                description: Some(
                                    "SAMP"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: Some(
                                    ReadOnly
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "RXM",
                                description: Some(
                                    "RXM"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: Some(
                                    ReadOnly
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "TXM",
                                description: Some(
                                    "TXM"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: Some(
                                    ReadOnly
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "SLAKI",
                                description: Some(
                                    "SLAKI"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "WKUI",
                                description: Some(
                                    "WKUI"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "ERRI",
                                description: Some(
                                    "ERRI"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "SLAK",
                                description: Some(
                                    "SLAK"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: Some(
                                    ReadOnly
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "INAK",
                                description: Some(
                                    "INAK"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: Some(
                                    ReadOnly
                                ),
                                enumerated_values: []
                            }
                        ]
                    )
                }
            ),
            Single(
                RegisterInfo {
                    name: "CAN_TSR",
                    description: "CAN_TSR",
                    address_offset: 8,
                    size: Some(
                        32
                    ),
                    access: None,
                    reset_value: Some(
                        0
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "LOW2",
                                description: Some(
                                    "Lowest priority flag for mailbox\r\n              2"
                                ),
                                bit_range: BitRange {
                                    offset: 31,
                                    width: 1
                                },
                                access: Some(
                                    ReadOnly
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "LOW1",
                                description: Some(
                                    "Lowest priority flag for mailbox\r\n              1"
                                ),
                                bit_range: BitRange {
                                    offset: 30,
                                    width: 1
                                },
                                access: Some(
                                    ReadOnly
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "LOW0",
                                description: Some(
                                    "Lowest priority flag for mailbox\r\n              0"
                                ),
                                bit_range: BitRange {
                                    offset: 29,
                                    width: 1
                                },
                                access: Some(
                                    ReadOnly
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "TME2",
                                description: Some(
                                    "Lowest priority flag for mailbox\r\n              2"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 1
                                },
                                access: Some(
                                    ReadOnly
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "TME1",
                                description: Some(
                                    "Lowest priority flag for mailbox\r\n              1"
                                ),
                                bit_range: BitRange {
                                    offset: 27,
                                    width: 1
                                },
                                access: Some(
                                    ReadOnly
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "TME0",
                                description: Some(
                                    "Lowest priority flag for mailbox\r\n              0"
                                ),
                                bit_range: BitRange {
                                    offset: 26,
                                    width: 1
                                },
                                access: Some(
                                    ReadOnly
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "CODE",
                                description: Some(
                                    "CODE"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 2
                                },
                                access: Some(
                                    ReadOnly
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "ABRQ2",
                                description: Some(
                                    "ABRQ2"
                                ),
                                bit_range: BitRange {
                                    offset: 23,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "TERR2",
                                description: Some(
                                    "TERR2"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "ALST2",
                                description: Some(
                                    "ALST2"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "TXOK2",
                                description: Some(
                                    "TXOK2"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "RQCP2",
                                description: Some(
                                    "RQCP2"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "ABRQ1",
                                description: Some(
                                    "ABRQ1"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "TERR1",
                                description: Some(
                                    "TERR1"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "ALST1",
                                description: Some(
                                    "ALST1"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "TXOK1",
                                description: Some(
                                    "TXOK1"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "RQCP1",
                                description: Some(
                                    "RQCP1"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "ABRQ0",
                                description: Some(
                                    "ABRQ0"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "TERR0",
                                description: Some(
                                    "TERR0"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "ALST0",
                                description: Some(
                                    "ALST0"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "TXOK0",
                                description: Some(
                                    "TXOK0"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "RQCP0",
                                description: Some(
                                    "RQCP0"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            }
                        ]
                    )
                }
            ),
            Single(
                RegisterInfo {
                    name: "CAN_RF0R",
                    description: "CAN_RF0R",
                    address_offset: 12,
                    size: Some(
                        32
                    ),
                    access: None,
                    reset_value: Some(
                        0
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "RFOM0",
                                description: Some(
                                    "RFOM0"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "FOVR0",
                                description: Some(
                                    "FOVR0"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "FULL0",
                                description: Some(
                                    "FULL0"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "FMP0",
                                description: Some(
                                    "FMP0"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 2
                                },
                                access: Some(
                                    ReadOnly
                                ),
                                enumerated_values: []
                            }
                        ]
                    )
                }
            ),
            Single(
                RegisterInfo {
                    name: "CAN_RF1R",
                    description: "CAN_RF1R",
                    address_offset: 16,
                    size: Some(
                        32
                    ),
                    access: None,
                    reset_value: Some(
                        0
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "RFOM1",
                                description: Some(
                                    "RFOM1"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "FOVR1",
                                description: Some(
                                    "FOVR1"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "FULL1",
                                description: Some(
                                    "FULL1"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "FMP1",
                                description: Some(
                                    "FMP1"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 2
                                },
                                access: Some(
                                    ReadOnly
                                ),
                                enumerated_values: []
                            }
                        ]
                    )
                }
            ),
            Single(
                RegisterInfo {
                    name: "CAN_IER",
                    description: "CAN_IER",
                    address_offset: 20,
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
                                name: "SLKIE",
                                description: Some(
                                    "SLKIE"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "WKUIE",
                                description: Some(
                                    "WKUIE"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ERRIE",
                                description: Some(
                                    "ERRIE"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "LECIE",
                                description: Some(
                                    "LECIE"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BOFIE",
                                description: Some(
                                    "BOFIE"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EPVIE",
                                description: Some(
                                    "EPVIE"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EWGIE",
                                description: Some(
                                    "EWGIE"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FOVIE1",
                                description: Some(
                                    "FOVIE1"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FFIE1",
                                description: Some(
                                    "FFIE1"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FMPIE1",
                                description: Some(
                                    "FMPIE1"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FOVIE0",
                                description: Some(
                                    "FOVIE0"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FFIE0",
                                description: Some(
                                    "FFIE0"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FMPIE0",
                                description: Some(
                                    "FMPIE0"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TMEIE",
                                description: Some(
                                    "TMEIE"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
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
                    name: "CAN_ESR",
                    description: "CAN_ESR",
                    address_offset: 24,
                    size: Some(
                        32
                    ),
                    access: None,
                    reset_value: Some(
                        0
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "REC",
                                description: Some(
                                    "REC"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 8
                                },
                                access: Some(
                                    ReadOnly
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "TEC",
                                description: Some(
                                    "TEC"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 8
                                },
                                access: Some(
                                    ReadOnly
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "LEC",
                                description: Some(
                                    "LEC"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 3
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "BOFF",
                                description: Some(
                                    "BOFF"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: Some(
                                    ReadOnly
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "EPVF",
                                description: Some(
                                    "EPVF"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: Some(
                                    ReadOnly
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "EWGF",
                                description: Some(
                                    "EWGF"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: Some(
                                    ReadOnly
                                ),
                                enumerated_values: []
                            }
                        ]
                    )
                }
            ),
            Single(
                RegisterInfo {
                    name: "CAN_BTR",
                    description: "CAN_BTR",
                    address_offset: 28,
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
                                name: "SILM",
                                description: Some(
                                    "SILM"
                                ),
                                bit_range: BitRange {
                                    offset: 31,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "LBKM",
                                description: Some(
                                    "LBKM"
                                ),
                                bit_range: BitRange {
                                    offset: 30,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SJW",
                                description: Some(
                                    "SJW"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TS2",
                                description: Some(
                                    "TS2"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 3
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TS1",
                                description: Some(
                                    "TS1"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BRP",
                                description: Some(
                                    "BRP"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 10
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
                    name: "CAN_TI0R",
                    description: "CAN_TI0R",
                    address_offset: 384,
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
                                name: "STID",
                                description: Some(
                                    "STID"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 11
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EXID",
                                description: Some(
                                    "EXID"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 18
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IDE",
                                description: Some(
                                    "IDE"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "RTR",
                                description: Some(
                                    "RTR"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TXRQ",
                                description: Some(
                                    "TXRQ"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
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
                    name: "CAN_TDT0R",
                    description: "CAN_TDT0R",
                    address_offset: 388,
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
                                name: "TIME",
                                description: Some(
                                    "TIME"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 16
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TGT",
                                description: Some(
                                    "TGT"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DLC",
                                description: Some(
                                    "DLC"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 4
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
                    name: "CAN_TDL0R",
                    description: "CAN_TDL0R",
                    address_offset: 392,
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
                                name: "DATA3",
                                description: Some(
                                    "DATA3"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATA2",
                                description: Some(
                                    "DATA2"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATA1",
                                description: Some(
                                    "DATA1"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATA0",
                                description: Some(
                                    "DATA0"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 8
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
                    name: "CAN_TDH0R",
                    description: "CAN_TDH0R",
                    address_offset: 396,
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
                                name: "DATA7",
                                description: Some(
                                    "DATA7"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATA6",
                                description: Some(
                                    "DATA6"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATA5",
                                description: Some(
                                    "DATA5"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATA4",
                                description: Some(
                                    "DATA4"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 8
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
                    name: "CAN_TI1R",
                    description: "CAN_TI1R",
                    address_offset: 400,
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
                                name: "STID",
                                description: Some(
                                    "STID"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 11
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EXID",
                                description: Some(
                                    "EXID"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 18
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IDE",
                                description: Some(
                                    "IDE"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "RTR",
                                description: Some(
                                    "RTR"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TXRQ",
                                description: Some(
                                    "TXRQ"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
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
                    name: "CAN_TDT1R",
                    description: "CAN_TDT1R",
                    address_offset: 404,
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
                                name: "TIME",
                                description: Some(
                                    "TIME"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 16
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TGT",
                                description: Some(
                                    "TGT"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DLC",
                                description: Some(
                                    "DLC"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 4
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
                    name: "CAN_TDL1R",
                    description: "CAN_TDL1R",
                    address_offset: 408,
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
                                name: "DATA3",
                                description: Some(
                                    "DATA3"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATA2",
                                description: Some(
                                    "DATA2"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATA1",
                                description: Some(
                                    "DATA1"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATA0",
                                description: Some(
                                    "DATA0"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 8
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
                    name: "CAN_TDH1R",
                    description: "CAN_TDH1R",
                    address_offset: 412,
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
                                name: "DATA7",
                                description: Some(
                                    "DATA7"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATA6",
                                description: Some(
                                    "DATA6"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATA5",
                                description: Some(
                                    "DATA5"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATA4",
                                description: Some(
                                    "DATA4"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 8
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
                    name: "CAN_TI2R",
                    description: "CAN_TI2R",
                    address_offset: 416,
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
                                name: "STID",
                                description: Some(
                                    "STID"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 11
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EXID",
                                description: Some(
                                    "EXID"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 18
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IDE",
                                description: Some(
                                    "IDE"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "RTR",
                                description: Some(
                                    "RTR"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TXRQ",
                                description: Some(
                                    "TXRQ"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
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
                    name: "CAN_TDT2R",
                    description: "CAN_TDT2R",
                    address_offset: 420,
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
                                name: "TIME",
                                description: Some(
                                    "TIME"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 16
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TGT",
                                description: Some(
                                    "TGT"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DLC",
                                description: Some(
                                    "DLC"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 4
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
                    name: "CAN_TDL2R",
                    description: "CAN_TDL2R",
                    address_offset: 424,
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
                                name: "DATA3",
                                description: Some(
                                    "DATA3"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATA2",
                                description: Some(
                                    "DATA2"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATA1",
                                description: Some(
                                    "DATA1"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATA0",
                                description: Some(
                                    "DATA0"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 8
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
                    name: "CAN_TDH2R",
                    description: "CAN_TDH2R",
                    address_offset: 428,
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
                                name: "DATA7",
                                description: Some(
                                    "DATA7"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATA6",
                                description: Some(
                                    "DATA6"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATA5",
                                description: Some(
                                    "DATA5"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATA4",
                                description: Some(
                                    "DATA4"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 8
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
                    name: "CAN_RI0R",
                    description: "CAN_RI0R",
                    address_offset: 432,
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
                                name: "STID",
                                description: Some(
                                    "STID"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 11
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EXID",
                                description: Some(
                                    "EXID"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 18
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IDE",
                                description: Some(
                                    "IDE"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "RTR",
                                description: Some(
                                    "RTR"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
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
                    name: "CAN_RDT0R",
                    description: "CAN_RDT0R",
                    address_offset: 436,
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
                                name: "TIME",
                                description: Some(
                                    "TIME"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 16
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FMI",
                                description: Some(
                                    "FMI"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DLC",
                                description: Some(
                                    "DLC"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 4
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
                    name: "CAN_RDL0R",
                    description: "CAN_RDL0R",
                    address_offset: 440,
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
                                name: "DATA3",
                                description: Some(
                                    "DATA3"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATA2",
                                description: Some(
                                    "DATA2"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATA1",
                                description: Some(
                                    "DATA1"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATA0",
                                description: Some(
                                    "DATA0"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 8
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
                    name: "CAN_RDH0R",
                    description: "CAN_RDH0R",
                    address_offset: 444,
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
                                name: "DATA7",
                                description: Some(
                                    "DATA7"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATA6",
                                description: Some(
                                    "DATA6"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATA5",
                                description: Some(
                                    "DATA5"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATA4",
                                description: Some(
                                    "DATA4"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 8
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
                    name: "CAN_RI1R",
                    description: "CAN_RI1R",
                    address_offset: 448,
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
                                name: "STID",
                                description: Some(
                                    "STID"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 11
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EXID",
                                description: Some(
                                    "EXID"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 18
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IDE",
                                description: Some(
                                    "IDE"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "RTR",
                                description: Some(
                                    "RTR"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
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
                    name: "CAN_RDT1R",
                    description: "CAN_RDT1R",
                    address_offset: 452,
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
                                name: "TIME",
                                description: Some(
                                    "TIME"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 16
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FMI",
                                description: Some(
                                    "FMI"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DLC",
                                description: Some(
                                    "DLC"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 4
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
                    name: "CAN_RDL1R",
                    description: "CAN_RDL1R",
                    address_offset: 456,
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
                                name: "DATA3",
                                description: Some(
                                    "DATA3"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATA2",
                                description: Some(
                                    "DATA2"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATA1",
                                description: Some(
                                    "DATA1"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATA0",
                                description: Some(
                                    "DATA0"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 8
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
                    name: "CAN_RDH1R",
                    description: "CAN_RDH1R",
                    address_offset: 460,
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
                                name: "DATA7",
                                description: Some(
                                    "DATA7"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATA6",
                                description: Some(
                                    "DATA6"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATA5",
                                description: Some(
                                    "DATA5"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATA4",
                                description: Some(
                                    "DATA4"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 8
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
                    name: "CAN_FMR",
                    description: "CAN_FMR",
                    address_offset: 512,
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
                                name: "FINIT",
                                description: Some(
                                    "FINIT"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
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
                    name: "CAN_FM1R",
                    description: "CAN_FM1R",
                    address_offset: 516,
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
                                name: "FBM0",
                                description: Some(
                                    "Filter mode"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FBM1",
                                description: Some(
                                    "Filter mode"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FBM2",
                                description: Some(
                                    "Filter mode"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FBM3",
                                description: Some(
                                    "Filter mode"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FBM4",
                                description: Some(
                                    "Filter mode"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FBM5",
                                description: Some(
                                    "Filter mode"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FBM6",
                                description: Some(
                                    "Filter mode"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FBM7",
                                description: Some(
                                    "Filter mode"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FBM8",
                                description: Some(
                                    "Filter mode"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FBM9",
                                description: Some(
                                    "Filter mode"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FBM10",
                                description: Some(
                                    "Filter mode"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FBM11",
                                description: Some(
                                    "Filter mode"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FBM12",
                                description: Some(
                                    "Filter mode"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FBM13",
                                description: Some(
                                    "Filter mode"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
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
                    name: "CAN_FS1R",
                    description: "CAN_FS1R",
                    address_offset: 524,
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
                                name: "FSC0",
                                description: Some(
                                    "Filter scale configuration"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FSC1",
                                description: Some(
                                    "Filter scale configuration"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FSC2",
                                description: Some(
                                    "Filter scale configuration"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FSC3",
                                description: Some(
                                    "Filter scale configuration"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FSC4",
                                description: Some(
                                    "Filter scale configuration"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FSC5",
                                description: Some(
                                    "Filter scale configuration"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FSC6",
                                description: Some(
                                    "Filter scale configuration"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FSC7",
                                description: Some(
                                    "Filter scale configuration"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FSC8",
                                description: Some(
                                    "Filter scale configuration"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FSC9",
                                description: Some(
                                    "Filter scale configuration"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FSC10",
                                description: Some(
                                    "Filter scale configuration"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FSC11",
                                description: Some(
                                    "Filter scale configuration"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FSC12",
                                description: Some(
                                    "Filter scale configuration"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FSC13",
                                description: Some(
                                    "Filter scale configuration"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
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
                    name: "CAN_FFA1R",
                    description: "CAN_FFA1R",
                    address_offset: 532,
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
                                name: "FFA0",
                                description: Some(
                                    "Filter FIFO assignment for filter\r\n              0"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FFA1",
                                description: Some(
                                    "Filter FIFO assignment for filter\r\n              1"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FFA2",
                                description: Some(
                                    "Filter FIFO assignment for filter\r\n              2"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FFA3",
                                description: Some(
                                    "Filter FIFO assignment for filter\r\n              3"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FFA4",
                                description: Some(
                                    "Filter FIFO assignment for filter\r\n              4"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FFA5",
                                description: Some(
                                    "Filter FIFO assignment for filter\r\n              5"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FFA6",
                                description: Some(
                                    "Filter FIFO assignment for filter\r\n              6"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FFA7",
                                description: Some(
                                    "Filter FIFO assignment for filter\r\n              7"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FFA8",
                                description: Some(
                                    "Filter FIFO assignment for filter\r\n              8"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FFA9",
                                description: Some(
                                    "Filter FIFO assignment for filter\r\n              9"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FFA10",
                                description: Some(
                                    "Filter FIFO assignment for filter\r\n              10"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FFA11",
                                description: Some(
                                    "Filter FIFO assignment for filter\r\n              11"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FFA12",
                                description: Some(
                                    "Filter FIFO assignment for filter\r\n              12"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FFA13",
                                description: Some(
                                    "Filter FIFO assignment for filter\r\n              13"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
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
                    name: "CAN_FA1R",
                    description: "CAN_FA1R",
                    address_offset: 540,
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
                                name: "FACT0",
                                description: Some(
                                    "Filter active"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FACT1",
                                description: Some(
                                    "Filter active"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FACT2",
                                description: Some(
                                    "Filter active"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FACT3",
                                description: Some(
                                    "Filter active"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FACT4",
                                description: Some(
                                    "Filter active"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FACT5",
                                description: Some(
                                    "Filter active"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FACT6",
                                description: Some(
                                    "Filter active"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FACT7",
                                description: Some(
                                    "Filter active"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FACT8",
                                description: Some(
                                    "Filter active"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FACT9",
                                description: Some(
                                    "Filter active"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FACT10",
                                description: Some(
                                    "Filter active"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FACT11",
                                description: Some(
                                    "Filter active"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FACT12",
                                description: Some(
                                    "Filter active"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FACT13",
                                description: Some(
                                    "Filter active"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
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
                    name: "F0R1",
                    description: "Filter bank 0 register 1",
                    address_offset: 576,
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
                                name: "FB0",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB1",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB2",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB3",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB4",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB5",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB6",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB7",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB8",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB9",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB10",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB11",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB12",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB13",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB14",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB15",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB16",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB17",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB18",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB19",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB20",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB21",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB22",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB23",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 23,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB24",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB25",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 25,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB26",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 26,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB27",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 27,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB28",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB29",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 29,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB30",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 30,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB31",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 31,
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
                    name: "F0R2",
                    description: "Filter bank 0 register 2",
                    address_offset: 580,
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
                                name: "FB0",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB1",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB2",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB3",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB4",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB5",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB6",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB7",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB8",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB9",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB10",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB11",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB12",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB13",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB14",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB15",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB16",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB17",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB18",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB19",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB20",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB21",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB22",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB23",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 23,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB24",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB25",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 25,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB26",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 26,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB27",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 27,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB28",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB29",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 29,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB30",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 30,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB31",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 31,
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
                    name: "F1R1",
                    description: "Filter bank 1 register 1",
                    address_offset: 584,
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
                                name: "FB0",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB1",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB2",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB3",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB4",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB5",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB6",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB7",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB8",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB9",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB10",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB11",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB12",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB13",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB14",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB15",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB16",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB17",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB18",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB19",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB20",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB21",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB22",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB23",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 23,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB24",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB25",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 25,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB26",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 26,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB27",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 27,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB28",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB29",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 29,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB30",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 30,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB31",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 31,
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
                    name: "F1R2",
                    description: "Filter bank 1 register 2",
                    address_offset: 588,
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
                                name: "FB0",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB1",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB2",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB3",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB4",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB5",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB6",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB7",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB8",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB9",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB10",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB11",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB12",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB13",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB14",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB15",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB16",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB17",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB18",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB19",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB20",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB21",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB22",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB23",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 23,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB24",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB25",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 25,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB26",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 26,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB27",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 27,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB28",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB29",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 29,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB30",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 30,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB31",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 31,
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
                    name: "F2R1",
                    description: "Filter bank 2 register 1",
                    address_offset: 592,
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
                                name: "FB0",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB1",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB2",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB3",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB4",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB5",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB6",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB7",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB8",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB9",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB10",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB11",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB12",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB13",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB14",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB15",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB16",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB17",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB18",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB19",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB20",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB21",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB22",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB23",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 23,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB24",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB25",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 25,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB26",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 26,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB27",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 27,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB28",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB29",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 29,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB30",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 30,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB31",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 31,
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
                    name: "F2R2",
                    description: "Filter bank 2 register 2",
                    address_offset: 596,
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
                                name: "FB0",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB1",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB2",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB3",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB4",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB5",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB6",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB7",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB8",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB9",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB10",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB11",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB12",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB13",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB14",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB15",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB16",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB17",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB18",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB19",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB20",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB21",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB22",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB23",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 23,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB24",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB25",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 25,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB26",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 26,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB27",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 27,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB28",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB29",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 29,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB30",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 30,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB31",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 31,
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
                    name: "F3R1",
                    description: "Filter bank 3 register 1",
                    address_offset: 600,
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
                                name: "FB0",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB1",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB2",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB3",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB4",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB5",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB6",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB7",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB8",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB9",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB10",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB11",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB12",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB13",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB14",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB15",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB16",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB17",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB18",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB19",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB20",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB21",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB22",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB23",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 23,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB24",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB25",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 25,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB26",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 26,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB27",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 27,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB28",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB29",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 29,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB30",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 30,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB31",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 31,
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
                    name: "F3R2",
                    description: "Filter bank 3 register 2",
                    address_offset: 604,
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
                                name: "FB0",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB1",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB2",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB3",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB4",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB5",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB6",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB7",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB8",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB9",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB10",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB11",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB12",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB13",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB14",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB15",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB16",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB17",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB18",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB19",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB20",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB21",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB22",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB23",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 23,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB24",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB25",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 25,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB26",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 26,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB27",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 27,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB28",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB29",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 29,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB30",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 30,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB31",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 31,
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
                    name: "F4R1",
                    description: "Filter bank 4 register 1",
                    address_offset: 608,
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
                                name: "FB0",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB1",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB2",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB3",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB4",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB5",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB6",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB7",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB8",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB9",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB10",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB11",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB12",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB13",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB14",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB15",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB16",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB17",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB18",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB19",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB20",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB21",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB22",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB23",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 23,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB24",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB25",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 25,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB26",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 26,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB27",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 27,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB28",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB29",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 29,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB30",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 30,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB31",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 31,
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
                    name: "F4R2",
                    description: "Filter bank 4 register 2",
                    address_offset: 612,
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
                                name: "FB0",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB1",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB2",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB3",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB4",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB5",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB6",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB7",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB8",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB9",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB10",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB11",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB12",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB13",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB14",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB15",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB16",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB17",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB18",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB19",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB20",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB21",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB22",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB23",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 23,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB24",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB25",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 25,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB26",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 26,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB27",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 27,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB28",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB29",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 29,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB30",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 30,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB31",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 31,
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
                    name: "F5R1",
                    description: "Filter bank 5 register 1",
                    address_offset: 616,
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
                                name: "FB0",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB1",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB2",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB3",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB4",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB5",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB6",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB7",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB8",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB9",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB10",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB11",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB12",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB13",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB14",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB15",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB16",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB17",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB18",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB19",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB20",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB21",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB22",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB23",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 23,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB24",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB25",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 25,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB26",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 26,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB27",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 27,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB28",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB29",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 29,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB30",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 30,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB31",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 31,
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
                    name: "F5R2",
                    description: "Filter bank 5 register 2",
                    address_offset: 620,
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
                                name: "FB0",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB1",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB2",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB3",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB4",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB5",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB6",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB7",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB8",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB9",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB10",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB11",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB12",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB13",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB14",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB15",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB16",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB17",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB18",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB19",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB20",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB21",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB22",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB23",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 23,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB24",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB25",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 25,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB26",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 26,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB27",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 27,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB28",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB29",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 29,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB30",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 30,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB31",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 31,
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
                    name: "F6R1",
                    description: "Filter bank 6 register 1",
                    address_offset: 624,
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
                                name: "FB0",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB1",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB2",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB3",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB4",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB5",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB6",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB7",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB8",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB9",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB10",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB11",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB12",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB13",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB14",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB15",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB16",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB17",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB18",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB19",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB20",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB21",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB22",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB23",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 23,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB24",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB25",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 25,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB26",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 26,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB27",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 27,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB28",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB29",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 29,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB30",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 30,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB31",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 31,
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
                    name: "F6R2",
                    description: "Filter bank 6 register 2",
                    address_offset: 628,
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
                                name: "FB0",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB1",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB2",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB3",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB4",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB5",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB6",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB7",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB8",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB9",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB10",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB11",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB12",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB13",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB14",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB15",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB16",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB17",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB18",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB19",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB20",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB21",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB22",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB23",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 23,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB24",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB25",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 25,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB26",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 26,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB27",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 27,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB28",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB29",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 29,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB30",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 30,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB31",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 31,
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
                    name: "F7R1",
                    description: "Filter bank 7 register 1",
                    address_offset: 632,
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
                                name: "FB0",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB1",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB2",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB3",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB4",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB5",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB6",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB7",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB8",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB9",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB10",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB11",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB12",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB13",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB14",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB15",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB16",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB17",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB18",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB19",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB20",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB21",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB22",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB23",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 23,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB24",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB25",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 25,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB26",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 26,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB27",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 27,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB28",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB29",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 29,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB30",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 30,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB31",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 31,
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
                    name: "F7R2",
                    description: "Filter bank 7 register 2",
                    address_offset: 636,
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
                                name: "FB0",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB1",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB2",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB3",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB4",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB5",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB6",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB7",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB8",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB9",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB10",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB11",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB12",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB13",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB14",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB15",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB16",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB17",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB18",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB19",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB20",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB21",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB22",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB23",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 23,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB24",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB25",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 25,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB26",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 26,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB27",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 27,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB28",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB29",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 29,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB30",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 30,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB31",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 31,
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
                    name: "F8R1",
                    description: "Filter bank 8 register 1",
                    address_offset: 640,
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
                                name: "FB0",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB1",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB2",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB3",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB4",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB5",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB6",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB7",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB8",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB9",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB10",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB11",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB12",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB13",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB14",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB15",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB16",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB17",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB18",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB19",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB20",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB21",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB22",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB23",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 23,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB24",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB25",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 25,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB26",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 26,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB27",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 27,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB28",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB29",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 29,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB30",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 30,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB31",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 31,
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
                    name: "F8R2",
                    description: "Filter bank 8 register 2",
                    address_offset: 644,
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
                                name: "FB0",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB1",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB2",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB3",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB4",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB5",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB6",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB7",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB8",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB9",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB10",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB11",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB12",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB13",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB14",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB15",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB16",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB17",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB18",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB19",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB20",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB21",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB22",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB23",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 23,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB24",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB25",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 25,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB26",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 26,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB27",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 27,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB28",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB29",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 29,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB30",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 30,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB31",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 31,
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
                    name: "F9R1",
                    description: "Filter bank 9 register 1",
                    address_offset: 648,
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
                                name: "FB0",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB1",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB2",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB3",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB4",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB5",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB6",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB7",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB8",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB9",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB10",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB11",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB12",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB13",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB14",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB15",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB16",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB17",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB18",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB19",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB20",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB21",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB22",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB23",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 23,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB24",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB25",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 25,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB26",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 26,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB27",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 27,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB28",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB29",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 29,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB30",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 30,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB31",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 31,
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
                    name: "F9R2",
                    description: "Filter bank 9 register 2",
                    address_offset: 652,
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
                                name: "FB0",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB1",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB2",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB3",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB4",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB5",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB6",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB7",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB8",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB9",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB10",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB11",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB12",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB13",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB14",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB15",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB16",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB17",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB18",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB19",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB20",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB21",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB22",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB23",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 23,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB24",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB25",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 25,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB26",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 26,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB27",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 27,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB28",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB29",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 29,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB30",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 30,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB31",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 31,
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
                    name: "F10R1",
                    description: "Filter bank 10 register 1",
                    address_offset: 656,
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
                                name: "FB0",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB1",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB2",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB3",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB4",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB5",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB6",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB7",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB8",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB9",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB10",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB11",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB12",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB13",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB14",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB15",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB16",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB17",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB18",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB19",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB20",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB21",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB22",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB23",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 23,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB24",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB25",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 25,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB26",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 26,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB27",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 27,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB28",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB29",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 29,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB30",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 30,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB31",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 31,
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
                    name: "F10R2",
                    description: "Filter bank 10 register 2",
                    address_offset: 660,
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
                                name: "FB0",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB1",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB2",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB3",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB4",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB5",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB6",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB7",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB8",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB9",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB10",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB11",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB12",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB13",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB14",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB15",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB16",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB17",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB18",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB19",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB20",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB21",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB22",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB23",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 23,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB24",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB25",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 25,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB26",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 26,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB27",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 27,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB28",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB29",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 29,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB30",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 30,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB31",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 31,
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
                    name: "F11R1",
                    description: "Filter bank 11 register 1",
                    address_offset: 664,
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
                                name: "FB0",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB1",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB2",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB3",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB4",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB5",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB6",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB7",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB8",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB9",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB10",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB11",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB12",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB13",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB14",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB15",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB16",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB17",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB18",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB19",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB20",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB21",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB22",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB23",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 23,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB24",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB25",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 25,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB26",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 26,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB27",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 27,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB28",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB29",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 29,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB30",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 30,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB31",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 31,
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
                    name: "F11R2",
                    description: "Filter bank 11 register 2",
                    address_offset: 668,
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
                                name: "FB0",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB1",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB2",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB3",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB4",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB5",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB6",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB7",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB8",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB9",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB10",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB11",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB12",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB13",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB14",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB15",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB16",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB17",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB18",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB19",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB20",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB21",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB22",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB23",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 23,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB24",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB25",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 25,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB26",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 26,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB27",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 27,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB28",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB29",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 29,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB30",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 30,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB31",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 31,
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
                    name: "F12R1",
                    description: "Filter bank 4 register 1",
                    address_offset: 672,
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
                                name: "FB0",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB1",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB2",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB3",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB4",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB5",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB6",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB7",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB8",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB9",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB10",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB11",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB12",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB13",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB14",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB15",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB16",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB17",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB18",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB19",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB20",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB21",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB22",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB23",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 23,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB24",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB25",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 25,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB26",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 26,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB27",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 27,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB28",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB29",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 29,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB30",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 30,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB31",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 31,
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
                    name: "F12R2",
                    description: "Filter bank 12 register 2",
                    address_offset: 676,
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
                                name: "FB0",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB1",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB2",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB3",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB4",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB5",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB6",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB7",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB8",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB9",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB10",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB11",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB12",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB13",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB14",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB15",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB16",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB17",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB18",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB19",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB20",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB21",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB22",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB23",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 23,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB24",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB25",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 25,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB26",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 26,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB27",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 27,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB28",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB29",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 29,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB30",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 30,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB31",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 31,
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
                    name: "F13R1",
                    description: "Filter bank 13 register 1",
                    address_offset: 680,
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
                                name: "FB0",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB1",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB2",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB3",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB4",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB5",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB6",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB7",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB8",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB9",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB10",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB11",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB12",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB13",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB14",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB15",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB16",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB17",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB18",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB19",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB20",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB21",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB22",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB23",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 23,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB24",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB25",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 25,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB26",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 26,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB27",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 27,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB28",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB29",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 29,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB30",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 30,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB31",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 31,
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
                    name: "F13R2",
                    description: "Filter bank 13 register 2",
                    address_offset: 684,
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
                                name: "FB0",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB1",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB2",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB3",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB4",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB5",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB6",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB7",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB8",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB9",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB10",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB11",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB12",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB13",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB14",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB15",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB16",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB17",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB18",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB19",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB20",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB21",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB22",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB23",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 23,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB24",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB25",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 25,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB26",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 26,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB27",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 27,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB28",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB29",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 29,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB30",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 30,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FB31",
                                description: Some(
                                    "Filter bits"
                                ),
                                bit_range: BitRange {
                                    offset: 31,
                                    width: 1
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
