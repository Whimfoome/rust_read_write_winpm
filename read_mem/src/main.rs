use winapi;                                         // https://docs.rs/winapi/0.3.9/winapi/index.html -> very useful
use winapi::shared::ntdef::{HANDLE, NULL};          // You need to enable this as feature in Cargo.toml
use winapi::shared::minwindef::{FALSE, LPVOID};
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
    print!("Memory address of var_int to read: ");
    io::stdout().flush().unwrap(); // Print doesn't show without this
    int_address = read_line_address();
    println!("Reading 0x{:x} ...", int_address); // Prints mem_address as hex

    let int_read = read_mem::<i32>(proc_h, int_address);
    println!("int_read: {}", int_read);
    
    println!();
    

    let ptr_address: u64;
    print!("Memory address of ptr2int to read: ");
    io::stdout().flush().unwrap();
    ptr_address = read_line_address();
    println!("Reading 0x{:x} ...", ptr_address);

    let ptr2int_read = read_mem::<u64>(proc_h, ptr_address);
    println!("ptr2int_read: 0x{:x}", ptr2int_read);

    let ptr_pointed = read_mem::<i32>(proc_h, ptr2int_read);
    println!("value_pointed: {}", ptr_pointed);


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

fn read_mem<T: Default>(proc_h: HANDLE, address: u64) -> T {
    use winapi::um::memoryapi::ReadProcessMemory;   // You need to enable this as feature in Cargo.toml

    let mut ret: T = Default::default();

    unsafe {
        let rpm_return = ReadProcessMemory(proc_h, address as *mut _,
            &mut ret as *mut T as LPVOID, std::mem::size_of::<T>(), 
            NULL as *mut usize);
        if rpm_return == FALSE {
            println!("ReadProcessMemory failed. Error: {:?}", std::io::Error::last_os_error());
        }
    }

    return ret;
}