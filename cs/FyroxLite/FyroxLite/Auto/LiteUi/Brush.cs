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

// fyrox_lite::lite_ui::Brush
[StructLayout(LayoutKind.Sequential)]
public partial struct Brush
{
    public Color? SolidColor {
        #region get/set with wrapping/unwrapping
        get => NativeColor_optional.ToFacade(_solid_color);
        set => _solid_color = NativeColor_optional.FromFacade(value);
        #endregion
    }
    public LinearGradient? LinearGradient {
        #region get/set with wrapping/unwrapping
        get => LinearGradient_optional.ToFacade(_linear_gradient);
        set => _linear_gradient = LinearGradient_optional.FromFacade(value);
        #endregion
    }
    public RadialGradient? RadialGradient {
        #region get/set with wrapping/unwrapping
        get => RadialGradient_optional.ToFacade(_radial_gradient);
        set => _radial_gradient = RadialGradient_optional.FromFacade(value);
        #endregion
    }
#region Native Fields
//===============================================================
// private fields for all properties (not only mapped),
// because it makes ABI much more readable.
// I hope, NativeAOT will optimize out this.
//===============================================================
    private NativeColor_optional _solid_color;
    private LinearGradient_optional _linear_gradient;
    private RadialGradient_optional _radial_gradient;
#endregion
}
#region internal wrappers


[StructLayout(LayoutKind.Sequential)]
internal struct Brush_optional
{
    internal Brush value;
    internal int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Brush? ToFacade(in Brush_optional value)
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
    public static Brush_optional FromFacade(in Brush? value)
    {
        if (value == null)
        {
            return new Brush_optional { value = default, has_value = 0 };
        }
        var __item = value.Value;
        var __item_from_facade = __item;
        return new Brush_optional { value = __item_from_facade, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal partial struct Brush_slice
{
    internal unsafe Brush* begin;
    internal int length;

    internal unsafe Brush_slice(Brush* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe List<Brush> ToFacade(in Brush_slice self)
    {
        var fetched = new List<Brush>();

        for (var i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    [ThreadStatic]
    private static Brush[]? _uploadBuffer;

    internal static Brush_slice FromFacade(in List<Brush> self)
    {
        _uploadBuffer ??= new Brush[1024];
        while (_uploadBuffer.Length < self.Count)
        {
            _uploadBuffer = new Brush[_uploadBuffer.Length * 2];
        }

        for (var i = 0; i < self.Count; i++)
        {
            var __item = self[i];
            var __item_from_facade = __item;
            _uploadBuffer[i] = __item_from_facade;
        }

        unsafe
        {
            fixed (Brush* buffer_ptr = _uploadBuffer)
            {
                var native_slice = fyrox_lite_upload_fyrox_lite_lite_ui_Brush_slice(new Brush_slice(buffer_ptr, self.Count));
                return native_slice;
            }
        }
    }

    [LibraryImport("fyrox_c", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    internal static unsafe partial Brush_slice fyrox_lite_upload_fyrox_lite_lite_ui_Brush_slice(Brush_slice managed);
}

[StructLayout(LayoutKind.Sequential)]
internal struct Brush_result
{
    internal int ok;
    internal Brush_result_value value;

    internal static unsafe Brush ToFacade(in Brush_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value.ok;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(NativeString.ToFacade(self.value.err));
    }

    internal static Brush_result FromFacade(in Brush self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new Brush_result {ok = 1, value = new Brush_result_value { ok = __item_from_facade } };
    }

    internal static Brush_result FromFacadeError(in string err)
    {
        return new Brush_result {ok = 0, value = new Brush_result_value { err = NativeString.FromFacade(err) } };
    }
}

[StructLayout(LayoutKind.Explicit)]
internal struct Brush_result_value
{
    [FieldOffset(0)]
    internal Brush ok;

    [FieldOffset(0)]
    internal NativeString err;
}
#endregion