using System.Runtime.Versioning;

namespace Kawaiifi.Net;

/// <summary>The status of a BSS.</summary>
[SupportedOSPlatform("linux")]
public enum BssStatus
{
    /// <summary>The local station is authenticated with the BSS.</summary>
    Authenticated,
    /// <summary>The local station is associated with the BSS.</summary>
    Associated,
    /// <summary>The local station has joined the IBSS.</summary>
    IbssJoined,
}
