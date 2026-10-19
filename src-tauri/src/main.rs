#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    #[cfg(target_os = "linux")]
    coolapk_desktop_lib::configure_linux_webkit_environment();

    if coolapk_desktop_lib::try_run_portable_update_helper() {
        return;
    }
    coolapk_desktop_lib::run();
}
