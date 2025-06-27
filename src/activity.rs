#[cfg(target_os = "windows")]
use windows::{
    core::PCWSTR,
    Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextW, FindWindowW, PostMessageW, WM_CLOSE},
    Win32::Foundation::{HWND, WPARAM,LPARAM}
};
use widestring::U16CString;


#[cfg(target_os="windows")]
pub fn get_active_window_title() -> Option<String> {
    let hwnd: HWND = unsafe {GetForegroundWindow()};
    if hwnd.0 == 0 {
        return None;
    }

    let mut buf = [0u16; 512];
    let len = unsafe { GetWindowTextW(hwnd, &mut buf)} as usize;

    if len > 0 {
        Some(String::from_utf16_lossy(&buf[..len]))
    } else {
        None
    }
}

#[cfg(target_os="windows")]
pub fn close_window_by_title(title:&str){
    let wide = U16CString::from_str(title).unwrap();
    
    let hwnd = unsafe { FindWindowW(PCWSTR::null(), PCWSTR(wide.as_ptr()))};
    if hwnd.0 != 0 {
        unsafe { PostMessageW(hwnd, WM_CLOSE, WPARAM(0), LPARAM(0))};
    }
}