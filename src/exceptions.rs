
fn nmi(){
    bkpt!();
}

fn hard_fault() {
    bkpt!();
}

fn mem_manage_fault() {
    bkpt!();
}

fn bus_fault() {
    bkpt!();
}

fn debug_monitor() {
    bkpt!();
}

fn usage_fault() {
    bkpt!();
}

fn svcall() {
    bkpt!();
}

fn pendsv() {
    bkpt!();
}

fn sys_tick(){
    bkpt!();
}

pub type Handler = fn();

#[export_name = "_EXCEPTIONS"]
pub static EXCEPTIONS: [Option<Handler>; 14] = [
    Some(nmi),
    Some(hard_fault),
    Some(mem_manage_fault),
    Some(bus_fault),
    Some(usage_fault),
    None,
    None,
    None,
    None,
    Some(svcall),
    Some(debug_monitor),
    None,
    Some(pendsv),
    Some(sys_tick)
];

