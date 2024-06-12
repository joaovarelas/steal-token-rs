use std::{env, io};

use windows::core::PSTR;
use windows::Win32::Foundation::{BOOL, FALSE, HANDLE, LUID, TRUE};
use windows::Win32::Security::{DuplicateTokenEx, LookupPrivilegeValueW, PrivilegeCheck, SE_DEBUG_NAME, SE_PRIVILEGE_ENABLED, SecurityImpersonation, TOKEN_ACCESS_MASK, TOKEN_ALL_ACCESS, TOKEN_ASSIGN_PRIMARY, TOKEN_DUPLICATE, TOKEN_IMPERSONATE, TOKEN_PRIVILEGES, TOKEN_QUERY, TokenImpersonation};
use windows::Win32::Security::AdjustTokenPrivileges;
use windows::Win32::Security::LUID_AND_ATTRIBUTES;
use windows::Win32::Security::PRIVILEGE_SET;
use windows::Win32::System::SystemServices::MAXIMUM_ALLOWED;
use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken, PROCESS_QUERY_INFORMATION, SetThreadToken};
use windows::Win32::System::Threading::OpenProcess;
use windows::Win32::System::WindowsProgramming::GetUserNameA;

fn main() {
    unsafe {

        println!("Hello, world!");

        let proc_handle = GetCurrentProcess();
        let mut luid: LUID = LUID { LowPart: 0, HighPart: 0 };
        let mut token_handle: HANDLE = HANDLE(0);

        if LookupPrivilegeValueW(None, SE_DEBUG_NAME, &mut luid).is_err() {
            println!("Error LookupPrivilegeValue");
            return;
        }

        if OpenProcessToken(proc_handle, TOKEN_ALL_ACCESS, &mut token_handle).is_err() {
            println!("Error OpenProcessToken");
            return;
        }

        let token_priv: TOKEN_PRIVILEGES = TOKEN_PRIVILEGES {
            PrivilegeCount: 1,
            Privileges: [LUID_AND_ATTRIBUTES { Luid: luid, Attributes: SE_PRIVILEGE_ENABLED }],
        };
        if AdjustTokenPrivileges(token_handle, FALSE, Some(&token_priv), 0, None, None).is_err() {
            println!("Error AdjustTokenPrivileges");
            return;
        }

        let mut priv_set: PRIVILEGE_SET = PRIVILEGE_SET {
            PrivilegeCount: 1,
            Control: 1u32, // PRIVILEGE_SET_ALL_NECESSARY
            Privilege: [LUID_AND_ATTRIBUTES { Luid: luid, Attributes: SE_PRIVILEGE_ENABLED }],
        };

        let mut priv_enabled: BOOL = BOOL(1);

        if PrivilegeCheck(token_handle, &mut priv_set, &mut priv_enabled).is_err() {
            println!("Error PrivilegeCheck");
            return;
        }

        println!("SeDebugPrivilege is: {:?}", priv_enabled);

        let mut buffer = vec![0u8; 257];
        let lpbuffer: PSTR = PSTR(buffer.as_mut_ptr());
        let mut pcbbuffer: u32 = buffer.len() as u32;

        if GetUserNameA(lpbuffer, &mut pcbbuffer).is_err() {
            println!("Error GetUserNameA");
            return;
        }
        println!("Username is: {}", lpbuffer.display());


        let pid: u32 = match env::args().nth(1) {
            None => {
                println!("PID arg missing");
                return;
            }
            Some(pid) => pid.to_string().parse().unwrap()
        };
        println!("PID: {}", pid);

        let proc_handle = match OpenProcess(PROCESS_QUERY_INFORMATION, TRUE, pid) {
            Ok(handle) => handle,
            Err(_) => {
                println!("Error OpenProcess");
                return;
            }
        };

        if OpenProcessToken(proc_handle, TOKEN_QUERY | TOKEN_IMPERSONATE | TOKEN_DUPLICATE | TOKEN_ASSIGN_PRIMARY, &mut token_handle).is_err() {
            println!("Error OpenProcessToken");
            return;
        }


        let mut new_token: HANDLE = HANDLE(0);

        if DuplicateTokenEx(token_handle, TOKEN_ACCESS_MASK(MAXIMUM_ALLOWED), None, SecurityImpersonation, TokenImpersonation, &mut new_token).is_err() {
            println!("Error DuplicateTokenEx");
            return;
        }


        if SetThreadToken(None, new_token).is_err() {
            println!("Error SetThreadToken");
            return;
        }


        let mut buffer = vec![0u8; 257];
        let lpbuffer: PSTR = PSTR(buffer.as_mut_ptr());
        let mut pcbbuffer: u32 = buffer.len() as u32;

        if GetUserNameA(lpbuffer, &mut pcbbuffer).is_err() {
            println!("Error GetUserNameA");
            return;
        }
        println!("Username is: {}", lpbuffer.display());


        // pause
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Cant read key press");
    } // end unsafe
}
