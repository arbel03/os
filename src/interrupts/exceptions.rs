use super::idt::ExceptionStackFrame;

 pub(in super) extern "C" fn general_protection_fault(stack_frame: &ExceptionStackFrame, error_code: u32) {
    println!("Exception! General Protection Fault.");
    println!("Error code: {:b}", error_code);
    if error_code != 0 {
        let tbl_num = (error_code & 6) >> 1;
        println!("Error in {}", if tbl_num == 1 { "IDT" } else { "GDT" });
        println!("Error in index: {}", error_code >> 3);
    }
    println!("{}", stack_frame);
    loop {};
}

pub(in super) extern "C" fn double_fault(stack_frame: &ExceptionStackFrame, error_code: u32) {
    println!("Exception! Double Fault.");
    println!("Error Code: {:b}", error_code);
    println!("{}", stack_frame);
    loop {};
}

pub(in super) extern "C" fn invalid_tss(stack_frame: &ExceptionStackFrame, error_code: u32) {
    println!("Exception! Invalid TSS.");
    println!("Error Code: {:b}", error_code);
    println!("{}", stack_frame);
    loop {};
}
