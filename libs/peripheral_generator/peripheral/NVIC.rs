// NVIC
// Nested Vectored Interrupt Controller
pub const ADDRESS: u32 = 0xE000E000;
pub mod ICTR {
}
pub mod STIR {
}
pub mod ISER0 {
}
pub mod ISER1 {
}
pub mod ICER0 {
}
pub mod ICER1 {
}
pub mod ISPR0 {
}
pub mod ISPR1 {
}
pub mod ICPR0 {
}
pub mod ICPR1 {
}
pub mod IABR0 {
}
pub mod IABR1 {
}
pub mod IPR0 {
}
pub mod IPR1 {
}
pub mod IPR2 {
}
pub mod IPR3 {
}
pub mod IPR4 {
}
pub mod IPR5 {
}
pub mod IPR6 {
}
pub mod IPR7 {
}
pub mod IPR8 {
}
pub mod IPR9 {
}
pub mod IPR10 {
}
pub mod IPR11 {
}
pub mod IPR12 {
}
pub mod IPR13 {
}
pub mod IPR14 {
}
/*
Peripheral {
    name: "NVIC",
    group_name: Some(
        "NVIC"
    ),
    description: Some(
        "Nested Vectored Interrupt\r\n      Controller"
    ),
    base_address: 3758153728,
    interrupt: None,
    registers: Some(
        [
            Single(
                RegisterInfo {
                    name: "ICTR",
                    description: "Interrupt Controller Type\r\n          Register",
                    address_offset: 4,
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
                                name: "INTLINESNUM",
                                description: Some(
                                    "Total number of interrupt lines in\r\n              groups"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
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
                    name: "STIR",
                    description: "Software Triggered Interrupt\r\n          Register",
                    address_offset: 3840,
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
                                name: "INTID",
                                description: Some(
                                    "interrupt to be triggered"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 9
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
                    name: "ISER0",
                    description: "Interrupt Set-Enable Register",
                    address_offset: 256,
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
                                name: "SETENA",
                                description: Some(
                                    "SETENA"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 32
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
                    name: "ISER1",
                    description: "Interrupt Set-Enable Register",
                    address_offset: 260,
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
                                name: "SETENA",
                                description: Some(
                                    "SETENA"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 32
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
                    name: "ICER0",
                    description: "Interrupt Clear-Enable\r\n          Register",
                    address_offset: 384,
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
                                name: "CLRENA",
                                description: Some(
                                    "CLRENA"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 32
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
                    name: "ICER1",
                    description: "Interrupt Clear-Enable\r\n          Register",
                    address_offset: 388,
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
                                name: "CLRENA",
                                description: Some(
                                    "CLRENA"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 32
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
                    name: "ISPR0",
                    description: "Interrupt Set-Pending Register",
                    address_offset: 512,
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
                                name: "SETPEND",
                                description: Some(
                                    "SETPEND"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 32
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
                    name: "ISPR1",
                    description: "Interrupt Set-Pending Register",
                    address_offset: 516,
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
                                name: "SETPEND",
                                description: Some(
                                    "SETPEND"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 32
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
                    name: "ICPR0",
                    description: "Interrupt Clear-Pending\r\n          Register",
                    address_offset: 640,
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
                                name: "CLRPEND",
                                description: Some(
                                    "CLRPEND"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 32
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
                    name: "ICPR1",
                    description: "Interrupt Clear-Pending\r\n          Register",
                    address_offset: 644,
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
                                name: "CLRPEND",
                                description: Some(
                                    "CLRPEND"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 32
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
                    name: "IABR0",
                    description: "Interrupt Active Bit Register",
                    address_offset: 768,
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
                                name: "ACTIVE",
                                description: Some(
                                    "ACTIVE"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 32
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
                    name: "IABR1",
                    description: "Interrupt Active Bit Register",
                    address_offset: 772,
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
                                name: "ACTIVE",
                                description: Some(
                                    "ACTIVE"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 32
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
                    name: "IPR0",
                    description: "Interrupt Priority Register",
                    address_offset: 1024,
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
                                name: "IPR_N0",
                                description: Some(
                                    "IPR_N0"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N1",
                                description: Some(
                                    "IPR_N1"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N2",
                                description: Some(
                                    "IPR_N2"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N3",
                                description: Some(
                                    "IPR_N3"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
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
                    name: "IPR1",
                    description: "Interrupt Priority Register",
                    address_offset: 1028,
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
                                name: "IPR_N0",
                                description: Some(
                                    "IPR_N0"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N1",
                                description: Some(
                                    "IPR_N1"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N2",
                                description: Some(
                                    "IPR_N2"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N3",
                                description: Some(
                                    "IPR_N3"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
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
                    name: "IPR2",
                    description: "Interrupt Priority Register",
                    address_offset: 1032,
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
                                name: "IPR_N0",
                                description: Some(
                                    "IPR_N0"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N1",
                                description: Some(
                                    "IPR_N1"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N2",
                                description: Some(
                                    "IPR_N2"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N3",
                                description: Some(
                                    "IPR_N3"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
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
                    name: "IPR3",
                    description: "Interrupt Priority Register",
                    address_offset: 1036,
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
                                name: "IPR_N0",
                                description: Some(
                                    "IPR_N0"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N1",
                                description: Some(
                                    "IPR_N1"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N2",
                                description: Some(
                                    "IPR_N2"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N3",
                                description: Some(
                                    "IPR_N3"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
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
                    name: "IPR4",
                    description: "Interrupt Priority Register",
                    address_offset: 1040,
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
                                name: "IPR_N0",
                                description: Some(
                                    "IPR_N0"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N1",
                                description: Some(
                                    "IPR_N1"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N2",
                                description: Some(
                                    "IPR_N2"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N3",
                                description: Some(
                                    "IPR_N3"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
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
                    name: "IPR5",
                    description: "Interrupt Priority Register",
                    address_offset: 1044,
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
                                name: "IPR_N0",
                                description: Some(
                                    "IPR_N0"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N1",
                                description: Some(
                                    "IPR_N1"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N2",
                                description: Some(
                                    "IPR_N2"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N3",
                                description: Some(
                                    "IPR_N3"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
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
                    name: "IPR6",
                    description: "Interrupt Priority Register",
                    address_offset: 1048,
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
                                name: "IPR_N0",
                                description: Some(
                                    "IPR_N0"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N1",
                                description: Some(
                                    "IPR_N1"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N2",
                                description: Some(
                                    "IPR_N2"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N3",
                                description: Some(
                                    "IPR_N3"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
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
                    name: "IPR7",
                    description: "Interrupt Priority Register",
                    address_offset: 1052,
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
                                name: "IPR_N0",
                                description: Some(
                                    "IPR_N0"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N1",
                                description: Some(
                                    "IPR_N1"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N2",
                                description: Some(
                                    "IPR_N2"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N3",
                                description: Some(
                                    "IPR_N3"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
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
                    name: "IPR8",
                    description: "Interrupt Priority Register",
                    address_offset: 1056,
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
                                name: "IPR_N0",
                                description: Some(
                                    "IPR_N0"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N1",
                                description: Some(
                                    "IPR_N1"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N2",
                                description: Some(
                                    "IPR_N2"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N3",
                                description: Some(
                                    "IPR_N3"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
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
                    name: "IPR9",
                    description: "Interrupt Priority Register",
                    address_offset: 1060,
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
                                name: "IPR_N0",
                                description: Some(
                                    "IPR_N0"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N1",
                                description: Some(
                                    "IPR_N1"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N2",
                                description: Some(
                                    "IPR_N2"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N3",
                                description: Some(
                                    "IPR_N3"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
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
                    name: "IPR10",
                    description: "Interrupt Priority Register",
                    address_offset: 1064,
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
                                name: "IPR_N0",
                                description: Some(
                                    "IPR_N0"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N1",
                                description: Some(
                                    "IPR_N1"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N2",
                                description: Some(
                                    "IPR_N2"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N3",
                                description: Some(
                                    "IPR_N3"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
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
                    name: "IPR11",
                    description: "Interrupt Priority Register",
                    address_offset: 1068,
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
                                name: "IPR_N0",
                                description: Some(
                                    "IPR_N0"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N1",
                                description: Some(
                                    "IPR_N1"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N2",
                                description: Some(
                                    "IPR_N2"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N3",
                                description: Some(
                                    "IPR_N3"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
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
                    name: "IPR12",
                    description: "Interrupt Priority Register",
                    address_offset: 1072,
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
                                name: "IPR_N0",
                                description: Some(
                                    "IPR_N0"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N1",
                                description: Some(
                                    "IPR_N1"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N2",
                                description: Some(
                                    "IPR_N2"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N3",
                                description: Some(
                                    "IPR_N3"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
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
                    name: "IPR13",
                    description: "Interrupt Priority Register",
                    address_offset: 1076,
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
                                name: "IPR_N0",
                                description: Some(
                                    "IPR_N0"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N1",
                                description: Some(
                                    "IPR_N1"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N2",
                                description: Some(
                                    "IPR_N2"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N3",
                                description: Some(
                                    "IPR_N3"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
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
                    name: "IPR14",
                    description: "Interrupt Priority Register",
                    address_offset: 1080,
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
                                name: "IPR_N0",
                                description: Some(
                                    "IPR_N0"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N1",
                                description: Some(
                                    "IPR_N1"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N2",
                                description: Some(
                                    "IPR_N2"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IPR_N3",
                                description: Some(
                                    "IPR_N3"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
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
