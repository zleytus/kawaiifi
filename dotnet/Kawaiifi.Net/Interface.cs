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

    private unsafe CsBindgen.Interface* Ptr
    {
        get
        {
            ObjectDisposedException.ThrowIf(_disposed, this);
            return _ptr;
        }
    }

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
    [SupportedOSPlatform("macos")]
    public string Name
    {
        get
        {
            unsafe
            {
                if (OperatingSystem.IsLinux())
                {
                    var name = NativeMethodsLinux.kawaiifi_interface_name(Ptr);
                    var result = Marshal.PtrToStringUTF8((IntPtr)name);
                    NativeMethods.kawaiifi_string_free(name);
                    return result ?? "";
                }

                if (OperatingSystem.IsMacOS())
                {
                    var name = NativeMethodsMacOS.kawaiifi_interface_name(Ptr);
                    var result = Marshal.PtrToStringUTF8((IntPtr)name);
                    NativeMethods.kawaiifi_string_free(name);
                    return result ?? "";
                }
            }

            throw new PlatformNotSupportedException();
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
                return NativeMethodsLinux.kawaiifi_interface_index(Ptr);
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
                return NativeMethodsLinux.kawaiifi_interface_wiphy(Ptr);
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
                return NativeMethodsLinux.kawaiifi_interface_wdev(Ptr);
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
                NativeMethodsLinux.kawaiifi_interface_mac_address(Ptr, macAddress);
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
                return NativeMethodsLinux.kawaiifi_interface_generation(Ptr);
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
                return NativeMethodsLinux.kawaiifi_interface_four_address(Ptr);
            }
        }
    }

    /// <summary>The SSID the interface is currently associated with, or null if not associated.</summary>
    [SupportedOSPlatform("linux")]
    [SupportedOSPlatform("macos")]
    public string? Ssid
    {
        get
        {
            unsafe
            {
                if (OperatingSystem.IsLinux())
                {
                    var ssid = NativeMethodsLinux.kawaiifi_interface_ssid(Ptr);
                    if (ssid == null) return null;
                    var result = Marshal.PtrToStringUTF8((IntPtr)ssid);
                    NativeMethods.kawaiifi_string_free(ssid);
                    return result;
                }

                if (OperatingSystem.IsMacOS())
                {
                    var ssid = NativeMethodsMacOS.kawaiifi_interface_ssid(Ptr);
                    if (ssid == null) return null;
                    var result = Marshal.PtrToStringUTF8((IntPtr)ssid);
                    NativeMethods.kawaiifi_string_free(ssid);
                    return result;
                }
            }

            throw new PlatformNotSupportedException();
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
                return NativeMethodsLinux.kawaiifi_interface_wiphy_freq_mhz(Ptr, &val) ? val : null;
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
                return NativeMethodsLinux.kawaiifi_interface_wiphy_freq_offset_khz(Ptr, &val) ? val : null;
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
                return NativeMethodsLinux.kawaiifi_interface_wiphy_tx_power_level_mbm(Ptr, &wiphyTxPowerLevelMbm)
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
                return NativeMethodsLinux.kawaiifi_interface_center_freq_1_mhz(Ptr, &val) ? val : null;
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
                return NativeMethodsLinux.kawaiifi_interface_center_freq_2_mhz(Ptr, &val) ? val : null;
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
                if (!NativeMethodsLinux.kawaiifi_interface_channel_width(Ptr, &channelWidth)) return null;
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
                return NativeMethodsLinux.kawaiifi_interface_vif_radio_mask(Ptr, &vifRadioMask) ? vifRadioMask : null;
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
                return NativeMethodsLinux.kawaiifi_interface_vendor_id(Ptr, &vendorId) ? vendorId : null;
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
                return NativeMethodsLinux.kawaiifi_interface_device_id(Ptr, &deviceId) ? deviceId : null;
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
                var vendorName = NativeMethodsLinux.kawaiifi_interface_vendor_name(Ptr);
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
                var deviceName = NativeMethodsLinux.kawaiifi_interface_device_name(Ptr);
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
                var driver = NativeMethodsLinux.kawaiifi_interface_driver(Ptr);
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
                return NativeMethodsLinux.kawaiifi_interface_bus_type(Ptr) switch
                {
                    CsBindgen.BusType.Pci => Net.BusType.Pci,
                    CsBindgen.BusType.Usb => Net.BusType.Usb,
                    CsBindgen.BusType.Sdio => Net.BusType.Sdio,
                    _ => null,
                };
            }
        }
    }

    /// <summary>Whether the Wi-Fi interface is powered on.</summary>
    [SupportedOSPlatform("macos")]
    public bool PowerOn
    {
        get
        {
            unsafe
            {
                return NativeMethodsMacOS.kawaiifi_interface_power_on(Ptr);
            }
        }
    }

    /// <summary>The currently active PHY mode.</summary>
    [SupportedOSPlatform("macos")]
    public CwPhyMode ActivePhyMode
    {
        get
        {
            unsafe
            {
                return NativeMethodsMacOS.kawaiifi_interface_active_phy_mode(Ptr) switch
                {
                    CsBindgen.CwPhyMode.A => CwPhyMode.A,
                    CsBindgen.CwPhyMode.B => CwPhyMode.B,
                    CsBindgen.CwPhyMode.G => CwPhyMode.G,
                    CsBindgen.CwPhyMode.N => CwPhyMode.N,
                    CsBindgen.CwPhyMode.AC => CwPhyMode.Ac,
                    CsBindgen.CwPhyMode.AX => CwPhyMode.Ax,
                    _ => CwPhyMode.None,
                };
            }
        }
    }

    /// <summary>The current security type.</summary>
    [SupportedOSPlatform("macos")]
    public CwSecurity Security
    {
        get
        {
            unsafe
            {
                return NativeMethodsMacOS.kawaiifi_interface_security(Ptr) switch
                {
                    CsBindgen.CwSecurity.None => CwSecurity.None,
                    CsBindgen.CwSecurity.Wep => CwSecurity.Wep,
                    CsBindgen.CwSecurity.WpaPersonal => CwSecurity.WpaPersonal,
                    CsBindgen.CwSecurity.WpaPersonalMixed => CwSecurity.WpaPersonalMixed,
                    CsBindgen.CwSecurity.Wpa2Personal => CwSecurity.Wpa2Personal,
                    CsBindgen.CwSecurity.Personal => CwSecurity.Personal,
                    CsBindgen.CwSecurity.DynamicWep => CwSecurity.DynamicWep,
                    CsBindgen.CwSecurity.WpaEnterprise => CwSecurity.WpaEnterprise,
                    CsBindgen.CwSecurity.WpaEnterpriseMixed => CwSecurity.WpaEnterpriseMixed,
                    CsBindgen.CwSecurity.Wpa2Enterprise => CwSecurity.Wpa2Enterprise,
                    CsBindgen.CwSecurity.Enterprise => CwSecurity.Enterprise,
                    CsBindgen.CwSecurity.Wpa3Personal => CwSecurity.Wpa3Personal,
                    CsBindgen.CwSecurity.Wpa3Enterprise => CwSecurity.Wpa3Enterprise,
                    CsBindgen.CwSecurity.Wpa3Transition => CwSecurity.Wpa3Transition,
                    CsBindgen.CwSecurity.Owe => CwSecurity.Owe,
                    CsBindgen.CwSecurity.OweTransition => CwSecurity.OweTransition,
                    _ => CwSecurity.Unknown,
                };
            }
        }
    }

    /// <summary>The current operating mode.</summary>
    [SupportedOSPlatform("macos")]
    public CwInterfaceMode Mode
    {
        get
        {
            unsafe
            {
                return NativeMethodsMacOS.kawaiifi_interface_mode(Ptr) switch
                {
                    CsBindgen.CwInterfaceMode.Station => CwInterfaceMode.Station,
                    CsBindgen.CwInterfaceMode.Ibss => CwInterfaceMode.Ibss,
                    CsBindgen.CwInterfaceMode.HostAp => CwInterfaceMode.HostAp,
                    _ => CwInterfaceMode.None,
                };
            }
        }
    }

    /// <summary>The current transmit rate in Mbps.</summary>
    [SupportedOSPlatform("macos")]
    public double TransmitRateMbps
    {
        get
        {
            unsafe
            {
                return NativeMethodsMacOS.kawaiifi_interface_transmit_rate_mbps(Ptr);
            }
        }
    }

    /// <summary>The current transmit power in mW.</summary>
    [SupportedOSPlatform("macos")]
    public int TransmitPowerMw
    {
        get
        {
            unsafe
            {
                return NativeMethodsMacOS.kawaiifi_interface_transmit_power_mw(Ptr);
            }
        }
    }

    /// <summary>The currently adopted country code.</summary>
    [SupportedOSPlatform("macos")]
    public string? CountryCode
    {
        get
        {
            unsafe
            {
                var countryCode = NativeMethodsMacOS.kawaiifi_interface_country_code(Ptr);
                if (countryCode == null) return null;
                var result = Marshal.PtrToStringUTF8((IntPtr)countryCode);
                NativeMethods.kawaiifi_string_free(countryCode);
                return result;
            }
        }
    }

    /// <summary>The current basic service set identifier (BSSID).</summary>
    [SupportedOSPlatform("macos")]
    public byte[]? Bssid
    {
        get
        {
            unsafe
            {
                var bssid = stackalloc byte[6];
                if (NativeMethodsMacOS.kawaiifi_interface_bssid(Ptr, bssid))
                {
                    return new Span<byte>(bssid, 6).ToArray();
                }
                else
                {
                    return null;
                }
            }
        }
    }

    /// <summary>The hardware MAC address of the Wi-Fi interface.</summary>
    [SupportedOSPlatform("macos")]
    public string? HardwareAddress
    {
        get
        {
            unsafe
            {
                var hardwareAddress = NativeMethodsMacOS.kawaiifi_interface_hardware_address(Ptr);
                if (hardwareAddress == null) return null;
                var result = Marshal.PtrToStringUTF8((IntPtr)hardwareAddress);
                NativeMethods.kawaiifi_string_free(hardwareAddress);
                return result;
            }
        }
    }

    /// <summary>The current signal strength in dBm.</summary>
    [SupportedOSPlatform("macos")]
    public int SignalDbm
    {
        get
        {
            unsafe
            {
                return NativeMethodsMacOS.kawaiifi_interface_signal_dbm(Ptr);
            }
        }
    }

    /// <summary>The current aggregate noise measurement in dBm.</summary>
    [SupportedOSPlatform("macos")]
    public int NoiseDbm
    {
        get
        {
            unsafe
            {
                return NativeMethodsMacOS.kawaiifi_interface_noise_dbm(Ptr);
            }
        }
    }

    /// <summary>Whether the network service is active.</summary>
    [SupportedOSPlatform("macos")]
    public bool ServiceActive
    {
        get
        {
            unsafe
            {
                return NativeMethodsMacOS.kawaiifi_interface_service_active(Ptr);
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
                return NativeMethodsWindows.kawaiifi_interface_guid(Ptr);
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
                var description = NativeMethodsWindows.kawaiifi_interface_description(Ptr);
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
                Backend.Nl80211 => NativeMethodsLinux.kawaiifi_interface_scan(Ptr, CsBindgen.Backend.Nl80211),
                Backend.NetworkManager => NativeMethodsLinux.kawaiifi_interface_scan(Ptr,
                    CsBindgen.Backend.NetworkManager),
                _ => throw new ArgumentOutOfRangeException(nameof(backend)),
            };

            return new Scan(scan);
        }
    }

    /// <summary>
    /// Performs a blocking Wi-Fi scan and returns the results.
    /// Dispose the returned <see cref="Kawaiifi.Net.Scan"/> when done.
    /// </summary>
    [SupportedOSPlatform("macos")]
    [SupportedOSPlatform("windows")]
    public Scan Scan()
    {
        unsafe
        {
            if (OperatingSystem.IsMacOS())
            {
                var scan = NativeMethodsMacOS.kawaiifi_interface_scan(Ptr);
                return new Scan(scan);
            }

            if (OperatingSystem.IsWindows())
            {
                var scan = NativeMethodsWindows.kawaiifi_interface_scan(Ptr);
                return new Scan(scan);
            }

            throw new PlatformNotSupportedException();
        }
    }

    /// <summary>
    /// Returns a list of cached BSSs from previous Wi-Fi scans.
    /// Dispose the returned <see cref="Kawaiifi.Net.BssList"/> when done.
    /// </summary>
    public BssList CachedBssList()
    {
        unsafe
        {
            return new BssList(NativeMethods.kawaiifi_interface_cached_bss_list(Ptr));
        }
    }
}