using System.Runtime.Versioning;

namespace Kawaiifi.Net;

/// <summary>The authentication or association status of this device with a BSS.</summary>
[SupportedOSPlatform("linux")]
public enum BssStatus
{
    /// <summary>Authenticated but not associated.</summary>
    Authenticated,
    /// <summary>Authenticated and associated.</summary>
    Associated,
    /// <summary>Joined an IBSS (ad-hoc) network.</summary>
    IbssJoined,
}
