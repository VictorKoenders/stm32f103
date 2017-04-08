// FLASH
// FLASH
pub const ADDRESS: u32 = 0x40022000;
/*
Peripheral {
    name: "FLASH",
    group_name: Some(
        "FLASH"
    ),
    description: Some(
        "FLASH"
    ),
    base_address: 1073881088,
    interrupt: Some(
        Interrupt {
            name: "FLASH",
            description: Some(
                "Flash global interrupt"
            ),
            value: 4
        }
    ),
    registers: Some(
        [
            Single(
                RegisterInfo {
                    name: "ACR",
                    description: "Flash access control register",
                    address_offset: 0,
                    size: Some(
                        32
                    ),
                    access: None,
                    reset_value: Some(
                        48
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "LATENCY",
                                description: Some(
                                    "Latency"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 3
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "HLFCYA",
                                description: Some(
                                    "Flash half cycle access\r\n              enable"
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
                                name: "PRFTBE",
                                description: Some(
                                    "Prefetch buffer enable"
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
                                name: "PRFTBS",
                                description: Some(
                                    "Prefetch buffer status"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
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
                    name: "KEYR",
                    description: "Flash key register",
                    address_offset: 4,
                    size: Some(
                        32
                    ),
                    access: Some(
                        WriteOnly
                    ),
                    reset_value: Some(
                        0
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "KEY",
                                description: Some(
                                    "FPEC key"
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
                    name: "OPTKEYR",
                    description: "Flash option key register",
                    address_offset: 8,
                    size: Some(
                        32
                    ),
                    access: Some(
                        WriteOnly
                    ),
                    reset_value: Some(
                        0
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "OPTKEY",
                                description: Some(
                                    "Option byte key"
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
                    name: "SR",
                    description: "Status register",
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
                                name: "EOP",
                                description: Some(
                                    "End of operation"
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
                                name: "WRPRTERR",
                                description: Some(
                                    "Write protection error"
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
                                name: "PGERR",
                                description: Some(
                                    "Programming error"
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
                                name: "BSY",
                                description: Some(
                                    "Busy"
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
                    name: "CR",
                    description: "Control register",
                    address_offset: 16,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        128
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "PG",
                                description: Some(
                                    "Programming"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PER",
                                description: Some(
                                    "Page Erase"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MER",
                                description: Some(
                                    "Mass Erase"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "OPTPG",
                                description: Some(
                                    "Option byte programming"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "OPTER",
                                description: Some(
                                    "Option byte erase"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "STRT",
                                description: Some(
                                    "Start"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "LOCK",
                                description: Some(
                                    "Lock"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "OPTWRE",
                                description: Some(
                                    "Option bytes write enable"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ERRIE",
                                description: Some(
                                    "Error interrupt enable"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EOPIE",
                                description: Some(
                                    "End of operation interrupt\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
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
                    name: "AR",
                    description: "Flash address register",
                    address_offset: 20,
                    size: Some(
                        32
                    ),
                    access: Some(
                        WriteOnly
                    ),
                    reset_value: Some(
                        0
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "FAR",
                                description: Some(
                                    "Flash Address"
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
                    name: "OBR",
                    description: "Option byte register",
                    address_offset: 28,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadOnly
                    ),
                    reset_value: Some(
                        67108860
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "OPTERR",
                                description: Some(
                                    "Option byte error"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "RDPRT",
                                description: Some(
                                    "Read protection"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "WDG_SW",
                                description: Some(
                                    "WDG_SW"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "nRST_STOP",
                                description: Some(
                                    "nRST_STOP"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "nRST_STDBY",
                                description: Some(
                                    "nRST_STDBY"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "Data0",
                                description: Some(
                                    "Data0"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "Data1",
                                description: Some(
                                    "Data1"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
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
                    name: "WRPR",
                    description: "Write protection register",
                    address_offset: 32,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadOnly
                    ),
                    reset_value: Some(
                        4294967295
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "WRP",
                                description: Some(
                                    "Write protect"
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
