// This file defines the memory-mapped addresses for GPIO ports and specific pins
// for STM32F103C8T6.

#![allow(dead_code)]

use crate::utils;
pub static mut SYSCLK: u32 = 8_000_000; // 8 MHz clock frequency
pub static mut PCLK1:  u32 = 8_000_000;
pub static mut PCLK2:  u32 = 8_000_000;
/*
 * Memory-mapped addresses for GPIO ports
 */
pub const AFIO_BASE: u32 = 0x4001_0000; // AFIO base address
pub const AFIO_EXTICR1: u32 = AFIO_BASE + 0x08; // AFIO_EXTICR1 offset
pub const AFIO_EXTICR2: u32 = AFIO_BASE + 0x0C; // AFIO_EXTICR2 offset
pub const AFIO_EXTICR3: u32 = AFIO_BASE + 0x10; // AFIO_EXTICR3 offset
pub const AFIO_EXTICR4: u32 = AFIO_BASE + 0x14; // AFIO_EXTICR4 offset

/*
 * GPIO pin definitions
 */
pub const GPIO00:u32 = 0x00;
pub const GPIO01:u32 = 0x01;
pub const GPIO02:u32 = 0x02;
pub const GPIO03:u32 = 0x03;
pub const GPIO04:u32 = 0x04;
pub const GPIO05:u32 = 0x05;
pub const GPIO06:u32 = 0x06;
pub const GPIO07:u32 = 0x07;
pub const GPIO08:u32 = 0x08;
pub const GPIO09:u32 = 0x09;
pub const GPIO10:u32 = 0x0A;
pub const GPIO11:u32 = 0x0B;
pub const GPIO12:u32 = 0x0C;
pub const GPIO13:u32 = 0x0D;
pub const GPIO14:u32 = 0x0E;
pub const GPIO15:u32 = 0x0F;

/*
 * Memory-mapped addresses for FLASH
 */
pub const FLASH_BASE: u32 = 0x0800_0000; // FLASH base address
pub const FLASH_ACR: u32 = 0x4002_2000; // FLASH access control register
pub const FLASH_KEYR: u32 = 0x4002_2004; // FLASH key register 

/*
 * Memory-mapped addresses for USART
 */
pub const USART1_BASE: u32 = 0x4001_3800;
pub const USART2_BASE: u32 = 0x4000_4400;
pub const USART3_BASE: u32 = 0x4000_4800;

pub const USART_SR: u32 = 0x00;   // Status register
pub const USART_DR: u32 = 0x04;   // Data register (read = RX, write = TX)
pub const USART_BRR: u32 = 0x08;  // Baud rate register
pub const USART_CR1: u32 = 0x0C;  // Control register 1
pub const USART_CR2: u32 = 0x10;  // Control register 2
pub const USART_CR3: u32 = 0x14;  // Control register 3
pub const USART_GTPR: u32 = 0x18; // Guard time and prescaler register

pub const USART_SR_RXNE: u32 = 1 << 5; // Read data register not empty
pub const USART_SR_TXE: u32  = 1 << 7; // Transmit data register empty

/*
 * Memory-mapped addresses for I2C
 */
pub const I2C1_BASE: u32 = 0x4000_5400; // I2C1 base address
pub const I2C2_BASE: u32 = 0x4000_5800; // I2C2 base address
//pub const I2C3_BASE: u32 = 0x4000_5C00; // I2C3 base address

