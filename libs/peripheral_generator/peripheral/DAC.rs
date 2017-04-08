// DAC
// Digital to analog converter
pub const ADDRESS: u32 = 0x40007400;
/*
Peripheral {
    name: "DAC",
    group_name: Some(
        "DAC"
    ),
    description: Some(
        "Digital to analog converter"
    ),
    base_address: 1073771520,
    interrupt: None,
    registers: Some(
        [
            Single(
                RegisterInfo {
                    name: "CR",
                    description: "Control register (DAC_CR)",
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
                                name: "EN1",
                                description: Some(
                                    "DAC channel1 enable"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BOFF1",
                                description: Some(
                                    "DAC channel1 output buffer\r\n              disable"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TEN1",
                                description: Some(
                                    "DAC channel1 trigger\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TSEL1",
                                description: Some(
                                    "DAC channel1 trigger\r\n              selection"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 3
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "WAVE1",
                                description: Some(
                                    "DAC channel1 noise/triangle wave\r\n              generation enable"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MAMP1",
                                description: Some(
                                    "DAC channel1 mask/amplitude\r\n              selector"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DMAEN1",
                                description: Some(
                                    "DAC channel1 DMA enable"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EN2",
                                description: Some(
                                    "DAC channel2 enable"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BOFF2",
                                description: Some(
                                    "DAC channel2 output buffer\r\n              disable"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TEN2",
                                description: Some(
                                    "DAC channel2 trigger\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TSEL2",
                                description: Some(
                                    "DAC channel2 trigger\r\n              selection"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 3
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "WAVE2",
                                description: Some(
                                    "DAC channel2 noise/triangle wave\r\n              generation enable"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MAMP2",
                                description: Some(
                                    "DAC channel2 mask/amplitude\r\n              selector"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DMAEN2",
                                description: Some(
                                    "DAC channel2 DMA enable"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
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
                    name: "SWTRIGR",
                    description: "DAC software trigger register\r\n          (DAC_SWTRIGR)",
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
                                name: "SWTRIG1",
                                description: Some(
                                    "DAC channel1 software\r\n              trigger"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SWTRIG2",
                                description: Some(
                                    "DAC channel2 software\r\n              trigger"
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
                    name: "DHR12R1",
                    description: "DAC channel1 12-bit right-aligned data\r\n          holding register(DAC_DHR12R1)",
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
                                name: "DACC1DHR",
                                description: Some(
                                    "DAC channel1 12-bit right-aligned\r\n              data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 12
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
                    name: "DHR12L1",
                    description: "DAC channel1 12-bit left aligned data\r\n          holding register (DAC_DHR12L1)",
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
                                name: "DACC1DHR",
                                description: Some(
                                    "DAC channel1 12-bit left-aligned\r\n              data"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 12
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
                    name: "DHR8R1",
                    description: "DAC channel1 8-bit right aligned data\r\n          holding register (DAC_DHR8R1)",
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
                                name: "DACC1DHR",
                                description: Some(
                                    "DAC channel1 8-bit right-aligned\r\n              data"
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
                    name: "DHR12R2",
                    description: "DAC channel2 12-bit right aligned data\r\n          holding register (DAC_DHR12R2)",
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
                                name: "DACC2DHR",
                                description: Some(
                                    "DAC channel2 12-bit right-aligned\r\n              data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 12
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
                    name: "DHR12L2",
                    description: "DAC channel2 12-bit left aligned data\r\n          holding register (DAC_DHR12L2)",
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
                                name: "DACC2DHR",
                                description: Some(
                                    "DAC channel2 12-bit left-aligned\r\n              data"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 12
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
                    name: "DHR8R2",
                    description: "DAC channel2 8-bit right-aligned data\r\n          holding register (DAC_DHR8R2)",
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
                                name: "DACC2DHR",
                                description: Some(
                                    "DAC channel2 8-bit right-aligned\r\n              data"
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
                    name: "DHR12RD",
                    description: "Dual DAC 12-bit right-aligned data holding\r\n          register (DAC_DHR12RD), Bits 31:28 Reserved, Bits 15:12\r\n          Reserved",
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
                                name: "DACC1DHR",
                                description: Some(
                                    "DAC channel1 12-bit right-aligned\r\n              data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 12
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DACC2DHR",
                                description: Some(
                                    "DAC channel2 12-bit right-aligned\r\n              data"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 12
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
                    name: "DHR12LD",
                    description: "DUAL DAC 12-bit left aligned data holding\r\n          register (DAC_DHR12LD), Bits 19:16 Reserved, Bits 3:0\r\n          Reserved",
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
                                name: "DACC1DHR",
                                description: Some(
                                    "DAC channel1 12-bit left-aligned\r\n              data"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 12
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DACC2DHR",
                                description: Some(
                                    "DAC channel2 12-bit right-aligned\r\n              data"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 12
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
                    name: "DHR8RD",
                    description: "DUAL DAC 8-bit right aligned data holding\r\n          register (DAC_DHR8RD), Bits 31:16 Reserved",
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
                                name: "DACC1DHR",
                                description: Some(
                                    "DAC channel1 8-bit right-aligned\r\n              data"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DACC2DHR",
                                description: Some(
                                    "DAC channel2 8-bit right-aligned\r\n              data"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
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
                    name: "DOR1",
                    description: "DAC channel1 data output register\r\n          (DAC_DOR1)",
                    address_offset: 44,
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
                                name: "DACC1DOR",
                                description: Some(
                                    "DAC channel1 data output"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 12
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
                    name: "DOR2",
                    description: "DAC channel2 data output register\r\n          (DAC_DOR2)",
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
                                name: "DACC2DOR",
                                description: Some(
                                    "DAC channel2 data output"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 12
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
