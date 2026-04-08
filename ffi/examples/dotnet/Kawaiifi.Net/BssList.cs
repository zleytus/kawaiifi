using System.Collections;
using CsBindgen;

namespace Kawaiifi.Net;

/// <summary>
/// A list of BSSs. Dispose this instance when done to free the underlying native memory.
/// </summary>
public class BssList : IReadOnlyList<Bss>, IDisposable
{
    private readonly unsafe CsBindgen.BssList* _ptr;
    private bool _disposed;

    internal unsafe BssList(CsBindgen.BssList* ptr)
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
            NativeMethods.kawaiifi_interface_bss_list_free(_ptr);
        }

        GC.SuppressFinalize(this);
    }

    ~BssList()
    {
        Dispose();
    }

    /// <summary>The number of BSSs in the list.</summary>
    public int Count
    {
        get
        {
            unsafe
            {
                return (int)NativeMethods.kawaiifi_bss_list_count(_ptr);
            }
        }
    }

    /// <summary>Returns the Bss at the specified index.</summary>
    public Bss this[int index]
    {
        get
        {
            if (index < 0 || index >= Count)
            {
                throw new ArgumentOutOfRangeException(nameof(index));
            }

            unsafe
            {
                return new Bss(NativeMethods.kawaiifi_bss_list_get(_ptr, (nuint)index));
            }
        }
    }

    /// <inheritdoc/>
    IEnumerator IEnumerable.GetEnumerator() => GetEnumerator();

    /// <inheritdoc/>
    public IEnumerator<Bss> GetEnumerator()
    {
        for (var i = 0; i < Count; i++)
        {
            yield return this[i];
        }
    }
}