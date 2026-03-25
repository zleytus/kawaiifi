using System.Runtime.InteropServices;
using CsBindgen;

namespace Kawaiifi.Net;

/// <summary>An 802.11 information element (IE) from a BSS or scan.</summary>
public readonly struct Ie
{
    private readonly unsafe CsBindgen.Ie* _ptr;

    internal unsafe Ie(CsBindgen.Ie* ptr)
    {
        _ptr = ptr;
    }

    /// <summary>The element ID.</summary>
    public byte Id
    {
        get
        {
            unsafe
            {
                return NativeMethods.kawaiifi_ie_id(_ptr);
            }
        }
    }

    /// <summary>The length in bytes of the element's data.</summary>
    public byte Length
    {
        get
        {
            unsafe
            {
                return NativeMethods.kawaiifi_ie_len(_ptr);
            }
        }
    }

    /// <summary>The extended element ID, or null if this IE has no extension ID.</summary>
    public byte? IdExt
    {
        get
        {
            unsafe
            {
                byte idExt = 0;
                return NativeMethods.kawaiifi_ie_id_ext(_ptr, &idExt) ? idExt : null;
            }
        }
    }

    /// <summary>The human-readable name of this IE type (e.g. "SSID" or "RSN").</summary>
    public string Name
    {
        get
        {
            unsafe
            {
                var name = NativeMethods.kawaiifi_ie_name(_ptr);
                var result = Marshal.PtrToStringUTF8((IntPtr)name);
                NativeMethods.kawaiifi_string_free(name);
                return result ?? "";
            }
        }
    }

    /// <summary>The raw bytes of the IE's data payload.</summary>
    public byte[] Bytes
    {
        get
        {
            unsafe
            {
                nuint count = 0;
                var ptr = NativeMethods.kawaiifi_ie_bytes(_ptr, &count);
                var bytes = new Span<byte>(ptr, (int)count).ToArray();
                NativeMethods.kawaiifi_bytes_free(ptr, count);
                return bytes;
            }
        }
    }

    /// <summary>A human-readable summary of the IE's contents.</summary>
    public string Summary
    {
        get
        {
            unsafe
            {
                var summary = NativeMethods.kawaiifi_ie_summary(_ptr);
                var result = Marshal.PtrToStringUTF8((IntPtr)summary);
                NativeMethods.kawaiifi_string_free(summary);
                return result ?? "";
            }
        }
    }

    /// <summary>
    /// The decoded fields of this IE. Allocates a new <see cref="FieldList"/> on every call —
    /// cache the result if you need to access it more than once. Dispose when done.
    /// </summary>
    public FieldList Fields
    {
        get
        {
            unsafe
            {
                return new FieldList(NativeMethods.kawaiifi_ie_fields(_ptr));
            }
        }
    }
}