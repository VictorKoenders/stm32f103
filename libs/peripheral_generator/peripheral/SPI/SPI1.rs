// SPI1
// Serial peripheral interface
pub const ADDRESS: u32 = 0x40013000;
/*
Peripheral {
    name: "SPI1",
    group_name: Some(
        "SPI"
    ),
    description: Some(
        "Serial peripheral interface"
    ),
    base_address: 1073819648,
    interrupt: Some(
        Interrupt {
            name: "SPI1",
            description: Some(
                "SPI1 global interrupt"
            ),
            value: 35
        }
    ),
    registers: Some(
        [
            Single(
                RegisterInfo {
                    name: "CR1",
                    description: "control register 1",
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
                                name: "BIDIMODE",
                                description: Some(
                                    "Bidirectional data mode\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BIDIOE",
                                description: Some(
                                    "Output enable in bidirectional\r\n              mode"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CRCEN",
                                description: Some(
                                    "Hardware CRC calculation\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CRCNEXT",
                                description: Some(
                                    "CRC transfer next"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DFF",
                                description: Some(
                                    "Data frame format"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "RXONLY",
                                description: Some(
                                    "Receive only"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SSM",
                                description: Some(
                                    "Software slave management"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SSI",
                                description: Some(
                                    "Internal slave select"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "LSBFIRST",
                                description: Some(
                                    "Frame format"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SPE",
                                description: Some(
                                    "SPI enable"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BR",
                                description: Some(
                                    "Baud rate control"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 3
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MSTR",
                                description: Some(
                                    "Master selection"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CPOL",
                                description: Some(
                                    "Clock polarity"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CPHA",
                                description: Some(
                                    "Clock phase"
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
                    description: "control register 2",
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
                                name: "TXEIE",
                                description: Some(
                                    "Tx buffer empty interrupt\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "RXNEIE",
                                description: Some(
                                    "RX buffer not empty interrupt\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ERRIE",
                                description: Some(
                                    "Error interrupt enable"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SSOE",
                                description: Some(
                                    "SS output enable"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TXDMAEN",
                                description: Some(
                                    "Tx buffer DMA enable"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "RXDMAEN",
                                description: Some(
                                    "Rx buffer DMA enable"
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
                    name: "SR",
                    description: "status register",
                    address_offset: 8,
                    size: Some(
                        32
                    ),
                    access: None,
                    reset_value: Some(
                        2
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "BSY",
                                description: Some(
                                    "Busy flag"
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
                                name: "OVR",
                                description: Some(
                                    "Overrun flag"
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
                                name: "MODF",
                                description: Some(
                                    "Mode fault"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: Some(
                                    ReadOnly
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "CRCERR",
                                description: Some(
                                    "CRC error flag"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "UDR",
                                description: Some(
                                    "Underrun flag"
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
                                name: "CHSIDE",
                                description: Some(
                                    "Channel side"
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
                                name: "TXE",
                                description: Some(
                                    "Transmit buffer empty"
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
                                name: "RXNE",
                                description: Some(
                                    "Receive buffer not empty"
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
                    name: "DR",
                    description: "data register",
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
                                name: "DR",
                                description: Some(
                                    "Data register"
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
                    name: "CRCPR",
                    description: "CRC polynomial register",
                    address_offset: 16,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        7
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "CRCPOLY",
                                description: Some(
                                    "CRC polynomial register"
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
                    name: "RXCRCR",
                    description: "RX CRC register",
                    address_offset: 20,
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
                                name: "RxCRC",
                                description: Some(
                                    "Rx CRC register"
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
                    name: "TXCRCR",
                    description: "TX CRC register",
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
                                name: "TxCRC",
                                description: Some(
                                    "Tx CRC register"
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
                    name: "I2SCFGR",
                    description: "I2S configuration register",
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
                                name: "I2SMOD",
                                description: Some(
                                    "I2S mode selection"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "I2SE",
                                description: Some(
                                    "I2S Enable"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "I2SCFG",
                                description: Some(
                                    "I2S configuration mode"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PCMSYNC",
                                description: Some(
                                    "PCM frame synchronization"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "I2SSTD",
                                description: Some(
                                    "I2S standard selection"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CKPOL",
                                description: Some(
                                    "Steady state clock\r\n              polarity"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DATLEN",
                                description: Some(
                                    "Data length to be\r\n              transferred"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CHLEN",
                                description: Some(
                                    "Channel length (number of bits per audio\r\n              channel)"
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
                    name: "I2SPR",
                    description: "I2S prescaler register",
                    address_offset: 32,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        10
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "MCKOE",
                                description: Some(
                                    "Master clock output enable"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ODD",
                                description: Some(
                                    "Odd factor for the\r\n              prescaler"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "I2SDIV",
                                description: Some(
                                    "I2S Linear prescaler"
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
            )
        ]
    ),
    derived_from: None
}
*/
