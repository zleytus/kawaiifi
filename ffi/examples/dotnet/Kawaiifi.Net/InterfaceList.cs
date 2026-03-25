using System.Collections;
using CsBindgen;

namespace Kawaiifi.Net;

/// <summary>
/// An ordered list of wireless interfaces. Dispose this instance when done to free the underlying
/// native memory. Interfaces obtained from this list are borrowed and must not be disposed individually.
/// </summary>
public class InterfaceList : IReadOnlyList<Interface>, IDisposable
{
    private readonly unsafe CsBindgen.InterfaceList* _ptr;
    private bool _disposed;

    internal unsafe InterfaceList(CsBindgen.InterfaceList* ptr)
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
            NativeMethods.kawaiifi_interface_list_free(_ptr);
        }

        GC.SuppressFinalize(this);
    }

    /// <inheritdoc/>
    ~InterfaceList()
    {
        Dispose();
    }

    /// <summary>The number of interfaces in the list.</summary>
    public int Count
    {
        get
        {
            unsafe
            {
                return (int)NativeMethods.kawaiifi_interface_list_count(_ptr);
            }
        }
    }

    /// <summary>Returns the interface at the specified index. Do not dispose the returned interface.</summary>
    public Interface this[int index]
    {
        get
        {
            unsafe
            {
                return Interface.FromBorrowed(NativeMethods.kawaiifi_interface_list_get(_ptr, (nuint)index));
            }
        }
    }

    /// <inheritdoc/>
    IEnumerator IEnumerable.GetEnumerator() => GetEnumerator();

    /// <inheritdoc/>
    public IEnumerator<Interface> GetEnumerator()
    {
        for (var i = 0; i < Count; i++)
        {
            yield return this[i];
        }
    }
}