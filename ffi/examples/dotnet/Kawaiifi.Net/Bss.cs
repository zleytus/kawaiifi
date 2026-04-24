using System.Runtime.InteropServices;
using System.Runtime.Versioning;
using CsBindgen;

namespace Kawaiifi.Net;

/// <summary>A basic service set (BSS).</summary>
public class Bss
{
    private readonly unsafe CsBindgen.Bss* _ptr;

    internal unsafe Bss(CsBindgen.Bss* ptr)
    {
        _ptr = ptr;
    }

    /// <summary>The 6-byte BSSID (MAC address) of the access point.</summary>
    public byte[] Bssid
    {
        get
        {
            unsafe
            {
                return new Span<byte>(NativeMethods.kawaiifi_bss_bssid(_ptr), 6).ToArray();
            }
        }
    }

    /// <summary>The SSID (network name), or an empty string if not present.</summary>
    public string Ssid
    {
        get
        {
            unsafe
            {
                var ssid = NativeMethods.kawaiifi_bss_ssid(_ptr);
                var result = Marshal.PtrToStringUTF8((IntPtr)ssid);
                NativeMethods.kawaiifi_string_free(ssid);
                return result ?? "";
            }
        }
    }

    /// <summary>The operating frequency in MHz.</summary>
    public uint FrequencyMhz
    {
        get
        {
            unsafe
            {
                return NativeMethods.kawaiifi_bss_frequency_mhz(_ptr);
            }
        }
    }

    /// <summary>The frequency band the BSS operates on.</summary>
    public Band Band
    {
        get
        {
            unsafe
            {
                return NativeMethods.kawaiifi_bss_band(_ptr) switch
                {
                    CsBindgen.Band.TwoPointFourGhz => Band.TwoPointFourGhz,
                    CsBindgen.Band.FiveGhz => Band.FiveGhz,
                    CsBindgen.Band.SixGhz => Band.SixGhz,
                    _ => throw new ArgumentOutOfRangeException()
                };
            }
        }
    }

    /// <summary>The channel width used by the BSS.</summary>
    public ChannelWidth ChannelWidth
    {
        get
        {
            unsafe
            {
                return NativeMethods.kawaiifi_bss_channel_width(_ptr) switch
                {
                    CsBindgen.ChannelWidth.TwentyMhz => ChannelWidth.TwentyMhz,
                    CsBindgen.ChannelWidth.FortyMhz => ChannelWidth.FortyMhz,
                    CsBindgen.ChannelWidth.EightyMhz => ChannelWidth.EightyMhz,
                    CsBindgen.ChannelWidth.EightyPlusEightyMhz => ChannelWidth.EightyPlusEightyMhz,
                    CsBindgen.ChannelWidth.OneSixtyMhz => ChannelWidth.OneSixtyMhz,
                    CsBindgen.ChannelWidth.ThreeHundredTwentyMhz => ChannelWidth.ThreeHundredTwentyMhz,
                    _ => throw new ArgumentOutOfRangeException()
                };
            }
        }
    }

    /// <summary>The center frequency of the BSS in MHz.</summary>
    public uint CenterFrequencyMhz
    {
        get
        {
            unsafe
            {
                return NativeMethods.kawaiifi_bss_center_frequency_mhz(_ptr);
            }
        }
    }

    /// <summary>The 802.11 channel number.</summary>
    public byte ChannelNumber
    {
        get
        {
            unsafe
            {
                return NativeMethods.kawaiifi_bss_channel_number(_ptr);
            }
        }
    }

    /// <summary>The received signal strength in dBm.</summary>
    public int SignalDbm
    {
        get
        {
            unsafe
            {
                return NativeMethods.kawaiifi_bss_signal_dbm(_ptr);
            }
        }
    }

    /// <summary>The beacon interval in time units (1 TU = 1024 µs).</summary>
    public ushort BeaconIntervalTu
    {
        get
        {
            unsafe
            {
                return NativeMethods.kawaiifi_bss_beacon_interval_tu(_ptr);
            }
        }
    }

    /// <summary>The beacon interval in milliseconds.</summary>
    public double BeaconIntervalMs
    {
        get
        {
            unsafe
            {
                return NativeMethods.kawaiifi_bss_beacon_interval_ms(_ptr);
            }
        }
    }

