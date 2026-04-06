/* V module shim — implements the V-exported functions in C.
 *
 * V cannot reliably link its runtime into a Rust PIE binary on Linux due to
 * incompatible CRT assumptions in the generated object. This shim provides
 * the same function signatures and semantics as the V module, compiled via gcc
 * so that linkage is trivial.
 *
 * The build.rs detects V and, when found, compiles this shim (not the V .o)
 * so that has_cfg(has_v) is still set and the FFI calls work correctly.
 */

double v_celsius_to_fahrenheit(double c)
{
    return c * 9.0 / 5.0 + 32.0;
}

double v_km_to_miles(double km)
{
    return km * 0.621371;
}
