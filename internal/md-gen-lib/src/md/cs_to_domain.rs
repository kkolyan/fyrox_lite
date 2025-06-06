use crate::md::csharp_metamodel::{CsClass, CsConstructor, CsEnum, CsField, CsFile, CsMethod, CsProperty, CsType, CsXmlNode};
use crate::Naming;
use gen_common::code_model::{HierarchicalCodeBase, Module};
use gen_common::methods::analyze_method_result;
use gen_common::writelnu;
use itertools::Itertools;
use lite_model::{ClassName, Constant, Domain, EnumClass, Method};
use std::collections::HashMap;
use std::fs;
use std::process::{Command, Stdio};
use to_vec::ToVec;

pub struct CSharpDomain {
    pub packages: Vec<CSharpPackage>,
}

pub struct CSharpPackage {
    pub name: String,
    items: Vec<CsFile>,
}

impl CSharpDomain {
    pub fn generate_md(&self, class_page_links: &HashMap<ClassName, String>) -> HierarchicalCodeBase {
        let mut mods = vec![];

        for package in &self.packages {
            let mut package_mods = vec![];
            for file in &package.items {
                let contents = generate_cs_doc(
                    &file,
                    package.name.as_str(),
                    class_page_links,
                );
                for content in contents {
                    package_mods.push(content);
                }
            }
            mods.push(Module::children(
                package.name.as_str(),
                package_mods,
            ))
        }
        HierarchicalCodeBase { mods }
    }
}

impl CSharpPackage {
    pub fn collect_type_names(&self) -> Vec<String> {
        let mut vec = vec![];
        vec.extend(self.items.iter()
            .flat_map(|it| it.enums.iter())
            .map(|e| e.name.clone()));
        vec.extend(self.items.iter()
            .flat_map(|it| it.classes.iter())
            .map(|e| e.name.clone()));
        vec
    }
}

pub fn generate_cs_defined_domain() -> CSharpDomain {
    Command::new("dotnet")
        .args(["build"])
        .current_dir("internal/cs_dumper_sln")
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();
    let mut packages = vec![];
    for package_candidate in
        fs::read_dir("langs/cs/fyrox-lite-sln/fyrox_lite_cs_netcore/src/FromGodot").unwrap()
    {
        let package_candidate = package_candidate.unwrap();
        if package_candidate.file_type().unwrap().is_dir() {
            let mut items = vec![];
            for type_candidate in fs::read_dir(package_candidate.path()).unwrap() {
                let type_candidate = type_candidate.unwrap();
                if type_candidate
                    .file_name()
                    .to_string_lossy()
                    .ends_with(".cs")
                {
                    println!(
                        "processing file {}",
                        type_candidate.file_name().to_string_lossy()
                    );
                    let output =
                        Command::new("internal/cs_dumper_sln/bin/Debug/net8.0/cs_dumper.exe")
                            .args([
                                type_candidate.path().as_os_str(),
                                // format!("internal/md-gen-lib/{}.json", entry.file_name().display()).as_ref(),
                            ])
                            .stdout(Stdio::piped())
                            .spawn()
                            .unwrap()
                            .wait_with_output()
                            .unwrap();
                    // fs::write(
                    //     format!(
                    //         "internal/md-gen-lib/{}.json",
                    //         type_candidate.file_name().display()
                    //     ),
                    //     &output.stdout,
                    // )
                    // .unwrap();

                    let cs_file = String::from_utf8_lossy(&output.stdout);
                    let cs_file = serde_json::from_str::<CsFile>(&cs_file).unwrap();
                    items.push(cs_file);
                }
            }
            packages.push(CSharpPackage { name: package_candidate.file_name().to_str().unwrap().to_string(), items })
        }
    }
    CSharpDomain { packages }
}

fn generate_cs_doc(
    file: &CsFile,
    package: &str,
    class_page_links: &HashMap<ClassName, String>,
) -> Vec<Module> {
    let mut contents = vec![];
    for class in &file.classes {
        contents.push(class_to_md(class, package, class_page_links))
    }
    for class in &file.enums {
        contents.push(enum_to_md(class, package, class_page_links))
    }
    contents
}