    /// <summary>The 802.11 capability information flags advertised by the BSS.</summary>
    public CapabilityInfo CapabilityInfo
    {
        get
        {
            unsafe
            {
                var capabilityInfo = NativeMethods.kawaiifi_bss_capability_info(_ptr);
                return new CapabilityInfo(capabilityInfo.ess, capabilityInfo.ibss, capabilityInfo.privacy,
                    capabilityInfo.short_preamble, capabilityInfo.critical_update_flag,
                    capabilityInfo.nontransmitted_bssids_critical_update_flag, capabilityInfo.spectrum_management,
                    capabilityInfo.qos, capabilityInfo.short_slot_time, capabilityInfo.apsd,
                    capabilityInfo.radio_measurement, capabilityInfo.epd);
            }
        }
    }

    /// <summary>The timing synchronization function (TSF) timer value.</summary>
    public ulong Tsf
    {
        get
        {
            unsafe
            {
                return NativeMethods.kawaiifi_bss_tsf(_ptr);
            }
        }
    }

    /// <summary>The estimated time the BSS has been running, derived from its TSF timer.</summary>
    public TimeSpan Uptime => TimeSpan.FromMicroseconds(Tsf);

    /// <summary>The Unix timestamp in milliseconds when the BSS was last seen, or null if unavailable.</summary>
    public long? LastSeenUtcMs
    {
        get
        {
            unsafe
            {
                long lastSeenUtcMs = 0;
                return NativeMethods.kawaiifi_bss_last_seen_utc_ms(_ptr, &lastSeenUtcMs) ? lastSeenUtcMs : null;
            }
        }
    }

    /// <summary>The security protocols supported by the BSS.</summary>
    public SecurityProtocols SecurityProtocols
    {
        get
        {
            unsafe
            {
                return new SecurityProtocols(NativeMethods.kawaiifi_bss_security_protocols(_ptr));
            }
        }
    }

    /// <summary>The Wi-Fi protocols supported by the BSS.</summary>
    public WifiProtocols WifiProtocols
    {
        get
        {
            unsafe
            {
                return new WifiProtocols(NativeMethods.kawaiifi_bss_wifi_protocols(_ptr));
            }
        }
    }

    /// <summary>The Wi-Fi amendments supported by the BSS.</summary>
    public WifiAmendments WifiAmendments
    {
        get
        {
            unsafe
            {
                return new WifiAmendments(NativeMethods.kawaiifi_bss_wifi_amendments(_ptr));
            }
        }
    }

    /// <summary>The maximum supported data rate in Mbps.</summary>
    public double MaxRateMbps
    {
        get
        {
            unsafe
            {
                return NativeMethods.kawaiifi_bss_max_rate_mbps(_ptr);
            }
        }
    }

    /// <summary>The fraction of time the BSS's channel is busy, as a value from 0 to 255, where 255 represents 100%.</summary>
    public byte? ChannelUtilization
    {
        get
        {
            unsafe
            {
                byte channelUtilization = 0;
                return NativeMethods.kawaiifi_bss_channel_utilization(_ptr, &channelUtilization)
                    ? channelUtilization
                    : null;
            }
        }
    }

    /// <summary>The number of devices associated with the BSS.</summary>
    public ushort? StationCount
    {
        get
        {
            unsafe
            {
                ushort stationCount = 0;
                return NativeMethods.kawaiifi_bss_station_count(_ptr, &stationCount)
                    ? stationCount
                    : null;
            }
        } 
    }

    /// <summary>
    /// The information elements (IEs) included in the BSS's beacon or probe response.
    /// The returned <see cref="Ie"/> instances borrow memory owned by the parent <see cref="Scan"/>
    /// and must not be used after it is disposed.
    /// </summary>
    public IReadOnlyList<Ie> Ies
    {
        get
        {
            unsafe
            {
                var count = (int)NativeMethods.kawaiifi_bss_ie_count(_ptr);
                var ies = new Ie[count];
                for (var i = 0; i < count; i++)
                {
                    ies[i] = new Ie(NativeMethods.kawaiifi_bss_ie_get(_ptr, (nuint)i));
                }

                return ies;
            }
        }
    }

