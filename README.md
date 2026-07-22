# Sensory Fusion 🚀

An embedded Rust project focused on **sensor fusion** using the **STM32F103 Blue Pill** and the following sensors:

- 🛰️ **GPS NEO-6M** for positioning and navigation (NMEA/UBX protocols)
- 🧭 **~~MPU6050~~ / ~~MPU9250~~ / ICM20948** for accelerometer, gyroscope, and magnetometer measurements

## Overview

The goal of this project is to build a reliable and high-precision navigation platform by integrating data from multiple sensors using sensor fusion techniques on resource-constrained embedded systems.

The project is being developed from scratch, including peripheral drivers, communication protocols, and the sensor fusion algorithms.

## Features

- GPS communication over UART with NMEA/UBX parsing
- I²C driver and support for the ICM20948 IMU
- USB Full-Speed device driver with CDC implementation
- Low-level peripheral drivers for the STM32F103
- Circular buffer implementation for efficient data handling
- Designed for `#![no_std]` environments
- Future support for sensor fusion algorithms (Complementary Filter, Kalman Filter, etc.)

## Project Structure

```txt
src/
├── main.rs                 # Application entry point
│
├── bridge.rs               # Integration layer between modules
├── ccb.rs                  # Circular buffer implementation
├── debug.rs                # Debug logging utilities
├── utils.rs                # General utility functions
│
├── checksum.rs             # Communication checksum algorithms
├── crc.rs                  # CRC implementations
│
├── button.rs               # Push-button driver
├── gpio.rs                 # GPIO driver
├── exti.rs                 # External interrupt controller
├── irq.rs                  # Interrupt management
├── led.rs                  # LED driver
├── rcc.rs                  # Reset and Clock Control (RCC)
├── watchdog.rs             # Watchdog driver
├── startup_stm32f103.rs    # Startup code and interrupt vector table
├── mcu.rs                  # MCU peripheral definitions and register mappings
│
├── usart.rs                # USART/UART driver
├── i2c.rs                  # I²C driver
├── spi.rs                  # SPI driver (work in progress)
├── dma.rs                  # DMA driver (work in progress)
│
├── usb_driver.rs           # USB Full-Speed driver
├── usb_cdc.rs              # USB CDC class implementation
├── usb_control.rs          # USB control transfer handling
├── usb_endpoint.rs         # USB endpoint management
├── usb_peripheral.rs       # USB peripheral interface
├── usb_types.rs            # USB descriptors and protocol definitions
├── usb_class.rs            # USB class definitions
├── usb_core.rs             # USB stack core (work in progress)
│
├── gps_neo6m.rs            # GPS NEO-6M driver and NMEA parser
├── icm20948.rs             # ICM20948 IMU driver
├── mpu9250.rs              # MPU9250 IMU driver
├── mpu6050.rs              # MPU6050 IMU driver
│
├── kalman_filter.rs        # Kalman filter implementation
└── fusion.rs               # Sensor fusion algorithms (work in progress)
```

## Building 🛠️

This project targets the **STM32F103C8T6** and requires the embedded Rust toolchain.

```bash
rustup target add thumbv7m-none-eabi

cargo build --release --target thumbv7m-none-eabi

# Flashing may vary depending on your setup
cargo flash --chip STM32F103C8T6 --release
```

<!-- ## Dependencies

- embedded-hal
- cortex-m
- cortex-m-rt
- stm32f1xx-hal

(Currently being replaced by custom low-level drivers.) -->

## License

Released under the MIT License.

Created by José as a learning and research project exploring embedded systems, low-level firmware development, and sensor fusion.