using System.Runtime.Versioning;

namespace Kawaiifi.Net;

/// <summary>The CoreWLAN operating mode of a Wi-Fi interface.</summary>
[SupportedOSPlatform("macos")]
public enum CwInterfaceMode
{
    /// <summary>No interface mode.</summary>
    None,
    /// <summary>Station/client mode.</summary>
    Station,
    /// <summary>IBSS/ad-hoc mode.</summary>
    Ibss,
    /// <summary>Host access point mode.</summary>
    HostAp,
}
