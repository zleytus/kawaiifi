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

    private unsafe CsBindgen.BssList* Ptr
    {
        get
        {
            ObjectDisposedException.ThrowIf(_disposed, this);
            return _ptr;
        }
    }

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
            NativeMethods.kawaiifi_bss_list_free(_ptr);
        }

        GC.SuppressFinalize(this);
    }

    /// <inheritdoc/>
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
                return (int)NativeMethods.kawaiifi_bss_list_count(Ptr);
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
                return new Bss(NativeMethods.kawaiifi_bss_list_get(Ptr, (nuint)index));
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