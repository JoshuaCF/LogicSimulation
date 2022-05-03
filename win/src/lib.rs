//! A module containing safe wrappers for various Win32 function calls, as well as the parent to the [types] module which contains
//! wrappers for Win32 types

mod win_raw;

/// A module containing various Rust structures as wrappers of Win32 data types.
/// This allows enforcement of Rust's type safety at compile time.
pub mod types {
    use crate::win_raw::*;

    macro_rules! win_wrapper {
        ($name:ident,$wrapped:ty) => {
            #[doc=concat!("Simple `", stringify!($wrapped),"` wrapper to enforce type safety")]
            pub struct $name($wrapped);
            impl $name {
                pub fn new(val: $wrapped) -> Self {
                    $name(val)
                }
                pub fn get(&self) -> $wrapped { // IMO it's easier to read intent with .get() rather than doing .0, and I don't want to copy this for every wrapper
                    self.0
                }
            }
        }
    }

    win_wrapper!(ErrorCode, DWORD);
    win_wrapper!(WinClassAtom, ATOM);
    win_wrapper!(HInstance, HINSTANCE);
    win_wrapper!(HWindow, HWND);
    win_wrapper!(WinProc, WNDPROC);
    win_wrapper!(HIcon, HICON);
    win_wrapper!(HCursor, HCURSOR);
    win_wrapper!(HBrush, HBRUSH);

    /// Simple `String` wrapper that will have a null value appended for use as a C String.
    /// If the `String` provided already has a null value at the end, then no null will be appended.
    pub struct LPCString(Option<String>);
    impl LPCString {
        /// Creates a null `LPCString`
        pub fn null() -> Self {
            Self(None)
        }
        /// Creates a new `LPCString` structure
        pub fn new(val: String) -> Self {
            let mut val = val; // Move the value into a mutable owner

            let last_char = val.chars().last().unwrap();
            if last_char != '\x00' {
                val.push('\x00');
            }

            LPCString(Some(val))
        }
        /// Returns a const pointer to the first character of the internal `String`
        /// 
        /// THIS CAN RETURN A NULL POINTER!
        pub fn as_cstr(&self) -> *const u8 {
            match &self.0 {
                Some(v) => v.as_ptr(),
                None => 0 as *const u8
            }
        }
    }
    /// Simple wrapper for [WNDCLASSEXA]
    pub struct WinClass {
        pub style: UINT,
        pub win_proc: WinProc,
        pub cls_extra: i32,
        pub win_extra: i32,
        pub h_instance: HInstance,
        pub h_icon: HIcon,
        pub h_cursor: HCursor,
        pub h_br_background: HBrush,
        pub menu_name: LPCString,
        pub class_name: LPCString,
        pub h_icon_small: HIcon
    }
    impl Default for WinClass {
        fn default() -> WinClass {
            WinClass {
                style: 0,
                win_proc: WinProc::new(None), // TODO: I don't like how I have three different ways of doing nulls, this should be uniform
                cls_extra: 0,
                win_extra: 0,
                h_instance: HInstance(0 as HANDLE),
                h_icon: HIcon(0 as HICON),
                h_cursor: HCursor(0 as HCURSOR),
                h_br_background: HBrush(0 as HBRUSH),
                menu_name: LPCString::null(),
                class_name: LPCString::null(),
                h_icon_small: HIcon(0 as HICON)
            }
        }
    }
    impl WinClass {
        /// Converts a [WinClass] struct into the [WNDCLASSEXA] struct used internally by Win32
        pub fn convert(&self) -> WNDCLASSEXA {
            let ret = WNDCLASSEXA::new(&self);

            println!("{:?}", self.h_instance.0);

            ret
        }
    }
}


use core::ffi::c_void;

use types::*;

/// Calls the Win32 function GetLastError() and returns an [ErrorCode] containing the value from the Win32 call
#[doc(alias = "GetLastError")]
pub fn get_last_error() -> ErrorCode {
    let error: u32;

    unsafe {
        error = win_raw::GetLastError();
    }

    ErrorCode::new(error)
}
/// Calls the Win32 function RegisterClassExA() and returns a [Result] containing [WinClassAtom] on success, 
/// or [ErrorCode] on a failure
#[doc(alias = "RegisterClassExA")]
pub fn register_class(class: &WinClass) -> Result<WinClassAtom, ErrorCode> {
    let ret: WinClassAtom;

    let win_class_internal = class.convert();

    unsafe {
        ret = WinClassAtom::new(win_raw::RegisterClassExA(&win_class_internal));
    }

    if ret.get() == 0 {
        Err(get_last_error())
    } else {
        Ok(ret)
    }
}
/// Calls the Win32 function GetModuleHandleA and returns a [Result] containing [HInstance] on success, 
/// or [ErrorCode] on a failure
#[doc(alias = "GetModuleHandleA")]
pub fn get_module_handle() -> Result<HInstance, ErrorCode> {
    let ret;
    let mod_name_cstr = 0;

    unsafe {
        ret = win_raw::GetModuleHandleA(mod_name_cstr as *const u8);
    }

    if ret as usize == 0 {
        Err(get_last_error())
    } else {
        Ok(HInstance::new(ret))
    }
}
/// Calls the Win32 function CreateWindowExA and returns a [Result] containing [HWindow] on success,
/// or [ErrorCode] on a failure
#[doc(alias = "CreateWindowExA")]
pub fn create_window(class_name: &LPCString, window_name: &LPCString, h_instance: &HInstance) -> Result<HWindow, ErrorCode> {
    // TODO: Implement full functionality for this function
    let ret;
    let class_name_cstr = class_name.as_cstr();
    let window_name_cstr = window_name.as_cstr();

    unsafe {
        // TODO: The constants that configure a window style can be a vector of enums that just contain u32s
        ret = win_raw::CreateWindowExA(0, class_name_cstr, window_name_cstr, 
        win_raw::WS_OVERLAPPEDWINDOW, 20, 20, 80, 80, 0 as *mut c_void, 0 as *mut c_void, h_instance.get(), 0 as *mut c_void);
    }

    println!("{:?}", ret);
    if ret == 0 as *mut c_void {
        Err(get_last_error())
    } else {
        Ok(HWindow::new(ret))
    }
}
/// Calls the Win32 function ShowWindow
#[doc(alias = "ShowWindow")]
pub fn show_window(h_window: &HWindow, cmd_show: i32) {
    unsafe {
        win_raw::ShowWindow(h_window.get(), cmd_show);
    }
}