pub const I2C_CR1: u32 = 0x00; // Control register 1
pub const I2C_CR2: u32 = 0x04; // Control register 2
pub const I2C_OAR1: u32 = 0x08; // Own address register 1
pub const I2C_OAR2: u32 = 0x0C; // Own address register 2
pub const I2C_DR: u32 = 0x10; // Data register
pub const I2C_SR1: u32 = 0x14; // Status register 1
pub const I2C_SR2: u32 = 0x18; // Status register 2
pub const I2C_CCR: u32 = 0x1C; // Clock control register
pub const I2C_TRISE: u32 = 0x20; // TRISE register
pub const I2C_SR1_SB: u32 = 1 << 0; // Start bit
pub const I2C_SR1_ADDR: u32 = 1 << 1; // Address sent (master mode) or matched (slave mode)
pub const I2C_SR1_BTF: u32 = 1 << 2; // Byte transfer finished
pub const I2C_SR1_RXNE: u32 = 1 << 6; // Data register not empty (receiving)
pub const I2C_SR1_TXE: u32 = 1 << 7; // Data register empty (transmitting)
pub const I2C_SR1_STOPF: u32 = 1 << 4; // Stop detection (slave mode)
pub const I2C_SR1_AF: u32 = 1 << 10; // Acknowledge failure
pub const I2C_SR1_ARLO: u32 = 1 << 9; // Arbitration lost (master mode)
pub const I2C_SR1_BUSY: u32 = 1 << 1; // Bus busy
pub const I2C_SR1_OVR: u32 = 1 << 11; // Overrun/underrun (slave mode)

/*
 * Memory-mapped addresses for SPI
 */

/*
 * Memory-mapped addresses for TIM
 */
pub const TIM2_BASE: u32 = 0x4000_0000; // TIM2 base address
pub const TIM3_BASE: u32 = 0x4000_0400; // TIM3 base address
pub const TIM4_BASE: u32 = 0x4000_0800; // TIM4 base address
pub const TIM5_BASE: u32 = 0x4000_0C00; // TIM5 base address
pub const TIM6_BASE: u32 = 0x4000_1000; // TIM6 base address
pub const TIM7_BASE: u32 = 0x4000_1400; // TIM7 base address
pub const TIM8_BASE: u32 = 0x4001_0000; // TIM8 base address
pub const TIM_CR1: u32 = 0x00; // Control register 1
pub const TIM_CR2: u32 = 0x04; // Control register 2
pub const TIM_SMCR: u32 = 0x08; // Slave mode control register
pub const TIM_DIER: u32 = 0x0C; // DMA/interrupt enable register
pub const TIM_SR: u32 = 0x10; // Status register
pub const TIM_EGR: u32 = 0x14; // Event generation register
pub const TIM_CCMR1: u32 = 0x18; // Capture/compare mode register 1
pub const TIM_CCMR2: u32 = 0x1C; // Capture/compare mode register 2
pub const TIM_CCER: u32 = 0x20; // Capture/compare enable register
pub const TIM_CNT: u32 = 0x24; // Counter register
pub const TIM_PSC: u32 = 0x28; // Prescaler register
pub const TIM_ARR: u32 = 0x2C; // Auto-reload register
pub const TIM_CCR1: u32 = 0x34; // Capture/compare register 1
pub const TIM_CCR2: u32 = 0x38; // Capture/compare register 2
pub const TIM_CCR3: u32 = 0x3C; // Capture/compare register 3
pub const TIM_CCR4: u32 = 0x40; // Capture/compare register 4
pub const TIM_BDTR: u32 = 0x44; // Break and dead-time register
pub const TIM_DCR: u32 = 0x48; // DMA control register

/*
 * Memory-mapped addresses for DMA
 */
pub const DMA1_BASE: u32 = 0x4002_0000; // DMA1 base address
pub const DMA2_BASE: u32 = 0x4002_0400; // DMA2 base address
/*
 * Memory-mapped addresses for ADC
 */
pub const ADC1_BASE: u32 = 0x4001_2400; // ADC1 base address
pub const ADC2_BASE: u32 = 0x4001_2800; // ADC2 base address
pub const ADC3_BASE: u32 = 0x4001_2C00; // ADC3 base address

/*
 * Memory-mapped addresses for GPIO ports
 */
