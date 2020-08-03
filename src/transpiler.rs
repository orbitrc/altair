use regex::Regex;

enum DeriveType {
    Object,
    View,
    PaintedView,
}

#[derive(Debug)]
struct Module {
    name: String,
    version: String,
}

struct Member {
    member_type: String,
    name: String,
}

struct Method {
    name: String,
    return_type: String,
}

pub struct Transpiler {
    module: Module,
    class_name: String,
    class_derive: DeriveType,
    members: Vec<Member>,
    methods: Vec<Method>,
    properties: Vec<String>,
    signals: Vec<String>,
    slots: Vec<String>,
}

impl Transpiler {
    fn parse_module(s: &str) -> Module {
        let re = Regex::new(r"\[module\]\n(?s)[^\[]+").unwrap();
        let mat = match re.find(s) {
            Some(m) => m,
            None => panic!("Cannot find module part in the file."),
        };
        let module_part = &s[mat.start()..mat.end()];

        // Module name.
        let re = Regex::new("name = \"(.+)\"").unwrap();
        let cap = match re.captures(&module_part) {
            Some(c) => c,
            None => panic!("Module name not found."),
        };
        let module_name = cap.get(1).unwrap().as_str();

        // Module version.
        let re = Regex::new("version = \"(.+)\"").unwrap();
        let cap = match re.captures(&module_part) {
            Some(c) => c,
            None => panic!("Module version not found."),
        };
        let module_version = cap.get(1).unwrap().as_str();

        Module {
            name: module_name.to_string(),
            version: module_version.to_string(),
        }
    }

    // pub fn parse(s: &str) -> Transpiler {
    // }
}

#[cfg(test)]
mod tests {
    #[test]
    fn transpiler_parse_module() {
        let source = "[module]\nname = \"Foo\"\nversion = \"1.0\"\n";
        let module = super::Transpiler::parse_module(&source);
        println!("{:?}", module);
    }
}