namespace Kawaiifi.Net;

/// <summary>The Wi-Fi protocols supported by a BSS.</summary>
public readonly struct WifiProtocols(ushort value)
{
    /// <summary>802.11a (5 GHz, up to 54 Mbps).</summary>
    public bool A => (value & 1) != 0;
    /// <summary>802.11b (2.4 GHz, up to 11 Mbps).</summary>
    public bool B => (value & 2) != 0;
    /// <summary>802.11g (2.4 GHz, up to 54 Mbps).</summary>
    public bool G => (value & 4) != 0;
    /// <summary>802.11n / Wi-Fi 4 (up to 600 Mbps).</summary>
    public bool N => (value & 8) != 0;
    /// <summary>802.11ac / Wi-Fi 5 (up to 3.5 Gbps).</summary>
    public bool Ac => (value & 16) != 0;
    /// <summary>802.11ax / Wi-Fi 6 (up to 9.6 Gbps).</summary>
    public bool Ax => (value & 32) != 0;
    /// <summary>802.11be / Wi-Fi 7 (up to 46 Gbps).</summary>
    public bool Be => (value & 64) != 0;
}
