// DBG
// Debug support
pub const ADDRESS: u32 = 0xE0042000;
/*
Peripheral {
    name: "DBG",
    group_name: Some(
        "DBG"
    ),
    description: Some(
        "Debug support"
    ),
    base_address: 3758366720,
    interrupt: None,
    registers: Some(
        [
            Single(
                RegisterInfo {
                    name: "IDCODE",
                    description: "DBGMCU_IDCODE",
                    address_offset: 0,
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
                                name: "DEV_ID",
                                description: Some(
                                    "DEV_ID"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 12
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "REV_ID",
                                description: Some(
                                    "REV_ID"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
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
                    name: "CR",
                    description: "DBGMCU_CR",
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
                                name: "DBG_SLEEP",
                                description: Some(
                                    "DBG_SLEEP"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DBG_STOP",
                                description: Some(
                                    "DBG_STOP"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DBG_STANDBY",
                                description: Some(
                                    "DBG_STANDBY"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TRACE_IOEN",
                                description: Some(
                                    "TRACE_IOEN"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TRACE_MODE",
                                description: Some(
                                    "TRACE_MODE"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DBG_IWDG_STOP",
                                description: Some(
                                    "DBG_IWDG_STOP"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DBG_WWDG_STOP",
                                description: Some(
                                    "DBG_WWDG_STOP"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DBG_TIM1_STOP",
                                description: Some(
                                    "DBG_TIM1_STOP"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DBG_TIM2_STOP",
                                description: Some(
                                    "DBG_TIM2_STOP"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DBG_TIM3_STOP",
                                description: Some(
                                    "DBG_TIM3_STOP"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DBG_TIM4_STOP",
                                description: Some(
                                    "DBG_TIM4_STOP"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DBG_CAN1_STOP",
                                description: Some(
                                    "DBG_CAN1_STOP"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DBG_I2C1_SMBUS_TIMEOUT",
                                description: Some(
                                    "DBG_I2C1_SMBUS_TIMEOUT"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DBG_I2C2_SMBUS_TIMEOUT",
                                description: Some(
                                    "DBG_I2C2_SMBUS_TIMEOUT"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DBG_TIM8_STOP",
                                description: Some(
                                    "DBG_TIM8_STOP"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DBG_TIM5_STOP",
                                description: Some(
                                    "DBG_TIM5_STOP"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DBG_TIM6_STOP",
                                description: Some(
                                    "DBG_TIM6_STOP"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DBG_TIM7_STOP",
                                description: Some(
                                    "DBG_TIM7_STOP"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DBG_CAN2_STOP",
                                description: Some(
                                    "DBG_CAN2_STOP"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
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
