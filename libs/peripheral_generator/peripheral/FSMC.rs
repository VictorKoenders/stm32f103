// FSMC
// Flexible static memory controller
pub const ADDRESS: u32 = 0xA0000000;
/*
Peripheral {
    name: "FSMC",
    group_name: Some(
        "FSMC"
    ),
    description: Some(
        "Flexible static memory controller"
    ),
    base_address: 2684354560,
    interrupt: Some(
        Interrupt {
            name: "FSMC",
            description: Some(
                "FSMC global interrupt"
            ),
            value: 48
        }
    ),
    registers: Some(
        [
            Single(
                RegisterInfo {
                    name: "BCR1",
                    description: "SRAM/NOR-Flash chip-select control register\r\n          1",
                    address_offset: 0,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        12496
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "CBURSTRW",
                                description: Some(
                                    "CBURSTRW"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ASYNCWAIT",
                                description: Some(
                                    "ASYNCWAIT"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EXTMOD",
                                description: Some(
                                    "EXTMOD"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "WAITEN",
                                description: Some(
                                    "WAITEN"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "WREN",
                                description: Some(
                                    "WREN"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "WAITCFG",
                                description: Some(
                                    "WAITCFG"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "WAITPOL",
                                description: Some(
                                    "WAITPOL"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BURSTEN",
                                description: Some(
                                    "BURSTEN"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FACCEN",
                                description: Some(
                                    "FACCEN"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MWID",
                                description: Some(
                                    "MWID"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MTYP",
                                description: Some(
                                    "MTYP"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MUXEN",
                                description: Some(
                                    "MUXEN"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MBKEN",
                                description: Some(
                                    "MBKEN"
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
                    name: "BTR1",
                    description: "SRAM/NOR-Flash chip-select timing register\r\n          1",
                    address_offset: 4,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        4294967295
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "ACCMOD",
                                description: Some(
                                    "ACCMOD"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATLAT",
                                description: Some(
                                    "DATLAT"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CLKDIV",
                                description: Some(
                                    "CLKDIV"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BUSTURN",
                                description: Some(
                                    "BUSTURN"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATAST",
                                description: Some(
                                    "DATAST"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ADDHLD",
                                description: Some(
                                    "ADDHLD"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ADDSET",
                                description: Some(
                                    "ADDSET"
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
                    name: "BCR2",
                    description: "SRAM/NOR-Flash chip-select control register\r\n          2",
                    address_offset: 8,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        12496
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "CBURSTRW",
                                description: Some(
                                    "CBURSTRW"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ASYNCWAIT",
                                description: Some(
                                    "ASYNCWAIT"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EXTMOD",
                                description: Some(
                                    "EXTMOD"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "WAITEN",
                                description: Some(
                                    "WAITEN"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "WREN",
                                description: Some(
                                    "WREN"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "WAITCFG",
                                description: Some(
                                    "WAITCFG"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "WRAPMOD",
                                description: Some(
                                    "WRAPMOD"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "WAITPOL",
                                description: Some(
                                    "WAITPOL"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BURSTEN",
                                description: Some(
                                    "BURSTEN"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FACCEN",
                                description: Some(
                                    "FACCEN"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MWID",
                                description: Some(
                                    "MWID"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MTYP",
                                description: Some(
                                    "MTYP"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MUXEN",
                                description: Some(
                                    "MUXEN"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MBKEN",
                                description: Some(
                                    "MBKEN"
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
                    name: "BTR2",
                    description: "SRAM/NOR-Flash chip-select timing register\r\n          2",
                    address_offset: 12,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        4294967295
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "ACCMOD",
                                description: Some(
                                    "ACCMOD"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATLAT",
                                description: Some(
                                    "DATLAT"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CLKDIV",
                                description: Some(
                                    "CLKDIV"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BUSTURN",
                                description: Some(
                                    "BUSTURN"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATAST",
                                description: Some(
                                    "DATAST"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ADDHLD",
                                description: Some(
                                    "ADDHLD"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ADDSET",
                                description: Some(
                                    "ADDSET"
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
                    name: "BCR3",
                    description: "SRAM/NOR-Flash chip-select control register\r\n          3",
                    address_offset: 16,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        12496
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "CBURSTRW",
                                description: Some(
                                    "CBURSTRW"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ASYNCWAIT",
                                description: Some(
                                    "ASYNCWAIT"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EXTMOD",
                                description: Some(
                                    "EXTMOD"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "WAITEN",
                                description: Some(
                                    "WAITEN"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "WREN",
                                description: Some(
                                    "WREN"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "WAITCFG",
                                description: Some(
                                    "WAITCFG"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "WRAPMOD",
                                description: Some(
                                    "WRAPMOD"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "WAITPOL",
                                description: Some(
                                    "WAITPOL"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BURSTEN",
                                description: Some(
                                    "BURSTEN"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FACCEN",
                                description: Some(
                                    "FACCEN"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MWID",
                                description: Some(
                                    "MWID"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MTYP",
                                description: Some(
                                    "MTYP"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MUXEN",
                                description: Some(
                                    "MUXEN"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MBKEN",
                                description: Some(
                                    "MBKEN"
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
                    name: "BTR3",
                    description: "SRAM/NOR-Flash chip-select timing register\r\n          3",
                    address_offset: 20,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        4294967295
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "ACCMOD",
                                description: Some(
                                    "ACCMOD"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATLAT",
                                description: Some(
                                    "DATLAT"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CLKDIV",
                                description: Some(
                                    "CLKDIV"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BUSTURN",
                                description: Some(
                                    "BUSTURN"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATAST",
                                description: Some(
                                    "DATAST"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ADDHLD",
                                description: Some(
                                    "ADDHLD"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ADDSET",
                                description: Some(
                                    "ADDSET"
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
                    name: "BCR4",
                    description: "SRAM/NOR-Flash chip-select control register\r\n          4",
                    address_offset: 24,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        12496
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "CBURSTRW",
                                description: Some(
                                    "CBURSTRW"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ASYNCWAIT",
                                description: Some(
                                    "ASYNCWAIT"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EXTMOD",
                                description: Some(
                                    "EXTMOD"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "WAITEN",
                                description: Some(
                                    "WAITEN"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "WREN",
                                description: Some(
                                    "WREN"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "WAITCFG",
                                description: Some(
                                    "WAITCFG"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "WRAPMOD",
                                description: Some(
                                    "WRAPMOD"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "WAITPOL",
                                description: Some(
                                    "WAITPOL"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BURSTEN",
                                description: Some(
                                    "BURSTEN"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FACCEN",
                                description: Some(
                                    "FACCEN"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MWID",
                                description: Some(
                                    "MWID"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MTYP",
                                description: Some(
                                    "MTYP"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MUXEN",
                                description: Some(
                                    "MUXEN"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MBKEN",
                                description: Some(
                                    "MBKEN"
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
                    name: "BTR4",
                    description: "SRAM/NOR-Flash chip-select timing register\r\n          4",
                    address_offset: 28,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        4294967295
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "ACCMOD",
                                description: Some(
                                    "ACCMOD"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATLAT",
                                description: Some(
                                    "DATLAT"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CLKDIV",
                                description: Some(
                                    "CLKDIV"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BUSTURN",
                                description: Some(
                                    "BUSTURN"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATAST",
                                description: Some(
                                    "DATAST"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ADDHLD",
                                description: Some(
                                    "ADDHLD"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ADDSET",
                                description: Some(
                                    "ADDSET"
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
                    name: "PCR2",
                    description: "PC Card/NAND Flash control register\r\n          2",
                    address_offset: 96,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        24
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "ECCPS",
                                description: Some(
                                    "ECCPS"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 3
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TAR",
                                description: Some(
                                    "TAR"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TCLR",
                                description: Some(
                                    "TCLR"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ECCEN",
                                description: Some(
                                    "ECCEN"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PWID",
                                description: Some(
                                    "PWID"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PTYP",
                                description: Some(
                                    "PTYP"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PBKEN",
                                description: Some(
                                    "PBKEN"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PWAITEN",
                                description: Some(
                                    "PWAITEN"
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
                    name: "SR2",
                    description: "FIFO status and interrupt register\r\n          2",
                    address_offset: 100,
                    size: Some(
                        32
                    ),
                    access: None,
                    reset_value: Some(
                        64
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "FEMPT",
                                description: Some(
                                    "FEMPT"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: Some(
                                    ReadOnly
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "IFEN",
                                description: Some(
                                    "IFEN"
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
                                name: "ILEN",
                                description: Some(
                                    "ILEN"
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
                                name: "IREN",
                                description: Some(
                                    "IREN"
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
                                name: "IFS",
                                description: Some(
                                    "IFS"
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
                                name: "ILS",
                                description: Some(
                                    "ILS"
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
                                name: "IRS",
                                description: Some(
                                    "IRS"
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
                    name: "PMEM2",
                    description: "Common memory space timing register\r\n          2",
                    address_offset: 104,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        4244438268
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "MEMHIZx",
                                description: Some(
                                    "MEMHIZx"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MEMHOLDx",
                                description: Some(
                                    "MEMHOLDx"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MEMWAITx",
                                description: Some(
                                    "MEMWAITx"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MEMSETx",
                                description: Some(
                                    "MEMSETx"
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
                    name: "PATT2",
                    description: "Attribute memory space timing register\r\n          2",
                    address_offset: 108,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        4244438268
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "ATTHIZx",
                                description: Some(
                                    "Attribute memory x databus HiZ\r\n              time"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ATTHOLDx",
                                description: Some(
                                    "Attribute memory x hold\r\n              time"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ATTWAITx",
                                description: Some(
                                    "Attribute memory x wait\r\n              time"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ATTSETx",
                                description: Some(
                                    "Attribute memory x setup\r\n              time"
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
                    name: "ECCR2",
                    description: "ECC result register 2",
                    address_offset: 116,
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
                                name: "ECCx",
                                description: Some(
                                    "ECC result"
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
                    name: "PCR3",
                    description: "PC Card/NAND Flash control register\r\n          3",
                    address_offset: 128,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        24
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "ECCPS",
                                description: Some(
                                    "ECCPS"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 3
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TAR",
                                description: Some(
                                    "TAR"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TCLR",
                                description: Some(
                                    "TCLR"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ECCEN",
                                description: Some(
                                    "ECCEN"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PWID",
                                description: Some(
                                    "PWID"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PTYP",
                                description: Some(
                                    "PTYP"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PBKEN",
                                description: Some(
                                    "PBKEN"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PWAITEN",
                                description: Some(
                                    "PWAITEN"
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
                    name: "SR3",
                    description: "FIFO status and interrupt register\r\n          3",
                    address_offset: 132,
                    size: Some(
                        32
                    ),
                    access: None,
                    reset_value: Some(
                        64
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "FEMPT",
                                description: Some(
                                    "FEMPT"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: Some(
                                    ReadOnly
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "IFEN",
                                description: Some(
                                    "IFEN"
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
                                name: "ILEN",
                                description: Some(
                                    "ILEN"
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
                                name: "IREN",
                                description: Some(
                                    "IREN"
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
                                name: "IFS",
                                description: Some(
                                    "IFS"
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
                                name: "ILS",
                                description: Some(
                                    "ILS"
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
                                name: "IRS",
                                description: Some(
                                    "IRS"
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
                    name: "PMEM3",
                    description: "Common memory space timing register\r\n          3",
                    address_offset: 136,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        4244438268
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "MEMHIZx",
                                description: Some(
                                    "MEMHIZx"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MEMHOLDx",
                                description: Some(
                                    "MEMHOLDx"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MEMWAITx",
                                description: Some(
                                    "MEMWAITx"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MEMSETx",
                                description: Some(
                                    "MEMSETx"
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
                    name: "PATT3",
                    description: "Attribute memory space timing register\r\n          3",
                    address_offset: 140,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        4244438268
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "ATTHIZx",
                                description: Some(
                                    "ATTHIZx"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ATTHOLDx",
                                description: Some(
                                    "ATTHOLDx"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ATTWAITx",
                                description: Some(
                                    "ATTWAITx"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ATTSETx",
                                description: Some(
                                    "ATTSETx"
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
                    name: "ECCR3",
                    description: "ECC result register 3",
                    address_offset: 148,
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
                                name: "ECCx",
                                description: Some(
                                    "ECCx"
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
                    name: "PCR4",
                    description: "PC Card/NAND Flash control register\r\n          4",
                    address_offset: 160,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        24
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "ECCPS",
                                description: Some(
                                    "ECCPS"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 3
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TAR",
                                description: Some(
                                    "TAR"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TCLR",
                                description: Some(
                                    "TCLR"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ECCEN",
                                description: Some(
                                    "ECCEN"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PWID",
                                description: Some(
                                    "PWID"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PTYP",
                                description: Some(
                                    "PTYP"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PBKEN",
                                description: Some(
                                    "PBKEN"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PWAITEN",
                                description: Some(
                                    "PWAITEN"
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
                    name: "SR4",
                    description: "FIFO status and interrupt register\r\n          4",
                    address_offset: 164,
                    size: Some(
                        32
                    ),
                    access: None,
                    reset_value: Some(
                        64
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "FEMPT",
                                description: Some(
                                    "FEMPT"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: Some(
                                    ReadOnly
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "IFEN",
                                description: Some(
                                    "IFEN"
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
                                name: "ILEN",
                                description: Some(
                                    "ILEN"
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
                                name: "IREN",
                                description: Some(
                                    "IREN"
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
                                name: "IFS",
                                description: Some(
                                    "IFS"
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
                                name: "ILS",
                                description: Some(
                                    "ILS"
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
                                name: "IRS",
                                description: Some(
                                    "IRS"
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
                    name: "PMEM4",
                    description: "Common memory space timing register\r\n          4",
                    address_offset: 168,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        4244438268
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "MEMHIZx",
                                description: Some(
                                    "MEMHIZx"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MEMHOLDx",
                                description: Some(
                                    "MEMHOLDx"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MEMWAITx",
                                description: Some(
                                    "MEMWAITx"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MEMSETx",
                                description: Some(
                                    "MEMSETx"
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
                    name: "PATT4",
                    description: "Attribute memory space timing register\r\n          4",
                    address_offset: 172,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        4244438268
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "ATTHIZx",
                                description: Some(
                                    "ATTHIZx"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ATTHOLDx",
                                description: Some(
                                    "ATTHOLDx"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ATTWAITx",
                                description: Some(
                                    "ATTWAITx"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ATTSETx",
                                description: Some(
                                    "ATTSETx"
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
                    name: "PIO4",
                    description: "I/O space timing register 4",
                    address_offset: 176,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        4244438268
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "IOHIZx",
                                description: Some(
                                    "IOHIZx"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IOHOLDx",
                                description: Some(
                                    "IOHOLDx"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IOWAITx",
                                description: Some(
                                    "IOWAITx"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IOSETx",
                                description: Some(
                                    "IOSETx"
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
                    name: "BWTR1",
                    description: "SRAM/NOR-Flash write timing registers\r\n          1",
                    address_offset: 260,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        268435455
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "ACCMOD",
                                description: Some(
                                    "ACCMOD"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATLAT",
                                description: Some(
                                    "DATLAT"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CLKDIV",
                                description: Some(
                                    "CLKDIV"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATAST",
                                description: Some(
                                    "DATAST"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ADDHLD",
                                description: Some(
                                    "ADDHLD"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ADDSET",
                                description: Some(
                                    "ADDSET"
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
                    name: "BWTR2",
                    description: "SRAM/NOR-Flash write timing registers\r\n          2",
                    address_offset: 268,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        268435455
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "ACCMOD",
                                description: Some(
                                    "ACCMOD"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATLAT",
                                description: Some(
                                    "DATLAT"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CLKDIV",
                                description: Some(
                                    "CLKDIV"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATAST",
                                description: Some(
                                    "DATAST"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ADDHLD",
                                description: Some(
                                    "ADDHLD"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ADDSET",
                                description: Some(
                                    "ADDSET"
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
                    name: "BWTR3",
                    description: "SRAM/NOR-Flash write timing registers\r\n          3",
                    address_offset: 276,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        268435455
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "ACCMOD",
                                description: Some(
                                    "ACCMOD"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATLAT",
                                description: Some(
                                    "DATLAT"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CLKDIV",
                                description: Some(
                                    "CLKDIV"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATAST",
                                description: Some(
                                    "DATAST"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ADDHLD",
                                description: Some(
                                    "ADDHLD"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ADDSET",
                                description: Some(
                                    "ADDSET"
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
                    name: "BWTR4",
                    description: "SRAM/NOR-Flash write timing registers\r\n          4",
                    address_offset: 284,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        268435455
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "ACCMOD",
                                description: Some(
                                    "ACCMOD"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATLAT",
                                description: Some(
                                    "DATLAT"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CLKDIV",
                                description: Some(
                                    "CLKDIV"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATAST",
                                description: Some(
                                    "DATAST"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ADDHLD",
                                description: Some(
                                    "ADDHLD"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ADDSET",
                                description: Some(
                                    "ADDSET"
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
            )
        ]
    ),
    derived_from: None
}
*/
