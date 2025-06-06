use std::{fmt::Display, fs, process::{self, Stdio}};

#[derive(Default)]
pub struct HierarchicalCodeBase {
    pub mods: Vec<Module>,
}
impl HierarchicalCodeBase {
    pub fn write_rust(&self, target_dir: &str) {
        write_rust_mods(target_dir, &self.mods);
    }
    pub fn write_lua(&self, target_dir: &str) {
        write_lua_mods(target_dir, &self.mods);
    }
    pub fn write_md(&self, target_dir: &str) {
        write_md_mods(target_dir, &self.mods);
    }
}

pub struct Module {
    pub name: String,
    pub content: ModContent,
}

pub enum ModContent {
    Children(Vec<Module>),
    Code(String)
}

impl Module {
    pub fn code(name: impl Display, code: impl Display) -> Module {
        Module {
            name: name.to_string(),
            content: ModContent::Code(code.to_string()),
        }
    }
    
    pub fn children(name: impl Display, children: Vec<Module>) -> Module {
        Module {
            name: name.to_string(),
            content: ModContent::Children(children),
        }
    }

    fn write_rust(&self, parent_dir: &str)  {
        match &self.content {
            ModContent::Children(children) => {

                let dir = format!("{}/{}", parent_dir, self.name);
                
                write_rust_mods(&dir, children);
            },
            ModContent::Code(code) => {
                let file = format!("{}/{}.rs", parent_dir, self.name);
                fs::write(&file, code).unwrap();
                crate::fmt::fmt_file(file);
            },
        }
    }
    fn write_lua(&self, parent_dir: &str)  {
        match &self.content {
            ModContent::Children(children) => {

                let dir = format!("{}/{}", parent_dir, self.name);
                
                write_lua_mods(&dir, children);
            },
            ModContent::Code(code) => {
                let file = format!("{}/{}.lua", parent_dir, self.name);
                fs::write(&file, code).unwrap();
                crate::fmt::fmt_file(file);
            },
        }
    }
    fn write_md(&self, parent_dir: &str)  {
        match &self.content {
            ModContent::Children(children) => {

                let dir = format!("{}/{}", parent_dir, self.name);

                write_md_mods(&dir, children);
            },
            ModContent::Code(code) => {
                let file = format!("{}/{}.md", parent_dir, self.name);
                fs::write(&file, code).unwrap();
                crate::fmt::fmt_file(file);
            },
        }
    }
}

fn write_rust_mods(dir: &str, children: &[Module])  {
    let _ = fs::create_dir_all(dir);

    let lib_rs = children
        .iter()
        .map(|it| format!("pub mod {};\n", it.name))
        .collect::<Vec<_>>()
        .join("");

    fs::write(format!("{}/mod.rs", &dir), lib_rs).unwrap();

    for m in children.iter() {
        m.write_rust(dir);
    }
}

fn write_lua_mods(dir: &str, children: &[Module])  {
    let _ = fs::create_dir_all(dir);

    for m in children.iter() {
        m.write_lua(dir);
    }
}

fn write_md_mods(dir: &str, children: &[Module])  {
    let _ = fs::create_dir_all(dir);

    for m in children.iter() {
        m.write_md(dir);
    }
}