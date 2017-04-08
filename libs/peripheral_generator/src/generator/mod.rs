use std::collections::HashMap;
use svd_parser::Peripheral;
use std::str::FromStr;
use regex::Regex;

mod lib_generator;
mod peripherals;

pub use self::lib_generator::LibGenerator;

pub trait Generator {
    fn generate(&self, &mut OutputBuilder);
}

#[derive(Default)]
pub struct GeneratorOutput {
    generators: HashMap<String, Vec<Box<Generator>>>
}

#[derive(Default)]
pub struct OutputBuilder(Vec<Part>);

pub enum Part {
    Comment(String),
    Line(String),
    BlockStart(String),
    BlockEnd
}

impl OutputBuilder {
    pub fn add(&mut self, part: Part) {
        self.0.push(part);
    }
    pub fn addComment(&mut self, str: String) {
        self.0.push(Part::Comment(str));
    }

    pub fn to_string(self) -> String {
        let mut indent = String::new();
        let mut str = String::new();

        for part in self.0.into_iter() {
            match part {
                Part::Comment(line) => str += &format!("{}// {}\n", indent, OutputBuilder::to_single_line(line)),
                Part::Line(line) => str += &format!("{}{}\n", indent, line),
                Part::BlockStart(line) => {
                    str += &format!("{}{} {{\n", indent, line);
                    indent += "  ";
                },
                Part::BlockEnd => {
                    let index = indent.len() - 2;
                    indent.split_off(index);
                    str += &format!("{}}}\n", indent);
                }
            }
        }

        str
    }

    fn to_single_line(string: String) -> String {
        WHITESPACE_REGEX.replace_all(&string, " ").into_owned()
    }
}

lazy_static! {
    static ref WHITESPACE_REGEX: Regex = Regex::from_str("[\\s\\r\\n]{2,}").unwrap();
}

impl GeneratorOutput {
    pub fn add<T>(&mut self, file: String, generator: T) where T : Generator + 'static {
        let entry = self.generators.entry(file).or_insert_with(Vec::new);
        entry.push(Box::new(generator));
    }

    pub fn generate(self) -> HashMap<String, String> {
        let mut result = HashMap::new();
        for (key, generators) in self.generators {
            let mut output = OutputBuilder::default();
            for generator in generators {
                generator.generate(&mut output);
            }
            result.insert(key, output.to_string());
        }
        result
    }
}

pub fn get_generator_for_peripheral(peripheral: &Peripheral) -> impl Generator {
    peripherals::SimpleGenerator::new(peripheral.clone())
}
