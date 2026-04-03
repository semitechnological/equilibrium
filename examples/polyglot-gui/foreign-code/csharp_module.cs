// C# module — geometric calculations
// Compiled with: dotnet publish or csc -target:library

using System;
using System.Runtime.InteropServices;

public static class CSharpModule
{
    [UnmanagedCallersOnly(EntryPoint = "cs_circle_area_x100")]
    public static int CircleAreaX100(int radius)
    {
        // Returns area * 100 as integer (avoids floats in FFI)
        double area = Math.PI * radius * radius;
        return (int)(area * 100);
    }

    [UnmanagedCallersOnly(EntryPoint = "cs_hypotenuse_x100")]
    public static int HypotenuseX100(int a, int b)
    {
        double h = Math.Sqrt(a * a + b * b);
        return (int)(h * 100);
    }
}
