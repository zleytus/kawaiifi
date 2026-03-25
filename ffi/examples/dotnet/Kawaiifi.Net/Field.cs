using System.Runtime.InteropServices;
using CsBindgen;

namespace Kawaiifi.Net;

/// <summary>A single decoded field within an information element.</summary>
public readonly struct Field
{
    private readonly unsafe CsBindgen.Field* _ptr;

    internal unsafe Field(CsBindgen.Field* ptr)
    {
        _ptr = ptr;
    }

    /// <summary>The field's display title.</summary>
    public string Title
    {
        get
        {
            unsafe
            {
                var title = NativeMethods.kawaiifi_field_title(_ptr);
                var result = Marshal.PtrToStringUTF8((IntPtr)title);
                NativeMethods.kawaiifi_string_free(title);
                return result ?? "";
            }
        }
    }

    /// <summary>The field's value as a human-readable string.</summary>
    public string Value
    {
        get
        {
            unsafe
            {
                var value = NativeMethods.kawaiifi_field_value(_ptr);
                var result = Marshal.PtrToStringUTF8((IntPtr)value);
                NativeMethods.kawaiifi_string_free(value);
                return result ?? "";
            }
        }
    }

    /// <summary>The field's value as a single byte, or null if this field is not a byte value.</summary>
    public byte? Byte
    {
        get
        {
            unsafe
            {
                byte b = 0;
                return NativeMethods.kawaiifi_field_byte(_ptr, &b) ? b : null;
            }
        }
    }

    /// <summary>The field's value as a byte array, or null if this field has no byte array value.</summary>
    public byte[]? Bytes
    {
        get
        {
            unsafe
            {
                nuint count = 0;
                var bytes = NativeMethods.kawaiifi_field_bytes(_ptr, &count);
                return bytes != null ? new Span<byte>(bytes, (int)count).ToArray() : null;
            }
        }
    }

    /// <summary>A formatted string describing the field's bit range, or null if not applicable.</summary>
    public string? Bits
    {
        get
        {
            unsafe
            {
                var bits = NativeMethods.kawaiifi_field_bits(_ptr);
                var result = Marshal.PtrToStringUTF8((IntPtr)bits);
                NativeMethods.kawaiifi_string_free(bits);
                return result;
            }
        }
    }

    /// <summary>The units of the field's value (e.g. "dBm", "MHz"), or null if unitless.</summary>
    public string? Units
    {
        get
        {
            unsafe
            {
                var units = NativeMethods.kawaiifi_field_units(_ptr);
                var result = Marshal.PtrToStringUTF8((IntPtr)units);
                NativeMethods.kawaiifi_string_free(units);
                return result;
            }
        }
    }

    /// <summary>Nested subfields within this field, or an empty array if none.</summary>
    public Field[] Subfields
    {
        get
        {
            unsafe
            {
                var count = NativeMethods.kawaiifi_field_subfield_count(_ptr);
                var subfields = new Field[count];
                for (var i = 0; i < (int)count; i++)
                {
                    subfields[i] = new Field(NativeMethods.kawaiifi_field_subfield_get(_ptr, (nuint)i));
                }

                return subfields;
            }
        }
    }
}