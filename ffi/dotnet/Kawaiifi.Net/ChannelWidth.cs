namespace Kawaiifi.Net;

/// <summary>The channel width used by a BSS or interface.</summary>
public enum ChannelWidth
{
    /// <summary>20 MHz channel width.</summary>
    TwentyMhz,
    /// <summary>40 MHz channel width.</summary>
    FortyMhz,
    /// <summary>80 MHz channel width.</summary>
    EightyMhz,
    /// <summary>80+80 MHz non-contiguous channel width.</summary>
    EightyPlusEightyMhz,
    /// <summary>160 MHz channel width.</summary>
    OneSixtyMhz,
    /// <summary>320 MHz channel width.</summary>
    ThreeHundredTwentyMhz,
}
