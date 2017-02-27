use svd_parser::{Device, Peripheral};
use utils::iter_slice_before_after;
use std::path::Path;

pub fn resolve_derives(device: &mut Device) {
    iter_slice_before_after(&mut device.peripherals, |before, peripheral, after| {
        let parent = match peripheral.derived_from { Some(ref n) => n, None => return };
        let parent = {
            if let Some(index) = before.iter().position(|p| p.name == *parent) { &before[index] }
            else if let Some(index) = after.iter().position(|p| p.name == *parent) { &after[index] }
            else { panic!("Could not find parent {:?}", parent) }
        };
        
        if peripheral.group_name.is_none() { peripheral.group_name = parent.group_name.clone(); }
        if peripheral.description.is_none() { peripheral.description = parent.description.clone(); }
        if peripheral.interrupt.is_none() { peripheral.interrupt = parent.interrupt.clone(); }
        if peripheral.registers.is_none() { peripheral.registers = parent.registers.clone(); }
    });
}

pub fn get_name(device: &Device, peripheral: &Peripheral) -> String {
    let mut filename = Path::new(&peripheral.name).with_extension("rs");
    //println!("{:?}: {:?}", peripheral.name, peripheral.group_name);
    if let Some(ref group_name) = peripheral.group_name {
        let group_count = device.peripherals
            .iter()
            .filter_map(|p| p.group_name.clone())
            .filter(|name| name.eq(group_name))
            .count();
        if group_count > 1 {
            filename = Path::new(group_name).join(filename);
        }
    }
    filename.to_str().unwrap().to_owned()
}
