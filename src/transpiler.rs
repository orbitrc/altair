use regex::Regex;

#[derive(Debug)]
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

#[derive(Debug)]
struct Class {
    name: String,
    derive: DeriveType,
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
    class: Class,
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

    fn parse_class(s: &str) -> Class {
        let re = Regex::new(r"\[class\]\n(?s)[^\[]+").unwrap();
        let mat = match re.find(s) {
            Some(m) => m,
            None => panic!("Cannot find class part in the file."),
        };
        let class_part = &s[mat.start()..mat.end()];

        // Class name.
        let re = Regex::new("name = \"(.+)\"").unwrap();
        let cap = match re.captures(&class_part) {
            Some(c) => c,
            None => panic!("Class name not found."),
        };
        let class_name = cap.get(1).unwrap().as_str();

        // Class derive.
        let re = Regex::new("derive = \"(.+)\"").unwrap();
        let cap = match re.captures(&class_part) {
            Some(c) => c,
            None => panic!("Class derive not found."),
        };
        let class_derive = match cap.get(1).unwrap().as_str() {
            "object" => DeriveType::Object,
            "view" => DeriveType::View,
            "painted_view" => DeriveType::PaintedView,
            _ => panic!("Not a valid derive type."),
        };

        Class {
            name: class_name.to_string(),
            derive: class_derive,
        }
    }

    fn parse_members(s: &str) -> Vec<Member> {
        let re = Regex::new(r"\[members\]\n(?s)[^\[]+").unwrap();
        let mat = match re.find(s) {
            Some(m) => m,
            None => panic!("Cannot find members part in the file."),
        };
        let members_part = &s[mat.start()..mat.end()];
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

    #[test]
    fn transpiler_parse_class() {
        let source = "[module]\nname = \"Foo\"\nversion = \"1.0\"\n\n[class]\nname = \"Foo\"\nderive = \"object\"\n";
        let class = super::Transpiler::parse_class(&source);
        println!("{:?}", class);
    }
}