pub const GPIOA_BASE: u32 = 0x4001_0800; // GPIOA base address
pub const GPIOB_BASE: u32 = 0x4001_0C00; // GPIOB base address
pub const GPIOC_BASE: u32 = 0x4001_1000; // GPIOC base address
pub const GPIOD_BASE: u32 = 0x4001_1400; // GPIOD base address
pub const GPIOE_BASE: u32 = 0x4001_1800; // GPIOE base address
pub const GPIOF_BASE: u32 = 0x4001_1C00; // GPIOF base address
pub const GPIOG_BASE: u32 = 0x4001_2000; // GPIOG base address

/*
 * Memory-mapped addresses for RCC
 */
pub const RCC_ADDR: u32 = 0x4002_1000; // RCC base address
pub const RCC_CR: u32 = RCC_ADDR + 0x00; // RCC_CR offset
pub const RCC_CFGR: u32 = RCC_ADDR + 0x04; // RCC_CFGR offset

pub const RCC_AHBENR: u32 = RCC_ADDR + 0x14; // RCC_AHBENR offset
pub const RCC_AHBRSTR: u32 = RCC_ADDR + 0x28; // RCC_AHBRSTR offset
pub const RCC_AHBENR_DMA1: u32 = 0x0000_0001; // DMA1 clock enable
pub const RCC_AHBENR_DMA2: u32 = 0x0000_0002; // DMA2 clock enable
pub const RCC_AHBENR_SRAM: u32 = 0x0000_0004; // SRAM clock enable
pub const RCC_AHBENR_FLASH: u32 = 0x0000_0008; // Flash clock enable
pub const RCC_AHBENR_CRC: u32 = 0x0000_0010; // CRC clock enable
pub const RCC_AHBENR_OTG: u32 = 0x0000_0020; // OTG clock enable
pub const RCC_AHBENR_ETH: u32 = 0x0000_0040; // ETH MAC clock enable
pub const RCC_AHBENR_ETHTX: u32 = 0x0000_00; // ETH MAC TX clock enable
pub const RCC_AHBENR_ETHRX: u32 = 0x0000_00; // ETH MAC RX clock enable

pub const RCC_APB1ENR: u32 = RCC_ADDR + 0x1C; // RCC_APB1ENR offset
pub const RCC_APB1RSTR: u32 = RCC_ADDR + 0x20; // RCC_APB1RSTR offset
pub const RCC_APB1ENR_TIM2: u32 = 0x0000_0001; // TIM2 clock enable
pub const RCC_APB1ENR_TIM3: u32 = 0x0000_0002; // TIM3 clock enable
pub const RCC_APB1ENR_USART2: u32 = 0x0000_0004; // USART2 clock enable
pub const RCC_APB1ENR_RESERVED: u32 = 0x0000_0008; // Reserved
pub const RCC_APB1ENR_WWDG: u32 = 0x0000_0010; // Window Watchdog clock enable
pub const RCC_APB1ENR_I2C1: u32 = 0x0000_0020; // I2C1 clock enable
pub const RCC_APB1ENR_I2C2: u32 = 0x0000_0040; // I2C2 clock enable
pub const RCC_APB1ENR_CAN: u32 = 0x0000_0080; // CAN clock enable
pub const RCC_APB1ENR_BKP: u32 = 0x0000_0100; // Backup interface clock enable
pub const RCC_APB1ENR_PWR: u32 = 0x0000_0200; // Power interface clock enable

