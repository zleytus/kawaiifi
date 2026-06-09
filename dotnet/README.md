# Kawaiifi.Net

[![.NET 8+](https://img.shields.io/badge/.NET-8%2B-512BD4)](https://dotnet.microsoft.com/)
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

### Obtaining a Wi-Fi Interface

Use `Interface.Default()` to get the first available interface.

```csharp
using var defaultInterface = Interface.Default();
```

Use `Interface.All()` to get all available interfaces.

```csharp
using var interfaces = Interface.All();
Console.WriteLine($"Found {interfaces.Count} interface(s)");
```

Some `Interface` properties are platform-specific.

```csharp
if (OperatingSystem.IsLinux())
{
    Console.WriteLine($"Index: {defaultInterface?.Index}");
}

if (OperatingSystem.IsMacOS())
{
    Console.WriteLine($"Noise: {defaultInterface?.NoiseDbm} dBm");
}

if (OperatingSystem.IsWindows())
{
    Console.WriteLine($"Description: {defaultInterface?.Description}");
}
```

### Triggering a Wi-Fi Scan

On Linux, scans can be triggered through either [NetworkManager](https://networkmanager.dev/) or [nl80211](https://wireless.docs.kernel.org/en/latest/en/developers/documentation/nl80211.html) (Netlink), so a `Backend` must be specified.

On macOS and Windows, scans are triggered through [CoreWLAN](https://developer.apple.com/documentation/CoreWLAN) and [Native Wifi](https://learn.microsoft.com/en-us/windows/win32/nativewifi/portal) respectively.

```csharp
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

See [`Scan/Program.cs`](https://github.com/zleytus/kawaiifi/blob/master/dotnet/examples/Scan/Program.cs).

### Accessing BSS Data

`Scan` contains a list of BSSs that are accessed through `Scan.BssList`.

```csharp
BssList bssList = scan.BssList;
Console.WriteLine($"Found {bssList.Count} BSS(s)");
```

`Bss` exposes common properties that are available on all platforms.

```csharp
Console.WriteLine($"BSSID: {BitConverter.ToString(bss.Bssid).Replace('-', ':')}");
Console.WriteLine($"SSID: {bss.Ssid}");
Console.WriteLine($"Frequency: {bss.FrequencyMhz} MHz");
Console.WriteLine($"Band: {bss.Band.ToDisplayString()}");
Console.WriteLine($"Channel: {bss.ChannelNumber}");
Console.WriteLine($"Channel Width: {bss.ChannelWidth.ToDisplayString()}");
Console.WriteLine($"Signal: {bss.SignalDbm} dBm");
Console.WriteLine($"Security: {bss.SecurityProtocols.ToString()}");
Console.WriteLine($"Wi-Fi Protocols: {bss.WifiProtocols.ToString()}");
Console.WriteLine($"Wi-Fi Amendments: {bss.WifiAmendments.ToString()}");
Console.WriteLine($"Max Rate: {bss.MaxRateMbps:F2} Mbps");
```

Some `Bss` properties are platform-specific.

```csharp
if (OperatingSystem.IsLinux())
{
    Console.WriteLine($"Status: {bss.Status}");
}

if (OperatingSystem.IsMacOS())
{
    Console.WriteLine($"Noise: {bss.NoiseDbm} dBm");
}

if (OperatingSystem.IsWindows())
{
    Console.WriteLine($"Link Quality: {bss.LinkQuality}");
}
```

See [`BssData/Program.cs`](https://github.com/zleytus/kawaiifi/blob/master/dotnet/examples/BssData/Program.cs).

### Accessing Information Elements

`Bss` contains a list of 802.11 Information Elements (IEs) that are accessed through `Bss.Ies`.

```csharp
IReadOnlyList<Ie> ies = bss.Ies;
Console.WriteLine($"Found {ies.Count} IE(s)");
```

`Ie` exposes basic properties such as the information element's name, ID,
and a summary.

```csharp
Console.WriteLine($"{ie.Name} ({ie.Id}) - {ie.Summary}");
```

`Ie` also exposes its decoded fields through `Ie.Fields`. Each `Field` has a `Title`,
`Value`, optional `Units`, and nested `Subfields`.

```csharp
using FieldList fields = ie.Fields;
foreach (Field field in fields)
{
    Console.WriteLine($"{field.Title}: {field.Value}");
}
```

See [`Ies/Program.cs`](https://github.com/zleytus/kawaiifi/blob/master/dotnet/examples/Ies/Program.cs).

## Platform-Specific APIs

`Kawaiifi.Net` exposes platform-specific APIs via [`[SupportedOSPlatform]`](https://learn.microsoft.com/en-us/dotnet/api/system.runtime.versioning.supportedosplatformattribute) attributes.
The Roslyn analyzer will warn if platform-specific APIs are called without an OS check.

For example, on Linux and macOS, `Interface` has a `Name` property, while on Windows,
`Interface` has a `Description` property.

```csharp
using var defaultInterface = Interface.Default();

if (OperatingSystem.IsLinux() || OperatingSystem.IsMacOS())
{
    Console.WriteLine($"Interface name: {defaultInterface?.Name}");
}
else if (OperatingSystem.IsWindows())
{
    Console.WriteLine($"Interface description: {defaultInterface?.Description}");
}
```

## Troubleshooting

See the repository [troubleshooting notes](https://github.com/zleytus/kawaiifi#troubleshooting) for
platform-specific permissions and location-services behavior.

## License

Dual-licensed under [MIT](https://github.com/zleytus/kawaiifi/blob/master/LICENSE-MIT) or [Apache 2.0](https://github.com/zleytus/kawaiifi/blob/master/LICENSE-APACHE).
