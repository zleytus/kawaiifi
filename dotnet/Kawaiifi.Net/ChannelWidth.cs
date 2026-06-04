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

/// <summary>Extension methods for <see cref="ChannelWidth"/>.</summary>
public static class ChannelWidthExtensions
{
    /// <summary>Returns a display string such as <c>20 MHz</c> or <c>80+80 MHz</c>.</summary>
    public static string ToDisplayString(this ChannelWidth channelWidth) => channelWidth switch
    {
        ChannelWidth.TwentyMhz => "20 MHz",
        ChannelWidth.FortyMhz => "40 MHz",
        ChannelWidth.EightyMhz => "80 MHz",
        ChannelWidth.EightyPlusEightyMhz => "80+80 MHz",
        ChannelWidth.OneSixtyMhz => "160 MHz",
        ChannelWidth.ThreeHundredTwentyMhz => "320 MHz",
        _ => channelWidth.ToString()
    };
}
