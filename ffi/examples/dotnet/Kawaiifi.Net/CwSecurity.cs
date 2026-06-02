using System.Runtime.Versioning;

namespace Kawaiifi.Net;

/// <summary>The CoreWLAN security type of a Wi-Fi interface.</summary>
[SupportedOSPlatform("macos")]
public enum CwSecurity
{
    /// <summary>No security.</summary>
    None,
    /// <summary>WEP security.</summary>
    Wep,
    /// <summary>WPA Personal security.</summary>
    WpaPersonal,
    /// <summary>Mixed WPA Personal security.</summary>
    WpaPersonalMixed,
    /// <summary>WPA2 Personal security.</summary>
    Wpa2Personal,
    /// <summary>Personal security.</summary>
    Personal,
    /// <summary>Dynamic WEP security.</summary>
    DynamicWep,
    /// <summary>WPA Enterprise security.</summary>
    WpaEnterprise,
    /// <summary>Mixed WPA Enterprise security.</summary>
    WpaEnterpriseMixed,
    /// <summary>WPA2 Enterprise security.</summary>
    Wpa2Enterprise,
    /// <summary>Enterprise security.</summary>
    Enterprise,
    /// <summary>WPA3 Personal security.</summary>
    Wpa3Personal,
    /// <summary>WPA3 Enterprise security.</summary>
    Wpa3Enterprise,
    /// <summary>WPA3 transition security.</summary>
    Wpa3Transition,
    /// <summary>Opportunistic Wireless Encryption security.</summary>
    Owe,
    /// <summary>Opportunistic Wireless Encryption transition security.</summary>
    OweTransition,
    /// <summary>Unknown security type.</summary>
    Unknown,
}
