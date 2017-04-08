// GPIOF
// General purpose I/O
pub const ADDRESS: u32 = 0x40011C00;
/*
Peripheral {
    name: "GPIOF",
    group_name: Some(
        "GPIO"
    ),
    description: Some(
        "General purpose I/O"
    ),
    base_address: 1073814528,
    interrupt: None,
    registers: Some(
        [
            Single(
                RegisterInfo {
                    name: "CRL",
                    description: "Port configuration register low\r\n          (GPIOn_CRL)",
                    address_offset: 0,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        1145324612
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "MODE0",
                                description: Some(
                                    "Port n.0 mode bits"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CNF0",
                                description: Some(
                                    "Port n.0 configuration\r\n              bits"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MODE1",
                                description: Some(
                                    "Port n.1 mode bits"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CNF1",
                                description: Some(
                                    "Port n.1 configuration\r\n              bits"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MODE2",
                                description: Some(
                                    "Port n.2 mode bits"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CNF2",
                                description: Some(
                                    "Port n.2 configuration\r\n              bits"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MODE3",
                                description: Some(
                                    "Port n.3 mode bits"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CNF3",
                                description: Some(
                                    "Port n.3 configuration\r\n              bits"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MODE4",
                                description: Some(
                                    "Port n.4 mode bits"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CNF4",
                                description: Some(
                                    "Port n.4 configuration\r\n              bits"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MODE5",
                                description: Some(
                                    "Port n.5 mode bits"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CNF5",
                                description: Some(
                                    "Port n.5 configuration\r\n              bits"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MODE6",
                                description: Some(
                                    "Port n.6 mode bits"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CNF6",
                                description: Some(
                                    "Port n.6 configuration\r\n              bits"
                                ),
                                bit_range: BitRange {
                                    offset: 26,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MODE7",
                                description: Some(
                                    "Port n.7 mode bits"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CNF7",
                                description: Some(
                                    "Port n.7 configuration\r\n              bits"
                                ),
                                bit_range: BitRange {
                                    offset: 30,
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
                    name: "CRH",
                    description: "Port configuration register high\r\n          (GPIOn_CRL)",
                    address_offset: 4,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        1145324612
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "MODE8",
                                description: Some(
                                    "Port n.8 mode bits"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CNF8",
                                description: Some(
                                    "Port n.8 configuration\r\n              bits"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MODE9",
                                description: Some(
                                    "Port n.9 mode bits"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CNF9",
                                description: Some(
                                    "Port n.9 configuration\r\n              bits"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MODE10",
                                description: Some(
                                    "Port n.10 mode bits"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CNF10",
                                description: Some(
                                    "Port n.10 configuration\r\n              bits"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MODE11",
                                description: Some(
                                    "Port n.11 mode bits"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CNF11",
                                description: Some(
                                    "Port n.11 configuration\r\n              bits"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MODE12",
                                description: Some(
                                    "Port n.12 mode bits"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CNF12",
                                description: Some(
                                    "Port n.12 configuration\r\n              bits"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MODE13",
                                description: Some(
                                    "Port n.13 mode bits"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CNF13",
                                description: Some(
                                    "Port n.13 configuration\r\n              bits"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MODE14",
                                description: Some(
                                    "Port n.14 mode bits"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CNF14",
                                description: Some(
                                    "Port n.14 configuration\r\n              bits"
                                ),
                                bit_range: BitRange {
                                    offset: 26,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MODE15",
                                description: Some(
                                    "Port n.15 mode bits"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CNF15",
                                description: Some(
                                    "Port n.15 configuration\r\n              bits"
                                ),
                                bit_range: BitRange {
                                    offset: 30,
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
                    name: "IDR",
                    description: "Port input data register\r\n          (GPIOn_IDR)",
                    address_offset: 8,
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
                                name: "IDR0",
                                description: Some(
                                    "Port input data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IDR1",
                                description: Some(
                                    "Port input data"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IDR2",
                                description: Some(
                                    "Port input data"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IDR3",
                                description: Some(
                                    "Port input data"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IDR4",
                                description: Some(
                                    "Port input data"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IDR5",
                                description: Some(
                                    "Port input data"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IDR6",
                                description: Some(
                                    "Port input data"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IDR7",
                                description: Some(
                                    "Port input data"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IDR8",
                                description: Some(
                                    "Port input data"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IDR9",
                                description: Some(
                                    "Port input data"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IDR10",
                                description: Some(
                                    "Port input data"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IDR11",
                                description: Some(
                                    "Port input data"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IDR12",
                                description: Some(
                                    "Port input data"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IDR13",
                                description: Some(
                                    "Port input data"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IDR14",
                                description: Some(
                                    "Port input data"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IDR15",
                                description: Some(
                                    "Port input data"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
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
                    name: "ODR",
                    description: "Port output data register\r\n          (GPIOn_ODR)",
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
                                name: "ODR0",
                                description: Some(
                                    "Port output data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ODR1",
                                description: Some(
                                    "Port output data"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ODR2",
                                description: Some(
                                    "Port output data"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ODR3",
                                description: Some(
                                    "Port output data"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ODR4",
                                description: Some(
                                    "Port output data"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ODR5",
                                description: Some(
                                    "Port output data"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ODR6",
                                description: Some(
                                    "Port output data"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ODR7",
                                description: Some(
                                    "Port output data"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ODR8",
                                description: Some(
                                    "Port output data"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ODR9",
                                description: Some(
                                    "Port output data"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ODR10",
                                description: Some(
                                    "Port output data"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ODR11",
                                description: Some(
                                    "Port output data"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ODR12",
                                description: Some(
                                    "Port output data"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ODR13",
                                description: Some(
                                    "Port output data"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ODR14",
                                description: Some(
                                    "Port output data"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ODR15",
                                description: Some(
                                    "Port output data"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
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
                    name: "BSRR",
                    description: "Port bit set/reset register\r\n          (GPIOn_BSRR)",
                    address_offset: 16,
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
                                name: "BS0",
                                description: Some(
                                    "Set bit 0"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BS1",
                                description: Some(
                                    "Set bit 1"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BS2",
                                description: Some(
                                    "Set bit 1"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BS3",
                                description: Some(
                                    "Set bit 3"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BS4",
                                description: Some(
                                    "Set bit 4"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BS5",
                                description: Some(
                                    "Set bit 5"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BS6",
                                description: Some(
                                    "Set bit 6"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BS7",
                                description: Some(
                                    "Set bit 7"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BS8",
                                description: Some(
                                    "Set bit 8"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BS9",
                                description: Some(
                                    "Set bit 9"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BS10",
                                description: Some(
                                    "Set bit 10"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BS11",
                                description: Some(
                                    "Set bit 11"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BS12",
                                description: Some(
                                    "Set bit 12"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BS13",
                                description: Some(
                                    "Set bit 13"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BS14",
                                description: Some(
                                    "Set bit 14"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BS15",
                                description: Some(
                                    "Set bit 15"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BR0",
                                description: Some(
                                    "Reset bit 0"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BR1",
                                description: Some(
                                    "Reset bit 1"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BR2",
                                description: Some(
                                    "Reset bit 2"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BR3",
                                description: Some(
                                    "Reset bit 3"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BR4",
                                description: Some(
                                    "Reset bit 4"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BR5",
                                description: Some(
                                    "Reset bit 5"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BR6",
                                description: Some(
                                    "Reset bit 6"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BR7",
                                description: Some(
                                    "Reset bit 7"
                                ),
                                bit_range: BitRange {
                                    offset: 23,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BR8",
                                description: Some(
                                    "Reset bit 8"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BR9",
                                description: Some(
                                    "Reset bit 9"
                                ),
                                bit_range: BitRange {
                                    offset: 25,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BR10",
                                description: Some(
                                    "Reset bit 10"
                                ),
                                bit_range: BitRange {
                                    offset: 26,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BR11",
                                description: Some(
                                    "Reset bit 11"
                                ),
                                bit_range: BitRange {
                                    offset: 27,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BR12",
                                description: Some(
                                    "Reset bit 12"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BR13",
                                description: Some(
                                    "Reset bit 13"
                                ),
                                bit_range: BitRange {
                                    offset: 29,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BR14",
                                description: Some(
                                    "Reset bit 14"
                                ),
                                bit_range: BitRange {
                                    offset: 30,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BR15",
                                description: Some(
                                    "Reset bit 15"
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
                    name: "BRR",
                    description: "Port bit reset register\r\n          (GPIOn_BRR)",
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
                                name: "BR0",
                                description: Some(
                                    "Reset bit 0"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BR1",
                                description: Some(
                                    "Reset bit 1"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BR2",
                                description: Some(
                                    "Reset bit 1"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BR3",
                                description: Some(
                                    "Reset bit 3"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BR4",
                                description: Some(
                                    "Reset bit 4"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BR5",
                                description: Some(
                                    "Reset bit 5"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BR6",
                                description: Some(
                                    "Reset bit 6"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BR7",
                                description: Some(
                                    "Reset bit 7"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BR8",
                                description: Some(
                                    "Reset bit 8"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BR9",
                                description: Some(
                                    "Reset bit 9"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BR10",
                                description: Some(
                                    "Reset bit 10"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BR11",
                                description: Some(
                                    "Reset bit 11"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BR12",
                                description: Some(
                                    "Reset bit 12"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BR13",
                                description: Some(
                                    "Reset bit 13"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BR14",
                                description: Some(
                                    "Reset bit 14"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BR15",
                                description: Some(
                                    "Reset bit 15"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
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
                    name: "LCKR",
                    description: "Port configuration lock\r\n          register",
                    address_offset: 24,
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
                                name: "LCK0",
                                description: Some(
                                    "Port A Lock bit 0"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "LCK1",
                                description: Some(
                                    "Port A Lock bit 1"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "LCK2",
                                description: Some(
                                    "Port A Lock bit 2"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "LCK3",
                                description: Some(
                                    "Port A Lock bit 3"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "LCK4",
                                description: Some(
                                    "Port A Lock bit 4"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "LCK5",
                                description: Some(
                                    "Port A Lock bit 5"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "LCK6",
                                description: Some(
                                    "Port A Lock bit 6"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "LCK7",
                                description: Some(
                                    "Port A Lock bit 7"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "LCK8",
                                description: Some(
                                    "Port A Lock bit 8"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "LCK9",
                                description: Some(
                                    "Port A Lock bit 9"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "LCK10",
                                description: Some(
                                    "Port A Lock bit 10"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "LCK11",
                                description: Some(
                                    "Port A Lock bit 11"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "LCK12",
                                description: Some(
                                    "Port A Lock bit 12"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "LCK13",
                                description: Some(
                                    "Port A Lock bit 13"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "LCK14",
                                description: Some(
                                    "Port A Lock bit 14"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "LCK15",
                                description: Some(
                                    "Port A Lock bit 15"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "LCKK",
                                description: Some(
                                    "Lock key"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
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
    derived_from: Some(
        "GPIOA"
    )
}
*/