pub const RCC_APB2ENR: u32 = RCC_ADDR + 0x18; // RCC_APB2ENR offset
pub const RCC_APB2RSTR: u32 = RCC_ADDR + 0x24; // RCC_APB2RSTR offset
pub const RCC_APB2ENR_AFIOEN: u32 = 0x0000_0000; // AFIO clock enable
pub const RCC_APB2ENR_RESERVED: u32 = 0x0000_0001; // Reserved
pub const RCC_APB2ENR_IOPAEN: u32 = 0x0000_0002; // GPIOA clock enable
pub const RCC_APB2ENR_IOPBEN: u32 = 0x0000_0003; // GPIOB clock enable
pub const RCC_APB2ENR_IOPCEN: u32 = 0x0000_0004; // GPIOC clock enable
pub const RCC_APB2ENR_IOPDEN: u32 = 0x0000_0005; // GPIOD clock enable
pub const RCC_APB2ENR_IOPEEN: u32 = 0x0000_0006; // GPIOE clock enable
pub const RCC_APB2ENR_IOPFEN: u32 = 0x0000_0007; // GPIOF clock enable
pub const RCC_APB2ENR_IOPGEN: u32 = 0x0000_0008; // GPIOG clock enable
pub const RCC_APB2ENR_ADC1EN: u32 = 0x0000_0009; // ADC1 clock enable
pub const RCC_APB2ENR_ADC2EN: u32 = 0x0000_0010; // ADC2 clock enable
pub const RCC_APB2ENR_ADC3EN: u32 = 0x0000_000F; // ADC3 clock enable
pub const RCC_APB2ENR_USART1EN: u32 = 0x0000_4000; // USART1 clock enable
pub const RCC_APB2ENR_USART2EN: u32 = 0x0000_8000; // USART2 clock enable
pub const RCC_APB2ENR_USART3EN: u32 = 0x0001_0000; // USART3 clock enable
pub const RCC_APB2ENR_SPI1EN: u32 = 0x0002_0000; // SPI1 clock enable
pub const RCC_APB2ENR_SPI2EN: u32 = 0x0004_0000; // SPI2 clock enable
pub const RCC_APB2ENR_I2C1EN: u32 = 0x0008_0000; // I2C1 clock enable
pub const RCC_APB2ENR_I2C2EN: u32 = 0x0010_0000; // I2C2 clock enable
pub const RCC_APB2ENR_TIM1EN: u32 = 0x0020_0000; // TIM1 clock enable
pub const RCC_APB2ENR_TIM2EN: u32 = 0x0040_0000; // TIM2 clock enable
pub const RCC_APB2ENR_TIM3EN: u32 = 0x0080_0000; // TIM3 clock enable
pub const RCC_APB2ENR_TIM4EN: u32 = 0x0100_0000; // TIM4 clock enable
pub const RCC_APB2ENR_TIM5EN: u32 = 0x0200_0000; // TIM5 clock enable
pub const RCC_APB2ENR_TIM6EN: u32 = 0x0400_0000; // TIM6 clock enable
pub const RCC_APB2ENR_TIM7EN: u32 = 0x0800_0000; // TIM7 clock enable
pub const RCC_APB2ENR_TIM8EN: u32 = 0x1000_0000; // TIM8 clock enable
pub const RCC_APB2ENR_DMA1EN: u32 = 0x2000_0000; // DMA1 clock enable
pub const RCC_APB2ENR_DMA2EN: u32 = 0x4000_0000; // DMA2 clock enable
pub const RCC_APB2ENR_USBEN: u32 = 0x8000_0000; // USB clock enable
pub const RCC_APB2ENR_CANEN: u32 = 0x0000_0001; // CAN clock enable
pub const RCC_APB2ENR_SDIOEN: u32 = 0x0000_0002; // SDIO clock enable
pub const RCC_APB2ENR_FSMCEN: u32 = 0x0000_0004; // FSMC clock enable
pub const RCC_APB2ENR_ETHMACEN: u32 = 0x0000_0008; // Ethernet MAC clock enable
pub const RCC_APB2ENR_ETHMACTXEN: u32 = 0x0000_0010; // Ethernet MAC TX clock enable
pub const RCC_APB2ENR_ETHMACRXEN: u32 = 0x0000_0020; // Ethernet MAC RX clock enable
pub const RCC_APB2ENR_ETHMACPTPEN: u32 = 0x0000_0040; // Ethernet MAC PTP clock enable

/*
 * Memory-mapped addresses for SYSTICK
 */
pub const SYSTICK_CTRL: u32 = 0xE000_E010; // SysTick control and status register
pub const SYSTICK_LOAD: u32 = 0xE000_E014; // SysTick reload value register
pub const SYSTICK_VAL: u32 = 0xE000_E018; // SysTick current value register

