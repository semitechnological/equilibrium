#!/bin/bash
# Build the C library

echo "Building mathlib..."
gcc -shared -fPIC -o libmathlib.so mathlib.c

if [ $? -eq 0 ]; then
    echo "✓ Built libmathlib.so"
    echo ""
    echo "Functions exported:"
    nm -D libmathlib.so | grep " T " | awk '{print "  - " $3}'
else
    echo "✗ Build failed"
    exit 1
fi
