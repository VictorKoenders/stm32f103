// RTC
// Real time clock
pub const ADDRESS: u32 = 0x40002800;
/*
Peripheral {
    name: "RTC",
    group_name: Some(
        "RTC"
    ),
    description: Some(
        "Real time clock"
    ),
    base_address: 1073752064,
    interrupt: Some(
        Interrupt {
            name: "RTC",
            description: Some(
                "RTC global interrupt"
            ),
            value: 3
        }
    ),
    registers: Some(
        [
            Single(
                RegisterInfo {
                    name: "CRH",
                    description: "RTC Control Register High",
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
                                name: "SECIE",
                                description: Some(
                                    "Second interrupt Enable"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ALRIE",
                                description: Some(
                                    "Alarm interrupt Enable"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "OWIE",
                                description: Some(
                                    "Overflow interrupt Enable"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
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
                    name: "CRL",
                    description: "RTC Control Register Low",
                    address_offset: 4,
                    size: Some(
                        32
                    ),
                    access: None,
                    reset_value: Some(
                        32
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "SECF",
                                description: Some(
                                    "Second Flag"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "ALRF",
                                description: Some(
                                    "Alarm Flag"
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
                                name: "OWF",
                                description: Some(
                                    "Overflow Flag"
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
                                name: "RSF",
                                description: Some(
                                    "Registers Synchronized\r\n              Flag"
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
                                name: "CNF",
                                description: Some(
                                    "Configuration Flag"
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
                                name: "RTOFF",
                                description: Some(
                                    "RTC operation OFF"
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
                    name: "PRLH",
                    description: "RTC Prescaler Load Register\r\n          High",
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
                                name: "PRLH",
                                description: Some(
                                    "RTC Prescaler Load Register\r\n              High"
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
                    name: "PRLL",
                    description: "RTC Prescaler Load Register\r\n          Low",
                    address_offset: 12,
                    size: Some(
                        32
                    ),
                    access: Some(
                        WriteOnly
                    ),
                    reset_value: Some(
                        32768
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "PRLL",
                                description: Some(
                                    "RTC Prescaler Divider Register\r\n              Low"
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
                    name: "DIVH",
                    description: "RTC Prescaler Divider Register\r\n          High",
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
                                name: "DIVH",
                                description: Some(
                                    "RTC prescaler divider register\r\n              high"
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
                    name: "DIVL",
                    description: "RTC Prescaler Divider Register\r\n          Low",
                    address_offset: 20,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadOnly
                    ),
                    reset_value: Some(
                        32768
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "DIVL",
                                description: Some(
                                    "RTC prescaler divider register\r\n              Low"
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
                    name: "CNTH",
                    description: "RTC Counter Register High",
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
                                name: "CNTH",
                                description: Some(
                                    "RTC counter register high"
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
                    name: "CNTL",
                    description: "RTC Counter Register Low",
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
                                name: "CNTL",
                                description: Some(
                                    "RTC counter register Low"
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
                    name: "ALRH",
                    description: "RTC Alarm Register High",
                    address_offset: 32,
                    size: Some(
                        32
                    ),
                    access: Some(
                        WriteOnly
                    ),
                    reset_value: Some(
                        65535
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "ALRH",
                                description: Some(
                                    "RTC alarm register high"
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
                    name: "ALRL",
                    description: "RTC Alarm Register Low",
                    address_offset: 36,
                    size: Some(
                        32
                    ),
                    access: Some(
                        WriteOnly
                    ),
                    reset_value: Some(
                        65535
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "ALRL",
                                description: Some(
                                    "RTC alarm register low"
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
            )
        ]
    ),
    derived_from: None
}
*/
