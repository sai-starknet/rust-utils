# sai-conversion

Ergonomic conversion helpers for Option, Result, Vec, HashMap, HashSet, and iterators.

Provides extension traits that make it easy to convert inner types using `Into` without verbose manual mapping.

## Usage

```rust
use sai_conversion::{OptionInto, ResultInto, VecInto, HashMapInto};

// Option
let o: Option<u8> = Some(5);
let wide: Option<u16> = o.opt_into();

// Result
let r: Result<u8, &str> = Ok(1);
let wide: Result<u16, &str> = r.ok_into();

// Vec
let v: Vec<u8> = vec![1, 2, 3];
let wide: Vec<u16> = v.vec_into();

// HashMap
use std::collections::HashMap;
let mut map = HashMap::new();
map.insert(1u8, 10u8);
let wide: HashMap<u16, u16> = map.entries_into();
```

## Feature flags

| Feature | Default | Implies | Description |
|---------|---------|---------|-------------|
| `hash` | **yes** | `std` | All hash-collection traits (`HashMapInto`, `HashSetInto`, `IntoHashMap`) |
| `std` | yes (via `hash`) | `alloc` | Hash-collection conversion (requires `std`) |
| `alloc` | yes (via `std`) | — | `VecInto` and `ItemsInto` (requires the `alloc` crate) |

The core traits `OptionInto` and `ResultInto` are always available (no feature flags required, `no_std` compatible).

## License

MIT
