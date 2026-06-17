using Kawaiifi.Net;

using var defaultInterface = Interface.Default();
using var scan = defaultInterface?.Scan();

PrintScanBssData(scan);

static void PrintScanBssData(Scan? scan)
{
    if (scan is null)
    {
        return;
    }

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
}
