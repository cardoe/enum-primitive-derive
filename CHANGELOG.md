# ChangeLog

## Unreleased

### Changed

- Upgrade [syn](https://crates.io/crates/syn) and [quote](https://crates.io/crates/quote) to 1.0
- add a better diagnostic for the case where a discriminant isn't specified for
	an enum
- Move unnecessary [`num-traits`](https://crates.io/crates/num-traits) dependency to `dev-dependencies`

## 0.1.2

### Changed

- drop `extern crate core;` as core is unused

## 0.1.1

### Added

- Support for more casts on discriminants

## 0.1.0

Initial version
