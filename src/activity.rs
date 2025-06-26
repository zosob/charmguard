#[cfg(target_os = "windows")]
use windows::{
    Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextW},
    Win32::Foundation::HWND,
};

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