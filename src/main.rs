// #![allow(unused_imports)]
use windows::Win32::Graphics::Gdi;
use windows::Win32::Foundation::{LPARAM, BOOL, RECT};
use windows::Win32::Devices::Display::SetMonitorBrightness;
use windows::Win32::Devices::Display::GetPhysicalMonitorsFromHMONITOR;
use windows::Win32::Devices::Display::PHYSICAL_MONITOR;

fn main() {
    unsafe {
        // https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumdisplaymonitors

        unsafe extern "system" fn handle_monitor(
            param0: Gdi::HMONITOR,
            _: Gdi::HDC,
            _: *mut RECT,
            _: LPARAM
        ) -> BOOL {
            let first_command = std::env::args().skip(1).take(1).next().unwrap_or("None".to_string());
            let new_brightness: u32;
            let monitor_handle: isize;
            if first_command == "set" {
                monitor_handle = std::env::args().skip(2).take(1).next().unwrap_or("0".to_string()).parse().unwrap_or(0);
                new_brightness = std::env::args().skip(3).take(1).next().unwrap_or("100".to_string()).parse().unwrap_or(100);

                if param0.0 == monitor_handle {
                    let mut physical_monitor = [PHYSICAL_MONITOR::default(); 1];
                    _ = GetPhysicalMonitorsFromHMONITOR(param0, &mut physical_monitor);
                    _ = SetMonitorBrightness(physical_monitor[0].hPhysicalMonitor, new_brightness);
                }
            }

            if first_command == "list" {
                println!("{:?}", param0.0);
            }
            return BOOL(1);
        }

        _ = Gdi::EnumDisplayMonitors(None, None, Some(handle_monitor), None);
    }
}
