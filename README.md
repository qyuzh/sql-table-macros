# sql-table-macros

Helper macros to generate SQL table definitions

## Example

```rust
#![allow(dead_code)]
use sql_table_macros::table_common_fields;

type Id = u64;

#[table_common_fields(Id, std::time::Duration)]
#[derive(Default, Debug)]
struct User {
    id: String,
    name: String,
}

fn main() {
    let user = User::default();
    assert_eq!(
        format!("{user:?}"),
        r#"User { id: "", name: "", created_by: 0, updated_by: 0, created_at: 0ns, updated_at: 0ns, is_deleted: false }"#
    );
}
```
