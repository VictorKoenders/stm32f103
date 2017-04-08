// BKP
// Backup registers
pub const ADDRESS: u32 = 0x40006C04;
/*
Peripheral {
    name: "BKP",
    group_name: Some(
        "BKP"
    ),
    description: Some(
        "Backup registers"
    ),
    base_address: 1073769476,
    interrupt: None,
    registers: Some(
        [
            Single(
                RegisterInfo {
                    name: "DR1",
                    description: "Backup data register (BKP_DR)",
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
                                name: "D1",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR2",
                    description: "Backup data register (BKP_DR)",
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
                                name: "D2",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR3",
                    description: "Backup data register (BKP_DR)",
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
                                name: "D3",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR4",
                    description: "Backup data register (BKP_DR)",
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
                                name: "D4",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR5",
                    description: "Backup data register (BKP_DR)",
                    address_offset: 16,
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
                                name: "D5",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR6",
                    description: "Backup data register (BKP_DR)",
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
                                name: "D6",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR7",
                    description: "Backup data register (BKP_DR)",
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
                                name: "D7",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR8",
                    description: "Backup data register (BKP_DR)",
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
                                name: "D8",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR9",
                    description: "Backup data register (BKP_DR)",
                    address_offset: 32,
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
                                name: "D9",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR10",
                    description: "Backup data register (BKP_DR)",
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
                                name: "D10",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR11",
                    description: "Backup data register (BKP_DR)",
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
                                name: "DR11",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR12",
                    description: "Backup data register (BKP_DR)",
                    address_offset: 64,
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
                                name: "DR12",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR13",
                    description: "Backup data register (BKP_DR)",
                    address_offset: 68,
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
                                name: "DR13",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR14",
                    description: "Backup data register (BKP_DR)",
                    address_offset: 72,
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
                                name: "D14",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR15",
                    description: "Backup data register (BKP_DR)",
                    address_offset: 76,
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
                                name: "D15",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR16",
                    description: "Backup data register (BKP_DR)",
                    address_offset: 80,
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
                                name: "D16",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR17",
                    description: "Backup data register (BKP_DR)",
                    address_offset: 84,
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
                                name: "D17",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR18",
                    description: "Backup data register (BKP_DR)",
                    address_offset: 88,
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
                                name: "D18",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR19",
                    description: "Backup data register (BKP_DR)",
                    address_offset: 92,
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
                                name: "D19",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR20",
                    description: "Backup data register (BKP_DR)",
                    address_offset: 96,
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
                                name: "D20",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR21",
                    description: "Backup data register (BKP_DR)",
                    address_offset: 100,
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
                                name: "D21",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR22",
                    description: "Backup data register (BKP_DR)",
                    address_offset: 104,
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
                                name: "D22",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR23",
                    description: "Backup data register (BKP_DR)",
                    address_offset: 108,
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
                                name: "D23",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR24",
                    description: "Backup data register (BKP_DR)",
                    address_offset: 112,
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
                                name: "D24",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR25",
                    description: "Backup data register (BKP_DR)",
                    address_offset: 116,
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
                                name: "D25",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR26",
                    description: "Backup data register (BKP_DR)",
                    address_offset: 120,
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
                                name: "D26",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR27",
                    description: "Backup data register (BKP_DR)",
                    address_offset: 124,
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
                                name: "D27",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR28",
                    description: "Backup data register (BKP_DR)",
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
                                name: "D28",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR29",
                    description: "Backup data register (BKP_DR)",
                    address_offset: 132,
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
                                name: "D29",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR30",
                    description: "Backup data register (BKP_DR)",
                    address_offset: 136,
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
                                name: "D30",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR31",
                    description: "Backup data register (BKP_DR)",
                    address_offset: 140,
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
                                name: "D31",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR32",
                    description: "Backup data register (BKP_DR)",
                    address_offset: 144,
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
                                name: "D32",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR33",
                    description: "Backup data register (BKP_DR)",
                    address_offset: 148,
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
                                name: "D33",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR34",
                    description: "Backup data register (BKP_DR)",
                    address_offset: 152,
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
                                name: "D34",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR35",
                    description: "Backup data register (BKP_DR)",
                    address_offset: 156,
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
                                name: "D35",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR36",
                    description: "Backup data register (BKP_DR)",
                    address_offset: 160,
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
                                name: "D36",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR37",
                    description: "Backup data register (BKP_DR)",
                    address_offset: 164,
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
                                name: "D37",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR38",
                    description: "Backup data register (BKP_DR)",
                    address_offset: 168,
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
                                name: "D38",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR39",
                    description: "Backup data register (BKP_DR)",
                    address_offset: 172,
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
                                name: "D39",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR40",
                    description: "Backup data register (BKP_DR)",
                    address_offset: 176,
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
                                name: "D40",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR41",
                    description: "Backup data register (BKP_DR)",
                    address_offset: 180,
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
                                name: "D41",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "DR42",
                    description: "Backup data register (BKP_DR)",
                    address_offset: 184,
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
                                name: "D42",
                                description: Some(
                                    "Backup data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 16
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
                    name: "RTCCR",
                    description: "RTC clock calibration register\r\n          (BKP_RTCCR)",
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
                                name: "CAL",
                                description: Some(
                                    "Calibration value"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 7
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CCO",
                                description: Some(
                                    "Calibration Clock Output"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ASOE",
                                description: Some(
                                    "Alarm or second output\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ASOS",
                                description: Some(
                                    "Alarm or second output\r\n              selection"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
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
                    name: "CR",
                    description: "Backup control register\r\n          (BKP_CR)",
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
                                name: "TPE",
                                description: Some(
                                    "Tamper pin enable"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TPAL",
                                description: Some(
                                    "Tamper pin active level"
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
                    name: "CSR",
                    description: "BKP_CSR control/status register\r\n          (BKP_CSR)",
                    address_offset: 48,
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
                                name: "CTE",
                                description: Some(
                                    "Clear Tamper event"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: Some(
                                    WriteOnly
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "CTI",
                                description: Some(
                                    "Clear Tamper Interrupt"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: Some(
                                    WriteOnly
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "TPIE",
                                description: Some(
                                    "Tamper Pin interrupt\r\n              enable"
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
                                name: "TEF",
                                description: Some(
                                    "Tamper Event Flag"
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
                                name: "TIF",
                                description: Some(
                                    "Tamper Interrupt Flag"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
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
            )
        ]
    ),
    derived_from: None
}
*/
