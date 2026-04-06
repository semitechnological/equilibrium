// V module — temperature and unit conversions
// Compiled with: v -gc none -o v_module.o -prod v_module.v

module main

@[export: 'v_celsius_to_fahrenheit']
pub fn v_celsius_to_fahrenheit(c f64) f64 {
	return c * 9.0 / 5.0 + 32.0
}

@[export: 'v_km_to_miles']
pub fn v_km_to_miles(km f64) f64 {
	return km * 0.621371
}

fn main() {}
