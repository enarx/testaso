![lint](https://github.com/enarx/enarx/workflows/lint/badge.svg)
![enarxbot](https://github.com/enarx/enarx/workflows/enarxbot/badge.svg)
[![Workflow Status](https://github.com/enarx/testaso/workflows/test/badge.svg)](https://github.com/enarx/testaso/actions?query=workflow%3A%22test%22)
[![Average time to resolve an issue](https://isitmaintained.com/badge/resolution/enarx/testaso.svg)](https://isitmaintained.com/project/enarx/testaso "Average time to resolve an issue")
[![Percentage of issues still open](https://isitmaintained.com/badge/open/enarx/testaso.svg)](https://isitmaintained.com/project/enarx/testaso "Percentage of issues still open")
![Maintenance](https://img.shields.io/badge/maintenance-activly--developed-brightgreen.svg)

# testaso

Macro to test alignment, size and offsets of structs

This is mostly useful for creating FFI structures.

The crucial field offset calculation was extracted from the `memoffset` crate.
Kudos to Gilad Naaman and Ralf Jung and all the other contributors.

## Examples
```rust
#[repr(C)]
struct Simple {
    a: u32,
    b: [u8; 2],
    c: i64,
}

#[repr(C, packed)]
struct SimplePacked {
    a: u32,
    b: [u8; 2],
    c: i64,
}
#[cfg(test)]
mod test {
    use testaso::testaso;

    use super::Simple;
    use super::SimplePacked;

    testaso! {
        struct Simple: 8, 16 => {
            a: 0,
            b: 4,
            c: 8
        }

        struct SimplePacked: 1, 14 => {
            a: 0,
            b: 4,
            c: 6
        }
    }
}
```

License: MIT
