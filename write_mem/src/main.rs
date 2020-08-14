use winapi;                                         // https://docs.rs/winapi/0.3.9/winapi/index.html -> very useful
use winapi::shared::ntdef::{HANDLE, NULL};          // You need to enable this as feature in Cargo.toml
use winapi::shared::minwindef::{FALSE, LPCVOID};
use winapi::um::winnt::PROCESS_ALL_ACCESS;
use winapi::um::processthreadsapi::OpenProcess;     // You need to enable this as feature in Cargo.toml
use winapi::um::handleapi::CloseHandle;             // You need to enable this as feature in Cargo.toml

fn main() {
    use std::io;
    use std::io::Write;

    let pid_target: u32;
    print!("ProcessID Target: ");
    io::stdout().flush().unwrap(); // Print doesn't show without this

    let mut pid_buffer = String::new();
    io::stdin().read_line(&mut pid_buffer).expect("Failed to read line");
    pid_target = pid_buffer.trim().parse::<u32>().unwrap();

    // https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocess?redirectedfrom=MSDN
    let proc_h: HANDLE = unsafe { OpenProcess(PROCESS_ALL_ACCESS, FALSE, pid_target) };
    if proc_h == NULL {
        println!("OpenProcess failed. Error: {:?}", io::Error::last_os_error());
    }

    let int_address: u64;
    print!("Memory address of integer to overwrite: ");
    io::stdout().flush().unwrap(); // Print doesn't show without this
    int_address = read_line_address();

    let int_to_write: i32;
    print!("Overwrite to: ");
    io::stdout().flush().unwrap(); // Print doesn't show without this
    let mut string_buffer = String::new();
    io::stdin().read_line(&mut string_buffer).expect("Failed to read line");
    int_to_write = string_buffer.trim().parse::<i32>().unwrap();

    write_mem::<i32>(proc_h, int_address, int_to_write);

    println!("Overwritten successfully");


    unsafe {CloseHandle(proc_h)};
    let mut read_buffer = String::new();
    io::stdin().read_line(&mut read_buffer).expect("Failed to read line");
}



fn read_line_address() -> u64 {
    let mut address_buffer = String::new();
    std::io::stdin().read_line(&mut address_buffer).expect("Failed to read line");
    let address_trim = address_buffer.trim().trim_start_matches("0x");
    return u64::from_str_radix(address_trim, 16).unwrap();
}

fn write_mem<T: Default>(proc_h: HANDLE, address: u64, mut value: T) {
    use winapi::um::memoryapi::WriteProcessMemory;   // You need to enable this as feature in Cargo.toml

    unsafe {
        let wpm_return = WriteProcessMemory(proc_h, address as *mut _, 
            &mut value as *mut T as LPCVOID, std::mem::size_of::<T>(), 
            NULL as *mut usize);
        if wpm_return == FALSE {
            println!("WriteProcessMemory failed. Error: {:?}", std::io::Error::last_os_error());
        }
    }
}