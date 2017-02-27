use super::{Generator, OutputBuilder, Part};
use itertools::Itertools;
use svd_parser::Device;

pub struct LibGenerator {
    device: Device,
}

impl LibGenerator {
    pub fn new(device: Device) -> LibGenerator {
        LibGenerator {
            device: device
        }
    }
}

impl Generator for LibGenerator {
    fn generate(&self, builder: &mut OutputBuilder) {
        builder.add(Part::Line(format_args!("#![no_std]")));
        builder.add(Part::Line(format_args!("")));
        
        for peripheral_name in self.device.peripherals
            .iter()
            .filter_map(|p| p.group_name.clone())
            .unique() {
            builder.add(Part::Line(format_args!("pub mod {};", peripheral_name.to_lowercase())));
        }
    }
}

