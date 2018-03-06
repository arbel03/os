use super::idt::ExceptionStackFrame;

pub(in super) extern "x86-interrupt" fn general_protection_fault(stack_frame: &ExceptionStackFrame, error_code: u32) {
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

pub(in super) extern "x86-interrupt" fn bound_range_exceeded(stack_frame: &mut ExceptionStackFrame) {
    println!("Exception! Bound Range Exceeded.");
    println!("{}", stack_frame);
    loop {};
}

pub(in super) extern "x86-interrupt" fn double_fault(stack_frame: &ExceptionStackFrame, error_code: u32) {
    println!("Exception! Double Fault.");
    println!("Error Code: {:b}", error_code);
    println!("{}", stack_frame);
    loop {};
}
