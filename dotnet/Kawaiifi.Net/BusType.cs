using System.Runtime.Versioning;

namespace Kawaiifi.Net;

/// <summary>The hardware bus type of a wireless interface.</summary>
[SupportedOSPlatform("linux")]
public enum BusType
{
    /// <summary>PCI or PCIe bus.</summary>
    Pci,
    /// <summary>USB bus.</summary>
    Usb,
    /// <summary>SDIO bus.</summary>
    Sdio,
}