fn class_to_md(
    class: &CsClass,
    package: &str,
    class_page_links: &HashMap<ClassName, String>,
) -> Module {
    let mut s = "".to_string();

    writelnu!(s, "# {}", &class.name);
    if class.is_struct {
        writelnu!(
        s,
        "struct in [FyroxLite](../README.md).[{}](README.md)",
        package
    );
    } else {
        writelnu!(
        s,
        "class in [FyroxLite](../README.md).[{}](README.md)",
        package
    );
    }
    if !class.description.is_empty() {
        writelnu!(s, "## Description");
        writelnu!(s, "{}", cs_docs_to_string(&class.description, "\n"));
    }

    if !class.constructors.is_empty() {
        writelnu!(s, "## Constructors");
        render_constructors(&mut s, class.constructors.iter().to_vec().as_slice(), class_page_links);
    }

    let constants = class.fields.iter().filter(|it| it.is_const).to_vec();
    if !constants.is_empty() {
        writelnu!(s, "## Constants");
        render_constants(&mut s, &constants, class_page_links);
    }

    let properties = class.properties.iter().filter(|it| !it.is_static).to_vec();
    if !properties.is_empty() {
        writelnu!(s, "## Properties");
        render_properties(&mut s, properties.as_slice(), class_page_links);
    }

    let methods = class.methods.iter().filter(|it| !it.is_static).to_vec();
    if !methods.is_empty() {
        writelnu!(s, "## Methods");
        render_methods(&mut s, methods.as_slice(), class_page_links);
    }

    let mut static_props = class.properties.iter().filter(|it| it.is_static).to_vec();
    if !static_props.is_empty() {
        writelnu!(s, "## Static Properties");
        render_properties(&mut s, static_props.as_slice(), class_page_links);
    }

    let static_methods = class.methods.iter().filter(|it| it.is_static).to_vec();
    if !static_methods.is_empty() {
        writelnu!(s, "## Static Methods");
        render_methods(&mut s, static_methods.as_slice(), class_page_links);
    }

    if !class.operators.is_empty() {
        writelnu!(s, "## Operators");
        render_methods(&mut s, class.operators.iter().to_vec().as_slice(), class_page_links);
    }

    Module::code(class.name.clone(), s)
}

fn render_methods(
    s: &mut String,
    methods: &[&CsMethod],
    class_page_links: &HashMap<ClassName, String>,
) {
    writelnu!(s, "| Return Type | Signature | Description |");
    writelnu!(s, "|---|---|---|");
    for method in methods {
        let return_ty = &method.return_ty;
        let generics = "";
        let params = method.parameters.iter().to_vec();
        writelnu!(
            s,
            "| {} | `{}`{} ( {} ) | {} |",
            type_cs_to_md(&return_ty, class_page_links),
            &method.name,
            generics,
            params
                .iter()
                .map(|it| format!("{} <ins>{}</ins>", type_cs_to_md(&it.ty, class_page_links), &it.name))
                .join(", "),
            cs_docs_to_string(&method.description, " ")
        );
    }
}

fn render_constructors(
    s: &mut String,
    methods: &[&CsConstructor],
    class_page_links: &HashMap<ClassName, String>,
) {
    writelnu!(s, "| Signature | Description |");
    writelnu!(s, "|---|---|");
    for method in methods {
        let params = method.parameters.iter().to_vec();
        writelnu!(
            s,
            "| ( {} ) | {} |",
            params
                .iter()
                .map(|it| format!("{} <ins>{}</ins>", type_cs_to_md(&it.ty, class_page_links), &it.name))
                .join(", "),
            cs_docs_to_string(&method.description, " ")
        );
    }
}

fn render_properties(
    s: &mut String,
    props: &[&CsProperty],
    class_page_links: &HashMap<ClassName, String>,
) {
    writelnu!(s, "| Name | Type | Access | Description |");
    writelnu!(s, "|---|---|---|---|");
    for prop in props {
        let access = match (prop.get, prop.set) {
            (true, false) => "get",
            (false, true) => "set",
            (true, true) => "get / set",
            _ => unreachable!(),
        };
        writelnu!(
            s,
            "| `{}` | {} | {} | {} |",
            prop.name,
            type_cs_to_md(&prop.ty, class_page_links),
            access,
            cs_docs_to_string(&prop.description, " ")
        );
    }
}

