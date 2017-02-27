// IWDG
// Independent watchdog
pub const ADDRESS: u32 = 0x40003000;
pub mod KR {
}
pub mod PR {
}
pub mod RLR {
}
pub mod SR {
}
/*
Peripheral {
    name: "IWDG",
    group_name: Some(
        "IWDG"
    ),
    description: Some(
        "Independent watchdog"
    ),
    base_address: 1073754112,
    interrupt: None,
    registers: Some(
        [
            Single(
                RegisterInfo {
                    name: "KR",
                    description: "Key register (IWDG_KR)",
                    address_offset: 0,
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
                                name: "KEY",
                                description: Some(
                                    "Key value"
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
                    name: "PR",
                    description: "Prescaler register (IWDG_PR)",
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
                                name: "PR",
                                description: Some(
                                    "Prescaler divider"
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
                    name: "RLR",
                    description: "Reload register (IWDG_RLR)",
                    address_offset: 8,
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
                                name: "RL",
                                description: Some(
                                    "Watchdog counter reload\r\n              value"
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
                    name: "SR",
                    description: "Status register (IWDG_SR)",
                    address_offset: 12,
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
                                name: "PVU",
                                description: Some(
                                    "Watchdog prescaler value\r\n              update"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "RVU",
                                description: Some(
                                    "Watchdog counter reload value\r\n              update"
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
            )
        ]
    ),
    derived_from: None
}
*/
