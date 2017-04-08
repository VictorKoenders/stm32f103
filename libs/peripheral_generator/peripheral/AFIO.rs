// AFIO
// Alternate function I/O
pub const ADDRESS: u32 = 0x40010000;
/*
Peripheral {
    name: "AFIO",
    group_name: Some(
        "AFIO"
    ),
    description: Some(
        "Alternate function I/O"
    ),
    base_address: 1073807360,
    interrupt: None,
    registers: Some(
        [
            Single(
                RegisterInfo {
                    name: "EVCR",
                    description: "Event Control Register\r\n          (AFIO_EVCR)",
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
                                name: "PIN",
                                description: Some(
                                    "Pin selection"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PORT",
                                description: Some(
                                    "Port selection"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 3
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EVOE",
                                description: Some(
                                    "Event Output Enable"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
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
                    name: "MAPR",
                    description: "AF remap and debug I/O configuration\r\n          register (AFIO_MAPR)",
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
                                name: "SPI1_REMAP",
                                description: Some(
                                    "SPI1 remapping"
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
                                name: "I2C1_REMAP",
                                description: Some(
                                    "I2C1 remapping"
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
                                name: "USART1_REMAP",
                                description: Some(
                                    "USART1 remapping"
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
                                name: "USART2_REMAP",
                                description: Some(
                                    "USART2 remapping"
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
                                name: "USART3_REMAP",
                                description: Some(
                                    "USART3 remapping"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 2
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "TIM1_REMAP",
                                description: Some(
                                    "TIM1 remapping"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 2
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "TIM2_REMAP",
                                description: Some(
                                    "TIM2 remapping"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 2
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "TIM3_REMAP",
                                description: Some(
                                    "TIM3 remapping"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 2
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "TIM4_REMAP",
                                description: Some(
                                    "TIM4 remapping"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "CAN_REMAP",
                                description: Some(
                                    "CAN1 remapping"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 2
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "PD01_REMAP",
                                description: Some(
                                    "Port D0/Port D1 mapping on\r\n              OSCIN/OSCOUT"
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
                                name: "TIM5CH4_IREMAP",
                                description: Some(
                                    "Set and cleared by\r\n              software"
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
                                name: "ADC1_ETRGINJ_REMAP",
                                description: Some(
                                    "ADC 1 External trigger injected\r\n              conversion remapping"
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
                                name: "ADC1_ETRGREG_REMAP",
                                description: Some(
                                    "ADC 1 external trigger regular\r\n              conversion remapping"
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
                                name: "ADC2_ETRGINJ_REMAP",
                                description: Some(
                                    "ADC 2 external trigger injected\r\n              conversion remapping"
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
                                name: "ADC2_ETRGREG_REMAP",
                                description: Some(
                                    "ADC 2 external trigger regular\r\n              conversion remapping"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "SWJ_CFG",
                                description: Some(
                                    "Serial wire JTAG\r\n              configuration"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 3
                                },
                                access: Some(
                                    WriteOnly
                                ),
                                enumerated_values: []
                            }
                        ]
                    )
                }
            ),
            Single(
                RegisterInfo {
                    name: "EXTICR1",
                    description: "External interrupt configuration register 1\r\n          (AFIO_EXTICR1)",
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
                                name: "EXTI0",
                                description: Some(
                                    "EXTI0 configuration"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EXTI1",
                                description: Some(
                                    "EXTI1 configuration"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EXTI2",
                                description: Some(
                                    "EXTI2 configuration"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EXTI3",
                                description: Some(
                                    "EXTI3 configuration"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
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
                    name: "EXTICR2",
                    description: "External interrupt configuration register 2\r\n          (AFIO_EXTICR2)",
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
                                name: "EXTI4",
                                description: Some(
                                    "EXTI4 configuration"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EXTI5",
                                description: Some(
                                    "EXTI5 configuration"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EXTI6",
                                description: Some(
                                    "EXTI6 configuration"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EXTI7",
                                description: Some(
                                    "EXTI7 configuration"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
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
                    name: "EXTICR3",
                    description: "External interrupt configuration register 3\r\n          (AFIO_EXTICR3)",
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
                                name: "EXTI8",
                                description: Some(
                                    "EXTI8 configuration"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EXTI9",
                                description: Some(
                                    "EXTI9 configuration"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EXTI10",
                                description: Some(
                                    "EXTI10 configuration"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EXTI11",
                                description: Some(
                                    "EXTI11 configuration"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
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
                    name: "EXTICR4",
                    description: "External interrupt configuration register 4\r\n          (AFIO_EXTICR4)",
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
                                name: "EXTI12",
                                description: Some(
                                    "EXTI12 configuration"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EXTI13",
                                description: Some(
                                    "EXTI13 configuration"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EXTI14",
                                description: Some(
                                    "EXTI14 configuration"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EXTI15",
                                description: Some(
                                    "EXTI15 configuration"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
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
                    name: "MAPR2",
                    description: "AF remap and debug I/O configuration\r\n          register",
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
                                name: "TIM9_REMAP",
                                description: Some(
                                    "TIM9 remapping"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TIM10_REMAP",
                                description: Some(
                                    "TIM10 remapping"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TIM11_REMAP",
                                description: Some(
                                    "TIM11 remapping"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TIM13_REMAP",
                                description: Some(
                                    "TIM13 remapping"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TIM14_REMAP",
                                description: Some(
                                    "TIM14 remapping"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FSMC_NADV",
                                description: Some(
                                    "NADV connect/disconnect"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
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
