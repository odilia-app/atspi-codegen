use std::path::Path;

macro_rules! include_files {
    ($($name: literal),*) => {
        const NAMES: &[&str] = &[
            $($name),*
        ];
        const FILE_NAMES: &[&str] = &[
            $(concat!("xml/", $name, ".xml")),*
        ];
        const FILES: &[&str] = &[
            $(include_str!(concat!("xml/", $name, ".xml"))),*
        ];
    }
}

include_files![
    "Accessibility",
    "Accessible",
    "Action",
    "Application",
    "Cache",
    "Collection",
    "Component",
    "DeviceEventController",
    "DeviceEventListener",
    "Document",
    "EditableText",
    "Event",
    "Hyperlink",
    "Hypertext",
    "Image",
    "Processed",
    "Registry",
    "Selection",
    "Socket",
    "TableCell",
    "Table",
    "Text",
    "Value"
];

fn main() {
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let mut opts = dbus_codegen::GenOpts::default();
    opts.methodtype = None; // Generate client code

    for i in 0..NAMES.len() {
        let name = NAMES[i];
        let file_name = FILE_NAMES[i];
        let file = FILES[i];

        println!("cargo:rerun-if-changed={}", file_name);
        let mut out_path = Path::new(&out_dir).join(cammel_to_snake(name));
        out_path.set_extension("rs");
        // Run codegen
        let code = dbus_codegen::generate(file, &opts).unwrap();
        // Save generated code
        std::fs::write(out_path, &code).unwrap();
    }
}

fn cammel_to_snake(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut chars = input.chars();
    if let Some(c) = chars.next() {
        for c in c.to_lowercase() {
            out.push(c);
        }
    }
    for c in chars {
        if c.is_uppercase() {
            out.push('_');
        }
        for c in c.to_lowercase() {
            out.push(c);
        }
    }
    out
}
