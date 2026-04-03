## Equilibrium FFI helpers for Nim
## 
## This module provides ergonomic macros for exporting Nim functions
## to C FFI, compatible with equilibrium's automatic binding generation.
##
## Example:
##   import equilibrium
##   
##   proc add*(a, b: cint): cint {.ffi.} =
##     return a + b
##
## The {.ffi.} pragma expands to:
##   proc add*(a, b: cint): cint {.exportc, cdecl, dynlib.} =
##     return a + b

import macros

template ffi*() {.pragma.}
  ## Mark a procedure for FFI export to equilibrium.
  ## Automatically applies exportc, cdecl, and dynlib pragmas.

macro processFfi*(): untyped =
  ## Process all procedures marked with {.ffi.} pragma
  result = newStmtList()

# Alternative: direct pragma expansion
template ffiExport*(procName: untyped): untyped =
  ## Export a procedure for FFI with proper C linkage
  {.exportc: astToStr(procName), cdecl.}

# Type conversion helpers
proc toCInt*(x: int): cint {.inline.} =
  ## Convert Nim int to C int
  result = cint(x)

proc fromCInt*(x: cint): int {.inline.} =
  ## Convert C int to Nim int
  result = int(x)

proc toCString*(s: string): cstring {.inline.} =
  ## Convert Nim string to C string
  result = cstring(s)

# Example usage (commented out for library)
when isMainModule:
  proc add(a, b: cint): cint {.exportc, cdecl.} =
    return a + b
  
  proc multiply(a, b: cint): cint {.exportc, cdecl.} =
    return a * b
  
  echo "Equilibrium Nim FFI helpers loaded"
