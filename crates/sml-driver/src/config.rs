use crate::compiler::Compiler;
use sml_util::interner::Interner;
use std::env;

#[derive(Default)]
pub struct CompilerBuilder {
    measure: Option<bool>,
    verbosity: Option<u8>,
    phase: Option<String>,
}

impl CompilerBuilder {
    pub fn build<'a>(self, arena: &'a sml_core::arenas::CoreArena<'a>) -> Compiler<'a> {
        Compiler {
            arena,
            src: String::default(),
            elab: sml_core::elaborate::Context::new(arena),
            interner: Interner::with_capacity(4096),
            measure: self.measure.unwrap_or(false),
            verbosity: self.verbosity.unwrap_or(0),
            stop_phase: self.phase.unwrap_or_default(),
            times: Vec::new(),
        }
    }

    pub fn verbosity(mut self, val: u8) -> Self {
        self.verbosity = Some(val);
        self
    }

    pub fn phase(mut self, val: String) -> Self {
        self.phase = Some(val);
        self
    }

    pub fn measure(mut self, val: bool) -> Self {
        self.measure = Some(val);
        self
    }
}

pub struct ArgParse {
    pub builder: CompilerBuilder,
    pub files: Vec<String>,
}

impl ArgParse {
    pub fn parse(args: env::Args) -> ArgParse {
        let mut stack = args.skip(1).rev().collect::<Vec<String>>();
        let mut files = Vec::new();
        let mut builder = CompilerBuilder::default();
        while !stack.is_empty() {
            let item = stack.pop().unwrap();
            if item.starts_with("--") {
                match item.as_ref() {
                    "--silent" => {
                        builder = builder.verbosity(0);
                    }
                    "--v" => {
                        builder = builder.verbosity(1);
                    }
                    "--vv" => {
                        builder = builder.verbosity(2);
                    }
                    "--measure" => {
                        builder = builder.measure(true);
                    }
                    "--phase" => {
                        let phase =
                            match stack.pop().expect("expected phase after --phase").as_ref() {
                                "parse" => "parse".into(),
                                "elab" => "elaborate".into(),
                                "mono" => "monomorphize".into(),
                                "flat" => "flatten".into(),
                                item => panic!("unrecognized compiler phase: {}", item),
                            };
                        builder = builder.phase(phase);
                    }
                    _ => panic!("unrecognized compiler flag: {}", item),
                }
            } else {
                files.push(item);
            }
        }

        ArgParse { builder, files }
    }
}