/*
 * Memory-mapped addresses for NVIC
 */
pub const IRQ_ADDR_WWDG:u32    = 0x0000_0040; // Window Watchdog interrupt
pub const IRQ_ADDR_PVD:u32     = 0x0000_0044; // PVD through EXTI Line detection interrupt
pub const IRQ_ADDR_TAMPER:u32  = 0x0000_0048; // Tamper interrupt
pub const IRQ_ADDR_RTC:u32     = 0x0000_004C; // RTC global interrupt
pub const IRQ_ADDR_FLASH:u32   = 0x0000_0050; // Flash global interrupt
pub const IRQ_ADDR_RCC:u32     = 0x0000_0054; // RCC global interrupt
pub const IRQ_ADDR_EXTI0:u32   = 0x0000_0058; // EXTI0 interrupt
pub const IRQ_ADDR_EXTI1:u32   = 0x0000_005C; // EXTI1 interrupt
pub const IRQ_ADDR_EXTI2:u32   = 0x0000_0060; // EXTI2 interrupt
pub const IRQ_ADDR_EXTI3:u32   = 0x0000_0064; // EXTI3 interrupt
pub const IRQ_ADDR_EXTI4:u32   = 0x0000_0068; // EXTI4 interrupt
pub const IRQ_ADDR_DMA1_CHANNEL1:u32 = 0x0000_006C; // DMA1 Channel 1 interrupt
pub const IRQ_ADDR_DMA1_CHANNEL2:u32 = 0x0000_0070; // DMA1 Channel 2 interrupt
pub const IRQ_ADDR_DMA1_CHANNEL3:u32 = 0x0000_0074; // DMA1 Channel 3 interrupt
pub const IRQ_ADDR_DMA1_CHANNEL4:u32 = 0x0000_0078; // DMA1 Channel 4 interrupt
pub const IRQ_ADDR_DMA1_CHANNEL5:u32 = 0x0000_007C; // DMA1 Channel 5 interrupt
pub const IRQ_ADDR_DMA1_CHANNEL6:u32 = 0x0000_0080; // DMA1 Channel 6 interrupt
pub const IRQ_ADDR_DMA1_CHANNEL7:u32 = 0x0000_0084; // DMA1 Channel 7 interrupt
pub const IRQ_ADDR_ADC1_2:u32  = 0x0000_0088; // ADC1 and ADC2 global interrupt
pub const IRQ_ADDR_CAN1_TX:u32 = 0x0000_008C; // USB High Priority or CAN1 TX interrupts
pub const IRQ_ADDR_CAN1_RX0:u32 = 0x0000_0090; // USB Low Priority or CAN1 RX0 interrupts
pub const IRQ_ADDR_CAN1_RX1:u32  = 0x0000_0094; // CAN1 RX1 interrupt
pub const IRQ_ADDR_CAN1_SCE:u32 = 0x0000_0098; // CAN1 SCE interrupt
pub const IRQ_ADDR_EXTI9_5:u32  = 0x0000_009C; // EXTI Line[9:5] interrupts
pub const IRQ_ADDR_TIM1_BRK:u32 = 0x0000_00A0; // TIM1 Break interrupt
pub const IRQ_ADDR_TIM1_UP:u32  = 0x0000_00A4; // TIM1 Update interrupt
pub const IRQ_ADDR_TIM1_TRG_COM:u32 = 0x0000_00A8; // TIM1 Trigger and Commutation interrupt
pub const IRQ_ADDR_TIM1_CC:u32  = 0x0000_00AC; // TIM1 Capture Compare interrupt
pub const IRQ_ADDR_TIM2:u32     = 0x0000_00B0; // TIM2 global interrupt
pub const IRQ_ADDR_TIM3:u32     = 0x0000_00B4; // TIM3 global interrupt
pub const IRQ_ADDR_TIM4:u32     = 0x0000_00B8; // TIM4 global interrupt
pub const IRQ_ADDR_I2C1_EV:u32 = 0x0000_00BC; // I2C1 Event interrupt
pub const IRQ_ADDR_I2C1_ER:u32 = 0x0000_00C0; // I2C1 Error interrupt
pub const IRQ_ADDR_I2C2_EV:u32 = 0x0000_00C4; // I2C2 Event interrupt
pub const IRQ_ADDR_I2C2_ER:u32 = 0x0000_00C8; // I2C2 Error interrupt
pub const IRQ_ADDR_SPI1:u32    = 0x0000_00CC; // SPI1 global interrupt
pub const IRQ_ADDR_SPI2:u32    = 0x0000_00D0; // SPI2 global interrupt
pub const IRQ_ADDR_USART1:u32  = 0x0000_00D4; // USART1 global interrupt
pub const IRQ_ADDR_USART2:u32  = 0x0000_00D8; // USART2 global interrupt
pub const IRQ_ADDR_USART3:u32  = 0x0000_00DC; // USART3 global interrupt
pub const IRQ_ADDR_EXTI15_10:u32 = 0x0000_00E0; // EXTI Line[15:10] interrupts
pub const IRQ_ADDR_RTC_ALARM:u32 = 0x0000_00E4; // RTC Alarm through EXTI Line interrupt
pub const IRQ_ADDR_USB_WAKEUP:u32 = 0x0000_00E8; // USB Wakeup from suspend through EXTI Line interrupt
pub const IRQ_ADDR_TIM5:u32     = 0x0000_0108; // TIM5 global interrupt
pub const IRQ_ADDR_SPI3:u32    = 0x0000_010C; // SPI3 global interrupt
pub const IRQ_ADDR_USART4:u32  = 0x0000_0110; // USART4 global interrupt
pub const IRQ_ADDR_USART5:u32  = 0x0000_0114; // USART5 global interrupt
pub const IRQ_ADDR_TIM6:u32     = 0x0000_0118; // TIM6 global interrupt
pub const IRQ_ADDR_TIM7:u32     = 0x0000_011C; // TIM7 global interrupt
pub const IRQ_ADDR_DMA2_CHANNEL1:u32 = 0x0000_0120; // DMA2 Channel 1 interrupt
pub const IRQ_ADDR_DMA2_CHANNEL2:u32 = 0x0000_0124; // DMA2 Channel 2 interrupt
pub const IRQ_ADDR_DMA2_CHANNEL3:u32 = 0x0000_0128; // DMA2 Channel 3 interrupt
pub const IRQ_ADDR_DMA2_CHANNEL4:u32 = 0x0000_012C; // DMA2 Channel 4 interrupt
pub const IRQ_ADDR_DMA2_CHANNEL5:u32 = 0x0000_0130; // DMA2 Channel 5 interrupt
pub const IRQ_ADDR_ETHERNET:u32 = 0x0000_0134; // Ethernet interrupt
pub const IRQ_ADDR_ETHERNET_WAKEUP:u32 = 0x0000_0138; // Ethernet Wakeup interrupt
pub const IRQ_ADDR_CAN2_TX:u32 = 0x0000_013C; // USB High Priority or CAN2 TX interrupts
pub const IRQ_ADDR_CAN2_RX0:u32 = 0x0000_0140; // USB Low Priority or CAN2 RX0 interrupts
pub const IRQ_ADDR_CAN2_RX1:u32  = 0x0000_0144; // CAN2 RX1 interrupt
pub const IRQ_ADDR_CAN2_SCE:u32 = 0x0000_0148; // CAN2 SCE interrupt
pub const IRQ_ADDR_USB:u32 = 0x0000_014C; // USB On The Go FS global interrupt

