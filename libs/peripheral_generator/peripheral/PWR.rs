// PWR
// Power control
pub const ADDRESS: u32 = 0x40007000;
pub mod CR {
}
pub mod CSR {
}
/*
Peripheral {
    name: "PWR",
    group_name: Some(
        "PWR"
    ),
    description: Some(
        "Power control"
    ),
    base_address: 1073770496,
    interrupt: Some(
        Interrupt {
            name: "PVD",
            description: Some(
                "PVD through EXTI line detection\r\n        interrupt"
            ),
            value: 1
        }
    ),
    registers: Some(
        [
            Single(
                RegisterInfo {
                    name: "CR",
                    description: "Power control register\r\n          (PWR_CR)",
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
                                name: "LPDS",
                                description: Some(
                                    "Low Power Deep Sleep"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PDDS",
                                description: Some(
                                    "Power Down Deep Sleep"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CWUF",
                                description: Some(
                                    "Clear Wake-up Flag"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CSBF",
                                description: Some(
                                    "Clear STANDBY Flag"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PVDE",
                                description: Some(
                                    "Power Voltage Detector\r\n              Enable"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PLS",
                                description: Some(
                                    "PVD Level Selection"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 3
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DBP",
                                description: Some(
                                    "Disable Backup Domain write\r\n              protection"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
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
                    description: "Power control register\r\n          (PWR_CR)",
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
                                name: "WUF",
                                description: Some(
                                    "Wake-Up Flag"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: Some(
                                    ReadOnly
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "SBF",
                                description: Some(
                                    "STANDBY Flag"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: Some(
                                    ReadOnly
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "PVDO",
                                description: Some(
                                    "PVD Output"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: Some(
                                    ReadOnly
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "EWUP",
                                description: Some(
                                    "Enable WKUP pin"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
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
