use crate::gdt;
use crate::println;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

// The InterruptDescriptorTable struct is a wrapper around an array of
// InterruptDescriptor entries. Each entry is a struct that contains the
// interrupt handler function pointer and the interrupt stack table index.
// The interrupt stack table index is the stack table to use when the CPU
// switches to the interrupt stack table.
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(crate::gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

// Initialize the IDT
pub fn init_idt() {
    // Load the IDT
    // The load function is unsafe because it can cause undefined behavior if the
    // IDT is invalid. The IDT is invalid if it contains invalid interrupt handlers
    // or if the CPU is in a state where it should not be loading the IDT.
    IDT.load();
}

// The breakpoint exception is the perfect exception to test exception handling.
// Its only purpose is to temporarily pause a program when the breakpoint
// instruction int3 is executed.

// When the user sets a breakpoint, the debugger overwrites the corresponding
// instruction with the int3 instruction so that the CPU throws the breakpoint
// exception when it reaches that line.
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

#[test_case]
fn test_breakpoint_exception() {
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}
