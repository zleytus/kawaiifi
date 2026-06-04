# Kawaiifi.Net

[![.NET 10](https://img.shields.io/badge/.NET-10-512BD4)](https://dotnet.microsoft.com/)
[![License: MIT or Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/zleytus/kawaiifi/blob/master/LICENSE-MIT)

`Kawaiifi.Net` is a Wi-Fi scanning library for Linux, macOS, and Windows.

It wraps [`kawaiifi-ffi`](https://github.com/zleytus/kawaiifi/blob/master/ffi) and handles all P/Invoke interop, memory management, and platform differences internally. Callers never need to write unsafe code or manage
native memory directly.

## Building

First, build the native `kawaiifi-ffi` library:

```sh
cargo build -p kawaiifi-ffi --release
```

Then build the .NET solution:

```sh
dotnet build
```

## Usage

### Triggering a Wi-Fi Scan

On Linux, scans can be triggered through either [NetworkManager](https://networkmanager.dev/) or [nl80211](https://wireless.docs.kernel.org/en/latest/en/developers/documentation/nl80211.html) (Netlink), so a `Backend` must be specified.

On macOS and Windows, scans are triggered through [CoreWLAN](https://developer.apple.com/documentation/CoreWLAN) and [Native Wifi](https://learn.microsoft.com/en-us/windows/win32/nativewifi/portal) respectively.

```csharp
using Kawaiifi.Net;

using var defaultInterface = Interface.Default();

if (OperatingSystem.IsLinux())
{
    using var scan = defaultInterface?.Scan(Backend.NetworkManager);
    Console.WriteLine($"Found {scan?.BssList.Count} BSS(s)");
}

if (OperatingSystem.IsMacOS() || OperatingSystem.IsWindows())
{
    using var scan = defaultInterface?.Scan();
    Console.WriteLine($"Found {scan?.BssList.Count} BSS(s)");
}
```

See [`Scan/Program.cs`](https://github.com/zleytus/kawaiifi/blob/master/dotnet/examples/Scan/Program.cs)

### Accessing BSS Data

Each `Scan` contains a list of Basic Service Sets (BSSs) that is accessed
through `Scan.BssList`.

```csharp
foreach (var bss in scan.BssList)
{
    Console.WriteLine($"BSSID: {BitConverter.ToString(bss.Bssid).Replace('-', ':')}");
    Console.WriteLine($"SSID: {bss.Ssid}");
    Console.WriteLine($"Frequency: {bss.FrequencyMhz} MHz");
    Console.WriteLine($"Band: {bss.Band.ToDisplayString()}");
    Console.WriteLine($"Channel: {bss.ChannelNumber}");
    Console.WriteLine($"Channel Width: {bss.ChannelWidth.ToDisplayString()}");
    Console.WriteLine($"Signal: {bss.SignalDbm} dBm");
    Console.WriteLine($"Security: {bss.SecurityProtocols.ToString()}");
    Console.WriteLine($"Protocols: {bss.WifiProtocols.ToString()}");
    Console.WriteLine($"Amendments: {bss.WifiAmendments.ToString()}");
    Console.WriteLine($"Max Rate: {bss.MaxRateMbps:F2} Mbps");
    Console.WriteLine();
}
```

See [`BssData/Program.cs`](https://github.com/zleytus/kawaiifi/blob/master/dotnet/examples/BssData/Program.cs)

### Accessing Information Elements

Each `Bss` contains a list of 802.11 Information Elements (IEs) that is accessed
through `Bss.Ies`.

```csharp
foreach (var bss in scan.BssList)
{
    foreach (var ie in bss.Ies)
    {
        Console.WriteLine($"{ie.Name} ({ie.Id}) - {ie.Summary}");    
    }
    Console.WriteLine();
}
```

See [`Ies/Program.cs`](https://github.com/zleytus/kawaiifi/blob/master/dotnet/examples/Ies/Program.cs)

## Platform Notes

`Kawaiifi.Net` exposes platform-specific APIs via [`[SupportedOSPlatform]`](https://learn.microsoft.com/en-us/dotnet/api/system.runtime.versioning.supportedosplatformattribute) attributes.
The Roslyn analyzer will warn if platform-specific APIs are called without an OS check.

For example, on Linux and macOS, `Interface` has a `Name` property, while on Windows,
`Interface` has a `Description` property.

```csharp
using var defaultInterface = Interface.Default();

if (OperatingSystem.IsLinux() || OperatingSystem.IsMacOS())
{
    Console.WriteLine($"Interface's name is {defaultInterface?.Name}");
}

if (OperatingSystem.IsWindows())
{
    Console.WriteLine($"Interface's description is {defaultInterface?.Description}");
}
```

## Troubleshooting

See the repository [troubleshooting notes](https://github.com/zleytus/kawaiifi#troubleshooting) for
platform-specific permissions and location-services behavior.

## License

Dual-licensed under MIT or Apache 2.0.
