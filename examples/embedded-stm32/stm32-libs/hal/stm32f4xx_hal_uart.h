// Mock STM32F4xx HAL UART header

#ifndef STM32F4XX_HAL_UART_H
#define STM32F4XX_HAL_UART_H

#include <stdint.h>

typedef void* UART_HandleTypeDef;

// HAL status codes
typedef enum {
    HAL_OK = 0,
    HAL_ERROR = 1,
    HAL_BUSY = 2,
    HAL_TIMEOUT = 3
} HAL_StatusTypeDef;

// UART functions
HAL_StatusTypeDef HAL_UART_Transmit(UART_HandleTypeDef* huart, 
                                     uint8_t* pData, 
                                     uint16_t Size, 
                                     uint32_t Timeout);

HAL_StatusTypeDef HAL_UART_Receive(UART_HandleTypeDef* huart,
                                    uint8_t* pData,
                                    uint16_t Size,
                                    uint32_t Timeout);

void HAL_UART_IRQHandler(UART_HandleTypeDef* huart);

#endif
