# `assert_has_field` - a Rust macro for checking if a struct has a specific field

[![Crates.io](https://img.shields.io/crates/v/assert_has_field)](https://crates.io/crates/assert_has_field)
[![Downloads](https://img.shields.io/crates/d/assert_has_field.svg)](https://crates.io/crates/assert_has_field)
[![Documentation](https://docs.rs/assert_has_field/badge.svg)](https://docs.rs/assert_has_field)
[![License](https://img.shields.io/crates/l/assert_has_field)](https://crates.io/crates/assert_has_field)
[![Dependency Status](https://deps.rs/repo/github/JohnScience/assert_has_field/status.svg)](https://deps.rs/repo/github/JohnScience/assert_has_field)

This macro is designed to be used in Rust code to assert that a struct has a specific field and, if necessary, that this field of specific type.

## Usage

The macro offers three syntaxes for checking if a struct has a field

1. `assert_has_field!(Struct, field);` - checks if the struct has a field with the given name.
2. `assert_has_field!(Struct, field: Type);` - checks if the struct has a field with the given name and type.
3. `assert_has_field!(Struct, field :~ Type);` - checks if the struct has a field with the given name and type that can be coerced to the specified type `Type`.

### Checking that a struct has a field

```rust
use assert_has_field::assert_has_field;

#[allow(dead_code)]
struct MyStruct {
    field1: i32,
    field2: String,
}

assert_has_field!(MyStruct, field1); // This will compile
```

### Checking that a struct has a field of a specific type

```rust
use assert_has_field::assert_has_field;

#[allow(dead_code)]
struct MyStruct {
    field1: i32,
    field2: String,
}

assert_has_field!(MyStruct, field1: i32); // This will compile
```

### Checking that a struct has a field of a specific type (failure case for a totally different type)

```rust,compile_fail
use assert_has_field::assert_has_field;

#[allow(dead_code)]
struct MyStruct {
    field1: i32,
    field2: String,
}

assert_has_field!(MyStruct, field1: String); // This will fail to compile
```

### Checking that a struct has a field of a specific type (failure case for a type that can be coerced to)

```rust,compile_fail
struct Wrapper<T>(T);

impl core::ops::Deref for Wrapper<i32> {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[allow(dead_code)]
struct MyStruct {
    field1: &'static Wrapper<i32>,
    field2: String,
}

assert_has_field!(MyStruct, field1: &'static i32); // This will fail to compile
```

### Checking that a struct has a field of a type that can be coerced to another type

```rust
use assert_has_field::assert_has_field;

struct Wrapper<T>(T);

impl core::ops::Deref for Wrapper<i32> {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[allow(dead_code)]
struct MyStruct {
    field1: &'static Wrapper<i32>,
    field2: String,
}

assert_has_field!(MyStruct, field1 :~ &'static i32);
```

### Checking that a struct has a field of a type that can be coerced to another type (failure case)

```rust,compile_fail
use assert_has_field::assert_has_field;

struct Point {
    x: i32,
    y: i32,
}

assert_has_field!(Point, x :~ String); // This will not compile
```

## How it works

```rust
#[macro_export]
macro_rules! assert_has_field {
    (@ASSERT $unreachable_obj:ident, $field:ident) => {
        // Here, it is only checked that the field exists.
        let _: _ = $unreachable_obj.$field;
    };
    (@ASSERT $unreachable_obj:ident, $field:ident : $field_ty:ty) => {
        // Here, the value on the right hand side must be the same type as the type on the left hand side
        // and the field must exist.
        let _ : $field_ty = type_equalities::coerce($unreachable_obj.$field, type_equalities::refl());
    };
    (@ASSERT $unreachable_obj:ident, $field:ident :~ $field_ty:ty) => {
        // Here, the value on the right hand side can be coerced to the type on the left hand side
        // and the field must exist.
        let _ : $field_ty = $unreachable_obj.$field;
    };
    (
        $struct:ty,
        $field:ident
            $($rest:tt)*
    ) => {
        // The const block forces the const evaluation.
        #[allow(
            unreachable_code,
            unused_variables,
            clippy::diverging_sub_expression,
        )]
        const _: () = {
            // `if false { ... }` ensures that the unreacahble! macro invokation is indeed unreachable.
            if false {
                // Rust performs the type-checking at compile time even if the code is unreachable.
                //
                // The return type of core::unreachable!() is never type,
                // which can be assigned to any type.
                let unreachable_obj: $struct = core::unreachable!();
                assert_has_field!(@ASSERT unreachable_obj, $field $($rest)*);
            }
        };
    };
}
```

## On the real use-cases of this macro

Let's say that you're writing a backend server and have a DTO, which is meant
to be used on the frontend. Assume that this DTO aggregates different kinds of
data that pertains to a candidate. You may be in a situation where `candidate_id`
is stored in one of the fields-structures. You can use [`assert_has_field`] to
document that expectation and future-proof the type in case the field-structure
that used to store `candidate_id` is removed entirely or modified in a way that
moves or removes the `candidate_id`.
