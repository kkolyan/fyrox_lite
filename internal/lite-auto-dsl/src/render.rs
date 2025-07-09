use crate::ImportResult;
use fmt::Write;
use lite_model::DataType;
use std::path::Path;
use std::{fmt, fs};

pub fn render(file: impl AsRef<Path>, symbols: ImportResult) {
    let file = file.as_ref();
    let _ = fs::create_dir_all(file.parent().unwrap());

    let mut s = String::new();
    for (original_name, struct_) in symbols.struct_classes {
        let ty_name = struct_.class_name;
        let mut items = String::new();
        for field in struct_.fields {
            let field_name = field.name;
            let field_ty = ty_to_rs(&field.ty);
            write!(
                items,
                "
                pub {field_name}: {field_ty},
            "
            )
            .unwrap();
        }
        write!(
            &mut s,
            r###"
            pub struct {ty_name}
            {{{items}
            }}
            "###,
        )
        .unwrap();
    }
    for (original_name, adt_) in symbols.adt_classes {
        let ty_name = adt_.class_name;
        let mut items = String::new();
        for field in adt_.children {
            let field_name = field.name;
            let field_ty = ty_to_rs(&field.ty);
            write!(
                fields,
                "
                {field_name}(Lite{field_ty}),
            "
            )
            .unwrap();
        }
        write!(
            &mut s,
            r###"
            pub struct {ty_name}
            {{{items}
            }}
            "###,
        )
        .unwrap();
    }
    fs::write(file, s).unwrap();
}

fn ty_to_rs(ty: &DataType) -> String {
    match ty {
        DataType::UnresolvedClass(it) => format!("Lite{}", it),
        DataType::Unit => todo!(),
        DataType::Bool => todo!(),
        DataType::Byte => todo!(),
        DataType::I32 => format!("i32"),
        DataType::I64 => format!("i64"),
        DataType::F32 => format!("f32"),
        DataType::F64 => format!("f64"),
        DataType::String => todo!(),
        DataType::ClassName => todo!(),
        DataType::Vec(it) => format!("Vec<{}>", ty_to_rs(&it)),
        DataType::UserScript => todo!(),
        DataType::UserScriptMessage => todo!(),
        DataType::UserScriptGenericStub => todo!(),
        DataType::Object(_) => todo!(),
        DataType::Option(_) => todo!(),
        DataType::Result { .. } => todo!(),
    }
}