/*
 * Memory-mapped addresses for EXTI
 */
pub const EXTI_BASE: u32 = 0x4001_0400;         // EXTI base address
pub const EXTI_IMR: u32 = EXTI_BASE + 0x00;     // Interrupt Mask Register
pub const EXTI_EMR: u32 = EXTI_BASE + 0x04;     // Event Mask Register
pub const EXTI_RTSR: u32 = EXTI_BASE + 0x08;    // Rising Trigger Selection Register
pub const EXTI_FTSR: u32 = EXTI_BASE + 0x0C;    // Falling Trigger Selection Register
pub const EXTI_SWIER: u32 = EXTI_BASE + 0x10;   // Software Interrupt Event Register
pub const EXTI_PR: u32 = EXTI_BASE + 0x14;      // Pending Register

/*
 * Memory-mapped addresses for IWDG
 */
pub const IWDG_BASE: u32 = 0x4000_3000; // IWDG base address
pub const IWDG_KR: u32 = IWDG_BASE + 0x00; // Key register
pub const IWDG_PR: u32 = IWDG_BASE + 0x04; // Prescaler register
pub const IWDG_RLR: u32 = IWDG_BASE + 0x08; // Reload register
pub const IWDG_SR: u32 = IWDG_BASE + 0x0C; // Status register

