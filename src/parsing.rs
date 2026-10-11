use std::{error::Error, fmt::Display};

pub const TEMPLATE2D: &str = include_str!("./template2d.wesl.template");
pub const TEMPLATE3D: &str = include_str!("./template3d.wesl.template");

#[derive(PartialEq, Eq)]
enum Division {
    None,
    Input,
    Output,
    Pipe,
    Preprocess,
    PostProcess,
    Fragment,
}

#[derive(Debug, Clone, Copy)]
pub enum ParseError<'t> {
    ExpectedCommand { file: &'t str, line: &'t str },
    UnknownCommand { file: &'t str, line: &'t str },
}

impl Display for ParseError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn line_num(file: &str, line: &str) -> usize {
            let Some(range) = file.substr_range(line) else {
                return 0;
            };
            let Some(s) = file.as_bytes().get(0..range.start) else {
                return 0;
            };
            s.iter().filter(|x| **x == b'\n').count()
        }

        match self {
            ParseError::ExpectedCommand { file, line } => {
                let line_num = line_num(file, line);
                writeln!(f, "Error: expected command")?;
                writeln!(f, "   |\n{line_num:<3}| {line}\n   |")
            }
            ParseError::UnknownCommand { file, line } => {
                let line_num = line_num(file, line);
                writeln!(f, "Error: unknown command")?;
                writeln!(f, "   |\n{line_num:<3}| {line}\n   |")
            }
        }
    }
}

impl Error for ParseError<'_> {}

fn parse_name(s: &str) -> Option<&str> {
    let s = s.as_bytes();
    let colon = s.iter().position(|x| *x == b':')?;
    let s = s.get(0..colon)?.trim_ascii_end();
    let ws = s.iter().rposition(|x| matches!(x, b')' | b' '))?;
    core::str::from_utf8(s.get(ws + 1..)?).ok()
}

trait PushString {
    fn push_line(&mut self, s: &str);
    fn push_indented_line(&mut self, s: &str);
    fn push_indented_line_comma(&mut self, s: &str);
}

impl PushString for String {
    fn push_line(&mut self, s: &str) {
        self.push_str(s);
        self.push('\n');
    }

    fn push_indented_line(&mut self, s: &str) {
        self.push_str("    ");
        self.push_str(s);
        self.push('\n');
    }

    fn push_indented_line_comma(&mut self, s: &str) {
        self.push_str("    ");
        self.push_str(s);
        if !self.ends_with(',') {
            self.push(',');
        }
        self.push('\n');
    }
}

pub fn parse_file(file: &str) -> Result<String, ParseError<'_>> {
    let mut d2 = false;
    let mut division = Division::None;
    let mut input_fields = String::new();
    let mut output_fields = String::new();
    let mut pipe_buffer = String::new();
    let mut decompress_buffer = String::new();
    let mut preprocess_buffer = String::new();
    let mut postprocess_buffer = String::new();
    let mut fragment_buffer = String::new();
    for line in file.split('\n') {
        let content = line.trim();
        if content.is_empty() || content.starts_with("//") {
            continue;
        }
        if content.ends_with(':') {
            match content {
                "input:" => {
                    division = Division::Input;
                }
                "output:" => {
                    division = Division::Output;
                }
                "pipe:" => {
                    division = Division::Pipe;
                }
                "preprocess:" => {
                    division = Division::Preprocess;
                }
                "postprocess:" => {
                    division = Division::PostProcess;
                }
                "fragment:" => {
                    division = Division::Fragment;
                }
                _ => return Err(ParseError::UnknownCommand { file, line }),
            }
        } else {
            match division {
                Division::None if content == "2d" || content == "2D" => d2 = true,
                Division::None if content == "3d" || content == "3D" => d2 = false,
                Division::None => return Err(ParseError::ExpectedCommand { file, line }),
                Division::Input => {
                    input_fields.push_indented_line_comma(line);
                    if let Some(name) = parse_name(line) {
                        decompress_buffer.push_indented_line(&format!(
                            "uncompressed_vertex.{name} = vertex_in.{name};"
                        ));
                    }
                }
                Division::Output => {
                    output_fields.push_indented_line_comma(line);
                }
                Division::Pipe => {
                    input_fields.push_indented_line_comma(line);
                    output_fields.push_indented_line_comma(line);
                    if let Some(name) = parse_name(line) {
                        decompress_buffer.push_indented_line(&format!(
                            "uncompressed_vertex.{name} = vertex_in.{name};"
                        ));
                        pipe_buffer.push_indented_line(&format!("out.{name} = vertex.{name};"));
                    }
                }
                Division::Preprocess => {
                    preprocess_buffer.push_line(line);
                }
                Division::PostProcess => {
                    postprocess_buffer.push_line(line);
                }
                Division::Fragment => {
                    fragment_buffer.push_line(line);
                }
            }
        }
    }

    let mut template = if d2 {
        TEMPLATE2D.to_owned()
    } else {
        TEMPLATE3D.to_owned()
    };

    template = template.replace("{{input}}", &input_fields);
    template = template.replace("{{output}}", &output_fields);
    template = template.replace("{{pipe}}", &pipe_buffer);
    template = template.replace("{{decompress}}", &decompress_buffer);
    if !preprocess_buffer.is_empty() {
        template = template.replace("{{preprocess}}", "let _vert1 = preprocess(_vert0);");
        template.push('\n');
        template.push_str(&preprocess_buffer);
    } else {
        template = template.replace("{{preprocess}}", "let _vert1 = _vert0;");
    }

    if !postprocess_buffer.is_empty() {
        template = template.replace(
            "{{postprocess}}",
            "let out2 = postprocess(vertex, out, world_from_local);",
        );
        template.push('\n');
        template.push_str(&postprocess_buffer);
    } else {
        template = template.replace("{{postprocess}}", "let out2 = out;");
    }

    if !fragment_buffer.is_empty() {
        template.push('\n');
        template.push_str(&fragment_buffer);
    }

    Ok(template)
}
