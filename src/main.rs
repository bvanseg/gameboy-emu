
mod debug;
mod flag;
mod lr35902;
mod opcode;
mod register;

fn main() {
    let mut cpu = lr35902::LR35902::new();

    // Initial state.
    cpu.dump_state();

    // Check state after set.
    cpu.af.set(0xAFFF);
    cpu.dump_state();

    // Reset state.
    cpu.af.set(0x0000);
    cpu.dump_state();
}
