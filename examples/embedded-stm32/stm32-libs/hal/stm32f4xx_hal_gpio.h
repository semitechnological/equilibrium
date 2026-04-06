// Mock STM32F4xx HAL GPIO header
// In a real project, this would be from STM32CubeFW

#ifndef STM32F4XX_HAL_GPIO_H
#define STM32F4XX_HAL_GPIO_H

#include <stdint.h>

// GPIO pin states
typedef enum {
    GPIO_PIN_RESET = 0,
    GPIO_PIN_SET = 1
} GPIO_PinState;

// GPIO initialization structure
typedef struct {
    uint32_t Pin;
    uint32_t Mode;
    uint32_t Pull;
    uint32_t Speed;
} GPIO_InitTypeDef;

// GPIO handle (opaque)
typedef void* GPIO_TypeDef;

// Function prototypes
void HAL_GPIO_Init(GPIO_TypeDef* GPIOx, GPIO_InitTypeDef* GPIO_Init);
void HAL_GPIO_WritePin(GPIO_TypeDef* GPIOx, uint16_t GPIO_Pin, GPIO_PinState PinState);
GPIO_PinState HAL_GPIO_ReadPin(GPIO_TypeDef* GPIOx, uint16_t GPIO_Pin);
void HAL_GPIO_TogglePin(GPIO_TypeDef* GPIOx, uint16_t GPIO_Pin);

#endif
