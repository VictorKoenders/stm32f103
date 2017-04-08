// I2C1
// Inter integrated circuit
pub const ADDRESS: u32 = 0x40005400;
/*
Peripheral {
    name: "I2C1",
    group_name: Some(
        "I2C"
    ),
    description: Some(
        "Inter integrated circuit"
    ),
    base_address: 1073763328,
    interrupt: Some(
        Interrupt {
            name: "I2C1_EV",
            description: Some(
                "I2C1 event interrupt"
            ),
            value: 31
        }
    ),
    registers: Some(
        [
            Single(
                RegisterInfo {
                    name: "CR1",
                    description: "Control register 1",
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
                                name: "SWRST",
                                description: Some(
                                    "Software reset"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ALERT",
                                description: Some(
                                    "SMBus alert"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PEC",
                                description: Some(
                                    "Packet error checking"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "POS",
                                description: Some(
                                    "Acknowledge/PEC Position (for data\r\n              reception)"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ACK",
                                description: Some(
                                    "Acknowledge enable"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "STOP",
                                description: Some(
                                    "Stop generation"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "START",
                                description: Some(
                                    "Start generation"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "NOSTRETCH",
                                description: Some(
                                    "Clock stretching disable (Slave\r\n              mode)"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ENGC",
                                description: Some(
                                    "General call enable"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ENPEC",
                                description: Some(
                                    "PEC enable"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ENARP",
                                description: Some(
                                    "ARP enable"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SMBTYPE",
                                description: Some(
                                    "SMBus type"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SMBUS",
                                description: Some(
                                    "SMBus mode"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PE",
                                description: Some(
                                    "Peripheral enable"
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
                    description: "Control register 2",
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
                                name: "LAST",
                                description: Some(
                                    "DMA last transfer"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DMAEN",
                                description: Some(
                                    "DMA requests enable"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ITBUFEN",
                                description: Some(
                                    "Buffer interrupt enable"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ITEVTEN",
                                description: Some(
                                    "Event interrupt enable"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ITERREN",
                                description: Some(
                                    "Error interrupt enable"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FREQ",
                                description: Some(
                                    "Peripheral clock frequency"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 6
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
                    name: "OAR1",
                    description: "Own address register 1",
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
                                name: "ADDMODE",
                                description: Some(
                                    "Addressing mode (slave\r\n              mode)"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ADD10",
                                description: Some(
                                    "Interface address"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ADD7",
                                description: Some(
                                    "Interface address"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 7
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ADD0",
                                description: Some(
                                    "Interface address"
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
                    name: "OAR2",
                    description: "Own address register 2",
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
                                name: "ADD2",
                                description: Some(
                                    "Interface address"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 7
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ENDUAL",
                                description: Some(
                                    "Dual addressing mode\r\n              enable"
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
                    name: "DR",
                    description: "Data register",
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
                                name: "DR",
                                description: Some(
                                    "8-bit data register"
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
                    name: "SR1",
                    description: "Status register 1",
                    address_offset: 20,
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
                                name: "SMBALERT",
                                description: Some(
                                    "SMBus alert"
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
                                name: "TIMEOUT",
                                description: Some(
                                    "Timeout or Tlow error"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "PECERR",
                                description: Some(
                                    "PEC Error in reception"
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
                                name: "OVR",
                                description: Some(
                                    "Overrun/Underrun"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "AF",
                                description: Some(
                                    "Acknowledge failure"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "ARLO",
                                description: Some(
                                    "Arbitration lost (master\r\n              mode)"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "BERR",
                                description: Some(
                                    "Bus error"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "TxE",
                                description: Some(
                                    "Data register empty\r\n              (transmitters)"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: Some(
                                    ReadOnly
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "RxNE",
                                description: Some(
                                    "Data register not empty\r\n              (receivers)"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: Some(
                                    ReadOnly
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "STOPF",
                                description: Some(
                                    "Stop detection (slave\r\n              mode)"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: Some(
                                    ReadOnly
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "ADD10",
                                description: Some(
                                    "10-bit header sent (Master\r\n              mode)"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: Some(
                                    ReadOnly
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "BTF",
                                description: Some(
                                    "Byte transfer finished"
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
                                name: "ADDR",
                                description: Some(
                                    "Address sent (master mode)/matched\r\n              (slave mode)"
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
                                name: "SB",
                                description: Some(
                                    "Start bit (Master mode)"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: Some(
                                    ReadOnly
                                ),
                                enumerated_values: []
                            }
                        ]
                    )
                }
            ),
            Single(
                RegisterInfo {
                    name: "SR2",
                    description: "Status register 2",
                    address_offset: 24,
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
                                name: "PEC",
                                description: Some(
                                    "acket error checking\r\n              register"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DUALF",
                                description: Some(
                                    "Dual flag (Slave mode)"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SMBHOST",
                                description: Some(
                                    "SMBus host header (Slave\r\n              mode)"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SMBDEFAULT",
                                description: Some(
                                    "SMBus device default address (Slave\r\n              mode)"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "GENCALL",
                                description: Some(
                                    "General call address (Slave\r\n              mode)"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TRA",
                                description: Some(
                                    "Transmitter/receiver"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BUSY",
                                description: Some(
                                    "Bus busy"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MSL",
                                description: Some(
                                    "Master/slave"
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
                    name: "CCR",
                    description: "Clock control register",
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
                                name: "F_S",
                                description: Some(
                                    "I2C master mode selection"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DUTY",
                                description: Some(
                                    "Fast mode duty cycle"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CCR",
                                description: Some(
                                    "Clock control register in Fast/Standard\r\n              mode (Master mode)"
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
                    name: "TRISE",
                    description: "TRISE register",
                    address_offset: 32,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        2
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "TRISE",
                                description: Some(
                                    "Maximum rise time in Fast/Standard mode\r\n              (Master mode)"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 6
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
