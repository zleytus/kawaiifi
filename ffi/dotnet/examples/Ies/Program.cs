using Kawaiifi.Net;

using var defaultInterface = Interface.Default();

if (OperatingSystem.IsLinux())
{
    using var scan = defaultInterface?.Scan(Backend.NetworkManager);
    PrintScanIes(scan);
}

if (OperatingSystem.IsMacOS() || OperatingSystem.IsWindows())
{
    using var scan = defaultInterface?.Scan();
    PrintScanIes(scan);
}

return;

static void PrintScanIes(Scan? scan)
{
    if (scan is null)
    {
        return;
    }
    
    foreach (var bss in scan.BssList)
    {
        foreach (var ie in bss.Ies)
        {
            Console.WriteLine($"{ie.Name} ({ie.Id}) - {ie.Summary}");    
        }
        Console.WriteLine();
    }
}
