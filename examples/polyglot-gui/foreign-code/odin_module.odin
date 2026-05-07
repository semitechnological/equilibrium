// Odin module — collection utilities
// Compiled with: odin build odin_module.odin -out:odin_module.o -build-mode:obj

package odin_mod

import "core:math"

@(export)
odin_max :: proc "c" (a, b: i32) -> i32 {
    if a > b do return a
    return b
}

@(export)
odin_min :: proc "c" (a, b: i32) -> i32 {
    if a < b do return a
    return b
}

@(export)
odin_abs :: proc "c" (n: i32) -> i32 {
    if n < 0 do return -n
    return n
}

@(export)
odin_mix :: proc "c" (a, b: i32) -> i32 {
    return (a * 31) ~ (b * 17)
}

@(export)
odin_clamp :: proc "c" (n, lo, hi: i32) -> i32 {
    if n < lo do return lo
    if n > hi do return hi
    return n
}
