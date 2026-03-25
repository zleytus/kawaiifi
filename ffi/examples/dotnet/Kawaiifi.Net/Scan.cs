using System.Runtime.Versioning;
using CsBindgen;

namespace Kawaiifi.Net;

/// <summary>
/// The results of a Wi-Fi scan. Dispose this instance when done to free the underlying native memory.
/// </summary>
public class Scan : IDisposable
{
    private readonly unsafe CsBindgen.Scan* _ptr;
    private bool _disposed;

    internal unsafe Scan(CsBindgen.Scan* ptr)
    {
        _ptr = ptr;
    }

    /// <inheritdoc/>
    public void Dispose()
    {
        if (_disposed) return;
        _disposed = true;
        unsafe
        {
            NativeMethods.kawaiifi_scan_free(_ptr);
        }

        GC.SuppressFinalize(this);
    }

    /// <inheritdoc/>
    ~Scan()
    {
        Dispose();
    }

    /// <summary>
    /// The list of BSSs discovered during the scan. The returned <see cref="Bss"/> instances
    /// borrow memory owned by this <see cref="Scan"/> and must not be used after it is disposed.
    /// </summary>
    public IReadOnlyList<Bss> BssList
    {
        get
        {
            unsafe
            {
                var count = (int)NativeMethods.kawaiifi_scan_bss_count(_ptr);
                var bssList = new Bss[count];
                for (var i = 0; i < count; i++)
                {
                    bssList[i] = new Bss(NativeMethods.kawaiifi_scan_bss_get(_ptr, (nuint)i));
                }

                return bssList;
            }
        }
    }

    /// <summary>The Unix timestamp in milliseconds when the scan started.</summary>
    public long StartTimeUtcMs
    {
        get
        {
            unsafe
            {
                return NativeMethods.kawaiifi_scan_start_time_utc_ms(_ptr);
            }
        }
    }

    /// <summary>The Unix timestamp in milliseconds when the scan ended.</summary>
    public long EndTimeUtcMs
    {
        get
        {
            unsafe
            {
                return NativeMethods.kawaiifi_scan_end_time_utc_ms(_ptr);
            }
        }
    }

    /// <summary>The wiphy index of the radio that performed the scan.</summary>
    [SupportedOSPlatform("linux")]
    public uint Wiphy
    {
        get
        {
            unsafe
            {
                return NativeMethods.kawaiifi_scan_wiphy(_ptr);
            }
        }
    }

    /// <summary>The interface index (ifindex) that performed the scan.</summary>
    [SupportedOSPlatform("linux")]
    public uint IfIndex
    {
        get
        {
            unsafe
            {
                return NativeMethods.kawaiifi_scan_ifindex(_ptr);
            }
        }
    }

    /// <summary>The frequencies in MHz that were scanned.</summary>
    [SupportedOSPlatform("linux")]
    public uint[] FrequenciesMhz
    {
        get
        {
            unsafe
            {
                nuint count = 0;
                var ptr = NativeMethods.kawaiifi_scan_freqs_mhz(_ptr, &count);
                return new Span<uint>(ptr, (int)count).ToArray();
            }
        }
    }

    /// <summary>The information elements included in the scan probe request.</summary>
    [SupportedOSPlatform("linux")]
    public IReadOnlyList<Ie> Ies
    {
        get
        {
            unsafe
            {
                var count = (int)NativeMethods.kawaiifi_scan_ie_count(_ptr);
                var ies = new Ie[count];
                for (var i = 0; i < count; i++)
                {
                    ies[i] = new Ie(NativeMethods.kawaiifi_scan_ie_get(_ptr, (nuint)i));
                }

                return ies;
            }
        }
    }

    /// <summary>The scan flags that were set when the scan was initiated, or null if unavailable.</summary>
    [SupportedOSPlatform("linux")]
    public Flags? Flags
    {
        get
        {
            unsafe
            {
                var flags = new CsBindgen.Flags();
                if (NativeMethods.kawaiifi_scan_flags(_ptr, &flags))
                {
                    return new Flags(flags.low_priority, flags.flush, flags.ap, flags.random_addr,
                        flags.fils_max_channel_time,
                        flags.accept_bcast_probe_resp, flags.oce_probe_req_high_tx_rate,
                        flags.oce_probe_req_deferral_suppression, flags.low_span, flags.low_power, flags.high_accuracy,
                        flags.random_sn, flags.min_preq_content, flags.freq_khz, flags.colocated_6ghz);
                }

                return null;
            }
        }
    }
}

/// <summary>Flags that control or describe the behavior of a Wi-Fi scan.</summary>
/// <param name="LowPriority">The scan can be delayed to allow normal data transmission to proceed.</param>
/// <param name="Flush">Cached scan results were flushed before the scan started.</param>
/// <param name="Ap">The scan was forced even though the interface is in AP mode.</param>
/// <param name="RandomAddr">A random MAC address was used for probe requests.</param>
/// <param name="FilsMaxChannelTime">The dwell time was filled in the FILS request parameters IE.</param>
/// <param name="AcceptBcastProbeResp">Broadcast probe responses were accepted.</param>
/// <param name="OceProbeReqHighTxRate">Probe requests were sent at a rate of at least 5.5 Mbps.</param>
/// <param name="OceProbeReqDeferralSuppression">Probe request TX deferral and suppression was allowed.</param>
/// <param name="LowSpan">The scan used minimal time on each channel.</param>
/// <param name="LowPower">The scan was performed at lower power.</param>
/// <param name="HighAccuracy">The scan used the highest accuracy to find all available networks.</param>
/// <param name="RandomSn">Random sequence numbers were used in probe requests.</param>
/// <param name="MinPreqContent">Probe requests used minimal content.</param>
/// <param name="FrequencyKhz">Frequencies are specified in kHz rather than MHz.</param>
/// <param name="Colocated6Ghz">Colocated 6 GHz APs were discovered through RNR.</param>
[SupportedOSPlatform("linux")]
public readonly record struct Flags(
    bool LowPriority,
    bool Flush,
    bool Ap,
    bool RandomAddr,
    bool FilsMaxChannelTime,
    bool AcceptBcastProbeResp,
    bool OceProbeReqHighTxRate,
    bool OceProbeReqDeferralSuppression,
    bool LowSpan,
    bool LowPower,
    bool HighAccuracy,
    bool RandomSn,
    bool MinPreqContent,
    bool FrequencyKhz,
    bool Colocated6Ghz);

/// <summary>The scan backend to use when performing a Wi-Fi scan on Linux.</summary>
[SupportedOSPlatform("linux")]
public enum Backend
{
    /// <summary>Use the nl80211 netlink interface directly.</summary>
    Nl80211,

    /// <summary>Use NetworkManager as the scan backend.</summary>
    NetworkManager
}