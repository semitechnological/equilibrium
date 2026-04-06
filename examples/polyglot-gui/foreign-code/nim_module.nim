# Nim module — bit manipulation
# Compiled with: nim c --nimcache:/tmp/nim_cache --noMain --app:staticlib -o:/tmp/nim_module.a nim_module.nim

proc nim_popcount*(n: uint32): int32 {.exportc, cdecl, noSideEffect.} =
  var x = n
  var count: int32 = 0
  while x != 0:
    count += int32(x and 1)
    x = x shr 1
  count

proc nim_reverse_bits*(n: uint32): uint32 {.exportc, cdecl, noSideEffect.} =
  var x = n
  var rev: uint32 = 0
  for _ in 0..31:
    rev = (rev shl 1) or (x and 1)
    x = x shr 1
  rev
