// ReSharper disable InconsistentNaming
// ReSharper disable RedundantUnsafeContext
// ReSharper disable UnusedMember.Global
// ReSharper disable RedundantUsingDirective
using FyroxLite;
using System.Drawing;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Collections;
namespace FyroxLite;

// fyrox_lite::lite_ui::LinearGradient
[StructLayout(LayoutKind.Sequential)]
public partial struct LinearGradient
{
    public Vector2 From {
        #region get/set with wrapping/unwrapping
        get => NativeVector2.ToFacade(_from);
        set => _from = NativeVector2.FromFacade(value);
        #endregion
    }
    public Vector2 To {
        #region get/set with wrapping/unwrapping
        get => NativeVector2.ToFacade(_to);
        set => _to = NativeVector2.FromFacade(value);
        #endregion
    }
    public List<GradientPoint> Stops {
        #region get/set with wrapping/unwrapping
        get => GradientPoint_slice.ToFacade(_stops);
        set => _stops = GradientPoint_slice.FromFacade(value);
        #endregion
    }
#region Native Fields
//===============================================================
// private fields for all properties (not only mapped),
// because it makes ABI much more readable.
// I hope, NativeAOT will optimize out this.
//===============================================================
    private NativeVector2 _from;
    private NativeVector2 _to;
    private GradientPoint_slice _stops;
#endregion
}
#region internal wrappers


[StructLayout(LayoutKind.Sequential)]
internal struct LinearGradient_optional
{
    internal LinearGradient value;
    internal int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static LinearGradient? ToFacade(in LinearGradient_optional value)
    {
        if (value.has_value != 0)
        {
            var __item = value.value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        return null;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static LinearGradient_optional FromFacade(in LinearGradient? value)
    {
        if (value == null)
        {
            return new LinearGradient_optional { value = default, has_value = 0 };
        }
        var __item = value.Value;
        var __item_from_facade = __item;
        return new LinearGradient_optional { value = __item_from_facade, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal partial struct LinearGradient_slice
{
    internal unsafe LinearGradient* begin;
    internal int length;

    internal unsafe LinearGradient_slice(LinearGradient* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe List<LinearGradient> ToFacade(in LinearGradient_slice self)
    {
        var fetched = new List<LinearGradient>();

        for (var i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    [ThreadStatic]
    private static LinearGradient[]? _uploadBuffer;

    internal static LinearGradient_slice FromFacade(in List<LinearGradient> self)
    {
        _uploadBuffer ??= new LinearGradient[1024];
        while (_uploadBuffer.Length < self.Count)
        {
            _uploadBuffer = new LinearGradient[_uploadBuffer.Length * 2];
        }

        for (var i = 0; i < self.Count; i++)
        {
            var __item = self[i];
            var __item_from_facade = __item;
            _uploadBuffer[i] = __item_from_facade;
        }

        unsafe
        {
            fixed (LinearGradient* buffer_ptr = _uploadBuffer)
            {
                var native_slice = fyrox_lite_upload_fyrox_lite_lite_ui_LinearGradient_slice(new LinearGradient_slice(buffer_ptr, self.Count));
                return native_slice;
            }
        }
    }

    [LibraryImport("fyrox_lite_cs", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    internal static unsafe partial LinearGradient_slice fyrox_lite_upload_fyrox_lite_lite_ui_LinearGradient_slice(LinearGradient_slice managed);
}

[StructLayout(LayoutKind.Sequential)]
internal struct LinearGradient_result
{
    internal int ok;
    internal LinearGradient_result_value value;

    internal static unsafe LinearGradient ToFacade(in LinearGradient_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value.ok;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(NativeString.ToFacade(self.value.err));
    }

    internal static LinearGradient_result FromFacade(in LinearGradient self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new LinearGradient_result {ok = 1, value = new LinearGradient_result_value { ok = __item_from_facade } };
    }

    internal static LinearGradient_result FromFacadeError(in string err)
    {
        return new LinearGradient_result {ok = 0, value = new LinearGradient_result_value { err = NativeString.FromFacade(err) } };
    }
}

[StructLayout(LayoutKind.Explicit)]
internal struct LinearGradient_result_value
{
    [FieldOffset(0)]
    internal LinearGradient ok;

    [FieldOffset(0)]
    internal NativeString err;
}
#endregion