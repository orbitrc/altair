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

#[derive(Debug)]
struct Member {
    member_type: String,
    name: String,
}

#[derive(Debug)]
struct Arg {
    arg_type: String,
    name: String,
}

#[derive(Debug)]
struct Method {
    name: String,
    return_type: Option<String>,
    args: Vec<Arg>,
}

#[derive(Debug)]
struct Property {
    property_type: String,
    read_method: String,
    write_method: Option<String>,
    notify_method: Option<String>,
}

pub struct Transpiler {
    module: Module,
    class: Class,
    members: Vec<Member>,
    methods: Vec<Method>,
    properties: Vec<Property>,
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
        let mut members: Vec<Member> = vec![];
        for line in members_part.lines() {
            if line.starts_with("[members") {
                continue;
            }
            if line.trim() == "" {
                continue;
            }
            let kv = line.split(":").map(|x| x.trim()).collect::<Vec<&str>>();
            let member = Member {
                member_type: kv[1].to_string(),
                name: kv[0].to_string(),
            };
            members.push(member);
        }

        members
    }

    fn parse_methods(s: &str) -> Vec<Method> {
        let re = Regex::new(r"\[methods\]\n(?s)[^\[]+").unwrap();
        let mat = match re.find(s) {
            Some(m) => m,
            None => panic!("Cannot find methods part in the file."),
        };
        let methods_part = &s[mat.start()..mat.end()];
        let mut methods: Vec<Method> = vec![];
        for line in methods_part.lines() {
            if line.starts_with("[methods") {
                continue;
            }
            if line.trim() == "" {
                continue;
            }
            // Method name.
            let name_re = Regex::new(r"[^\(]+").unwrap();
            let mat = match name_re.find(&line) {
                Some(m) => m,
                None => panic!("Cannot find a method name. line: {}", line),
            };
            let method_name = &line[mat.start()..mat.end()];
            // Method args.
            let mut method_args: Vec<Arg> = vec![];
            let args_re = Regex::new(r"\(.*\)").unwrap();
            let mat = match args_re.find(&line) {
                Some(m) => m,
                None => panic!(""),
            };
            let mut args_str = &line[mat.start()..mat.end()];
            // Remove (, ).
            args_str = &args_str[1..args_str.len() - 1];
            for arg_str in args_str.split(",") {
                if args_str == "" {
                    continue;
                }
                let v = arg_str.split(":").map(|x| x.trim()).collect::<Vec<&str>>();
                let arg = Arg {
                    arg_type: v[1].to_string(),
                    name: v[0].to_string(),
                };
                method_args.push(arg);
            }
            // Method return type.
            let mut method_return: Option<String> = None;
            if line.contains("->") {
                let re = Regex::new(r"-> *(.+)").unwrap();
                let cap = match re.captures(&line) {
                    Some(c) => c,
                    None => panic!("Method returns but not captured."),
                };
                method_return = Some(cap.get(1).unwrap().as_str().to_string());
            }
            let method = Method {
                name: method_name.to_string(),
                return_type: method_return,
                args: method_args,
            };
            methods.push(method);
        }

        methods
    }

    fn parse_properties(s: &str) -> Vec<Property> {
        let mut properties: Vec<Property> = vec![];

        let re = Regex::new(r"\[properties\]\n(?s)[^\[]+").unwrap();
        let mat = match re.find(s) {
            Some(m) => m,
            None => panic!("Cannot find properties part in the file."),
        };
        let properties_part = &s[mat.start()..mat.end()];
        for line in properties_part.lines() {
            if line.starts_with("[properties") {
                continue;
            }
            if line.trim() == "" {
                continue;
            }
            let re = Regex::new(r"").unwrap();
        }

        properties
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

    #[test]
    fn transpiler_parse_members() {
        let source = "[members]\nname: String\nage: i32\n";
        let members = super::Transpiler::parse_members(&source);
        println!("{:?}", members);
    }

    #[test]
    fn transpiler_parse_methods() {
        let source = "[methods]\nget_older()\nfoo(a: String) -> String\n";
        let methods = super::Transpiler::parse_methods(&source);
        println!("{:?}", methods);
    }
}