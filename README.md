# enum-primitive-derive

This is a custom derive, using procedural macros, implementation of
[enum_primitive](https://crates.io/crates/enum_primitive).

## Documentation

https:/docs.rs/enum-primitive-derive/

## Usage

Add the following to `Cargo.toml`:

```
[dependencies]
enum-primitive-derive = "^0.1"
num-traits = "^0.1"
```

Then to your code add:

```rust
#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

#[derive(Primitive)]
enum Variant {
    Value = 1,
    Another = 2,
}
```

To be really useful you need `use num_traits::FromPrimitive` or
`use num_traits::ToPrimitive` or both. You will then be able to
use
[num_traits::FromPrimitive](https://rust-num.github.io/num/num/trait.FromPrimitive.html)
and/or
[num_traits::ToPrimitive](https://rust-num.github.io/num/num/trait.ToPrimitive.html)
on your enum.

## Full Example

```rust
#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Primitive)]
enum Foo {
    Bar = 32,
    Dead = 42,
    Beef = 50,
}

fn main() {
    assert_eq!(Foo::from_i32(32), Some(Foo::Bar));
    assert_eq!(Foo::from_i32(42), Some(Foo::Dead));
    assert_eq!(Foo::from_i64(50), Some(Foo::Beef));
    assert_eq!(Foo::from_isize(17), None);

    let bar = Foo::Bar;
    assert_eq!(bar.to_i32(), Some(32));

    let dead = Foo::Dead;
    assert_eq!(dead.to_isize(), Some(42));
}
```

# Complex Example

In this case we attempt to use values created by
[bindgen](https://crates.io/crates/bindgen).

```rust
#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

use num_traits::{FromPrimitive, ToPrimitive};

pub const ABC: ::std::os::raw::c_uint = 1;
pub const DEF: ::std::os::raw::c_uint = 2;
pub const GHI: ::std::os::raw::c_uint = 4;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Primitive)]
enum BindGenLike {
    ABC = ABC as isize,
    DEF = DEF as isize,
    GHI = GHI as isize,
}

fn main() {
    assert_eq!(BindGenLike::from_isize(4), Some(BindGenLike::GHI));
    assert_eq!(BindGenLike::from_u32(2), Some(BindGenLike::DEF));
    assert_eq!(BindGenLike::from_u32(8), None);

    let abc = BindGenLike::ABC;
    assert_eq!(abc.to_u32(), Some(1));
}
```
