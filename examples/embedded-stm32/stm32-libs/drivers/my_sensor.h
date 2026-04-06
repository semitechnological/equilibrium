// Custom driver example - shows how your own C code gets auto-discovered too!

#ifndef MY_SENSOR_DRIVER_H
#define MY_SENSOR_DRIVER_H

#include <stdint.h>

typedef struct {
    uint8_t address;
    void* i2c_handle;
} sensor_t;

// Your custom sensor driver functions
int sensor_init(sensor_t* sensor, uint8_t i2c_addr);
int sensor_read_temperature(sensor_t* sensor, float* temp);
int sensor_read_humidity(sensor_t* sensor, float* humidity);
void sensor_power_down(sensor_t* sensor);

#endif
