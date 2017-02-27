#![allow(non_snake_case)]

// Window watchdog
// 0x0000_0040
fn WWDG() { bkpt!(); }

// PVD through EXTI Line detection
// 0x0000_0044
fn PVD() { bkpt!(); }

// Tamper
// 0x0000_0048
fn TAMPER() { bkpt!(); }

// RTC global
// 0x0000_004C
fn RTC() { bkpt!(); }

// Flash global
// 0x0000_0050
fn FLASH() { bkpt!(); }

// RCC global
// 0x0000_0054
fn RCC() { bkpt!(); }

// EXTI Line0
// 0x0000_0058
fn EXTI0() { bkpt!(); }

// EXTI Line1
// 0x0000_005C
fn EXTI1() { bkpt!(); }

// EXTI Line2
// 0x0000_0060
fn EXTI2() { bkpt!(); }

// EXTI Line3
// 0x0000_0064
fn EXTI3() { bkpt!(); }

// EXTI Line4
// 0x0000_0068
fn EXTI4() { bkpt!(); }

// DMA1 Channel1 global
// 0x0000_006C
fn DMA1_Channel1() { bkpt!(); }

// DMA1 Channel2 global
// 0x0000_0070
fn DMA1_Channel2() { bkpt!(); }

// DMA1 Channel3 global
// 0x0000_0074
fn DMA1_Channel3() { bkpt!(); }

// DMA1 Channel4 global
// 0x0000_0078
fn DMA1_Channel4() { bkpt!(); }

// DMA1 Channel5 global
// 0x0000_007C
fn DMA1_Channel5() { bkpt!(); }

// DMA1 Channel6 global
// 0x0000_0080
fn DMA1_Channel6() { bkpt!(); }

// DMA1 Channel7 global
// 0x0000_0084
fn DMA1_Channel7() { bkpt!(); }

// ADC1 and ADC2 global
// 0x0000_0088
fn ADC1_2() { bkpt!(); }

// USB High Priority or CAN TX
// 0x0000_008C
fn USB_HP_CAN_TX() { bkpt!(); }

// USB Low Priority or CAN RX0
// 0x0000_0090
fn USB_LP_CAN_RX0() { bkpt!(); }

// CAN RX1
// 0x0000_0094
fn CAN_RX1() { bkpt!(); }

// CAN SCE
// 0x0000_0098
fn CAN_SCE() { bkpt!(); }

// EXTI Line[9:5]
// 0x0000_009C
fn EXTI9_5() { bkpt!(); }

// TIM1 Break
// 0x0000_00A0
fn TIM1_BRK() { bkpt!(); }

// TIM1 Update
// 0x0000_00A4
fn TIM1_UP() { bkpt!(); }

// TIM1 Trigger and Commutation
// 0x0000_00A8
fn TIM1_TRG_COM() { bkpt!(); }

// TIM1 Capture Compare
// 0x0000_00AC
fn TIM1_CC() { bkpt!(); }

// TIM2 global
// 0x0000_00B0
fn TIM2() { bkpt!(); }

// TIM3 global
// 0x0000_00B4
fn TIM3() { bkpt!(); }

// TIM4 global
// 0x0000_00B8
fn TIM4() { bkpt!(); }

// I2C1 event
// 0x0000_00BC
fn I2C1_EV() { bkpt!(); }

// I2C1 error
// 0x0000_00C0
fn I2C1_ER() { bkpt!(); }

// I2C2 event
// 0x0000_00C4
fn I2C2_EV() { bkpt!(); }

// I2C2 error
// 0x0000_00C8
fn I2C2_ER() { bkpt!(); }

// SPI1 global
// 0x0000_00CC
fn SPI1() { bkpt!(); }

// SPI2 global
// 0x0000_00D0
fn SPI2() { bkpt!(); }

// USART1 global
// 0x0000_00D4
fn USART1() { bkpt!(); }

// USART2 global
// 0x0000_00D8
fn USART2() { bkpt!(); }

// USART3 global
// 0x0000_00DC
fn USART3() { bkpt!(); }

