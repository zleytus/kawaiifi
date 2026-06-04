namespace Kawaiifi.Net;

/// <summary>The frequency band a BSS operates on.</summary>
public enum Band
{
    /// <summary>2.4 GHz band.</summary>
    TwoPointFourGhz,
    /// <summary>5 GHz band.</summary>
    FiveGhz,
    /// <summary>6 GHz band.</summary>
    SixGhz,
}

/// <summary>Extension methods for <see cref="Band"/>.</summary>
public static class BandExtensions
{
    /// <summary>Returns a display string such as <c>2.4 GHz</c>.</summary>
    public static string ToDisplayString(this Band band) => band switch
    {
        Band.TwoPointFourGhz => "2.4 GHz",
        Band.FiveGhz => "5 GHz",
        Band.SixGhz => "6 GHz",
        _ => band.ToString()
    };
}