/*
 * Memory-mapped addresses for WWDG
 */
pub const WWDG_BASE: u32 = 0x4000_2C00; // WWDG base address
pub const WWDG_CR: u32 = WWDG_BASE + 0x00; // Control register
pub const WWDG_CFR: u32 = WWDG_BASE + 0x04; // Configuration register
pub const WWDG_SR: u32 = WWDG_BASE + 0x08; // Status register

/*
 * Memory-mapped addresses for CRC
 */
pub const CRC_BASE: u32 = 0x4002_3000; // CRC base address
pub const CRC_DR: u32 = CRC_BASE + 0x00; // Data register
pub const CRC_IDR: u32 = CRC_BASE + 0x04; // Independent
pub const CRC_CR: u32 = CRC_BASE + 0x08; // Control register
pub const CRC_INIT: u32 = CRC_BASE + 0x10; // Initial CRC value register
pub const CRC_POL: u32 = CRC_BASE + 0x14; // Polynomial register

/*
 * Memory-mapped addresses for USB
 */
pub const USB_BASE: u32 = 0x4000_5C00; // USB base address
pub const USB_EP0R: u32 = USB_BASE + 0x00; // Endpoint 0 register
pub const USB_EP1R: u32 = USB_BASE + 0x04; // Endpoint 1 register
pub const USB_EP2R: u32 = USB_BASE + 0x08; // Endpoint 2 register
pub const USB_EP3R: u32 = USB_BASE + 0x0C; // Endpoint 3 register
pub const USB_EP4R: u32 = USB_BASE + 0x10; // Endpoint 4 register
pub const USB_EP5R: u32 = USB_BASE + 0x14; // Endpoint 5 register
pub const USB_EP6R: u32 = USB_BASE + 0x18; // Endpoint 6 register
pub const USB_EP7R: u32 = USB_BASE + 0x1C; // Endpoint 7 register
pub const USB_CNTR: u32 = USB_BASE + 0x40; // Control register
pub const USB_ISTR: u32 = USB_BASE + 0x44; // Interrupt status register
pub const USB_FNR: u32 = USB_BASE + 0x48; // Frame number register
pub const USB_DADDR: u32 = USB_BASE + 0x4C; // Device address register
pub const USB_BTABLE: u32 = USB_BASE + 0x50; // Buffer table address register
pub const USB_LPMCSR: u32 = USB_BASE + 0x54; // LPM control and status register
pub const USB_BCDR: u32 = USB_BASE + 0x58; // Battery charging detector register

pub enum SysClock
{
    HSE8MHz  = 8_000_000,
    HSE24MHz = 24_000_000,
    HSE36MHz = 36_000_000,
    HSE48MHz = 48_000_000,
    HSE72MHz = 72_000_000,
}

