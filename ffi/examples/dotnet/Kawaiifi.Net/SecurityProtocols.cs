namespace Kawaiifi.Net;

/// <summary>The security protocols supported by a BSS.</summary>
public readonly struct SecurityProtocols(byte value)
{
    /// <summary>WEP (Wired Equivalent Privacy).</summary>
    public bool Wep => (value & 1) != 0;

    /// <summary>WPA (Wi-Fi Protected Access).</summary>
    public bool Wpa => (value & 2) != 0;

    /// <summary>WPA2 (Wi-Fi Protected Access 2).</summary>
    public bool Wpa2 => (value & 4) != 0;

    /// <summary>WPA3 (Wi-Fi Protected Access 3).</summary>
    public bool Wpa3 => (value & 8) != 0;
}