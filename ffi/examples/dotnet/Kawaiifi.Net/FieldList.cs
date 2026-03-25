using System.Collections;
using CsBindgen;

namespace Kawaiifi.Net;

/// <summary>
/// An ordered list of decoded <see cref="Field"/> values for an information element.
/// Dispose this instance when done to free the underlying native memory.
/// </summary>
public class FieldList : IReadOnlyList<Field>, IDisposable
{
    private readonly unsafe CsBindgen.FieldList* _ptr;
    private bool _disposed;

    internal unsafe FieldList(CsBindgen.FieldList* ptr)
    {
        _ptr = ptr;
    }

    /// <inheritdoc/>
    public void Dispose()
    {
        if (_disposed) return;
        _disposed = true;
        unsafe
        {
            NativeMethods.kawaiifi_field_list_free(_ptr);
        }

        GC.SuppressFinalize(this);
    }

    /// <inheritdoc/>
    ~FieldList()
    {
        Dispose();
    }

    /// <summary>The number of fields in the list.</summary>
    public int Count
    {
        get
        {
            unsafe
            {
                return (int)NativeMethods.kawaiifi_field_list_count(_ptr);
            }
        }
    }

    /// <summary>Returns the field at the specified index.</summary>
    public Field this[int index]
    {
        get
        {
            unsafe
            {
                return new Field(NativeMethods.kawaiifi_field_list_get(_ptr, (nuint)index));
            }
        }
    }

    /// <inheritdoc/>
    IEnumerator IEnumerable.GetEnumerator() => GetEnumerator();

    /// <inheritdoc/>
    public IEnumerator<Field> GetEnumerator()
    {
        for (var i = 0; i < Count; i++)
        {
            yield return this[i];
        }
    }
}