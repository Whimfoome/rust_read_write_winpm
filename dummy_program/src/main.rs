fn main() {
    let var_int: i32 = 123456;
    let var_string: String = String::from("DefaultString");

    let ptr2int: *const i32 = &var_int;
    let ptr2ptr: *const *const i32 = &ptr2int;
    let ptr2ptr2: *const *const *const i32 = &ptr2ptr;

    loop {
        println!();
        println!("ProcessID: {}", std::process::id());

        println!();
        println!("var_int ({:p}) = {}", &var_int, var_int);
        println!("var_string ({:p}) = {}", &var_string, var_string);

        println!();
        println!("ptr2int ({:p}) = {:p}", &ptr2int, ptr2int);
        println!("ptr2ptr ({:p}) = {:p}", &ptr2ptr, ptr2ptr);
        println!("ptr2ptr2 ({:p}) = {:p}", &ptr2ptr2, ptr2ptr2);

        println!();

        println!("Press ENTER to print again.");

        // Read Line
        let mut read_buffer = String::new();
        std::io::stdin().read_line(&mut read_buffer).expect("Failed to read line");
        println!();
        println!("----------------------------------------");
        println!();
    }
}
