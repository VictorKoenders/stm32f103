// RCC
// Reset and clock control
pub const ADDRESS: u32 = 0x40021000;
/*
Peripheral {
    name: "RCC",
    group_name: Some(
        "RCC"
    ),
    description: Some(
        "Reset and clock control"
    ),
    base_address: 1073876992,
    interrupt: Some(
        Interrupt {
            name: "RCC",
            description: Some(
                "RCC global interrupt"
            ),
            value: 5
        }
    ),
    registers: Some(
        [
            Single(
                RegisterInfo {
                    name: "CR",
                    description: "Clock control register",
                    address_offset: 0,
                    size: Some(
                        32
                    ),
                    access: None,
                    reset_value: Some(
                        131
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "HSION",
                                description: Some(
                                    "Internal High Speed clock\r\n              enable"
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
                                name: "HSIRDY",
                                description: Some(
                                    "Internal High Speed clock ready\r\n              flag"
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
                                name: "HSITRIM",
                                description: Some(
                                    "Internal High Speed clock\r\n              trimming"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 5
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "HSICAL",
                                description: Some(
                                    "Internal High Speed clock\r\n              Calibration"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 8
                                },
                                access: Some(
                                    ReadOnly
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "HSEON",
                                description: Some(
                                    "External High Speed clock\r\n              enable"
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
                                name: "HSERDY",
                                description: Some(
                                    "External High Speed clock ready\r\n              flag"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: Some(
                                    ReadOnly
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "HSEBYP",
                                description: Some(
                                    "External High Speed clock\r\n              Bypass"
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
                                name: "CSSON",
                                description: Some(
                                    "Clock Security System\r\n              enable"
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
                                name: "PLLON",
                                description: Some(
                                    "PLL enable"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "PLLRDY",
                                description: Some(
                                    "PLL clock ready flag"
                                ),
                                bit_range: BitRange {
                                    offset: 25,
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
                    name: "CFGR",
                    description: "Clock configuration register\r\n          (RCC_CFGR)",
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
                                name: "SW",
                                description: Some(
                                    "System clock Switch"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 2
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "SWS",
                                description: Some(
                                    "System Clock Switch Status"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 2
                                },
                                access: Some(
                                    ReadOnly
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "HPRE",
                                description: Some(
                                    "AHB prescaler"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 4
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "PPRE1",
                                description: Some(
                                    "APB Low speed prescaler\r\n              (APB1)"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 3
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "PPRE2",
                                description: Some(
                                    "APB High speed prescaler\r\n              (APB2)"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 3
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "ADCPRE",
                                description: Some(
                                    "ADC prescaler"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 2
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "PLLSRC",
                                description: Some(
                                    "PLL entry clock source"
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
                                name: "PLLXTPRE",
                                description: Some(
                                    "HSE divider for PLL entry"
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
                                name: "PLLMUL",
                                description: Some(
                                    "PLL Multiplication Factor"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 4
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "OTGFSPRE",
                                description: Some(
                                    "USB OTG FS prescaler"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "MCO",
                                description: Some(
                                    "Microcontroller clock\r\n              output"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 3
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            }
                        ]
                    )
                }
            ),
            Single(
                RegisterInfo {
                    name: "CIR",
                    description: "Clock interrupt register\r\n          (RCC_CIR)",
                    address_offset: 8,
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
                                name: "LSIRDYF",
                                description: Some(
                                    "LSI Ready Interrupt flag"
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
                                name: "LSERDYF",
                                description: Some(
                                    "LSE Ready Interrupt flag"
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
                                name: "HSIRDYF",
                                description: Some(
                                    "HSI Ready Interrupt flag"
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
                                name: "HSERDYF",
                                description: Some(
                                    "HSE Ready Interrupt flag"
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
                                name: "PLLRDYF",
                                description: Some(
                                    "PLL Ready Interrupt flag"
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
                                name: "CSSF",
                                description: Some(
                                    "Clock Security System Interrupt\r\n              flag"
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
                                name: "LSIRDYIE",
                                description: Some(
                                    "LSI Ready Interrupt Enable"
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
                                name: "LSERDYIE",
                                description: Some(
                                    "LSE Ready Interrupt Enable"
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
                                name: "HSIRDYIE",
                                description: Some(
                                    "HSI Ready Interrupt Enable"
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
                                name: "HSERDYIE",
                                description: Some(
                                    "HSE Ready Interrupt Enable"
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
                                name: "PLLRDYIE",
                                description: Some(
                                    "PLL Ready Interrupt Enable"
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
                                name: "LSIRDYC",
                                description: Some(
                                    "LSI Ready Interrupt Clear"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
                                    width: 1
                                },
                                access: Some(
                                    WriteOnly
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "LSERDYC",
                                description: Some(
                                    "LSE Ready Interrupt Clear"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: Some(
                                    WriteOnly
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "HSIRDYC",
                                description: Some(
                                    "HSI Ready Interrupt Clear"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 1
                                },
                                access: Some(
                                    WriteOnly
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "HSERDYC",
                                description: Some(
                                    "HSE Ready Interrupt Clear"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: Some(
                                    WriteOnly
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "PLLRDYC",
                                description: Some(
                                    "PLL Ready Interrupt Clear"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: Some(
                                    WriteOnly
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "CSSC",
                                description: Some(
                                    "Clock security system interrupt\r\n              clear"
                                ),
                                bit_range: BitRange {
                                    offset: 23,
                                    width: 1
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
                    name: "APB2RSTR",
                    description: "APB2 peripheral reset register\r\n          (RCC_APB2RSTR)",
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
                                name: "AFIORST",
                                description: Some(
                                    "Alternate function I/O\r\n              reset"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IOPARST",
                                description: Some(
                                    "IO port A reset"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IOPBRST",
                                description: Some(
                                    "IO port B reset"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IOPCRST",
                                description: Some(
                                    "IO port C reset"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IOPDRST",
                                description: Some(
                                    "IO port D reset"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IOPERST",
                                description: Some(
                                    "IO port E reset"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IOPFRST",
                                description: Some(
                                    "IO port F reset"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IOPGRST",
                                description: Some(
                                    "IO port G reset"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ADC1RST",
                                description: Some(
                                    "ADC 1 interface reset"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ADC2RST",
                                description: Some(
                                    "ADC 2 interface reset"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TIM1RST",
                                description: Some(
                                    "TIM1 timer reset"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SPI1RST",
                                description: Some(
                                    "SPI 1 reset"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TIM8RST",
                                description: Some(
                                    "TIM8 timer reset"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "USART1RST",
                                description: Some(
                                    "USART1 reset"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ADC3RST",
                                description: Some(
                                    "ADC 3 interface reset"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TIM9RST",
                                description: Some(
                                    "TIM9 timer reset"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TIM10RST",
                                description: Some(
                                    "TIM10 timer reset"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TIM11RST",
                                description: Some(
                                    "TIM11 timer reset"
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
            ),
            Single(
                RegisterInfo {
                    name: "APB1RSTR",
                    description: "APB1 peripheral reset register\r\n          (RCC_APB1RSTR)",
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
                                name: "TIM2RST",
                                description: Some(
                                    "Timer 2 reset"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TIM3RST",
                                description: Some(
                                    "Timer 3 reset"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TIM4RST",
                                description: Some(
                                    "Timer 4 reset"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TIM5RST",
                                description: Some(
                                    "Timer 5 reset"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TIM6RST",
                                description: Some(
                                    "Timer 6 reset"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TIM7RST",
                                description: Some(
                                    "Timer 7 reset"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TIM12RST",
                                description: Some(
                                    "Timer 12 reset"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TIM13RST",
                                description: Some(
                                    "Timer 13 reset"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TIM14RST",
                                description: Some(
                                    "Timer 14 reset"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "WWDGRST",
                                description: Some(
                                    "Window watchdog reset"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SPI2RST",
                                description: Some(
                                    "SPI2 reset"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SPI3RST",
                                description: Some(
                                    "SPI3 reset"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "USART2RST",
                                description: Some(
                                    "USART 2 reset"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "USART3RST",
                                description: Some(
                                    "USART 3 reset"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "UART4RST",
                                description: Some(
                                    "UART 4 reset"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "UART5RST",
                                description: Some(
                                    "UART 5 reset"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "I2C1RST",
                                description: Some(
                                    "I2C1 reset"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "I2C2RST",
                                description: Some(
                                    "I2C2 reset"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "USBRST",
                                description: Some(
                                    "USB reset"
                                ),
                                bit_range: BitRange {
                                    offset: 23,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CANRST",
                                description: Some(
                                    "CAN reset"
                                ),
                                bit_range: BitRange {
                                    offset: 25,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BKPRST",
                                description: Some(
                                    "Backup interface reset"
                                ),
                                bit_range: BitRange {
                                    offset: 27,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PWRRST",
                                description: Some(
                                    "Power interface reset"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DACRST",
                                description: Some(
                                    "DAC interface reset"
                                ),
                                bit_range: BitRange {
                                    offset: 29,
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
                    name: "AHBENR",
                    description: "AHB Peripheral Clock enable register\r\n          (RCC_AHBENR)",
                    address_offset: 20,
                    size: Some(
                        32
                    ),
                    access: Some(
                        ReadWrite
                    ),
                    reset_value: Some(
                        20
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "DMA1EN",
                                description: Some(
                                    "DMA1 clock enable"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DMA2EN",
                                description: Some(
                                    "DMA2 clock enable"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SRAMEN",
                                description: Some(
                                    "SRAM interface clock\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FLITFEN",
                                description: Some(
                                    "FLITF clock enable"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CRCEN",
                                description: Some(
                                    "CRC clock enable"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "FSMCEN",
                                description: Some(
                                    "FSMC clock enable"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SDIOEN",
                                description: Some(
                                    "SDIO clock enable"
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
            ),
            Single(
                RegisterInfo {
                    name: "APB2ENR",
                    description: "APB2 peripheral clock enable register\r\n          (RCC_APB2ENR)",
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
                                name: "AFIOEN",
                                description: Some(
                                    "Alternate function I/O clock\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IOPAEN",
                                description: Some(
                                    "I/O port A clock enable"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IOPBEN",
                                description: Some(
                                    "I/O port B clock enable"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IOPCEN",
                                description: Some(
                                    "I/O port C clock enable"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IOPDEN",
                                description: Some(
                                    "I/O port D clock enable"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IOPEEN",
                                description: Some(
                                    "I/O port E clock enable"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IOPFEN",
                                description: Some(
                                    "I/O port F clock enable"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "IOPGEN",
                                description: Some(
                                    "I/O port G clock enable"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ADC1EN",
                                description: Some(
                                    "ADC 1 interface clock\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 9,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ADC2EN",
                                description: Some(
                                    "ADC 2 interface clock\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 10,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TIM1EN",
                                description: Some(
                                    "TIM1 Timer clock enable"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SPI1EN",
                                description: Some(
                                    "SPI 1 clock enable"
                                ),
                                bit_range: BitRange {
                                    offset: 12,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TIM8EN",
                                description: Some(
                                    "TIM8 Timer clock enable"
                                ),
                                bit_range: BitRange {
                                    offset: 13,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "USART1EN",
                                description: Some(
                                    "USART1 clock enable"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "ADC3EN",
                                description: Some(
                                    "ADC3 interface clock\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TIM9EN",
                                description: Some(
                                    "TIM9 Timer clock enable"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TIM10EN",
                                description: Some(
                                    "TIM10 Timer clock enable"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TIM11EN",
                                description: Some(
                                    "TIM11 Timer clock enable"
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
            ),
            Single(
                RegisterInfo {
                    name: "APB1ENR",
                    description: "APB1 peripheral clock enable register\r\n          (RCC_APB1ENR)",
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
                                name: "TIM2EN",
                                description: Some(
                                    "Timer 2 clock enable"
                                ),
                                bit_range: BitRange {
                                    offset: 0,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TIM3EN",
                                description: Some(
                                    "Timer 3 clock enable"
                                ),
                                bit_range: BitRange {
                                    offset: 1,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TIM4EN",
                                description: Some(
                                    "Timer 4 clock enable"
                                ),
                                bit_range: BitRange {
                                    offset: 2,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TIM5EN",
                                description: Some(
                                    "Timer 5 clock enable"
                                ),
                                bit_range: BitRange {
                                    offset: 3,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TIM6EN",
                                description: Some(
                                    "Timer 6 clock enable"
                                ),
                                bit_range: BitRange {
                                    offset: 4,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TIM7EN",
                                description: Some(
                                    "Timer 7 clock enable"
                                ),
                                bit_range: BitRange {
                                    offset: 5,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TIM12EN",
                                description: Some(
                                    "Timer 12 clock enable"
                                ),
                                bit_range: BitRange {
                                    offset: 6,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TIM13EN",
                                description: Some(
                                    "Timer 13 clock enable"
                                ),
                                bit_range: BitRange {
                                    offset: 7,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "TIM14EN",
                                description: Some(
                                    "Timer 14 clock enable"
                                ),
                                bit_range: BitRange {
                                    offset: 8,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "WWDGEN",
                                description: Some(
                                    "Window watchdog clock\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 11,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SPI2EN",
                                description: Some(
                                    "SPI 2 clock enable"
                                ),
                                bit_range: BitRange {
                                    offset: 14,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "SPI3EN",
                                description: Some(
                                    "SPI 3 clock enable"
                                ),
                                bit_range: BitRange {
                                    offset: 15,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "USART2EN",
                                description: Some(
                                    "USART 2 clock enable"
                                ),
                                bit_range: BitRange {
                                    offset: 17,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "USART3EN",
                                description: Some(
                                    "USART 3 clock enable"
                                ),
                                bit_range: BitRange {
                                    offset: 18,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "UART4EN",
                                description: Some(
                                    "UART 4 clock enable"
                                ),
                                bit_range: BitRange {
                                    offset: 19,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "UART5EN",
                                description: Some(
                                    "UART 5 clock enable"
                                ),
                                bit_range: BitRange {
                                    offset: 20,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "I2C1EN",
                                description: Some(
                                    "I2C 1 clock enable"
                                ),
                                bit_range: BitRange {
                                    offset: 21,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "I2C2EN",
                                description: Some(
                                    "I2C 2 clock enable"
                                ),
                                bit_range: BitRange {
                                    offset: 22,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "USBEN",
                                description: Some(
                                    "USB clock enable"
                                ),
                                bit_range: BitRange {
                                    offset: 23,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "CANEN",
                                description: Some(
                                    "CAN clock enable"
                                ),
                                bit_range: BitRange {
                                    offset: 25,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "BKPEN",
                                description: Some(
                                    "Backup interface clock\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 27,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "PWREN",
                                description: Some(
                                    "Power interface clock\r\n              enable"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 1
                                },
                                access: None,
                                enumerated_values: []
                            },
                            Field {
                                name: "DACEN",
                                description: Some(
                                    "DAC interface clock enable"
                                ),
                                bit_range: BitRange {
                                    offset: 29,
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
                    name: "BDCR",
                    description: "Backup domain control register\r\n          (RCC_BDCR)",
                    address_offset: 32,
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
                                name: "LSEON",
                                description: Some(
                                    "External Low Speed oscillator\r\n              enable"
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
                                name: "LSERDY",
                                description: Some(
                                    "External Low Speed oscillator\r\n              ready"
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
                                name: "LSEBYP",
                                description: Some(
                                    "External Low Speed oscillator\r\n              bypass"
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
                                name: "RTCSEL",
                                description: Some(
                                    "RTC clock source selection"
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
                                name: "RTCEN",
                                description: Some(
                                    "RTC clock enable"
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
                                name: "BDRST",
                                description: Some(
                                    "Backup domain software\r\n              reset"
                                ),
                                bit_range: BitRange {
                                    offset: 16,
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
            ),
            Single(
                RegisterInfo {
                    name: "CSR",
                    description: "Control/status register\r\n          (RCC_CSR)",
                    address_offset: 36,
                    size: Some(
                        32
                    ),
                    access: None,
                    reset_value: Some(
                        201326592
                    ),
                    reset_mask: None,
                    fields: Some(
                        [
                            Field {
                                name: "LSION",
                                description: Some(
                                    "Internal low speed oscillator\r\n              enable"
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
                                name: "LSIRDY",
                                description: Some(
                                    "Internal low speed oscillator\r\n              ready"
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
                                name: "RMVF",
                                description: Some(
                                    "Remove reset flag"
                                ),
                                bit_range: BitRange {
                                    offset: 24,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "PINRSTF",
                                description: Some(
                                    "PIN reset flag"
                                ),
                                bit_range: BitRange {
                                    offset: 26,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "PORRSTF",
                                description: Some(
                                    "POR/PDR reset flag"
                                ),
                                bit_range: BitRange {
                                    offset: 27,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "SFTRSTF",
                                description: Some(
                                    "Software reset flag"
                                ),
                                bit_range: BitRange {
                                    offset: 28,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "IWDGRSTF",
                                description: Some(
                                    "Independent watchdog reset\r\n              flag"
                                ),
                                bit_range: BitRange {
                                    offset: 29,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "WWDGRSTF",
                                description: Some(
                                    "Window watchdog reset flag"
                                ),
                                bit_range: BitRange {
                                    offset: 30,
                                    width: 1
                                },
                                access: Some(
                                    ReadWrite
                                ),
                                enumerated_values: []
                            },
                            Field {
                                name: "LPWRRSTF",
                                description: Some(
                                    "Low-power reset flag"
                                ),
                                bit_range: BitRange {
                                    offset: 31,
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
