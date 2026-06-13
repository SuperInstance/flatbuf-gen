# FlatBuffer Code Generator

**A Rust library for generating Rust struct definitions and vtable layouts from FlatBuffer-style schema descriptions**, providing a lightweight alternative to the official `flatc` compiler for code generation pipelines.

## Why It Matters

FlatBuffers is Google's zero-copy serialization format used in games (Facebook's own mobile stack), embedded systems, and high-performance RPC frameworks. Unlike Protocol Buffers, FlatBuffers don't need a parsing step — data is accessed directly from the serialized buffer using vtable offsets. Generating the Rust bindings automatically from schema definitions eliminates manual maintenance of struct layouts, reduces bugs from offset misalignment, and enables build-time code generation in `build.rs` scripts.

## How It Works

The library models a FlatBuffer schema as `FlatBufferTable` structs containing `FlatBufferField` entries, each with a name, type, and 16-bit field ID. The field ID corresponds to the vtable slot index — FlatBuffer vtables store offsets as 16-bit integers at position `4 + 2 * (field_id + 2)` from the vtable start, where the first two slots are reserved for the vtable size and table length.

The `generate_rust_struct` function emits a `#[derive(Debug, Clone)]` struct definition with public fields, ready for inclusion in generated code. The `vtable_size` function computes the vtable byte size as `4 + 2 * (fields + 2)`, accounting for the 4-byte header (vtable size + table size) plus two bytes per slot (one per field, plus the two reserved slots).

## Quick Start

```rust
use flatbuf_gen::{FlatBufferTable, FlatBufferField, generate_rust_struct, vtable_size};

fn main() {
    let table = FlatBufferTable {
        name: "Monster".to_string(),
        fields: vec![
            FlatBufferField { name: "hp".to_string(), field_type: "i16".to_string(), id: 0 },
            FlatBufferField { name: "mana".to_string(), field_type: "i16".to_string(), id: 1 },
            FlatBufferField { name: "name".to_string(), field_type: "&str".to_string(), id: 3 },
        ],
    };

    let code = generate_rust_struct(&table);
    println!("{}", code);
    // #[derive(Debug, Clone)]
    // pub struct Monster {
    //     pub hp: i16,
    //     pub mana: i16,
    //     pub name: &str,
    // }

    println!("Vtable size: {} bytes", vtable_size(3)); // 12
}
```

## API

| Type / Function | Description |
|---|---|
| `FlatBufferField` | Schema field with name, type, and vtable slot ID |
| `FlatBufferTable` | Schema table containing named fields |
| `generate_rust_struct(table)` | Generate a Rust struct definition from a table schema — **O(n)** |
| `vtable_size(fields)` | Compute vtable byte size: `4 + 2 * (fields + 2)` |

## Architecture Notes

Part of the SuperInstance serialization toolkit. This code generator integrates with the build pipeline to produce Rust bindings from `.fbs` schema files. See the [Architecture Guide](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## License

MIT
