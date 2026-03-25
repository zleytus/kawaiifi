using System.Runtime.InteropServices;
using System.Runtime.Versioning;
using CsBindgen;

namespace Kawaiifi.Net;

/// <summary>
/// A wireless network interface. Dispose this instance when done to free the underlying native memory.
/// Interfaces obtained from an <c>InterfaceList</c> are borrowed and must not be disposed individually.
/// </summary>
public class Interface : IDisposable
{
    private readonly unsafe CsBindgen.Interface* _ptr;
    private readonly bool _owned;
    private bool _disposed;

    private unsafe Interface(CsBindgen.Interface* ptr, bool owned)
    {
        _ptr = ptr;
        _owned = owned;
    }

    private static unsafe Interface? FromOwned(CsBindgen.Interface* ptr) =>
        ptr == null ? null : new Interface(ptr, owned: true);

    internal static unsafe Interface FromBorrowed(CsBindgen.Interface* ptr) =>
        new Interface(ptr, owned: false);

    /// <summary>
    /// Returns all available wireless interfaces as a disposable list.
    /// Dispose the returned <see cref="InterfaceList"/> when done.
    /// Do not dispose individual interfaces obtained from the list.
    /// </summary>
    public static InterfaceList All()
    {
        unsafe
        {
            return new InterfaceList(NativeMethods.kawaiifi_interfaces());
        }
    }

    /// <summary>
    /// Returns the default wireless interface, or null if none is found.
    /// Dispose the returned instance when done.
    /// </summary>
    public static Interface? Default()
    {
        unsafe
        {
            return FromOwned(NativeMethods.kawaiifi_default_interface());
        }
    }

    /// <inheritdoc/>
    public void Dispose()
    {
        if (_disposed) return;
        _disposed = true;
        if (_owned)
        {
            unsafe
            {
                NativeMethods.kawaiifi_interface_free(_ptr);
            }
        }

        GC.SuppressFinalize(this);
    }

    /// <inheritdoc/>
    ~Interface()
    {
        Dispose();
    }

    /// <summary>The name of the interface (e.g. "wlan0").</summary>
    [SupportedOSPlatform("linux")]
    public string Name
    {
        get
        {
            unsafe
            {
                var name = NativeMethods.kawaiifi_interface_name(_ptr);
                var result = Marshal.PtrToStringUTF8((IntPtr)name);
                NativeMethods.kawaiifi_string_free(name);
                return result ?? "";
            }
        }
    }

    /// <summary>The interface index (ifindex).</summary>
    [SupportedOSPlatform("linux")]
    public uint Index
    {
        get
        {
            unsafe
            {
                return NativeMethods.kawaiifi_interface_index(_ptr);
            }
        }
    }

    /// <summary>The wiphy index of the underlying radio.</summary>
    [SupportedOSPlatform("linux")]
    public uint Wiphy
    {
        get
        {
            unsafe
            {
                return NativeMethods.kawaiifi_interface_wiphy(_ptr);
            }
        }
    }

    /// <summary>The wireless device identifier.</summary>
    [SupportedOSPlatform("linux")]
    public ulong Wdev
    {
        get
        {
            unsafe
            {
                return NativeMethods.kawaiifi_interface_wdev(_ptr);
            }
        }
    }

    /// <summary>The 6-byte MAC address of the interface.</summary>
    [SupportedOSPlatform("linux")]
    public byte[] MacAddress
    {
        get
        {
            unsafe
            {
                var macAddress = stackalloc byte[6];
                NativeMethods.kawaiifi_interface_mac_address(_ptr, macAddress);
                return new Span<byte>(macAddress, 6).ToArray();
            }
        }
    }

    /// <summary>The nl80211 generation counter for this interface.</summary>
    [SupportedOSPlatform("linux")]
    public uint Generation
    {
        get
        {
            unsafe
            {
                return NativeMethods.kawaiifi_interface_generation(_ptr);
            }
        }
    }

    /// <summary>Whether 4-address (WDS) mode is enabled on the interface.</summary>
    [SupportedOSPlatform("linux")]
    public bool FourAddress
    {
        get
        {
            unsafe
            {
                return NativeMethods.kawaiifi_interface_four_address(_ptr);
            }
        }
    }

