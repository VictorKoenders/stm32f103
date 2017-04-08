use generator::{Generator, OutputBuilder, Part};
use svd_parser::{Peripheral, Register, RegisterInfo};

pub struct SimpleGenerator {
    peripheral: Peripheral,
}

impl SimpleGenerator {
    pub fn new(peripheral: Peripheral) -> SimpleGenerator {
        SimpleGenerator {
            peripheral: peripheral
        }
    }
}

impl Generator for SimpleGenerator {
    fn generate(&self, output: &mut OutputBuilder) {
        output.add(Part::Comment(self.peripheral.name.clone()));
        output.add(Part::Comment(self.peripheral.description.clone().unwrap()));
        output.add(Part::Line(format!("pub const ADDRESS: u32 = 0x{:08X};", self.peripheral.base_address)));

        if let Some(ref registers) = self.peripheral.registers {
            for register in registers {
                assert!(if let &Register::Single(_) = register { true } else { false });
                let register: &RegisterInfo = register;
                //output.add
            }
        }

        output.add(Part::Line(format!("/*\n{:#?}\n*/", self.peripheral)));
    }
}