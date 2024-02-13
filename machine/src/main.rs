#![no_std]
#![no_main]

// #[export_name = "_start"]
// extern "C" fn entry() {

// }

mod fdt;
mod opaque;

extern "C" fn main(hart_id: usize, opaque: usize) {
    let signature = opaque::read_failsafe_signature(opaque);
    if opaque::is_fdt_blob(signature) {
        let _ = opaque::parse_fdt(opaque);
    }

    let _ = hart_id; // TODO

    // TODO
}
