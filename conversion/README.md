# sai-conversion

Ergonomic conversion helpers for Option, Result, Vec, HashMap, HashSet, and iterators.

Provides extension traits that make it easy to convert inner types using `Into` without verbose manual mapping.

## Usage

### Wrappers

```rust
use sai_conversion::{OptionInto, ResultInto};

// Option — convert the Some variant
let some: Option<u8> = Some(5);
let wide: Option<u16> = some.opt_into();

// Result — convert both Ok and Err
let r: Result<u8, u8> = Ok(1);
let wide: Result<u16, u16> = r.result_into();

// Result — convert only Ok
let r: Result<u8, &str> = Ok(5);
let wide: Result<u16, &str> = r.ok_into();

// Result — convert only Err
let r: Result<&str, u8> = Err(3);
let wide: Result<&str, u16> = r.err_into();
```

### Alloc

```rust
use sai_conversion::{VecInto, ItemsInto};

// Vec — element-wise conversion
let v: Vec<u8> = vec![1, 2, 3];
let wide: Vec<u16> = v.vec_into();

// ItemsInto — general iterator conversion into any collection
use std::collections::BTreeSet;
let v: Vec<u8> = vec![3, 1, 2, 1];
let set: BTreeSet<u16> = v.items_into();
```

### Hash collections

```rust
use sai_conversion::{HashMapInto, IntoHashMap, HashSetInto};
use std::collections::{HashMap, HashSet};

let mut map = HashMap::new();
map.insert(1u8, 10u8);

// Convert keys only
let wide_keys: HashMap<u16, u8> = map.clone().keys_into();

// Convert values only
let wide_vals: HashMap<u8, u16> = map.clone().values_into();

// Convert both keys and values
let wide: HashMap<u16, u16> = map.entries_into();

// Build a HashMap from an iterator of tuples
let pairs = vec![(1u8, "a"), (2u8, "b")];
let map: HashMap<u8, &str> = pairs.into_hashmap();

// HashSet — convert elements
let mut set = HashSet::new();
set.insert(1u8);
set.insert(2u8);
let wide: HashSet<u16> = set.set_items_into();
```

## Feature flags

| Feature | Default          | Implies | Description                                                              |
| ------- | ---------------- | ------- | ------------------------------------------------------------------------ |
| `hash`  | **yes**          | `std`   | All hash-collection traits (`HashMapInto`, `HashSetInto`, `IntoHashMap`) |
| `std`   | yes (via `hash`) | `alloc` | Hash-collection conversion (requires `std`)                              |
| `alloc` | yes (via `std`)  | —       | `VecInto` and `ItemsInto` (requires the `alloc` crate)                   |

The core traits `OptionInto` and `ResultInto` are always available (no feature flags required, `no_std` compatible).

## License

MIT
