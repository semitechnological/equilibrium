// V module — temperature and unit conversions
// Compiled with: v -backend c -o output.c v_module.v

module vmod

pub fn celsius_to_fahrenheit(c f64) f64 {
    return c * 9.0 / 5.0 + 32.0
}

pub fn km_to_miles(km f64) f64 {
    return km * 0.621371
}
