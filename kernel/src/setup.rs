use crate::hardware::binary_struct::BinaryStruct;
use crate::{asm, hardware};
use riscv_utils::*;

static mut SETUP: bool = false;

pub unsafe fn setup() {
    if SETUP {
        return;
    }
    SETUP = true;
    // set M Previous Privilege mode to User so mret returns to user mode.
    let mstatus: u64;
    read_machine_reg!("mstatus" => mstatus);
    let mut mstatus = BinaryStruct::from(mstatus);
    mstatus.write_register_entry(MSTATUS_MPP_U.0);
    mstatus.write_register_entry(MSTATUS_MPP_U.1);

    // enable machine-mode interrupts.
    mstatus.write_register_entry(MSTATUS_MIE);
    let mstatus = mstatus.get();
    write_machine_reg!(mstatus => "mstatus");

    // set the machine-mode trap handler.
    let trap_handler = asm::exception as u64;
    // disable paging for now.
    let paging = 0;
    write_machine_reg!(
        trap_handler => "mtvec",
        paging => "satp"
    );
    // configure Physical Memory Protection to give user mode access to all of physical memory.
    hardware::pmp::init();
    hardware::clint::init_timer();

    // enable software interrupts (ecall) in M mode. enable timer interrupts.
    let mie: u64;
    read_machine_reg!("mie" => mie);
    let mut mie = BinaryStruct::from(mie);
    mie.write_register_entry(MIE_MSIE);
    mie.write_register_entry(MIE_MTIE);
    write_machine_reg!(mie.get() => "mie");
}