    /// <summary>The SSID the interface is currently associated with, or null if not associated.</summary>
    [SupportedOSPlatform("linux")]
    public string? Ssid
    {
        get
        {
            unsafe
            {
                var ssid = NativeMethods.kawaiifi_interface_ssid(_ptr);
                if (ssid == null) return null;
                var result = Marshal.PtrToStringUTF8((IntPtr)ssid);
                NativeMethods.kawaiifi_string_free(ssid);
                return result;
            }
        }
    }

    /// <summary>The frequency the wiphy is tuned to in MHz, or null if unavailable.</summary>
    [SupportedOSPlatform("linux")]
    public uint? WiphyFrequencyMhz
    {
        get
        {
            unsafe
            {
                uint val = 0;
                return NativeMethods.kawaiifi_interface_wiphy_freq_mhz(_ptr, &val) ? val : null;
            }
        }
    }

    /// <summary>The frequency offset of the wiphy in kHz, or null if unavailable.</summary>
    [SupportedOSPlatform("linux")]
    public uint? WiphyFrequencyOffsetKhz
    {
        get
        {
            unsafe
            {
                uint val = 0;
                return NativeMethods.kawaiifi_interface_wiphy_freq_offset_khz(_ptr, &val) ? val : null;
            }
        }
    }

    /// <summary>The transmit power level of the wiphy in mBm, or null if unavailable.</summary>
    [SupportedOSPlatform("linux")]
    public uint? WiphyTxPowerLevelMbm
    {
        get
        {
            unsafe
            {
                uint wiphyTxPowerLevelMbm = 0;
                return NativeMethods.kawaiifi_interface_wiphy_tx_power_level_mbm(_ptr, &wiphyTxPowerLevelMbm)
                    ? wiphyTxPowerLevelMbm
                    : null;
            }
        }
    }

    /// <summary>The center frequency of the primary channel segment in MHz, or null if unavailable.</summary>
    [SupportedOSPlatform("linux")]
    public uint? CenterFrequency1Mhz
    {
        get
        {
            unsafe
            {
                uint val = 0;
                return NativeMethods.kawaiifi_interface_center_freq_1_mhz(_ptr, &val) ? val : null;
            }
        }
    }

    /// <summary>The center frequency of the secondary channel segment in MHz (80+80 MHz only), or null if unavailable.</summary>
    [SupportedOSPlatform("linux")]
    public uint? CenterFrequency2Mhz
    {
        get
        {
            unsafe
            {
                uint val = 0;
                return NativeMethods.kawaiifi_interface_center_freq_2_mhz(_ptr, &val) ? val : null;
            }
        }
    }

    /// <summary>The current channel width, or null if unavailable.</summary>
    [SupportedOSPlatform("linux")]
    public ChannelWidth? ChannelWidth
    {
        get
        {
            unsafe
            {
                CsBindgen.ChannelWidth channelWidth = default;
                if (!NativeMethods.kawaiifi_interface_channel_width(_ptr, &channelWidth)) return null;
                return channelWidth switch
                {
                    CsBindgen.ChannelWidth.TwentyMhz => Net.ChannelWidth.TwentyMhz,
                    CsBindgen.ChannelWidth.FortyMhz => Net.ChannelWidth.FortyMhz,
                    CsBindgen.ChannelWidth.EightyMhz => Net.ChannelWidth.EightyMhz,
                    CsBindgen.ChannelWidth.EightyPlusEightyMhz => Net.ChannelWidth.EightyPlusEightyMhz,
                    CsBindgen.ChannelWidth.OneSixtyMhz => Net.ChannelWidth.OneSixtyMhz,
                    CsBindgen.ChannelWidth.ThreeHundredTwentyMhz => Net.ChannelWidth.ThreeHundredTwentyMhz,
                    _ => null,
                };
            }
        }
    }

    /// <summary>The virtual interface radio mask, or null if unavailable.</summary>
    [SupportedOSPlatform("linux")]
    public uint? VifRadioMask
    {
        get
        {
            unsafe
            {
                uint vifRadioMask = 0;
                return NativeMethods.kawaiifi_interface_vif_radio_mask(_ptr, &vifRadioMask) ? vifRadioMask : null;
            }
        }
    }

    /// <summary>The PCI/USB vendor ID, or null if unavailable.</summary>
    [SupportedOSPlatform("linux")]
    public ushort? VendorId
    {
        get
        {
            unsafe
            {
                ushort vendorId = 0;
                return NativeMethods.kawaiifi_interface_vendor_id(_ptr, &vendorId) ? vendorId : null;
            }
        }
    }

