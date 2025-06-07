pub mod lite_api_attr;
pub mod parse_domain_metadata;
pub mod resolve_classes;
pub mod load_path;
pub mod extract_pod_enum;
pub mod extract_engine_class;
pub mod extract_pod_struct;
pub mod extract_ty;
mod doc_attr;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RustSymbol(pub String);