# Steal-Token-RS

## Overview
Steal-Token-RS is a Rust project showcasing how to obtain and impersonate a user's access token, enabling manipulation of system privileges.

## Features
- Retrieves the current user's access token.
- Impersonates another process and acquires its access token.
- Demonstrates the manipulation of system privileges.

## Prerequisites
- Rust programming language and Cargo build system installed.
- Windows operating system.

## Usage
1. Clone the repository: `git clone https://github.com/your-username/steal-token-rs.git`
2. Navigate to the project directory: `cd steal-token-rs`
3. Build the project: `cargo build --release`
4. Run the executable: `.\target\release\steal-token-rs.exe <PID>`
    - Replace `<PID>` with the Process ID (PID) of the target process whose token you want to steal.

## Example
```bash
cargo build --release
.\target\release\steal-token-rs.exe 1234
```



## References
- [Windows API and Impersonation: Part 1](https://0x00-0x00.github.io/research/2018/10/17/Windows-API-and-Impersonation-Part1.html)
- [Enumerating Windows Processes using Rust](https://bazizi.github.io/2022/12/29/enumerating-windows-processes-using-Rust.html)
- [Microsoft Windows Docs for Rust](https://microsoft.github.io/windows-docs-rs/doc/windows/)
- [Microsoft Win32 API Documentation](https://learn.microsoft.com/en-us/windows/win32/api/)
- [Using WinAPI in Rust to Interact with the Windows Process System](https://friendlyuser.github.io/posts/tech/rust/Using_WinAPI_in_Rust_to_Interact_with_the_Windows_Process_System/)
