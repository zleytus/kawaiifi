using Kawaiifi.Net;

using var defaultInterface = Interface.Default();

if (OperatingSystem.IsLinux())
{
    using var scan = defaultInterface?.Scan(Backend.NetworkManager);
    Console.WriteLine(
        $"Found {scan?.BssList.Count} BSS(s) in {scan?.EndTimeUtcMs - scan?.StartTimeUtcMs} ms on {scan?.FrequenciesMhz.Length} frequencies using {defaultInterface?.Name}");
}

if (OperatingSystem.IsWindows())
{
    using var scan = defaultInterface?.Scan();
    Console.WriteLine(
        $"Found {scan?.BssList.Count} BSS(s) in {scan?.EndTimeUtcMs - scan?.StartTimeUtcMs} ms using {defaultInterface?.Description}");
}