// TIM1
// Advanced timer
pub const ADDRESS: u32 = 0x40012C00;
/*
Peripheral {
    name: "TIM1",
    group_name: Some(
        "TIM"
    ),
    description: Some(
        "Advanced timer"
    ),
    base_address: 1073818624,
    interrupt: Some(
        Interrupt {
            name: "TIM1_BRK_TIM9",
            description: Some(
                "TIM1 Break interrupt and TIM9 global\r\n        interrupt"
            ),
            value: 24
        }
    ),
    registers: Some(
        [
            Single(
                RegisterInfo {
                    name: "CR1",
                    description: "control register 1",
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
                                name: "CKD",
                                description: Some(
                                    "Clock division"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ARPE",
                                description: Some(
                                    "Auto-reload preload enable"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CMS",
                                description: Some(
                                    "Center-aligned mode\r\n              selection"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DIR",
                                description: Some(
                                    "Direction"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "OPM",
                                description: Some(
                                    "One-pulse mode"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "URS",
                                description: Some(
                                    "Update request source"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "UDIS",
                                description: Some(
                                    "Update disable"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CEN",
                                description: Some(
                                    "Counter enable"
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
                    name: "CR2",
                    description: "control register 2",
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
                                name: "OIS4",
                                description: Some(
                                    "Output Idle state 4"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "OIS3N",
                                description: Some(
                                    "Output Idle state 3"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "OIS3",
                                description: Some(
                                    "Output Idle state 3"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "OIS2N",
                                description: Some(
                                    "Output Idle state 2"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "OIS2",
                                description: Some(
                                    "Output Idle state 2"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "OIS1N",
                                description: Some(
                                    "Output Idle state 1"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "OIS1",
                                description: Some(
                                    "Output Idle state 1"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TI1S",
                                description: Some(
                                    "TI1 selection"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MMS",
                                description: Some(
                                    "Master mode selection"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 3
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CCDS",
                                description: Some(
                                    "Capture/compare DMA\r\n              selection"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CCUS",
                                description: Some(
                                    "Capture/compare control update\r\n              selection"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CCPC",
                                description: Some(
                                    "Capture/compare preloaded\r\n              control"
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
                    name: "SMCR",
                    description: "slave mode control register",
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
                                name: "ETP",
                                description: Some(
                                    "External trigger polarity"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ECE",
                                description: Some(
                                    "External clock enable"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ETPS",
                                description: Some(
                                    "External trigger prescaler"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ETF",
                                description: Some(
                                    "External trigger filter"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MSM",
                                description: Some(
                                    "Master/Slave mode"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TS",
                                description: Some(
                                    "Trigger selection"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 3
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SMS",
                                description: Some(
                                    "Slave mode selection"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 3
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
                    name: "DIER",
                    description: "DMA/Interrupt enable register",
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
                                name: "TDE",
                                description: Some(
                                    "Trigger DMA request enable"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "COMDE",
                                description: Some(
                                    "COM DMA request enable"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CC4DE",
                                description: Some(
                                    "Capture/Compare 4 DMA request\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CC3DE",
                                description: Some(
                                    "Capture/Compare 3 DMA request\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CC2DE",
                                description: Some(
                                    "Capture/Compare 2 DMA request\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CC1DE",
                                description: Some(
                                    "Capture/Compare 1 DMA request\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "UDE",
                                description: Some(
                                    "Update DMA request enable"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TIE",
                                description: Some(
                                    "Trigger interrupt enable"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CC4IE",
                                description: Some(
                                    "Capture/Compare 4 interrupt\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CC3IE",
                                description: Some(
                                    "Capture/Compare 3 interrupt\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CC2IE",
                                description: Some(
                                    "Capture/Compare 2 interrupt\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CC1IE",
                                description: Some(
                                    "Capture/Compare 1 interrupt\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "UIE",
                                description: Some(
                                    "Update interrupt enable"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BIE",
                                description: Some(
                                    "Break interrupt enable"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "COMIE",
                                description: Some(
                                    "COM interrupt enable"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
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
                    name: "SR",
                    description: "status register",
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
                                name: "CC4OF",
                                description: Some(
                                    "Capture/Compare 4 overcapture\r\n              flag"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CC3OF",
                                description: Some(
                                    "Capture/Compare 3 overcapture\r\n              flag"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CC2OF",
                                description: Some(
                                    "Capture/compare 2 overcapture\r\n              flag"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CC1OF",
                                description: Some(
                                    "Capture/Compare 1 overcapture\r\n              flag"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BIF",
                                description: Some(
                                    "Break interrupt flag"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TIF",
                                description: Some(
                                    "Trigger interrupt flag"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "COMIF",
                                description: Some(
                                    "COM interrupt flag"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CC4IF",
                                description: Some(
                                    "Capture/Compare 4 interrupt\r\n              flag"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CC3IF",
                                description: Some(
                                    "Capture/Compare 3 interrupt\r\n              flag"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CC2IF",
                                description: Some(
                                    "Capture/Compare 2 interrupt\r\n              flag"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CC1IF",
                                description: Some(
                                    "Capture/compare 1 interrupt\r\n              flag"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "UIF",
                                description: Some(
                                    "Update interrupt flag"
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
                    name: "EGR",
                    description: "event generation register",
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
                                name: "BG",
                                description: Some(
                                    "Break generation"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TG",
                                description: Some(
                                    "Trigger generation"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "COMG",
                                description: Some(
                                    "Capture/Compare control update\r\n              generation"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CC4G",
                                description: Some(
                                    "Capture/compare 4\r\n              generation"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CC3G",
                                description: Some(
                                    "Capture/compare 3\r\n              generation"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CC2G",
                                description: Some(
                                    "Capture/compare 2\r\n              generation"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CC1G",
                                description: Some(
                                    "Capture/compare 1\r\n              generation"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "UG",
                                description: Some(
                                    "Update generation"
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
                    name: "CCMR1_Output",
                    description: "capture/compare mode register (output\r\n          mode)",
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
                                name: "OC2CE",
                                description: Some(
                                    "Output Compare 2 clear\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "OC2M",
                                description: Some(
                                    "Output Compare 2 mode"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 3
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "OC2PE",
                                description: Some(
                                    "Output Compare 2 preload\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "OC2FE",
                                description: Some(
                                    "Output Compare 2 fast\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CC2S",
                                description: Some(
                                    "Capture/Compare 2\r\n              selection"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "OC1CE",
                                description: Some(
                                    "Output Compare 1 clear\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "OC1M",
                                description: Some(
                                    "Output Compare 1 mode"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 3
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "OC1PE",
                                description: Some(
                                    "Output Compare 1 preload\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "OC1FE",
                                description: Some(
                                    "Output Compare 1 fast\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CC1S",
                                description: Some(
                                    "Capture/Compare 1\r\n              selection"
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
                    name: "CCMR1_Input",
                    description: "capture/compare mode register 1 (input\r\n          mode)",
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
                                name: "IC2F",
                                description: Some(
                                    "Input capture 2 filter"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IC2PCS",
                                description: Some(
                                    "Input capture 2 prescaler"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CC2S",
                                description: Some(
                                    "Capture/Compare 2\r\n              selection"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IC1F",
                                description: Some(
                                    "Input capture 1 filter"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ICPCS",
                                description: Some(
                                    "Input capture 1 prescaler"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CC1S",
                                description: Some(
                                    "Capture/Compare 1\r\n              selection"
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
                    name: "CCMR2_Output",
                    description: "capture/compare mode register (output\r\n          mode)",
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
                                name: "OC4CE",
                                description: Some(
                                    "Output compare 4 clear\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "OC4M",
                                description: Some(
                                    "Output compare 4 mode"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 3
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "OC4PE",
                                description: Some(
                                    "Output compare 4 preload\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "OC4FE",
                                description: Some(
                                    "Output compare 4 fast\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CC4S",
                                description: Some(
                                    "Capture/Compare 4\r\n              selection"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "OC3CE",
                                description: Some(
                                    "Output compare 3 clear\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "OC3M",
                                description: Some(
                                    "Output compare 3 mode"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 3
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "OC3PE",
                                description: Some(
                                    "Output compare 3 preload\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "OC3FE",
                                description: Some(
                                    "Output compare 3 fast\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CC3S",
                                description: Some(
                                    "Capture/Compare 3\r\n              selection"
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
                    name: "CCMR2_Input",
                    description: "capture/compare mode register 2 (input\r\n          mode)",
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
                                name: "IC4F",
                                description: Some(
                                    "Input capture 4 filter"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IC4PSC",
                                description: Some(
                                    "Input capture 4 prescaler"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CC4S",
                                description: Some(
                                    "Capture/Compare 4\r\n              selection"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IC3F",
                                description: Some(
                                    "Input capture 3 filter"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IC3PSC",
                                description: Some(
                                    "Input capture 3 prescaler"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CC3S",
                                description: Some(
                                    "Capture/compare 3\r\n              selection"
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
                    name: "CCER",
                    description: "capture/compare enable\r\n          register",
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
                                name: "CC4P",
                                description: Some(
                                    "Capture/Compare 3 output\r\n              Polarity"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CC4E",
                                description: Some(
                                    "Capture/Compare 4 output\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CC3NP",
                                description: Some(
                                    "Capture/Compare 3 output\r\n              Polarity"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CC3NE",
                                description: Some(
                                    "Capture/Compare 3 complementary output\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CC3P",
                                description: Some(
                                    "Capture/Compare 3 output\r\n              Polarity"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CC3E",
                                description: Some(
                                    "Capture/Compare 3 output\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CC2NP",
                                description: Some(
                                    "Capture/Compare 2 output\r\n              Polarity"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CC2NE",
                                description: Some(
                                    "Capture/Compare 2 complementary output\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CC2P",
                                description: Some(
                                    "Capture/Compare 2 output\r\n              Polarity"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CC2E",
                                description: Some(
                                    "Capture/Compare 2 output\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CC1NP",
                                description: Some(
                                    "Capture/Compare 1 output\r\n              Polarity"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CC1NE",
                                description: Some(
                                    "Capture/Compare 1 complementary output\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CC1P",
                                description: Some(
                                    "Capture/Compare 1 output\r\n              Polarity"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CC1E",
                                description: Some(
                                    "Capture/Compare 1 output\r\n              enable"
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
                    name: "CNT",
                    description: "counter",
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
                                name: "CNT",
                                description: Some(
                                    "counter value"
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
                    name: "PSC",
                    description: "prescaler",
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
                                name: "PSC",
                                description: Some(
                                    "Prescaler value"
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
                    name: "ARR",
                    description: "auto-reload register",
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
                                name: "ARR",
                                description: Some(
                                    "Auto-reload value"
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
                    name: "CCR1",
                    description: "capture/compare register 1",
                    address_offset: 52,
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
                                name: "CCR1",
                                description: Some(
                                    "Capture/Compare 1 value"
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
                    name: "CCR2",
                    description: "capture/compare register 2",
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
                                name: "CCR2",
                                description: Some(
                                    "Capture/Compare 2 value"
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
                    name: "CCR3",
                    description: "capture/compare register 3",
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
                                name: "CCR3",
                                description: Some(
                                    "Capture/Compare value"
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
                    name: "CCR4",
                    description: "capture/compare register 4",
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
                                name: "CCR4",
                                description: Some(
                                    "Capture/Compare value"
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
                    name: "DCR",
                    description: "DMA control register",
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
                                name: "DBL",
                                description: Some(
                                    "DMA burst length"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 5
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DBA",
                                description: Some(
                                    "DMA base address"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 5
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
                    name: "DMAR",
                    description: "DMA address for full transfer",
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
                                name: "DMAB",
                                description: Some(
                                    "DMA register for burst\r\n              accesses"
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
                    name: "RCR",
                    description: "repetition counter register",
                    address_offset: 48,
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
                                name: "REP",
                                description: Some(
                                    "Repetition counter value"
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
                    name: "BDTR",
                    description: "break and dead-time register",
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
                                name: "MOE",
                                description: Some(
                                    "Main output enable"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "AOE",
                                description: Some(
                                    "Automatic output enable"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BKP",
                                description: Some(
                                    "Break polarity"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BKE",
                                description: Some(
                                    "Break enable"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "OSSR",
                                description: Some(
                                    "Off-state selection for Run\r\n              mode"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "OSSI",
                                description: Some(
                                    "Off-state selection for Idle\r\n              mode"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "LOCK",
                                description: Some(
                                    "Lock configuration"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DTG",
                                description: Some(
                                    "Dead-time generator setup"
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
            )
        ]
    ),
    derived_from: None
}
*/
