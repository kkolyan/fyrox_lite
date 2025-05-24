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

// fyrox_lite::lite_input::Input
public static partial class Input
{
    public const int MouseLeft = 0;
    public const int MouseRight = 1;
    public const int MouseMiddle = 2;
    public const int MouseBack = 3;
    public const int MouseForward = 4;

    public static bool IsMouseButtonDown(int button)
    {
        #region native call
        unsafe {
            var _button = button;
            var __ret = fyrox_lite_lite_input_Input_is_mouse_button_down(_button);
            return NativeBool.ToFacade(__ret);
        }
        #endregion
    }

    public static bool IsMouseButtonUp(int button)
    {
        #region native call
        unsafe {
            var _button = button;
            var __ret = fyrox_lite_lite_input_Input_is_mouse_button_up(_button);
            return NativeBool.ToFacade(__ret);
        }
        #endregion
    }

    public static bool IsMouseButtonPressed(int button)
    {
        #region native call
        unsafe {
            var _button = button;
            var __ret = fyrox_lite_lite_input_Input_is_mouse_button_pressed(_button);
            return NativeBool.ToFacade(__ret);
        }
        #endregion
    }

    public static bool IsKeyDown(KeyCode key)
    {
        #region native call
        unsafe {
            var _key = key;
            var __ret = fyrox_lite_lite_input_Input_is_key_down(_key);
            return NativeBool.ToFacade(__ret);
        }
        #endregion
    }

    public static bool IsKeyUp(KeyCode key)
    {
        #region native call
        unsafe {
            var _key = key;
            var __ret = fyrox_lite_lite_input_Input_is_key_up(_key);
            return NativeBool.ToFacade(__ret);
        }
        #endregion
    }

    public static bool IsKeyPressed(KeyCode key)
    {
        #region native call
        unsafe {
            var _key = key;
            var __ret = fyrox_lite_lite_input_Input_is_key_pressed(_key);
            return NativeBool.ToFacade(__ret);
        }
        #endregion
    }
    public static Vector2 MouseMove
    {
        get
        {
            #region native call
            unsafe {
                var __ret = fyrox_lite_lite_input_Input_get_mouse_move();
                return NativeVector2.ToFacade(__ret);
            }
            #endregion
        }
    }
    public static Vector2 MouseScroll
    {
        get
        {
            #region native call
            unsafe {
                var __ret = fyrox_lite_lite_input_Input_get_mouse_scroll();
                return NativeVector2.ToFacade(__ret);
            }
            #endregion
        }
    }

    #region native internal methods

    [LibraryImport("fyrox_lite_cs", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial NativeBool fyrox_lite_lite_input_Input_is_mouse_button_down(int button);

    [LibraryImport("fyrox_lite_cs", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial NativeBool fyrox_lite_lite_input_Input_is_mouse_button_up(int button);

    [LibraryImport("fyrox_lite_cs", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial NativeBool fyrox_lite_lite_input_Input_is_mouse_button_pressed(int button);

    [LibraryImport("fyrox_lite_cs", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial NativeBool fyrox_lite_lite_input_Input_is_key_down(KeyCode key);

    [LibraryImport("fyrox_lite_cs", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial NativeBool fyrox_lite_lite_input_Input_is_key_up(KeyCode key);

    [LibraryImport("fyrox_lite_cs", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial NativeBool fyrox_lite_lite_input_Input_is_key_pressed(KeyCode key);

    [LibraryImport("fyrox_lite_cs", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial NativeVector2 fyrox_lite_lite_input_Input_get_mouse_move();

    [LibraryImport("fyrox_lite_cs", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial NativeVector2 fyrox_lite_lite_input_Input_get_mouse_scroll();
    #endregion

}
#region internal type wrappers

#endregion