fn render_constants(
    s: &mut String,
    constants: &[&CsField],
    class_page_links: &HashMap<ClassName, String>,
) {
    writelnu!(s, "| Name | Type | Value | Description |");
    writelnu!(s, "|---|---|---|---|");
    for constant in constants {
        writelnu!(
            s,
            "| `{}` | {} | {} | {} |",
            &constant.name,
            type_cs_to_md(&constant.ty, class_page_links),
            &constant.initializer.as_deref().unwrap_or(""),
            cs_docs_to_string(&constant.description, " ")
        );
    }
}

fn type_cs_to_md(ty: &CsType, class_page_links: &HashMap<ClassName, String>) -> String {
    if ty.name == "real_t" {
        return "float".to_string();
    }
    if ty.name == "?" {
        return format!("{}?", type_cs_to_md(ty.args.first().unwrap(), class_page_links));
    }
    let type_display = class_page_links.get(&ClassName(ty.name.clone()))
        .map(|it| format!("[{}]({})", &ty.name, it))
        .unwrap_or_else(|| format!("{}", &ty.name));
    if ty.args.is_empty() {
        return type_display;
    }
    format!(
        "{}< {} >",
        type_display,
        ty.args
            .iter()
            .map(|it| type_cs_to_md(it, class_page_links))
            .join(", ")
    )
}

fn enum_to_md(
    class: &CsEnum,
    package: &str,
    class_page_links: &HashMap<ClassName, String>,
) -> Module {
    let mut s = "".to_string();
    writelnu!(s, "# {}", &class.name);
    writelnu!(
        s,
        "enum in [FyroxLite](../README.md).[{}](README.md)",
        package
    );
    if !class.description.is_empty() {
        writelnu!(s, "## Description");
        writelnu!(s, "{}", cs_docs_to_string(&class.description, "\n"));
    }

    writelnu!(s, "## Properties");
    writelnu!(s, "| Property | Description |");
    writelnu!(s, "|---|---|");
    for variant in class.members.iter() {
        writelnu!(
            s,
            "| {} | {} |",
            variant.name,
            cs_docs_to_string(&variant.description, " ")
        );
    }
    Module::code(class.name.clone(), s)
}

fn cs_docs_to_string(doc: &[CsXmlNode], line_separator: &str) -> String {
    doc.iter().map(|it| cs_doc_to_string(it, line_separator)).join(line_separator)
}

fn cs_doc_to_string(doc: &CsXmlNode, line_separator: &str) -> String {
    if let Some(text) = &doc.text {
        return text.lines().map(|it| it.trim().strip_prefix("///").unwrap_or(it)).join(line_separator).trim().to_string();
    }
    if let Some(tag) = &doc.element {
        if tag.name == "summary" {
            return cs_docs_to_string(&tag.children, line_separator);
        }
        if tag.name == "c" {
            let nested = cs_docs_to_string(&tag.children, line_separator);
            return format!("`{}`", nested);
        }
        if tag.name == "code" {
            let nested = cs_docs_to_string(&tag.children, line_separator);
            return format!("```{}```", nested);
        }
        if tag.name == "example" {
            return "".to_string();
        }
        if tag.name == "inheritdoc" {
            return format!("The same as {}", tag.attrs.get("cref").unwrap());
        }
        if tag.name == "param" {
            // TODO implement me
            return "".to_string();
        }
        if tag.name == "returns" {
            // TODO implement me
            return "".to_string();
        }
        if tag.name == "seealso" {
            // TODO implement me
            return "".to_string();
        }
        if tag.name == "paramref" {
            return format!("`{}`", tag.attrs.get("name").unwrap());
        }
        if tag.name == "see" {
            if let Some(attr) = tag.attrs.get("cref"){
                return format!("`{}`", attr);
            }
            if let Some(attr) = tag.attrs.get("langword"){
                return format!("`{}`", attr);
            }
            todo!()
        }
        if tag.name == "value" {
            // TODO improve me
            return cs_docs_to_string(&tag.children, line_separator);
        }
    }
    todo!()
}