    /// <summary>The PCI/USB device ID, or null if unavailable.</summary>
    [SupportedOSPlatform("linux")]
    public ushort? DeviceId
    {
        get
        {
            unsafe
            {
                ushort deviceId = 0;
                return NativeMethods.kawaiifi_interface_device_id(_ptr, &deviceId) ? deviceId : null;
            }
        }
    }

    /// <summary>The hardware vendor name (e.g. "Intel Corporation"), or null if unavailable.</summary>
    [SupportedOSPlatform("linux")]
    public string? VendorName
    {
        get
        {
            unsafe
            {
                var vendorName = NativeMethods.kawaiifi_interface_vendor_name(_ptr);
                if (vendorName == null) return null;
                var result = Marshal.PtrToStringUTF8((IntPtr)vendorName);
                NativeMethods.kawaiifi_string_free(vendorName);
                return result;
            }
        }
    }

    /// <summary>The hardware device name (e.g. "Wi-Fi 6 AX200"), or null if unavailable.</summary>
    [SupportedOSPlatform("linux")]
    public string? DeviceName
    {
        get
        {
            unsafe
            {
                var deviceName = NativeMethods.kawaiifi_interface_device_name(_ptr);
                if (deviceName == null) return null;
                var result = Marshal.PtrToStringUTF8((IntPtr)deviceName);
                NativeMethods.kawaiifi_string_free(deviceName);
                return result;
            }
        }
    }

    /// <summary>The kernel driver name (e.g. "iwlwifi"), or null if unavailable.</summary>
    [SupportedOSPlatform("linux")]
    public string? Driver
    {
        get
        {
            unsafe
            {
                var driver = NativeMethods.kawaiifi_interface_driver(_ptr);
                if (driver == null) return null;
                var result = Marshal.PtrToStringUTF8((IntPtr)driver);
                NativeMethods.kawaiifi_string_free(driver);
                return result;
            }
        }
    }

    /// <summary>The hardware bus type, or null if unavailable.</summary>
    [SupportedOSPlatform("linux")]
    public BusType? BusType
    {
        get
        {
            unsafe
            {
                return NativeMethods.kawaiifi_interface_bus_type(_ptr) switch
                {
                    CsBindgen.BusType.Pci => Net.BusType.Pci,
                    CsBindgen.BusType.Usb => Net.BusType.Usb,
                    CsBindgen.BusType.Sdio => Net.BusType.Sdio,
                    _ => null,
                };
            }
        }
    }

    /// <summary>The GUID that uniquely identifies this interface.</summary>
    [SupportedOSPlatform("windows")]
    public Guid Guid
    {
        get
        {
            unsafe
            {
                return NativeMethods.kawaiifi_interface_guid(_ptr);
            }
        }
    }

    /// <summary>The human-readable description of the interface, or null if unavailable.</summary>
    [SupportedOSPlatform("windows")]
    public string? Description
    {
        get
        {
            unsafe
            {
                var description = NativeMethods.kawaiifi_interface_description(_ptr);
                if (description == null) return null;
                var result = Marshal.PtrToStringUTF8((IntPtr)description);
                NativeMethods.kawaiifi_string_free(description);
                return result;
            }
        }
    }

    /// <summary>
    /// Performs a blocking Wi-Fi scan using the specified backend and returns the results.
    /// Dispose the returned <see cref="Kawaiifi.Net.Scan"/> when done.
    /// </summary>
    [SupportedOSPlatform("linux")]
    public Scan Scan(Backend backend)
    {
        unsafe
        {
            var scan = backend switch
            {
                Backend.Nl80211 => NativeMethods.kawaiifi_interface_scan(_ptr, CsBindgen.Backend.Nl80211),
                Backend.NetworkManager => NativeMethods.kawaiifi_interface_scan(_ptr, CsBindgen.Backend.NetworkManager),
                _ => throw new ArgumentOutOfRangeException(nameof(backend)),
            };

            return new Scan(scan);
        }
    }

    /// <summary>
    /// Performs a blocking Wi-Fi scan and returns the results.
    /// Dispose the returned <see cref="Kawaiifi.Net.Scan"/> when done.
    /// </summary>
    [SupportedOSPlatform("windows")]
    public Scan Scan()
    {
        unsafe
        {
            var scan = NativeMethods.kawaiifi_interface_scan(_ptr);
            return new Scan(scan);
        }
    }
}