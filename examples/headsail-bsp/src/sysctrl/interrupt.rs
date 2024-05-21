#[cfg(feature = "sysctrl-rt")]
#[export_name = "_setup_interrupts"]
fn setup_interrupt_vector() {
    use riscv::register::mtvec;

    // Set the trap vector
    unsafe {
        extern "C" {
            fn _trap_vector();
        }

        // Set all the trap vectors for good measure
        let bits = _trap_vector as usize;
        mtvec::write(bits, mtvec::TrapMode::Vectored);
    }
}

// The vector table
//
// Do the ESP trick and route all interrupts to the direct dispatcher.
//
// N.b. vectors length must be exactly 0x80
#[cfg(feature = "sysctrl-rt")]
core::arch::global_asm!(
    "
.section .vectors, \"ax\"
    .global _trap_vector
    // Trap vector base address must always be aligned on a 4-byte boundary
    .align 4
_trap_vector:
    j _start_trap
    .rept 31
    j _start_trap // 1..31
    .endr
"
);
