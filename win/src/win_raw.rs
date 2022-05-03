//! A module containing type aliases for Win32 types, as well as the external function calls for Win32 functions.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::mem;
use core::ffi::c_void;

// This is why I should have just used the Windows crate, but here I am...
pub const WS_BORDER: u32 = 0x00800000;
pub const WS_CAPTION: u32 = 0x00C00000;
pub const WS_CHILD: u32 = 0x40000000;
pub const WS_CLIPCHILDREN: u32 = 0x02000000;
pub const WS_CLIPSIBLINGS: u32 = 0x04000000;
pub const WS_DISABLED: u32 = 0x08000000;
pub const WS_DLGFRAME: u32 = 0x00400000;
pub const WS_GROUP: u32 = 0x00020000;
pub const WS_HSCROLL: u32 = 0x00100000;
pub const WS_MAXIMIZE: u32 = 0x01000000;
pub const WS_MAXIMIZEBOX: u32 = 0x00010000;
pub const WS_MINIMIZE: u32 = 0x20000000;
pub const WS_MINIMIZEBOX: u32 = 0x00020000;
pub const WS_OVERLAPPED: u32 = 0x00000000;
pub const WS_OVERLAPPEDWINDOW: u32 = WS_OVERLAPPED
    | WS_CAPTION
    | WS_SYSMENU
    | WS_SIZEBOX
    | WS_MINIMIZEBOX
    | WS_MAXIMIZEBOX;
pub const WS_POPUP: u32 = 0x80000000;
pub const WS_POPUPWINDOW: u32 = WS_POPUP 
    | WS_BORDER 
    | WS_SYSMENU;
pub const WS_SIZEBOX: u32 = 0x00040000;
pub const WS_SYSMENU: u32 = 0x00080000;
pub const WS_TABSTOP: u32 = 0x00010000;
pub const WS_VISIBLE: u32 = 0x10000000;
pub const WS_VSCROLL: u32 = 0x00200000;

pub type VOID = *mut c_void;
pub type PVOID = *mut c_void;
pub type LPVOID = *mut c_void;

pub type BOOL = bool;

pub type WORD = u16;
pub type ATOM = WORD;

pub type UINT = u32;
pub type DWORD = u32;
pub type LPDWORD = *mut DWORD;

pub type UINT_PTR = usize;
pub type WPARAM = UINT_PTR;

pub type LONG_PTR = isize;
pub type LPARAM = LONG_PTR;
pub type LRESULT = LONG_PTR;

pub type LPCSTR = *const u8;

pub type HANDLE = PVOID;
pub type HWND = HANDLE;
pub type HINSTANCE = HANDLE;
pub type HMENU = HANDLE;
pub type HMODULE = HINSTANCE;
pub type HICON = HANDLE;
pub type HCURSOR = HICON;
pub type HBRUSH = HANDLE;

pub type WNDPROC = Option<extern "system" fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT>;

#[repr(C)]
pub struct WNDCLASSEXA {
    cbSize: UINT,
    style: UINT,
    lpfnWndProc: WNDPROC,
    cbClsExtra: i32,
    cbWndExtra: i32,
    hInstance: HINSTANCE,
    hIcon: HICON,
    hCursor: HCURSOR,
    hbrBackground: HBRUSH,
    lpszMenuName: LPCSTR,
    lpszClassName: LPCSTR,
    hIconSm: HICON
}
impl WNDCLASSEXA { // TODO: Implement full functionality for this structure
    pub fn new(wrapper: &crate::types::WinClass) -> Self {
        WNDCLASSEXA {
            cbSize: mem::size_of::<WNDCLASSEXA>() as UINT, 
            style: wrapper.style, 
            lpfnWndProc: wrapper.win_proc.get(), 
            cbClsExtra: wrapper.cls_extra, 
            cbWndExtra: wrapper.win_extra, 
            hInstance: wrapper.h_instance.get(), 
            hIcon: wrapper.h_icon.get(), 
            hCursor: wrapper.h_cursor.get(), 
            hbrBackground: wrapper.h_br_background.get(), 
            lpszMenuName: wrapper.menu_name.as_cstr(), 
            lpszClassName: wrapper.class_name.as_cstr(), 
            hIconSm: wrapper.h_icon_small.get()
        }
    }
}

#[link(name="Kernel32")]
extern "system" {
    pub fn GetLastError() -> DWORD;
    pub fn GetModuleHandleA(lpModuleName: LPCSTR) -> HMODULE;
}

#[link(name="User32")]
extern "system" {
    pub fn ShowWindow(hWnd: HWND, nCmdShow: i32) -> BOOL;
    pub fn DefWindowProcA(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    pub fn RegisterClassExA(lpWndClass: *const WNDCLASSEXA) -> ATOM;
    pub fn CreateWindowExA(dwExStyle: DWORD, lpClassName: LPCSTR, lpWindowName: LPCSTR, dwStyle: DWORD, x: i32, y: i32, 
        nWidth: i32, nHeight: i32, hWndParent: HWND, hMenu: HMENU, hInstance: HINSTANCE, lpParam: LPVOID) -> HWND;
}

// The following function definition is kept for reference

// pub extern "system" fn callback(hWnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
//     let ret;
//     unsafe {
//         ret = DefWindowProcA(hWnd, uMsg, wParam, lParam);
//     }

//     ret
// }