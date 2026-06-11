use std::fmt::Write;

#[derive(Debug, Clone)]
pub struct FlatBufferField {
    pub name: String,
    pub field_type: String,
    pub id: u16,
}

pub struct FlatBufferTable {
    pub name: String,
    pub fields: Vec<FlatBufferField>,
}

pub fn generate_rust_struct(table: &FlatBufferTable) -> String {
    let mut out = String::new();
    writeln!(out, "#[derive(Debug, Clone)]").unwrap();
    writeln!(out, "pub struct {} {{", table.name).unwrap();
    for f in &table.fields {
        writeln!(out, "    pub {}: {},", f.name, f.field_type).unwrap();
    }
    writeln!(out, "}}").unwrap();
    out
}

pub fn vtable_size(fields: usize) -> usize {
    4 + 2 * (fields + 2) // header + offsets
}
