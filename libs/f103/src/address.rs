// All addresses are in the form of (0, 1)
// So the first address is at 0
// and the last available address is at 1
// .0 - 1 and .1 + 1 are out of range of these addresses

pub const FSMC: (usize, usize) = (0xA000_0000, 0xA000_0FFF); //  Section 21.6.9 on page 566
pub const OTG_USB: (usize, usize) = (0x5000_0000, 0x5003_FFFF); //  OTG FS Section 28.16.6 on page 916
pub const ETHERNET: (usize, usize) = (0x4002_8000, 0x4002_9FFF); //  Section 29.8.5 on page 1073
pub const CRC: (usize, usize) = (0x4002_3000, 0x4002_33FF); //  Section 4.4.4 on page 64
pub const FLASH: (usize, usize) = (0x4002_2000, 0x4002_23FF); //  memory interface -
pub const RESET: (usize, usize) = (0x4002_1000, 0x4002_13FF); //  and clock control RCC Section 7.3.11 on page 120
pub const DMA2: (usize, usize) = (0x4002_0400, 0x4002_07FF); //  Section 13.4.7 on page 289
pub const DMA1: (usize, usize) = (0x4002_0000, 0x4002_03FF); // 
pub const SDIO: (usize, usize) = (0x4001_8000, 0x4001_83FF); //  Section 31.9.16 on page 1066
pub const TIM11: (usize, usize) = (0x4001_5400, 0x4001_57FF); //  timer Section 16.5.11 on page 469
pub const TIM10: (usize, usize) = (0x4001_5000, 0x4001_53FF); //  timer Section 16.5.11 on page 469
pub const TIM9: (usize, usize) = (0x4001_4C00, 0x4001_4FFF); //  timer Section 16.4.13 on page 459
pub const ADC3: (usize, usize) = (0x4001_3C00, 0x4001_3FFF); //  Section 11.12.15 on page 252
pub const USART1: (usize, usize) = (0x4001_3800, 0x4001_3BFF); //  Section 27.6.8 on page 832
pub const TIM8: (usize, usize) = (0x4001_3400, 0x4001_37FF); //  timer Section 14.4.21 on page 362
pub const SPI1: (usize, usize) = (0x4001_3000, 0x4001_33FF); //  Section 25.5 on page 745
pub const TIM1: (usize, usize) = (0x4001_2C00, 0x4001_2FFF); //  timer Section 14.4.21 on page 362
pub const ADC2: (usize, usize) = (0x4001_2800, 0x4001_2BFF); //  Section 11.12.15 on page 252
pub const ADC1: (usize, usize) = (0x4001_2400, 0x4001_27FF); // 
pub const GPIOG: (usize, usize) = (0x4001_2000, 0x4001_23FF); //  Port G Section 9.5 on page 193
pub const GPIOF: (usize, usize) = (0x4001_1C00, 0x4001_1FFF); //  Port F
pub const GPIOE: (usize, usize) = (0x4001_1800, 0x4001_1BFF); //  Port E
pub const GPIOD: (usize, usize) = (0x4001_1400, 0x4001_17FF); //  Port D
pub const GPIOC: (usize, usize) = (0x4001_1000, 0x4001_13FF); //  Port C
pub const GPIOB: (usize, usize) = (0x4001_0C00, 0x4001_0FFF); //  Port B
pub const GPIOA: (usize, usize) = (0x4001_0800, 0x4001_0BFF); //  Port A
pub const EXTI: (usize, usize) = (0x4001_0400, 0x4001_07FF); //  Section 10.3.7 on page 213
pub const AFIO: (usize, usize) = (0x4001_0000, 0x4001_03FF); //  Section 9.5 on page 193
pub const DAC: (usize, usize) = (0x4000_7400, 0x4000_77FF); //  Section 12.5.14 on page 273
pub const POWER: (usize, usize) = (0x4000_7000, 0x4000_73FF); //  control PWR Section 5.4.3 on page 79
pub const BACKUP: (usize, usize) = (0x4000_6C00, 0x4000_6FFF); //  registers (BKP) Section 6.4.5 on page 84
pub const BXCAN1: (usize, usize) = (0x4000_6400, 0x4000_67FF); //  Section 24.9.5 on page 698
pub const BXCAN2: (usize, usize) = (0x4000_6800, 0x4000_6BFF); // 
pub const SHARED: (usize, usize) = (0x4000_6000, 0x4000_63FF); //  USB/CAN SRAM 512 bytes -
pub const DEVICE_USB: (usize, usize) = (0x4000_5C00, 0x4000_5FFF); //  device FS registers Section 23.5.4 on page 655
pub const I2C2: (usize, usize) = (0x4000_5800, 0x4000_5BFF); //  Section 26.6.10 on page 789
pub const I2C1: (usize, usize) = (0x4000_5400, 0x4000_57FF); // 
pub const UART5: (usize, usize) = (0x4000_5000, 0x4000_53FF); //  Section 27.6.8 on page 832
pub const UART4: (usize, usize) = (0x4000_4C00, 0x4000_4FFF); // 
pub const USART3: (usize, usize) = (0x4000_4800, 0x4000_4BFF); // 
pub const USART2: (usize, usize) = (0x4000_4400, 0x4000_47FF); // 
pub const SPI3: (usize, usize) = (0x4000_3C00, 0x4000_3FFF); // /I2S Section 25.5 on page 745
pub const SPI2: (usize, usize) = (0x4000_3800, 0x4000_3BFF); // /I2S Section 25.5 on page 745
pub const INDEPENDENT: (usize, usize) = (0x4000_3000, 0x4000_33FF); //  watchdog (IWDG) Section 19.4.5 on page 501
pub const WINDOW: (usize, usize) = (0x4000_2C00, 0x4000_2FFF); //  watchdog (WWDG) Section 20.6.4 on page 508
pub const RTC: (usize, usize) = (0x4000_2800, 0x4000_2BFF); //  Section 18.4.7 on page 495
pub const TIM14: (usize, usize) = (0x4000_2000, 0x4000_23FF); //  timer Section 16.5.11 on page 469
pub const TIM13: (usize, usize) = (0x4000_1C00, 0x4000_1FFF); //  timer
pub const TIM12: (usize, usize) = (0x4000_1800, 0x4000_1BFF); //  timer Section 16.4.13 on page 459
pub const TIM7: (usize, usize) = (0x4000_1400, 0x4000_17FF); //  timer Section 17.4.9 on page 483
pub const TIM6: (usize, usize) = (0x4000_1000, 0x4000_13FF); //  timer
pub const TIM5: (usize, usize) = (0x4000_0C00, 0x4000_0FFF); //  timer Section 15.4.19 on page 422
pub const TIM4: (usize, usize) = (0x4000_0800, 0x4000_0BFF); //  timer
pub const TIM3: (usize, usize) = (0x4000_0400, 0x4000_07FF); //  timer
pub const TIM2: (usize, usize) = (0x4000_0000, 0x4000_03FF); //  timer

// These address ranges are reserved
pub const RESERVED: [(usize, usize); 11] = [
    (0x4003_0000, 0x4FFF_FFFF),
    (0x4002_3400, 0x4002_7FFF),
    (0x4002_1400, 0x4002_1FFF),
    (0x4002_0800, 0x4002_0FFF),
    (0x4001_8400, 0x4001_FFFF),
    (0x4001_5800, 0x4001_7FFF),
    (0x4001_4000, 0x4001_4BFF),
    (0x4000_7800, 0x4000_FFFF),
    (0x4000_4000, 0x4000_43FF),
    (0x4000_3400, 0x4000_37FF),
    (0x4000_2400, 0x4000_27FF),
];
