// USB
// Universal serial bus full-speed device interface
pub const ADDRESS: u32 = 0x40005C00;
pub mod EP0R {
}
pub mod EP1R {
}
pub mod EP2R {
}
pub mod EP3R {
}
pub mod EP4R {
}
pub mod EP5R {
}
pub mod EP6R {
}
pub mod EP7R {
}
pub mod CNTR {
}
pub mod ISTR {
}
pub mod FNR {
}
pub mod DADDR {
}
pub mod BTABLE {
}
/*
Peripheral {
    name: "USB",
    group_name: Some(
        "USB"
    ),
    description: Some(
        "Universal serial bus full-speed device\r\n      interface"
    ),
    base_address: 1073765376,
    interrupt: Some(
        Interrupt {
            name: "USB_FS_WKUP",
            description: Some(
                "USB Device FS Wakeup through EXTI line\r\n        interrupt"
            ),
            value: 42
        }
    ),
    registers: Some(
        [
            Single(
                RegisterInfo {
                    name: "EP0R",
                    description: "endpoint 0 register",
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
                                name: "EA",
                                description: Some(
                                    "Endpoint address"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "STAT_TX",
                                description: Some(
                                    "Status bits, for transmission\r\n              transfers"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DTOG_TX",
                                description: Some(
                                    "Data Toggle, for transmission\r\n              transfers"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CTR_TX",
                                description: Some(
                                    "Correct Transfer for\r\n              transmission"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EP_KIND",
                                description: Some(
                                    "Endpoint kind"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EP_TYPE",
                                description: Some(
                                    "Endpoint type"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SETUP",
                                description: Some(
                                    "Setup transaction\r\n              completed"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "STAT_RX",
                                description: Some(
                                    "Status bits, for reception\r\n              transfers"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DTOG_RX",
                                description: Some(
                                    "Data Toggle, for reception\r\n              transfers"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CTR_RX",
                                description: Some(
                                    "Correct transfer for\r\n              reception"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
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
                    name: "EP1R",
                    description: "endpoint 1 register",
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
                                name: "EA",
                                description: Some(
                                    "Endpoint address"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "STAT_TX",
                                description: Some(
                                    "Status bits, for transmission\r\n              transfers"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DTOG_TX",
                                description: Some(
                                    "Data Toggle, for transmission\r\n              transfers"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CTR_TX",
                                description: Some(
                                    "Correct Transfer for\r\n              transmission"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EP_KIND",
                                description: Some(
                                    "Endpoint kind"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EP_TYPE",
                                description: Some(
                                    "Endpoint type"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SETUP",
                                description: Some(
                                    "Setup transaction\r\n              completed"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "STAT_RX",
                                description: Some(
                                    "Status bits, for reception\r\n              transfers"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DTOG_RX",
                                description: Some(
                                    "Data Toggle, for reception\r\n              transfers"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CTR_RX",
                                description: Some(
                                    "Correct transfer for\r\n              reception"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
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
                    name: "EP2R",
                    description: "endpoint 2 register",
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
                                name: "EA",
                                description: Some(
                                    "Endpoint address"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "STAT_TX",
                                description: Some(
                                    "Status bits, for transmission\r\n              transfers"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DTOG_TX",
                                description: Some(
                                    "Data Toggle, for transmission\r\n              transfers"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CTR_TX",
                                description: Some(
                                    "Correct Transfer for\r\n              transmission"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EP_KIND",
                                description: Some(
                                    "Endpoint kind"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EP_TYPE",
                                description: Some(
                                    "Endpoint type"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SETUP",
                                description: Some(
                                    "Setup transaction\r\n              completed"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "STAT_RX",
                                description: Some(
                                    "Status bits, for reception\r\n              transfers"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DTOG_RX",
                                description: Some(
                                    "Data Toggle, for reception\r\n              transfers"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CTR_RX",
                                description: Some(
                                    "Correct transfer for\r\n              reception"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
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
                    name: "EP3R",
                    description: "endpoint 3 register",
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
                                name: "EA",
                                description: Some(
                                    "Endpoint address"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "STAT_TX",
                                description: Some(
                                    "Status bits, for transmission\r\n              transfers"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DTOG_TX",
                                description: Some(
                                    "Data Toggle, for transmission\r\n              transfers"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CTR_TX",
                                description: Some(
                                    "Correct Transfer for\r\n              transmission"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EP_KIND",
                                description: Some(
                                    "Endpoint kind"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EP_TYPE",
                                description: Some(
                                    "Endpoint type"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SETUP",
                                description: Some(
                                    "Setup transaction\r\n              completed"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "STAT_RX",
                                description: Some(
                                    "Status bits, for reception\r\n              transfers"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DTOG_RX",
                                description: Some(
                                    "Data Toggle, for reception\r\n              transfers"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CTR_RX",
                                description: Some(
                                    "Correct transfer for\r\n              reception"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
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
                    name: "EP4R",
                    description: "endpoint 4 register",
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
                                name: "EA",
                                description: Some(
                                    "Endpoint address"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "STAT_TX",
                                description: Some(
                                    "Status bits, for transmission\r\n              transfers"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DTOG_TX",
                                description: Some(
                                    "Data Toggle, for transmission\r\n              transfers"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CTR_TX",
                                description: Some(
                                    "Correct Transfer for\r\n              transmission"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EP_KIND",
                                description: Some(
                                    "Endpoint kind"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EP_TYPE",
                                description: Some(
                                    "Endpoint type"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SETUP",
                                description: Some(
                                    "Setup transaction\r\n              completed"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "STAT_RX",
                                description: Some(
                                    "Status bits, for reception\r\n              transfers"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DTOG_RX",
                                description: Some(
                                    "Data Toggle, for reception\r\n              transfers"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CTR_RX",
                                description: Some(
                                    "Correct transfer for\r\n              reception"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
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
                    name: "EP5R",
                    description: "endpoint 5 register",
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
                                name: "EA",
                                description: Some(
                                    "Endpoint address"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "STAT_TX",
                                description: Some(
                                    "Status bits, for transmission\r\n              transfers"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DTOG_TX",
                                description: Some(
                                    "Data Toggle, for transmission\r\n              transfers"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CTR_TX",
                                description: Some(
                                    "Correct Transfer for\r\n              transmission"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EP_KIND",
                                description: Some(
                                    "Endpoint kind"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EP_TYPE",
                                description: Some(
                                    "Endpoint type"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SETUP",
                                description: Some(
                                    "Setup transaction\r\n              completed"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "STAT_RX",
                                description: Some(
                                    "Status bits, for reception\r\n              transfers"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DTOG_RX",
                                description: Some(
                                    "Data Toggle, for reception\r\n              transfers"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CTR_RX",
                                description: Some(
                                    "Correct transfer for\r\n              reception"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
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
                    name: "EP6R",
                    description: "endpoint 6 register",
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
                                name: "EA",
                                description: Some(
                                    "Endpoint address"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "STAT_TX",
                                description: Some(
                                    "Status bits, for transmission\r\n              transfers"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DTOG_TX",
                                description: Some(
                                    "Data Toggle, for transmission\r\n              transfers"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CTR_TX",
                                description: Some(
                                    "Correct Transfer for\r\n              transmission"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EP_KIND",
                                description: Some(
                                    "Endpoint kind"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EP_TYPE",
                                description: Some(
                                    "Endpoint type"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SETUP",
                                description: Some(
                                    "Setup transaction\r\n              completed"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "STAT_RX",
                                description: Some(
                                    "Status bits, for reception\r\n              transfers"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DTOG_RX",
                                description: Some(
                                    "Data Toggle, for reception\r\n              transfers"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CTR_RX",
                                description: Some(
                                    "Correct transfer for\r\n              reception"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
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
                    name: "EP7R",
                    description: "endpoint 7 register",
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
                                name: "EA",
                                description: Some(
                                    "Endpoint address"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "STAT_TX",
                                description: Some(
                                    "Status bits, for transmission\r\n              transfers"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DTOG_TX",
                                description: Some(
                                    "Data Toggle, for transmission\r\n              transfers"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CTR_TX",
                                description: Some(
                                    "Correct Transfer for\r\n              transmission"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EP_KIND",
                                description: Some(
                                    "Endpoint kind"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EP_TYPE",
                                description: Some(
                                    "Endpoint type"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SETUP",
                                description: Some(
                                    "Setup transaction\r\n              completed"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "STAT_RX",
                                description: Some(
                                    "Status bits, for reception\r\n              transfers"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DTOG_RX",
                                description: Some(
                                    "Data Toggle, for reception\r\n              transfers"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CTR_RX",
                                description: Some(
                                    "Correct transfer for\r\n              reception"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
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
                    name: "CNTR",
                    description: "control register",
                    address_offset: 64,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        3
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "FRES",
                                description: Some(
                                    "Force USB Reset"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PDWN",
                                description: Some(
                                    "Power down"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "LPMODE",
                                description: Some(
                                    "Low-power mode"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FSUSP",
                                description: Some(
                                    "Force suspend"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "RESUME",
                                description: Some(
                                    "Resume request"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ESOFM",
                                description: Some(
                                    "Expected start of frame interrupt\r\n              mask"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SOFM",
                                description: Some(
                                    "Start of frame interrupt\r\n              mask"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "RESETM",
                                description: Some(
                                    "USB reset interrupt mask"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SUSPM",
                                description: Some(
                                    "Suspend mode interrupt\r\n              mask"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "WKUPM",
                                description: Some(
                                    "Wakeup interrupt mask"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ERRM",
                                description: Some(
                                    "Error interrupt mask"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PMAOVRM",
                                description: Some(
                                    "Packet memory area over / underrun\r\n              interrupt mask"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CTRM",
                                description: Some(
                                    "Correct transfer interrupt\r\n              mask"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
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
                    name: "ISTR",
                    description: "interrupt status register",
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
                                name: "EP_ID",
                                description: Some(
                                    "Endpoint Identifier"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 4
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DIR",
                                description: Some(
                                    "Direction of transaction"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ESOF",
                                description: Some(
                                    "Expected start frame"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SOF",
                                description: Some(
                                    "start of frame"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "RESET",
                                description: Some(
                                    "reset request"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SUSP",
                                description: Some(
                                    "Suspend mode request"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "WKUP",
                                description: Some(
                                    "Wakeup"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ERR",
                                description: Some(
                                    "Error"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PMAOVR",
                                description: Some(
                                    "Packet memory area over /\r\n              underrun"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CTR",
                                description: Some(
                                    "Correct transfer"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
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
                    name: "FNR",
                    description: "frame number register",
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
                                name: "FN",
                                description: Some(
                                    "Frame number"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 11
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "LSOF",
                                description: Some(
                                    "Lost SOF"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "LCK",
                                description: Some(
                                    "Locked"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "RXDM",
                                description: Some(
                                    "Receive data - line status"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "RXDP",
                                description: Some(
                                    "Receive data + line status"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
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
                    name: "DADDR",
                    description: "device address",
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
                                name: "ADD",
                                description: Some(
                                    "Device address"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 7
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "EF",
                                description: Some(
                                    "Enable function"
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
                    name: "BTABLE",
                    description: "Buffer table address",
                    address_offset: 80,
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
                                name: "BTABLE",
                                description: Some(
                                    "Buffer table"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 13
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
