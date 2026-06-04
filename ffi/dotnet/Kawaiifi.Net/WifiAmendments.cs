namespace Kawaiifi.Net;

/// <summary>The Wi-Fi amendments supported by a BSS.</summary>
public readonly struct WifiAmendments(ushort value)
{
    /// <summary>802.11d</summary>
    public bool D => (value & 1) != 0;
    /// <summary>802.11e</summary>
    public bool E => (value & 2) != 0;
    /// <summary>802.11h</summary>
    public bool H => (value & 4) != 0;
    /// <summary>802.11i</summary>
    public bool I => (value & 8) != 0;
    /// <summary>802.11k</summary>
    public bool K => (value & 16) != 0;
    /// <summary>802.11r</summary>
    public bool R => (value & 32) != 0;
    /// <summary>802.11s</summary>
    public bool S => (value & 64) != 0;
    /// <summary>802.11v</summary>
    public bool V => (value & 128) != 0;
    /// <summary>802.11w</summary>
    public bool W => (value & 256) != 0;

    /// <inheritdoc />
    public override string ToString()
    {
        var amendments = new List<string>();
        if (D) amendments.Add("d");
        if (E) amendments.Add("e");
        if (H) amendments.Add("h");
        if (I) amendments.Add("i");
        if (K) amendments.Add("k");
        if (R) amendments.Add("r");
        if (S) amendments.Add("s");
        if (V) amendments.Add("v");
        if (W) amendments.Add("w");

        return string.Join("/", amendments);
    }
}
