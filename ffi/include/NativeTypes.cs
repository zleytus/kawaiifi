// This file is NOT auto-generated and should not be modified by build tools.
//
// csbindgen generates P/Invoke bindings for the kawaiifi FFI, but it only processes
// the FFI crate's own source files. The opaque types below (Bss, Scan, Interface, Ie,
// Field) are defined in the kawaiifi library, not the FFI crate, so csbindgen cannot
// see or generate them. Without these definitions the generated NativeMethods*.g.cs
// files will not compile, since they reference these types as pointers in function
// signatures. CapabilityInfo is defined here as a shared value type because it is
// returned from platform-specific native methods in more than one generated file.

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

    /// <summary>The status of a BSS.</summary>
    internal enum BssStatus : uint
    {
        /// <summary>The local station is authenticated with the BSS.</summary>
        Authenticated = 0,

        /// <summary>The local station is associated with the BSS.</summary>
        Associated,

        /// <summary>The local station has joined the IBSS.</summary>
        IbssJoined,

        /// <summary>The BSS status is unavailable or unknown.</summary>
        Unknown,
    }

    /// <summary>
    /// The 802.11 capability information flags advertised in beacon and probe response frames.
    /// </summary>
    [StructLayout(LayoutKind.Sequential)]
    internal struct CapabilityInfo
    {
        /// <summary>Set by an AP (1) or cleared by an IBSS or mesh STA (0).</summary>
        [MarshalAs(UnmanagedType.U1)] public bool ess;

        /// <summary>Set by an IBSS STA (1) or cleared by an AP or mesh STA (0).</summary>
        [MarshalAs(UnmanagedType.U1)] public bool ibss;

        /// <summary>Indicates data confidentiality is required for all Data frames exchanged within the BSS.</summary>
        [MarshalAs(UnmanagedType.U1)] public bool privacy;

        /// <summary>Indicates use of the short preamble is allowed within the BSS.</summary>
        [MarshalAs(UnmanagedType.U1)] public bool short_preamble;

        /// <summary>Set by an AP affiliated with an AP MLD to signal a critical update is pending. Reserved in other contexts.</summary>
        [MarshalAs(UnmanagedType.U1)] public bool critical_update_flag;

        /// <summary>Set by a transmitted-BSSID AP affiliated with an AP MLD if any nontransmitted BSS in its multiple BSSID set has a critical update pending. Reserved in other contexts.</summary>
        [MarshalAs(UnmanagedType.U1)] public bool nontransmitted_bssids_critical_update_flag;

        /// <summary>Indicates the STA implements spectrum management.</summary>
        [MarshalAs(UnmanagedType.U1)] public bool spectrum_management;

        /// <summary>Indicates the STA implements QoS.</summary>
        [MarshalAs(UnmanagedType.U1)] public bool qos;

        /// <summary>Indicates the BSS is currently using the short slot time. Always 0 for IBSS and mesh.</summary>
        [MarshalAs(UnmanagedType.U1)] public bool short_slot_time;

        /// <summary>Set by an AP to indicate Automatic Power Save Delivery (APSD) support. Always 0 for non-AP STAs.</summary>
        [MarshalAs(UnmanagedType.U1)] public bool apsd;

        /// <summary>Indicates the STA supports radio measurement.</summary>
        [MarshalAs(UnmanagedType.U1)] public bool radio_measurement;

        /// <summary>Indicates the STA implements EPD.</summary>
        [MarshalAs(UnmanagedType.U1)] public bool epd;
    }
}
