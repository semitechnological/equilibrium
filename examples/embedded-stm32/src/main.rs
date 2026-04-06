//! Frictionless Embedded STM32 Demo
//!
//! This example shows how equilibrium makes C library interop completely frictionless:
//! 1. Drop your STM32 HAL/CMSIS headers in stm32-libs/
//! 2. build.rs auto-discovers and generates bindings
//! 3. Just include!() the generated mod.rs and use them!
//!
//! NO manual bindgen configuration needed!

// Auto-generated bindings from ALL discovered C libraries
mod stm32 {
    #![allow(dead_code, non_camel_case_types, non_upper_case_globals)]
    include!(concat!(env!("OUT_DIR"), "/bindings/mod.rs"));
}

fn main() {
    println!("=== Frictionless STM32 FFI Demo ===\n");

    println!("Your Rust code can now call ALL discovered C functions:");
    println!("  • stm32::stm32f4xx_hal_gpio::HAL_GPIO_Init()");
    println!("  • stm32::stm32f4xx_hal_uart::HAL_UART_Transmit()");
    println!("  • stm32::core_cm4::SystemInit()");
    println!("  • stm32::my_sensor::sensor_init()");

    println!("\n--- Example: GPIO Control ---");
    unsafe {
        // These bindings were auto-generated from stm32f4xx_hal_gpio.h
        use stm32::stm32f4xx_hal_gpio::*;

        println!("Initializing GPIO pin...");
        // In a real embedded app, you'd have actual hardware handles
        // let gpio_config = GPIO_InitTypeDef { ... };
        // HAL_GPIO_Init(&GPIOA, &gpio_config);

        println!("Setting pin HIGH");
        // HAL_GPIO_WritePin(&GPIOA, GPIO_PIN_5, GPIO_PIN_SET);

        println!("Reading pin state");
        // let state = HAL_GPIO_ReadPin(&GPIOA, GPIO_PIN_5);
    }

    println!("\n--- Example: UART Communication ---");
    unsafe {
        use stm32::stm32f4xx_hal_uart::*;

        println!("Transmitting data over UART...");
        // let mut data = b"Hello from Rust!\n";
        // let status = HAL_UART_Transmit(&uart1, data.as_mut_ptr(), data.len() as u16, 1000);
        // if status == HAL_OK { println!("✓ Sent successfully"); }
    }

    println!("\n--- Example: Custom Sensor Driver ---");
    unsafe {
        use stm32::my_sensor::*;

        println!("Initializing custom I2C sensor...");
        // let mut sensor = sensor_t { address: 0x40, i2c_handle: null_mut() };
        // sensor_init(&mut sensor, 0x40);

        println!("Reading temperature...");
        // let mut temp = 0.0f32;
        // sensor_read_temperature(&sensor, &mut temp);
        // println!("Temperature: {}°C", temp);
    }

    println!("\n✨ Zero configuration required!");
    println!("\nHow it works:");
    println!("  1. build.rs calls: equilibrium::scan_c_libraries(\"stm32-libs\")");
    println!("  2. Equilibrium recursively discovers ALL .h files");
    println!("  3. Generates Rust FFI bindings for each header file");
    println!("  4. Creates mod.rs that re-exports each header as a module");
    println!("  5. Your code just: include!(concat!(env!(\"OUT_DIR\"), \"/bindings/mod.rs\"))");
    println!("\n🚀 Add more headers? They're auto-discovered on next build!");
}
