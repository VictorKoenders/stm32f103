// DMA2
// DMA controller
pub const ADDRESS: u32 = 0x40020400;
pub mod ISR {
}
pub mod IFCR {
}
pub mod CCR1 {
}
pub mod CNDTR1 {
}
pub mod CPAR1 {
}
pub mod CMAR1 {
}
pub mod CCR2 {
}
pub mod CNDTR2 {
}
pub mod CPAR2 {
}
pub mod CMAR2 {
}
pub mod CCR3 {
}
pub mod CNDTR3 {
}
pub mod CPAR3 {
}
pub mod CMAR3 {
}
pub mod CCR4 {
}
pub mod CNDTR4 {
}
pub mod CPAR4 {
}
pub mod CMAR4 {
}
pub mod CCR5 {
}
pub mod CNDTR5 {
}
pub mod CPAR5 {
}
pub mod CMAR5 {
}
pub mod CCR6 {
}
pub mod CNDTR6 {
}
pub mod CPAR6 {
}
pub mod CMAR6 {
}
pub mod CCR7 {
}
pub mod CNDTR7 {
}
pub mod CPAR7 {
}
pub mod CMAR7 {
}
/*
Peripheral {
    name: "DMA2",
    group_name: Some(
        "DMA"
    ),
    description: Some(
        "DMA controller"
    ),
    base_address: 1073873920,
    interrupt: Some(
        Interrupt {
            name: "DMA2_Channel1",
            description: Some(
                "DMA2 Channel1 global interrupt"
            ),
            value: 56
        }
    ),
    registers: Some(
        [
            Single(
                RegisterInfo {
                    name: "ISR",
                    description: "DMA interrupt status register\r\n          (DMA_ISR)",
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
                                name: "GIF1",
                                description: Some(
                                    "Channel 1 Global interrupt\r\n              flag"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TCIF1",
                                description: Some(
                                    "Channel 1 Transfer Complete\r\n              flag"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "HTIF1",
                                description: Some(
                                    "Channel 1 Half Transfer Complete\r\n              flag"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TEIF1",
                                description: Some(
                                    "Channel 1 Transfer Error\r\n              flag"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "GIF2",
                                description: Some(
                                    "Channel 2 Global interrupt\r\n              flag"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TCIF2",
                                description: Some(
                                    "Channel 2 Transfer Complete\r\n              flag"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "HTIF2",
                                description: Some(
                                    "Channel 2 Half Transfer Complete\r\n              flag"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TEIF2",
                                description: Some(
                                    "Channel 2 Transfer Error\r\n              flag"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "GIF3",
                                description: Some(
                                    "Channel 3 Global interrupt\r\n              flag"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TCIF3",
                                description: Some(
                                    "Channel 3 Transfer Complete\r\n              flag"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "HTIF3",
                                description: Some(
                                    "Channel 3 Half Transfer Complete\r\n              flag"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TEIF3",
                                description: Some(
                                    "Channel 3 Transfer Error\r\n              flag"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "GIF4",
                                description: Some(
                                    "Channel 4 Global interrupt\r\n              flag"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TCIF4",
                                description: Some(
                                    "Channel 4 Transfer Complete\r\n              flag"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "HTIF4",
                                description: Some(
                                    "Channel 4 Half Transfer Complete\r\n              flag"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TEIF4",
                                description: Some(
                                    "Channel 4 Transfer Error\r\n              flag"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "GIF5",
                                description: Some(
                                    "Channel 5 Global interrupt\r\n              flag"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TCIF5",
                                description: Some(
                                    "Channel 5 Transfer Complete\r\n              flag"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "HTIF5",
                                description: Some(
                                    "Channel 5 Half Transfer Complete\r\n              flag"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TEIF5",
                                description: Some(
                                    "Channel 5 Transfer Error\r\n              flag"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "GIF6",
                                description: Some(
                                    "Channel 6 Global interrupt\r\n              flag"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TCIF6",
                                description: Some(
                                    "Channel 6 Transfer Complete\r\n              flag"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "HTIF6",
                                description: Some(
                                    "Channel 6 Half Transfer Complete\r\n              flag"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TEIF6",
                                description: Some(
                                    "Channel 6 Transfer Error\r\n              flag"
                                ),
                                bit_range: BitRange {
                                    offset: 23,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "GIF7",
                                description: Some(
                                    "Channel 7 Global interrupt\r\n              flag"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TCIF7",
                                description: Some(
                                    "Channel 7 Transfer Complete\r\n              flag"
                                ),
                                bit_range: BitRange {
                                    offset: 25,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "HTIF7",
                                description: Some(
                                    "Channel 7 Half Transfer Complete\r\n              flag"
                                ),
                                bit_range: BitRange {
                                    offset: 26,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TEIF7",
                                description: Some(
                                    "Channel 7 Transfer Error\r\n              flag"
                                ),
                                bit_range: BitRange {
                                    offset: 27,
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
                    name: "IFCR",
                    description: "DMA interrupt flag clear register\r\n          (DMA_IFCR)",
                    address_offset: 4,
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
                                name: "CGIF1",
                                description: Some(
                                    "Channel 1 Global interrupt\r\n              clear"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CGIF2",
                                description: Some(
                                    "Channel 2 Global interrupt\r\n              clear"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CGIF3",
                                description: Some(
                                    "Channel 3 Global interrupt\r\n              clear"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CGIF4",
                                description: Some(
                                    "Channel 4 Global interrupt\r\n              clear"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CGIF5",
                                description: Some(
                                    "Channel 5 Global interrupt\r\n              clear"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CGIF6",
                                description: Some(
                                    "Channel 6 Global interrupt\r\n              clear"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CGIF7",
                                description: Some(
                                    "Channel 7 Global interrupt\r\n              clear"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CTCIF1",
                                description: Some(
                                    "Channel 1 Transfer Complete\r\n              clear"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CTCIF2",
                                description: Some(
                                    "Channel 2 Transfer Complete\r\n              clear"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CTCIF3",
                                description: Some(
                                    "Channel 3 Transfer Complete\r\n              clear"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CTCIF4",
                                description: Some(
                                    "Channel 4 Transfer Complete\r\n              clear"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CTCIF5",
                                description: Some(
                                    "Channel 5 Transfer Complete\r\n              clear"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CTCIF6",
                                description: Some(
                                    "Channel 6 Transfer Complete\r\n              clear"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CTCIF7",
                                description: Some(
                                    "Channel 7 Transfer Complete\r\n              clear"
                                ),
                                bit_range: BitRange {
                                    offset: 25,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CHTIF1",
                                description: Some(
                                    "Channel 1 Half Transfer\r\n              clear"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CHTIF2",
                                description: Some(
                                    "Channel 2 Half Transfer\r\n              clear"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CHTIF3",
                                description: Some(
                                    "Channel 3 Half Transfer\r\n              clear"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CHTIF4",
                                description: Some(
                                    "Channel 4 Half Transfer\r\n              clear"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CHTIF5",
                                description: Some(
                                    "Channel 5 Half Transfer\r\n              clear"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CHTIF6",
                                description: Some(
                                    "Channel 6 Half Transfer\r\n              clear"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CHTIF7",
                                description: Some(
                                    "Channel 7 Half Transfer\r\n              clear"
                                ),
                                bit_range: BitRange {
                                    offset: 26,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CTEIF1",
                                description: Some(
                                    "Channel 1 Transfer Error\r\n              clear"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CTEIF2",
                                description: Some(
                                    "Channel 2 Transfer Error\r\n              clear"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CTEIF3",
                                description: Some(
                                    "Channel 3 Transfer Error\r\n              clear"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CTEIF4",
                                description: Some(
                                    "Channel 4 Transfer Error\r\n              clear"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CTEIF5",
                                description: Some(
                                    "Channel 5 Transfer Error\r\n              clear"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CTEIF6",
                                description: Some(
                                    "Channel 6 Transfer Error\r\n              clear"
                                ),
                                bit_range: BitRange {
                                    offset: 23,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CTEIF7",
                                description: Some(
                                    "Channel 7 Transfer Error\r\n              clear"
                                ),
                                bit_range: BitRange {
                                    offset: 27,
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
                    name: "CCR1",
                    description: "DMA channel configuration register\r\n          (DMA_CCR)",
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
                                name: "EN",
                                description: Some(
                                    "Channel enable"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TCIE",
                                description: Some(
                                    "Transfer complete interrupt\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "HTIE",
                                description: Some(
                                    "Half Transfer interrupt\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TEIE",
                                description: Some(
                                    "Transfer error interrupt\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DIR",
                                description: Some(
                                    "Data transfer direction"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CIRC",
                                description: Some(
                                    "Circular mode"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PINC",
                                description: Some(
                                    "Peripheral increment mode"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MINC",
                                description: Some(
                                    "Memory increment mode"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PSIZE",
                                description: Some(
                                    "Peripheral size"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MSIZE",
                                description: Some(
                                    "Memory size"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PL",
                                description: Some(
                                    "Channel Priority level"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MEM2MEM",
                                description: Some(
                                    "Memory to memory mode"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
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
                    name: "CNDTR1",
                    description: "DMA channel 1 number of data\r\n          register",
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
                                name: "NDT",
                                description: Some(
                                    "Number of data to transfer"
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
                    name: "CPAR1",
                    description: "DMA channel 1 peripheral address\r\n          register",
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
                                name: "PA",
                                description: Some(
                                    "Peripheral address"
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
                    name: "CMAR1",
                    description: "DMA channel 1 memory address\r\n          register",
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
                                name: "MA",
                                description: Some(
                                    "Memory address"
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
                    name: "CCR2",
                    description: "DMA channel configuration register\r\n          (DMA_CCR)",
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
                                name: "EN",
                                description: Some(
                                    "Channel enable"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TCIE",
                                description: Some(
                                    "Transfer complete interrupt\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "HTIE",
                                description: Some(
                                    "Half Transfer interrupt\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TEIE",
                                description: Some(
                                    "Transfer error interrupt\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DIR",
                                description: Some(
                                    "Data transfer direction"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CIRC",
                                description: Some(
                                    "Circular mode"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PINC",
                                description: Some(
                                    "Peripheral increment mode"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MINC",
                                description: Some(
                                    "Memory increment mode"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PSIZE",
                                description: Some(
                                    "Peripheral size"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MSIZE",
                                description: Some(
                                    "Memory size"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PL",
                                description: Some(
                                    "Channel Priority level"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MEM2MEM",
                                description: Some(
                                    "Memory to memory mode"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
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
                    name: "CNDTR2",
                    description: "DMA channel 2 number of data\r\n          register",
                    address_offset: 32,
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
                                name: "NDT",
                                description: Some(
                                    "Number of data to transfer"
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
                    name: "CPAR2",
                    description: "DMA channel 2 peripheral address\r\n          register",
                    address_offset: 36,
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
                                name: "PA",
                                description: Some(
                                    "Peripheral address"
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
                    name: "CMAR2",
                    description: "DMA channel 2 memory address\r\n          register",
                    address_offset: 40,
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
                                name: "MA",
                                description: Some(
                                    "Memory address"
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
                    name: "CCR3",
                    description: "DMA channel configuration register\r\n          (DMA_CCR)",
                    address_offset: 48,
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
                                name: "EN",
                                description: Some(
                                    "Channel enable"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TCIE",
                                description: Some(
                                    "Transfer complete interrupt\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "HTIE",
                                description: Some(
                                    "Half Transfer interrupt\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TEIE",
                                description: Some(
                                    "Transfer error interrupt\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DIR",
                                description: Some(
                                    "Data transfer direction"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CIRC",
                                description: Some(
                                    "Circular mode"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PINC",
                                description: Some(
                                    "Peripheral increment mode"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MINC",
                                description: Some(
                                    "Memory increment mode"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PSIZE",
                                description: Some(
                                    "Peripheral size"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MSIZE",
                                description: Some(
                                    "Memory size"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PL",
                                description: Some(
                                    "Channel Priority level"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MEM2MEM",
                                description: Some(
                                    "Memory to memory mode"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
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
                    name: "CNDTR3",
                    description: "DMA channel 3 number of data\r\n          register",
                    address_offset: 52,
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
                                name: "NDT",
                                description: Some(
                                    "Number of data to transfer"
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
                    name: "CPAR3",
                    description: "DMA channel 3 peripheral address\r\n          register",
                    address_offset: 56,
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
                                name: "PA",
                                description: Some(
                                    "Peripheral address"
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
                    name: "CMAR3",
                    description: "DMA channel 3 memory address\r\n          register",
                    address_offset: 60,
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
                                name: "MA",
                                description: Some(
                                    "Memory address"
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
                    name: "CCR4",
                    description: "DMA channel configuration register\r\n          (DMA_CCR)",
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
                                name: "EN",
                                description: Some(
                                    "Channel enable"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TCIE",
                                description: Some(
                                    "Transfer complete interrupt\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "HTIE",
                                description: Some(
                                    "Half Transfer interrupt\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TEIE",
                                description: Some(
                                    "Transfer error interrupt\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DIR",
                                description: Some(
                                    "Data transfer direction"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CIRC",
                                description: Some(
                                    "Circular mode"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PINC",
                                description: Some(
                                    "Peripheral increment mode"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MINC",
                                description: Some(
                                    "Memory increment mode"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PSIZE",
                                description: Some(
                                    "Peripheral size"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MSIZE",
                                description: Some(
                                    "Memory size"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PL",
                                description: Some(
                                    "Channel Priority level"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MEM2MEM",
                                description: Some(
                                    "Memory to memory mode"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
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
                    name: "CNDTR4",
                    description: "DMA channel 4 number of data\r\n          register",
                    address_offset: 72,
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
                                name: "NDT",
                                description: Some(
                                    "Number of data to transfer"
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
                    name: "CPAR4",
                    description: "DMA channel 4 peripheral address\r\n          register",
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
                                name: "PA",
                                description: Some(
                                    "Peripheral address"
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
                    name: "CMAR4",
                    description: "DMA channel 4 memory address\r\n          register",
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
                                name: "MA",
                                description: Some(
                                    "Memory address"
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
                    name: "CCR5",
                    description: "DMA channel configuration register\r\n          (DMA_CCR)",
                    address_offset: 88,
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
                                name: "EN",
                                description: Some(
                                    "Channel enable"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TCIE",
                                description: Some(
                                    "Transfer complete interrupt\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "HTIE",
                                description: Some(
                                    "Half Transfer interrupt\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TEIE",
                                description: Some(
                                    "Transfer error interrupt\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DIR",
                                description: Some(
                                    "Data transfer direction"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CIRC",
                                description: Some(
                                    "Circular mode"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PINC",
                                description: Some(
                                    "Peripheral increment mode"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MINC",
                                description: Some(
                                    "Memory increment mode"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PSIZE",
                                description: Some(
                                    "Peripheral size"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MSIZE",
                                description: Some(
                                    "Memory size"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PL",
                                description: Some(
                                    "Channel Priority level"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MEM2MEM",
                                description: Some(
                                    "Memory to memory mode"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
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
                    name: "CNDTR5",
                    description: "DMA channel 5 number of data\r\n          register",
                    address_offset: 92,
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
                                name: "NDT",
                                description: Some(
                                    "Number of data to transfer"
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
                    name: "CPAR5",
                    description: "DMA channel 5 peripheral address\r\n          register",
                    address_offset: 96,
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
                                name: "PA",
                                description: Some(
                                    "Peripheral address"
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
                    name: "CMAR5",
                    description: "DMA channel 5 memory address\r\n          register",
                    address_offset: 100,
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
                                name: "MA",
                                description: Some(
                                    "Memory address"
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
                    name: "CCR6",
                    description: "DMA channel configuration register\r\n          (DMA_CCR)",
                    address_offset: 108,
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
                                name: "EN",
                                description: Some(
                                    "Channel enable"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TCIE",
                                description: Some(
                                    "Transfer complete interrupt\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "HTIE",
                                description: Some(
                                    "Half Transfer interrupt\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TEIE",
                                description: Some(
                                    "Transfer error interrupt\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DIR",
                                description: Some(
                                    "Data transfer direction"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CIRC",
                                description: Some(
                                    "Circular mode"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PINC",
                                description: Some(
                                    "Peripheral increment mode"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MINC",
                                description: Some(
                                    "Memory increment mode"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PSIZE",
                                description: Some(
                                    "Peripheral size"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MSIZE",
                                description: Some(
                                    "Memory size"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PL",
                                description: Some(
                                    "Channel Priority level"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MEM2MEM",
                                description: Some(
                                    "Memory to memory mode"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
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
                    name: "CNDTR6",
                    description: "DMA channel 6 number of data\r\n          register",
                    address_offset: 112,
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
                                name: "NDT",
                                description: Some(
                                    "Number of data to transfer"
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
                    name: "CPAR6",
                    description: "DMA channel 6 peripheral address\r\n          register",
                    address_offset: 116,
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
                                name: "PA",
                                description: Some(
                                    "Peripheral address"
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
                    name: "CMAR6",
                    description: "DMA channel 6 memory address\r\n          register",
                    address_offset: 120,
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
                                name: "MA",
                                description: Some(
                                    "Memory address"
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
                    name: "CCR7",
                    description: "DMA channel configuration register\r\n          (DMA_CCR)",
                    address_offset: 128,
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
                                name: "EN",
                                description: Some(
                                    "Channel enable"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TCIE",
                                description: Some(
                                    "Transfer complete interrupt\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "HTIE",
                                description: Some(
                                    "Half Transfer interrupt\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TEIE",
                                description: Some(
                                    "Transfer error interrupt\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DIR",
                                description: Some(
                                    "Data transfer direction"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CIRC",
                                description: Some(
                                    "Circular mode"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PINC",
                                description: Some(
                                    "Peripheral increment mode"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MINC",
                                description: Some(
                                    "Memory increment mode"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PSIZE",
                                description: Some(
                                    "Peripheral size"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MSIZE",
                                description: Some(
                                    "Memory size"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PL",
                                description: Some(
                                    "Channel Priority level"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 2
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "MEM2MEM",
                                description: Some(
                                    "Memory to memory mode"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
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
                    name: "CNDTR7",
                    description: "DMA channel 7 number of data\r\n          register",
                    address_offset: 132,
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
                                name: "NDT",
                                description: Some(
                                    "Number of data to transfer"
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
                    name: "CPAR7",
                    description: "DMA channel 7 peripheral address\r\n          register",
                    address_offset: 136,
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
                                name: "PA",
                                description: Some(
                                    "Peripheral address"
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
                    name: "CMAR7",
                    description: "DMA channel 7 memory address\r\n          register",
                    address_offset: 140,
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
                                name: "MA",
                                description: Some(
                                    "Memory address"
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
            )
        ]
    ),
    derived_from: Some(
        "DMA1"
    )
}
*/
