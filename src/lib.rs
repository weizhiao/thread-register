#![no_std]

pub trait ModifyRegister {
	/// Get thread register value
    fn get() -> usize;
	/// Set thread register value
    unsafe fn set(value: usize);
}

pub struct ThreadRegister;

#[cfg(target_arch = "x86_64")]
impl ModifyRegister for ThreadRegister {
    fn get() -> usize {
        let val: usize;
        unsafe {
            core::arch::asm!("mov {}, fs:0",out(reg) val);
        }
        val
    }

    unsafe fn set(value: usize) {
        unsafe {
            core::arch::asm!("mov fs:0,{}",in(reg) value);
        }
    }
}

#[cfg(target_arch = "aarch64")]
impl ModifyRegister for ThreadRegister {
    fn get() -> usize {
        let val: usize;
        unsafe {
            core::arch::asm!("mrs {},tpidr_el0",out(reg) val);
        }
        val
    }

    unsafe fn set(value: usize) {
        unsafe {
            unsafe {
                core::arch::asm!("msr tpidr_el0,{}",in(reg) val);
            }
        }
    }
}

#[cfg(any(target_arch = "riscv64", target_arch = "riscv32"))]
impl ModifyRegister for ThreadRegister {
    fn get() -> usize {
        let val: usize;
        unsafe {
            core::arch::asm!("mv {}, tp",out(reg) val);
        }
        val
    }

    unsafe fn set(value: usize) {
        unsafe {
            core::arch::asm!("mv tp,{}",in(reg) value);
        }
    }
}