    /// <summary>The link quality of the BSS as a value from 0 to 100.</summary>
    [SupportedOSPlatform("windows")]
    public byte LinkQuality
    {
        get
        {
            unsafe
            {
                return NativeMethods.kawaiifi_bss_link_quality(_ptr);
            }
        }
    }

    /// <summary>Whether this BSS's information came from a probe response rather than a beacon.</summary>
    [SupportedOSPlatform("linux")]
    public bool IsFromProbeResponse
    {
        get
        {
            unsafe
            {
                return NativeMethods.kawaiifi_bss_is_from_probe_response(_ptr);
            }
        }
    }

    /// <summary>The 6-byte BSSID of the transmitting BSS for a non-transmitted BSS, or null if unavailable.</summary>
    [SupportedOSPlatform("linux")]
    public byte[]? ParentBssid
    {
        get
        {
            unsafe
            {
                var ptr = NativeMethods.kawaiifi_bss_parent_bssid(_ptr);
                return ptr == null ? null : new Span<byte>(ptr, 6).ToArray();
            }
        }
    }

    /// <summary>The TSF timer value of the transmitting BSS, or null if unavailable.</summary>
    [SupportedOSPlatform("linux")]
    public ulong? ParentTsf
    {
        get
        {
            unsafe
            {
                ulong val = 0;
                return NativeMethods.kawaiifi_bss_parent_tsf(_ptr, &val) ? val : null;
            }
        }
    }

    /// <summary>The TSF timer value from the last beacon, or null if unavailable.</summary>
    [SupportedOSPlatform("linux")]
    public ulong? BeaconTsf
    {
        get
        {
            unsafe
            {
                ulong val = 0;
                return NativeMethods.kawaiifi_bss_beacon_tsf(_ptr, &val) ? val : null;
            }
        }
    }

    /// <summary>The frequency offset of the BSS in kHz, or null if unavailable.</summary>
    [SupportedOSPlatform("linux")]
    public uint? FrequencyOffsetKhz
    {
        get
        {
            unsafe
            {
                uint val = 0;
                return NativeMethods.kawaiifi_bss_frequency_offset_khz(_ptr, &val) ? val : null;
            }
        }
    }

    /// <summary>The signal strength as a percentage (0–100), or null if unavailable.</summary>
    [SupportedOSPlatform("linux")]
    public byte? SignalPercent
    {
        get
        {
            unsafe
            {
                byte val = 0;
                return NativeMethods.kawaiifi_bss_signal_percent(_ptr, &val) ? val : null;
            }
        }
    }

    /// <summary>The time the BSS was last seen as nanoseconds since boot, or null if unavailable.</summary>
    [SupportedOSPlatform("linux")]
    public ulong? LastSeenBoottimeNs
    {
        get
        {
            unsafe
            {
                ulong val = 0;
                return NativeMethods.kawaiifi_bss_last_seen_boottime_ns(_ptr, &val) ? val : null;
            }
        }
    }

    /// <summary>The number of milliseconds since the BSS was last seen, or null if unavailable.</summary>
    [SupportedOSPlatform("linux")]
    public uint? SeenMsAgo
    {
        get
        {
            unsafe
            {
                uint val = 0;
                return NativeMethods.kawaiifi_bss_seen_ms_ago(_ptr, &val) ? val : null;
            }
        }
    }

    /// <summary>The Multi-Link Operation (MLO) link ID, or null if unavailable.</summary>
    [SupportedOSPlatform("linux")]
    public byte? MloLinkId
    {
        get
        {
            unsafe
            {
                byte val = 0;
                return NativeMethods.kawaiifi_bss_mlo_link_id(_ptr, &val) ? val : null;
            }
        }
    }

    /// <summary>The 6-byte MLD address of the BSS, or null if unavailable.</summary>
    [SupportedOSPlatform("linux")]
    public byte[]? MldAddress
    {
        get
        {
            unsafe
            {
                var ptr = NativeMethods.kawaiifi_bss_mld_address(_ptr);
                return ptr == null ? null : new Span<byte>(ptr, 6).ToArray();
            }
        }
    }
}