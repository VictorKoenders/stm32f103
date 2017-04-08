// EXTI
// EXTI
pub const ADDRESS: u32 = 0x40010400;
/*
Peripheral {
    name: "EXTI",
    group_name: Some(
        "EXTI"
    ),
    description: Some(
        "EXTI"
    ),
    base_address: 1073808384,
    interrupt: Some(
        Interrupt {
            name: "TAMPER",
            description: Some(
                "Tamper interrupt"
            ),
            value: 2
        }
    ),
    registers: Some(
        [
            Single(
                RegisterInfo {
                    name: "IMR",
                    description: "Interrupt mask register\r\n          (EXTI_IMR)",
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
                                name: "MR0",
                                description: Some(
                                    "Interrupt Mask on line 0"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MR1",
                                description: Some(
                                    "Interrupt Mask on line 1"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MR2",
                                description: Some(
                                    "Interrupt Mask on line 2"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MR3",
                                description: Some(
                                    "Interrupt Mask on line 3"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MR4",
                                description: Some(
                                    "Interrupt Mask on line 4"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MR5",
                                description: Some(
                                    "Interrupt Mask on line 5"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MR6",
                                description: Some(
                                    "Interrupt Mask on line 6"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MR7",
                                description: Some(
                                    "Interrupt Mask on line 7"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MR8",
                                description: Some(
                                    "Interrupt Mask on line 8"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MR9",
                                description: Some(
                                    "Interrupt Mask on line 9"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MR10",
                                description: Some(
                                    "Interrupt Mask on line 10"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MR11",
                                description: Some(
                                    "Interrupt Mask on line 11"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MR12",
                                description: Some(
                                    "Interrupt Mask on line 12"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MR13",
                                description: Some(
                                    "Interrupt Mask on line 13"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MR14",
                                description: Some(
                                    "Interrupt Mask on line 14"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MR15",
                                description: Some(
                                    "Interrupt Mask on line 15"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MR16",
                                description: Some(
                                    "Interrupt Mask on line 16"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MR17",
                                description: Some(
                                    "Interrupt Mask on line 17"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MR18",
                                description: Some(
                                    "Interrupt Mask on line 18"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
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
                    name: "EMR",
                    description: "Event mask register (EXTI_EMR)",
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
                                name: "MR0",
                                description: Some(
                                    "Event Mask on line 0"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MR1",
                                description: Some(
                                    "Event Mask on line 1"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MR2",
                                description: Some(
                                    "Event Mask on line 2"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MR3",
                                description: Some(
                                    "Event Mask on line 3"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MR4",
                                description: Some(
                                    "Event Mask on line 4"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MR5",
                                description: Some(
                                    "Event Mask on line 5"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MR6",
                                description: Some(
                                    "Event Mask on line 6"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MR7",
                                description: Some(
                                    "Event Mask on line 7"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MR8",
                                description: Some(
                                    "Event Mask on line 8"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MR9",
                                description: Some(
                                    "Event Mask on line 9"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MR10",
                                description: Some(
                                    "Event Mask on line 10"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MR11",
                                description: Some(
                                    "Event Mask on line 11"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MR12",
                                description: Some(
                                    "Event Mask on line 12"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MR13",
                                description: Some(
                                    "Event Mask on line 13"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MR14",
                                description: Some(
                                    "Event Mask on line 14"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MR15",
                                description: Some(
                                    "Event Mask on line 15"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MR16",
                                description: Some(
                                    "Event Mask on line 16"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MR17",
                                description: Some(
                                    "Event Mask on line 17"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MR18",
                                description: Some(
                                    "Event Mask on line 18"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
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
                    name: "RTSR",
                    description: "Rising Trigger selection register\r\n          (EXTI_RTSR)",
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
                                name: "TR0",
                                description: Some(
                                    "Rising trigger event configuration of\r\n              line 0"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TR1",
                                description: Some(
                                    "Rising trigger event configuration of\r\n              line 1"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TR2",
                                description: Some(
                                    "Rising trigger event configuration of\r\n              line 2"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TR3",
                                description: Some(
                                    "Rising trigger event configuration of\r\n              line 3"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TR4",
                                description: Some(
                                    "Rising trigger event configuration of\r\n              line 4"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TR5",
                                description: Some(
                                    "Rising trigger event configuration of\r\n              line 5"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TR6",
                                description: Some(
                                    "Rising trigger event configuration of\r\n              line 6"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TR7",
                                description: Some(
                                    "Rising trigger event configuration of\r\n              line 7"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TR8",
                                description: Some(
                                    "Rising trigger event configuration of\r\n              line 8"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TR9",
                                description: Some(
                                    "Rising trigger event configuration of\r\n              line 9"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TR10",
                                description: Some(
                                    "Rising trigger event configuration of\r\n              line 10"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TR11",
                                description: Some(
                                    "Rising trigger event configuration of\r\n              line 11"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TR12",
                                description: Some(
                                    "Rising trigger event configuration of\r\n              line 12"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TR13",
                                description: Some(
                                    "Rising trigger event configuration of\r\n              line 13"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TR14",
                                description: Some(
                                    "Rising trigger event configuration of\r\n              line 14"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TR15",
                                description: Some(
                                    "Rising trigger event configuration of\r\n              line 15"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TR16",
                                description: Some(
                                    "Rising trigger event configuration of\r\n              line 16"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TR17",
                                description: Some(
                                    "Rising trigger event configuration of\r\n              line 17"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TR18",
                                description: Some(
                                    "Rising trigger event configuration of\r\n              line 18"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
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
                    name: "FTSR",
                    description: "Falling Trigger selection register\r\n          (EXTI_FTSR)",
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
                                name: "TR0",
                                description: Some(
                                    "Falling trigger event configuration of\r\n              line 0"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TR1",
                                description: Some(
                                    "Falling trigger event configuration of\r\n              line 1"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TR2",
                                description: Some(
                                    "Falling trigger event configuration of\r\n              line 2"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TR3",
                                description: Some(
                                    "Falling trigger event configuration of\r\n              line 3"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TR4",
                                description: Some(
                                    "Falling trigger event configuration of\r\n              line 4"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TR5",
                                description: Some(
                                    "Falling trigger event configuration of\r\n              line 5"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TR6",
                                description: Some(
                                    "Falling trigger event configuration of\r\n              line 6"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TR7",
                                description: Some(
                                    "Falling trigger event configuration of\r\n              line 7"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TR8",
                                description: Some(
                                    "Falling trigger event configuration of\r\n              line 8"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TR9",
                                description: Some(
                                    "Falling trigger event configuration of\r\n              line 9"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TR10",
                                description: Some(
                                    "Falling trigger event configuration of\r\n              line 10"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TR11",
                                description: Some(
                                    "Falling trigger event configuration of\r\n              line 11"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TR12",
                                description: Some(
                                    "Falling trigger event configuration of\r\n              line 12"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TR13",
                                description: Some(
                                    "Falling trigger event configuration of\r\n              line 13"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TR14",
                                description: Some(
                                    "Falling trigger event configuration of\r\n              line 14"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TR15",
                                description: Some(
                                    "Falling trigger event configuration of\r\n              line 15"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TR16",
                                description: Some(
                                    "Falling trigger event configuration of\r\n              line 16"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TR17",
                                description: Some(
                                    "Falling trigger event configuration of\r\n              line 17"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TR18",
                                description: Some(
                                    "Falling trigger event configuration of\r\n              line 18"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
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
                    name: "SWIER",
                    description: "Software interrupt event register\r\n          (EXTI_SWIER)",
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
                                name: "SWIER0",
                                description: Some(
                                    "Software Interrupt on line\r\n              0"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SWIER1",
                                description: Some(
                                    "Software Interrupt on line\r\n              1"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SWIER2",
                                description: Some(
                                    "Software Interrupt on line\r\n              2"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SWIER3",
                                description: Some(
                                    "Software Interrupt on line\r\n              3"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SWIER4",
                                description: Some(
                                    "Software Interrupt on line\r\n              4"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SWIER5",
                                description: Some(
                                    "Software Interrupt on line\r\n              5"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SWIER6",
                                description: Some(
                                    "Software Interrupt on line\r\n              6"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SWIER7",
                                description: Some(
                                    "Software Interrupt on line\r\n              7"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SWIER8",
                                description: Some(
                                    "Software Interrupt on line\r\n              8"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SWIER9",
                                description: Some(
                                    "Software Interrupt on line\r\n              9"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SWIER10",
                                description: Some(
                                    "Software Interrupt on line\r\n              10"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SWIER11",
                                description: Some(
                                    "Software Interrupt on line\r\n              11"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SWIER12",
                                description: Some(
                                    "Software Interrupt on line\r\n              12"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SWIER13",
                                description: Some(
                                    "Software Interrupt on line\r\n              13"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SWIER14",
                                description: Some(
                                    "Software Interrupt on line\r\n              14"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SWIER15",
                                description: Some(
                                    "Software Interrupt on line\r\n              15"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SWIER16",
                                description: Some(
                                    "Software Interrupt on line\r\n              16"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SWIER17",
                                description: Some(
                                    "Software Interrupt on line\r\n              17"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SWIER18",
                                description: Some(
                                    "Software Interrupt on line\r\n              18"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
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
                    name: "PR",
                    description: "Pending register (EXTI_PR)",
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
                                name: "PR0",
                                description: Some(
                                    "Pending bit 0"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PR1",
                                description: Some(
                                    "Pending bit 1"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PR2",
                                description: Some(
                                    "Pending bit 2"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PR3",
                                description: Some(
                                    "Pending bit 3"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PR4",
                                description: Some(
                                    "Pending bit 4"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PR5",
                                description: Some(
                                    "Pending bit 5"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PR6",
                                description: Some(
                                    "Pending bit 6"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PR7",
                                description: Some(
                                    "Pending bit 7"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PR8",
                                description: Some(
                                    "Pending bit 8"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PR9",
                                description: Some(
                                    "Pending bit 9"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PR10",
                                description: Some(
                                    "Pending bit 10"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PR11",
                                description: Some(
                                    "Pending bit 11"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PR12",
                                description: Some(
                                    "Pending bit 12"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PR13",
                                description: Some(
                                    "Pending bit 13"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PR14",
                                description: Some(
                                    "Pending bit 14"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PR15",
                                description: Some(
                                    "Pending bit 15"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PR16",
                                description: Some(
                                    "Pending bit 16"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PR17",
                                description: Some(
                                    "Pending bit 17"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PR18",
                                description: Some(
                                    "Pending bit 18"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
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
