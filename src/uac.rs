use std::env::{args, current_exe};
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use windows::core::{HSTRING, PCWSTR};
use windows::Win32::Foundation::{HANDLE, HWND};
use windows::Win32::Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY};
use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};
use windows::Win32::UI::Shell::ShellExecuteW;

pub fn is_admin() -> bool {
    unsafe {
        let mut token_handle: HANDLE = HANDLE::default();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token_handle).is_ok() {
            let mut elevation: TOKEN_ELEVATION = TOKEN_ELEVATION { TokenIsElevated: 0 };
            let mut ret_len = 0;
            if GetTokenInformation(
                token_handle,
                TokenElevation,
                Some(&mut elevation as *mut _ as *mut _),
                std::mem::size_of::<TOKEN_ELEVATION>() as u32,
                &mut ret_len,
            )
            .is_ok()
            {
                return elevation.TokenIsElevated != 0;
            }
        }
        false
    }
}

pub fn run_as_admin() {
    unsafe {
        let hstring = HSTRING::from(current_exe().unwrap().as_os_str());
        let msg_pcwstr = PCWSTR(hstring.as_ptr());

        let args = args().skip(1).collect::<Vec<_>>().join(" ");

        let hstring_args = HSTRING::from(args);
        let msg_pcwstr_args = PCWSTR(hstring_args.as_ptr());

        ShellExecuteW(
            Option::from(HWND::default()),
            windows::core::w!("runas"), // 以管理员权限运行
            msg_pcwstr,
            msg_pcwstr_args,
            None,
            windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL,
        );
    }
}

pub fn handle() {
    if !is_admin() {
        println!("即将申请UAC权限，如您不放心可以选择拒绝。");
        for i in 0..5 {
            print!("\r{}s 后执行", 5 - i);
            stdout().flush().unwrap();
            sleep(Duration::from_secs(1));
        }
        run_as_admin();
        std::process::exit(0);
    }
}
