// WWDG
// Window watchdog
pub const ADDRESS: u32 = 0x40002C00;
/*
Peripheral {
    name: "WWDG",
    group_name: Some(
        "WWDG"
    ),
    description: Some(
        "Window watchdog"
    ),
    base_address: 1073753088,
    interrupt: Some(
        Interrupt {
            name: "WWDG",
            description: Some(
                "Window Watchdog interrupt"
            ),
            value: 0
        }
    ),
    registers: Some(
        [
            Single(
                RegisterInfo {
                    name: "CR",
                    description: "Control register (WWDG_CR)",
                    address_offset: 0,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        127
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "T",
                                description: Some(
                                    "7-bit counter (MSB to LSB)"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 7
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "WDGA",
                                description: Some(
                                    "Activation bit"
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
                    name: "CFR",
                    description: "Configuration register\r\n          (WWDG_CFR)",
                    address_offset: 4,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        127
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "W",
                                description: Some(
                                    "7-bit window value"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 7
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "WDGTB",
                                description: Some(
                                    "Timer Base"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EWI",
                                description: Some(
                                    "Early Wakeup Interrupt"
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
                    name: "SR",
                    description: "Status register (WWDG_SR)",
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
                                name: "EWI",
                                description: Some(
                                    "Early Wakeup Interrupt"
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
            )
        ]
    ),
    derived_from: None
}
*/