pub fn init_clock(sys_clk: SysClock) -> u8
{
    let hse: u32 = 8_000_000;
    let sysclk = match sys_clk
    {
        SysClock::HSE8MHz => 8_000_000,
        SysClock::HSE24MHz => 24_000_000,
        SysClock::HSE36MHz => 36_000_000,
        SysClock::HSE48MHz => 48_000_000,
        SysClock::HSE72MHz => 72_000_000,
    };

    unsafe 
    {
        SYSCLK = sysclk;

        if sysclk > 36_000_000
        {
            PCLK1 = sysclk / 2;
        }
        else
        {
            PCLK1 = sysclk;
        }

        PCLK2 = sysclk;
    }

    // =========================
    // 1. Calcular PLL
    // =========================
    let pll_mul = sysclk / hse;

    if pll_mul < 2 || pll_mul > 16
    {
        //return Err("PLL multiplier invalid");
        return 0;
    }

    if sysclk != hse * pll_mul
    {
        // return Err("Frequency not multiple of HSE");
        return 0;
    }

    unsafe
    {
        let rcc_cr   = RCC_CR as *mut u32;
        let rcc_cfgr = RCC_CFGR as *mut u32;
        let flash_acr= FLASH_ACR as *mut u32;

        // =========================
        // HSE ON
        // =========================
        utils::set_bit32(rcc_cr, 16);

        while utils::read_bit32(rcc_cr, 17) == 0 {}

        // =========================
        // FLASH latency
        // =========================
        let latency = 
        if sysclk <= 24_000_000
        {
            0
        }
        else if sysclk <= 48_000_000
        {
            1
        }
        else
        {
            2
        };

        utils::write_register32(flash_acr, latency | (1 << 4)); // prefetch

        // =========================
        // Prescalers
        // =========================
        let mut cfgr = utils::read_register32(rcc_cfgr);

        // AHB = SYSCLK
        cfgr &= !(0b1111 << 4);

        // APB1
        if sysclk > 36_000_000
        {
            cfgr |= 0b100 << 8; // divide by 2
        }
        else
        {
            cfgr &= !(0b111 << 8);
        }

        // APB2 = full
        cfgr &= !(0b111 << 11);

        // =========================
        // PLL
        // =========================
        cfgr |= 1 << 16; // HSE source
        cfgr &= !(1 << 17);

        cfgr &= !(0b1111 << 18);
        cfgr |= (pll_mul - 2) << 18;

        if sysclk == 72_000_000
        {
            cfgr &= !(1 << 22); // /1.5
        }
        else if sysclk == 48_000_000
        {
            cfgr |= 1 << 22; // /1
        }

        utils::write_register32(rcc_cfgr, cfgr);

        // =========================
        // PLL ON
        // =========================
        utils::set_bit32(rcc_cr, 24);

        while utils::read_bit32(rcc_cr, 25) == 0 {}

        // =========================
        // Switch
        // =========================
        let mut cfgr = utils::read_register32(rcc_cfgr);
        cfgr &= !0b11;
        cfgr |= 0b10;

        utils::write_register32(rcc_cfgr, cfgr);

        while ((utils::read_register32(rcc_cfgr) >> 2) & 0b11) != 0b10 {}
    }

    unsafe
    {
        // let sws = (utils::read_register32(rcc_cfgr) >> 2) & 0b11;
        
        // MCO = PLL / 2
        let rcc_cfgr = RCC_CFGR as *mut u32;
        let mut cfgr = utils::read_register32(rcc_cfgr);

        // limpa MCO bits [26:24]
        cfgr &= !(0b110 << 24);

        // seleciona PLL/2 → 111
        cfgr |= 0b110 << 24;

        utils::write_register32(rcc_cfgr, cfgr);
    }

    1
}

pub fn get_sysclk() -> u32
{
    unsafe { SYSCLK }
}

pub fn get_pclk1() -> u32
{
    unsafe { PCLK1 }
}

pub fn get_pclk2() -> u32
{
    unsafe { PCLK2 }
}