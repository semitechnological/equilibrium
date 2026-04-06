// Mock CMSIS core header

#ifndef CORE_CM4_H
#define CORE_CM4_H

#include <stdint.h>

// System initialization
void SystemInit(void);
void SystemCoreClockUpdate(void);

// NVIC functions
void NVIC_EnableIRQ(int IRQn);
void NVIC_DisableIRQ(int IRQn);
void NVIC_SetPriority(int IRQn, uint32_t priority);

// System tick
uint32_t SysTick_Config(uint32_t ticks);

#endif
