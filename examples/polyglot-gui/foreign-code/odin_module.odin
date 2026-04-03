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
