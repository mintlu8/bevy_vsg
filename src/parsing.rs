pub const TEMPLATE2D: &str = include_str!("./template2d.wesl.template");
pub const TEMPLATE3D: &str = include_str!("./template3d.wesl.template");

#[derive(PartialEq, Eq)]
pub enum Division {
    None,
    Input,
    Output,
    Pipe,
    Preprocess,
    PostProcess,
    Fragment,
}

pub fn parse_name(s: &str) -> Option<&str> {
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

pub fn parse_file(s: &str) -> String {
    let mut d2 = false;
    let mut division = Division::None;
    let mut input_fields = String::new();
    let mut output_fields = String::new();
    let mut pipe_buffer = String::new();
    let mut decompress_buffer = String::new();
    let mut preprocess_buffer = String::new();
    let mut postprocess_buffer = String::new();
    let mut fragment_buffer = String::new();
    for line in s.split('\n') {
        if line.starts_with("//") {
            continue;
        }
        match line.trim() {
            "" => continue,
            "2d" | "2D" if division == Division::None => d2 = true,
            "3d" | "3D" if division == Division::None => d2 = false,
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
            _ => match division {
                Division::None => (),
                Division::Input => {
                    input_fields.push_indented_line_comma(line);
                    if let Some(name) = parse_name(line) {
                        decompress_buffer.push_str(&format!("    uncompressed_vertex.{name} = vertex_in.{name};\n"));
                    }
                }
                Division::Output => {
                    output_fields.push_indented_line_comma(line);
                }
                Division::Pipe => {
                    input_fields.push_indented_line_comma(line);
                    output_fields.push_indented_line_comma(line);
                    if let Some(name) = parse_name(line) {
                        decompress_buffer.push_str(&format!("    uncompressed_vertex.{name} = vertex_in.{name};\n"));
                        pipe_buffer.push_str(&format!("    out.{name} = vertex.{name};\n"));
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
            },
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

    template
}
