// This file is NOT auto-generated and should not be modified by build tools.
//
// csbindgen generates P/Invoke bindings for the kawaiifi FFI, but it only processes
// the FFI crate's own source files. The opaque types below (Bss, Scan, Interface, Ie,
// Field) are defined in the kawaiifi library, not the FFI crate, so csbindgen cannot
// see or generate them. Without these definitions the generated NativeMethods*.g.cs
// files will not compile, since they reference these types as pointers in function
// signatures.

using System.Runtime.InteropServices;

namespace CsBindgen
{
    [StructLayout(LayoutKind.Sequential)]
    internal unsafe partial struct Bss { }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe partial struct Scan { }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe partial struct Interface { }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe partial struct Ie { }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe partial struct Field { }
}
