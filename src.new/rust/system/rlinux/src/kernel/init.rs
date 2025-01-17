#[cfg(all(__noretpoline, not(MODULE)))]
#[macro_export]
macro_rules! __noinitretpoline {
    () => {
        __noretpoline!();
    };
}

#[cfg(not(all(__noretpoline, not(MODULE))))]
#[macro_export]
macro_rules! __noinitretpoline {() => {};}

#[macro_export]
macro_rules! __init {
    () => {
        unsafe {
            core::arch::asm!(
                ".section .init.text"
            )
        }
        __noinitretpoline!();
    };
}

#[macro_export]
macro_rules! __initdata {
    () => {
        unsafe {
            core::arch::asm!(
                ".section .init.data"
            )
        }
    };
}

#[macro_export]
macro_rules! __initconst {
    () => {
        unsafe {
            core::arch::asm!(
                ".section .init.rodata"
            )
        }
    };
}

#[macro_export]
macro_rules! __exitdata {
    () => {
        unsafe {
            core::arch::asm!(
                ".section .exit.data"
            )
        }
    };
}

#[macro_export]
macro_rules! __exit_call {
    () => {
        unsafe {
            core::arch::asm!(
                ".section .exitcall.exit"
            )
        }
    };
}

#[macro_export]
macro_rules! __ref {
    () => {
        unsafe {
            core::arch::asm!(
                ".section .ref.text"
            )
        }
    };
}

#[macro_export]
macro_rules! __refdata {
    () => {
        unsafe {
            core::arch::asm!(
                ".section .ref.data"
            )
        }
    };
}

#[macro_export]
macro_rules! __refconst {
    () => {
        unsafe {
            core::arch::asm!(
                ".section .ref.rodata"
            )
        }
    };
}

#[macro_export]
macro_rules! __exitused {() => {};}

#[macro_export]
macro_rules! __exit {
    () => {
        unsafe {
            core::arch::asm!(
                ".section .exit.text"
            )
        }
    };
}

#[cfg(CONFIG_MEMORY_HOTPLUG)]
#[macro_export]
macro_rules! __meminit {() => {};}

#[cfg(CONFIG_MEMORY_HOTPLUG)]
#[macro_export]
macro_rules! __meminitdata {() => {};}

#[cfg(CONFIG_MEMORY_HOTPLUG)]
#[macro_export]
macro_rules! __meminitconst {() => {};}

#[cfg(not(CONFIG_MEMORY_HOTPLUG))]
#[macro_export]
macro_rules! __meminit {() => {__init!()};}

#[cfg(not(CONFIG_MEMORY_HOTPLUG))]
#[macro_export]
macro_rules! __meminitdata {() => {__initdata!()};}


#[cfg(not(CONFIG_MEMORY_HOTPLUG))]
#[macro_export]
macro_rules! __meminitconst {() => {__initconst!()};}

macro_rules! __HEAD {
    () => {
        todo!()
    };
}

macro_rules! __INIT {
    () => {
        todo!()
    };
}

macro_rules! __FINIT {
    () => {
        todo!()
    };
}

macro_rules! __INITDATA {
    () => {
        todo!()
    };
}

macro_rules! __INITRODATA {
    () => {
        todo!()
    };
}

macro_rules! __FINITDATA {
    () => {
        todo!()
    };
}

macro_rules! __REF {
    () => {
        todo!()
    };
}

macro_rules! __REFDATA {
    () => {
        todo!()
    };
}

macro_rules! __REFCONST {
    () => {
        todo!()
    };
}

#[cfg(not(__ASSEMBLY__))]
pub type initcall_t = fn() -> i64;
#[cfg(not(__ASSEMBLY__))]
pub type exitcall_t = fn();

#[cfg(CONFIG_HAVE_ARCH_PREL32_RELOCATIONS)]
pub type initcall_entry_t = i64;

#[cfg(CONFIG_HAVE_ARCH_PREL32_RELOCATIONS)]
#[inline]
pub fn initcall_from_entry(entry: &initcall_entry_t) -> initcall_t{
    use super::compiler::offset_to_ptr;
    offset_to_ptr(off)
}

#[cfg(not(CONFIG_HAVE_ARCH_PREL32_RELOCATIONS))]
pub type initcall_entry_t = initcall_t;

#[cfg(not(CONFIG_HAVE_ARCH_PREL32_RELOCATIONS))]
#[inline]
pub fn initcall_from_entry(entry: &initcall_entry_t) -> initcall_t{
    *entry
}

pub fn __con_initcall_start() -> initcall_entry_t{
    todo!()
}

pub fn __con_initcall_end() -> initcall_entry_t{
    todo!()
}

pub type ctor_fn_t = fn() -> dyn core::any::Any;

pub struct file_system_type;

pub use crate::do_one_initcall;
/*TODO!
extern char __initdata boot_command_line[];
extern char *saved_command_line;
extern unsigned int saved_command_line_len;
extern unsigned int reset_devices;

void setup_arch(char **);
void prepare_namespace(void);
void __init init_rootfs(void);

void init_IRQ(void);
void time_init(void);
void poking_init(void);
void pgtable_cache_init(void);

extern initcall_entry_t __initcall_start[];
extern initcall_entry_t __initcall0_start[];
extern initcall_entry_t __initcall1_start[];
extern initcall_entry_t __initcall2_start[];
extern initcall_entry_t __initcall3_start[];
extern initcall_entry_t __initcall4_start[];
extern initcall_entry_t __initcall5_start[];
extern initcall_entry_t __initcall6_start[];
extern initcall_entry_t __initcall7_start[];
extern initcall_entry_t __initcall_end[];

extern struct file_system_type rootfs_fs_type;

extern bool rodata_enabled;
void mark_rodata_ro(void);

extern void (*late_time_init)(void);

extern bool initcall_debug;
*/

#[cfg(MODULE)]
/* TODO! */
todo!();

#[cfg(not(MODULE))]
#[macro_export]
macro_rules! THIS_MODULE {
    () => {
        
    };
}