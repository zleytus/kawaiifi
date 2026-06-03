namespace Kawaiifi.Net;

/// <summary>802.11 capability information flags advertised by a BSS.</summary>
/// <param name="Ess">The BSS is part of an extended service set (infrastructure mode).</param>
/// <param name="Ibss">The BSS is an independent BSS (ad-hoc mode).</param>
/// <param name="Privacy">The BSS requires authentication/encryption.</param>
/// <param name="ShortPreamble">The BSS supports short preambles.</param>
/// <param name="CriticalUpdateFlag">A critical BSS parameter has changed.</param>
/// <param name="NontransmittedBssidsCriticalUpdateFlag">A critical parameter has changed in a non-transmitted BSSID.</param>
/// <param name="SpectrumManagement">The BSS supports spectrum management (802.11h).</param>
/// <param name="Qos">The BSS supports QoS (802.11e).</param>
/// <param name="ShortSlotTime">The BSS uses short slot time.</param>
/// <param name="Apsd">The BSS supports automatic power save delivery.</param>
/// <param name="RadioMeasurement">The BSS supports radio measurement (802.11k).</param>
/// <param name="Epd">The BSS supports EPD (extended packet delimiter).</param>
public readonly record struct CapabilityInfo(
    bool Ess,
    bool Ibss,
    bool Privacy,
    bool ShortPreamble,
    bool CriticalUpdateFlag,
    bool NontransmittedBssidsCriticalUpdateFlag,
    bool SpectrumManagement,
    bool Qos,
    bool ShortSlotTime,
    bool Apsd,
    bool RadioMeasurement,
    bool Epd);