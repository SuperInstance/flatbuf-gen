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

/// FNV-1a 64 — the digest every substrate in the SuperInstance fleet agrees on.
pub const FNV_OFFSET: u64 = 0xcbf29ce484222325;
pub const FNV_PRIME: u64 = 0x100000001b3;

#[inline]
pub fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h = FNV_OFFSET;
    for &b in bytes {
        h = (h ^ b as u64).wrapping_mul(FNV_PRIME);
    }
    h
}

/// True if this crate's FNV-1a still agrees with the rest of the fleet.
pub fn canary_holds() -> bool {
    fnv1a64("café Δ 日本語".as_bytes()) == 0x024a555471370b18d
}
