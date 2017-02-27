// ADC3
// Analog to digital converter
pub const ADDRESS: u32 = 0x40013C00;
pub mod SR {
}
pub mod CR1 {
}
pub mod CR2 {
}
pub mod SMPR1 {
}
pub mod SMPR2 {
}
pub mod JOFR1 {
}
pub mod JOFR2 {
}
pub mod JOFR3 {
}
pub mod JOFR4 {
}
pub mod HTR {
}
pub mod LTR {
}
pub mod SQR1 {
}
pub mod SQR2 {
}
pub mod SQR3 {
}
pub mod JSQR {
}
pub mod JDR1 {
}
pub mod JDR2 {
}
pub mod JDR3 {
}
pub mod JDR4 {
}
pub mod DR {
}
/*
Peripheral {
    name: "ADC3",
    group_name: Some(
        "ADC"
    ),
    description: Some(
        "Analog to digital converter"
    ),
    base_address: 1073822720,
    interrupt: Some(
        Interrupt {
            name: "ADC3",
            description: Some(
                "ADC3 global interrupt"
            ),
            value: 47
        }
    ),
    registers: Some(
        [
            Single(
                RegisterInfo {
                    name: "SR",
                    description: "status register",
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
                                name: "STRT",
                                description: Some(
                                    "Regular channel start flag"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "JSTRT",
                                description: Some(
                                    "Injected channel start\r\n              flag"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "JEOC",
                                description: Some(
                                    "Injected channel end of\r\n              conversion"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EOC",
                                description: Some(
                                    "Regular channel end of\r\n              conversion"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "AWD",
                                description: Some(
                                    "Analog watchdog flag"
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
                    name: "CR1",
                    description: "control register 1",
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
                                name: "AWDEN",
                                description: Some(
                                    "Analog watchdog enable on regular\r\n              channels"
                                ),
                                bit_range: BitRange {
                                    offset: 23,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "JAWDEN",
                                description: Some(
                                    "Analog watchdog enable on injected\r\n              channels"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DISCNUM",
                                description: Some(
                                    "Discontinuous mode channel\r\n              count"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 3
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "JDISCEN",
                                description: Some(
                                    "Discontinuous mode on injected\r\n              channels"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DISCEN",
                                description: Some(
                                    "Discontinuous mode on regular\r\n              channels"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "JAUTO",
                                description: Some(
                                    "Automatic injected group\r\n              conversion"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "AWDSGL",
                                description: Some(
                                    "Enable the watchdog on a single channel\r\n              in scan mode"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SCAN",
                                description: Some(
                                    "Scan mode"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "JEOCIE",
                                description: Some(
                                    "Interrupt enable for injected\r\n              channels"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "AWDIE",
                                description: Some(
                                    "Analog watchdog interrupt\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EOCIE",
                                description: Some(
                                    "Interrupt enable for EOC"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "AWDCH",
                                description: Some(
                                    "Analog watchdog channel select\r\n              bits"
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
                    name: "CR2",
                    description: "control register 2",
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
                                name: "TSVREFE",
                                description: Some(
                                    "Temperature sensor and VREFINT\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 23,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SWSTART",
                                description: Some(
                                    "Start conversion of regular\r\n              channels"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "JSWSTART",
                                description: Some(
                                    "Start conversion of injected\r\n              channels"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EXTTRIG",
                                description: Some(
                                    "External trigger conversion mode for\r\n              regular channels"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EXTSEL",
                                description: Some(
                                    "External event select for regular\r\n              group"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 3
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "JEXTTRIG",
                                description: Some(
                                    "External trigger conversion mode for\r\n              injected channels"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "JEXTSEL",
                                description: Some(
                                    "External event select for injected\r\n              group"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 3
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ALIGN",
                                description: Some(
                                    "Data alignment"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DMA",
                                description: Some(
                                    "Direct memory access mode"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "RSTCAL",
                                description: Some(
                                    "Reset calibration"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CAL",
                                description: Some(
                                    "A/D calibration"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CONT",
                                description: Some(
                                    "Continuous conversion"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ADON",
                                description: Some(
                                    "A/D converter ON / OFF"
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
                    name: "SMPR1",
                    description: "sample time register 1",
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
                                name: "SMP10",
                                description: Some(
                                    "Channel 10 sample time\r\n              selection"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 3
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SMP11",
                                description: Some(
                                    "Channel 11 sample time\r\n              selection"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 3
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SMP12",
                                description: Some(
                                    "Channel 12 sample time\r\n              selection"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 3
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SMP13",
                                description: Some(
                                    "Channel 13 sample time\r\n              selection"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 3
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SMP14",
                                description: Some(
                                    "Channel 14 sample time\r\n              selection"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 3
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SMP15",
                                description: Some(
                                    "Channel 15 sample time\r\n              selection"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 3
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SMP16",
                                description: Some(
                                    "Channel 16 sample time\r\n              selection"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 3
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SMP17",
                                description: Some(
                                    "Channel 17 sample time\r\n              selection"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
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
                    name: "SMPR2",
                    description: "sample time register 2",
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
                                name: "SMP0",
                                description: Some(
                                    "Channel 0 sample time\r\n              selection"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 3
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SMP1",
                                description: Some(
                                    "Channel 1 sample time\r\n              selection"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 3
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SMP2",
                                description: Some(
                                    "Channel 2 sample time\r\n              selection"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 3
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SMP3",
                                description: Some(
                                    "Channel 3 sample time\r\n              selection"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 3
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SMP4",
                                description: Some(
                                    "Channel 4 sample time\r\n              selection"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 3
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SMP5",
                                description: Some(
                                    "Channel 5 sample time\r\n              selection"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 3
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SMP6",
                                description: Some(
                                    "Channel 6 sample time\r\n              selection"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 3
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SMP7",
                                description: Some(
                                    "Channel 7 sample time\r\n              selection"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 3
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SMP8",
                                description: Some(
                                    "Channel 8 sample time\r\n              selection"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 3
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SMP9",
                                description: Some(
                                    "Channel 9 sample time\r\n              selection"
                                ),
                                bit_range: BitRange {
                                    offset: 27,
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
                    name: "JOFR1",
                    description: "injected channel data offset register\r\n          x",
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
                                name: "JOFFSET1",
                                description: Some(
                                    "Data offset for injected channel\r\n              x"
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
                    name: "JOFR2",
                    description: "injected channel data offset register\r\n          x",
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
                                name: "JOFFSET2",
                                description: Some(
                                    "Data offset for injected channel\r\n              x"
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
                    name: "JOFR3",
                    description: "injected channel data offset register\r\n          x",
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
                                name: "JOFFSET3",
                                description: Some(
                                    "Data offset for injected channel\r\n              x"
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
                    name: "JOFR4",
                    description: "injected channel data offset register\r\n          x",
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
                                name: "JOFFSET4",
                                description: Some(
                                    "Data offset for injected channel\r\n              x"
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
                    name: "HTR",
                    description: "watchdog higher threshold\r\n          register",
                    address_offset: 36,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        4095
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "HT",
                                description: Some(
                                    "Analog watchdog higher\r\n              threshold"
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
                    name: "LTR",
                    description: "watchdog lower threshold\r\n          register",
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
                                name: "LT",
                                description: Some(
                                    "Analog watchdog lower\r\n              threshold"
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
                    name: "SQR1",
                    description: "regular sequence register 1",
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
                                name: "L",
                                description: Some(
                                    "Regular channel sequence\r\n              length"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SQ16",
                                description: Some(
                                    "16th conversion in regular\r\n              sequence"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 5
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SQ15",
                                description: Some(
                                    "15th conversion in regular\r\n              sequence"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 5
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SQ14",
                                description: Some(
                                    "14th conversion in regular\r\n              sequence"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 5
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SQ13",
                                description: Some(
                                    "13th conversion in regular\r\n              sequence"
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
                    name: "SQR2",
                    description: "regular sequence register 2",
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
                                name: "SQ12",
                                description: Some(
                                    "12th conversion in regular\r\n              sequence"
                                ),
                                bit_range: BitRange {
                                    offset: 25,
                                    width: 5
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SQ11",
                                description: Some(
                                    "11th conversion in regular\r\n              sequence"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 5
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SQ10",
                                description: Some(
                                    "10th conversion in regular\r\n              sequence"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 5
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SQ9",
                                description: Some(
                                    "9th conversion in regular\r\n              sequence"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 5
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SQ8",
                                description: Some(
                                    "8th conversion in regular\r\n              sequence"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 5
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SQ7",
                                description: Some(
                                    "7th conversion in regular\r\n              sequence"
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
                    name: "SQR3",
                    description: "regular sequence register 3",
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
                                name: "SQ6",
                                description: Some(
                                    "6th conversion in regular\r\n              sequence"
                                ),
                                bit_range: BitRange {
                                    offset: 25,
                                    width: 5
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SQ5",
                                description: Some(
                                    "5th conversion in regular\r\n              sequence"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 5
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SQ4",
                                description: Some(
                                    "4th conversion in regular\r\n              sequence"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 5
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SQ3",
                                description: Some(
                                    "3rd conversion in regular\r\n              sequence"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 5
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SQ2",
                                description: Some(
                                    "2nd conversion in regular\r\n              sequence"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 5
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SQ1",
                                description: Some(
                                    "1st conversion in regular\r\n              sequence"
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
                    name: "JSQR",
                    description: "injected sequence register",
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
                                name: "JL",
                                description: Some(
                                    "Injected sequence length"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "JSQ4",
                                description: Some(
                                    "4th conversion in injected\r\n              sequence"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 5
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "JSQ3",
                                description: Some(
                                    "3rd conversion in injected\r\n              sequence"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 5
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "JSQ2",
                                description: Some(
                                    "2nd conversion in injected\r\n              sequence"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 5
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "JSQ1",
                                description: Some(
                                    "1st conversion in injected\r\n              sequence"
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
                    name: "JDR1",
                    description: "injected data register x",
                    address_offset: 60,
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
                                name: "JDATA",
                                description: Some(
                                    "Injected data"
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
                    name: "JDR2",
                    description: "injected data register x",
                    address_offset: 64,
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
                                name: "JDATA",
                                description: Some(
                                    "Injected data"
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
                    name: "JDR3",
                    description: "injected data register x",
                    address_offset: 68,
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
                                name: "JDATA",
                                description: Some(
                                    "Injected data"
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
                    name: "JDR4",
                    description: "injected data register x",
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
                                name: "JDATA",
                                description: Some(
                                    "Injected data"
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
                    name: "DR",
                    description: "regular data register",
                    address_offset: 76,
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
                                name: "DATA",
                                description: Some(
                                    "Regular data"
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
    derived_from: Some(
        "ADC2"
    )
}
*/