// EXTI Line[15:10]
// 0x0000_00E0
fn EXTI15_10() { bkpt!(); }

// RTC alarm through EXTI line
// 0x0000_00E4
fn RTCAlarm() { bkpt!(); }

// USB wakeup from suspend through EXTI line
// 0x0000_00E8
fn USBWakeup() { bkpt!(); }

// TIM8 Break
// 0x0000_00EC
fn TIM8_BRK() { bkpt!(); }

// TIM8 Update
// 0x0000_00F0
fn TIM8_UP() { bkpt!(); }

// TIM8 Trigger and Commutation
// 0x0000_00F4
fn TIM8_TRG_COM() { bkpt!(); }

// TIM8 Capture Compare
// 0x0000_00F8
fn TIM8_CC() { bkpt!(); }

// ADC3 global
// 0x0000_00FC
fn ADC3() { bkpt!(); }

// FSMC global
// 0x0000_0100
fn FSMC() { bkpt!(); }

// SDIO global
// 0x0000_0104
fn SDIO() { bkpt!(); }

// TIM5 global
// 0x0000_0108
fn TIM5() { bkpt!(); }

// SPI3 global
// 0x0000_010C
fn SPI3() { bkpt!(); }

// UART4 global
// 0x0000_0110
fn UART4() { bkpt!(); }

// UART5 global
// 0x0000_0114
fn UART5() { bkpt!(); }

// TIM6 global
// 0x0000_0118
fn TIM6() { bkpt!(); }

// TIM7 global
// 0x0000_011C
fn TIM7() { bkpt!(); }

// DMA2 Channel1 global
// 0x0000_0120
fn DMA2_Channel1() { bkpt!(); }

// DMA2 Channel2 global
// 0x0000_0124
fn DMA2_Channel2() { bkpt!(); }

// DMA2 Channel3 global
// 0x0000_0128
fn DMA2_Channel3() { bkpt!(); }

// DMA2 Channel4 and DMA2 Channel5 global
// 0x0000_012C
fn DMA2_Channel4_5() { bkpt!(); }

pub type Handler = fn();

#[export_name = "_INTERRUPTS"]
pub static INTERRUPTS: [Option<Handler>; 60] = [
	Some(WWDG),
	Some(PVD),
	Some(TAMPER),
	Some(RTC),
	Some(FLASH),
	Some(RCC),
	Some(EXTI0),
	Some(EXTI1),
	Some(EXTI2),
	Some(EXTI3),
	Some(EXTI4),
	Some(DMA1_Channel1),
	Some(DMA1_Channel2),
	Some(DMA1_Channel3),
	Some(DMA1_Channel4),
	Some(DMA1_Channel5),
	Some(DMA1_Channel6),
	Some(DMA1_Channel7),
	Some(ADC1_2),
	Some(USB_HP_CAN_TX),
	Some(USB_LP_CAN_RX0),
	Some(CAN_RX1),
	Some(CAN_SCE),
	Some(EXTI9_5),
	Some(TIM1_BRK),
	Some(TIM1_UP),
	Some(TIM1_TRG_COM),
	Some(TIM1_CC),
	Some(TIM2),
	Some(TIM3),
	Some(TIM4),
	Some(I2C1_EV),
	Some(I2C1_ER),
	Some(I2C2_EV),
	Some(I2C2_ER),
	Some(SPI1),
	Some(SPI2),
	Some(USART1),
	Some(USART2),
	Some(USART3),
	Some(EXTI15_10),
	Some(RTCAlarm),
	Some(USBWakeup),
	Some(TIM8_BRK),
	Some(TIM8_UP),
	Some(TIM8_TRG_COM),
	Some(TIM8_CC),
	Some(ADC3),
	Some(FSMC),
	Some(SDIO),
	Some(TIM5),
	Some(SPI3),
	Some(UART4),
	Some(UART5),
	Some(TIM6),
	Some(TIM7),
	Some(DMA2_Channel1),
	Some(DMA2_Channel2),
	Some(DMA2_Channel3),
	Some(DMA2_Channel4_5),
];
