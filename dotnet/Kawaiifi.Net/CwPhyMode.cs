using System.Runtime.Versioning;

namespace Kawaiifi.Net;

/// <summary>The active CoreWLAN PHY mode of a Wi-Fi interface.</summary>
[SupportedOSPlatform("macos")]
public enum CwPhyMode
{
    /// <summary>No active PHY mode.</summary>
    None,
    /// <summary>802.11a.</summary>
    A,
    /// <summary>802.11b.</summary>
    B,
    /// <summary>802.11g.</summary>
    G,
    /// <summary>802.11n.</summary>
    N,
    /// <summary>802.11ac.</summary>
    Ac,
    /// <summary>802.11ax.</summary>
    Ax,
}
