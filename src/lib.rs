#![no_std]
#![doc = include_str!("../README.md")]

/// This macro performs a compile-time check if a struct has a specific field.
///
/// ```rust
/// use assert_has_field::assert_has_field;
///
/// #[allow(dead_code)]
/// struct Point {
///     x: u64,
///     y: u64,
/// }
///
/// // This will compile because `Point` has a field `x`.
/// assert_has_field!(Point, x);
/// ```
///
/// If the field is not present, the macro will cause a compile-time error.
///
/// ```rust,compile_fail
/// use assert_has_field::assert_has_field;
///
/// #[allow(dead_code)]
/// struct Point {
///    x: u64,
///    y: u64,
/// }
///
/// // This will cause a compile-time error because `Point` does not have a field `a`.
/// assert_has_field!(Point, a);
/// ```
///
/// You can also specify the type of the field to ensure it matches a specific type.
///
/// ```rust
/// use assert_has_field::assert_has_field;
///
/// #[allow(dead_code)]
/// struct Point {
///    x: u64,
///    y: u64,
/// }
///
/// // This will compile because `Point` has a field `x` of type `u64`.
/// assert_has_field!(Point, x: u64);
/// ```
///
/// ## On real use-cases
///
/// Let's say that you're writing a backend server and have a DTO, which is meant
/// to be used on the frontend. Assume that this DTO aggregates different kinds of
/// data that pertains to a candidate. You may be in a situation where `candidate_id`
/// is stored in one of the fields-structures. You can use [`assert_has_field`] to
/// document that expectation and future-proof the type in case the field-structure
/// that used to store `candidate_id` is removed entirely or modified in a way that
/// moves or removes the `candidate_id`.
#[macro_export]
macro_rules! assert_has_field {
    ($struct:ty, $field:ident $(: $field_ty:ty)?) => {
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
                let _ $(: $field_ty)? = unreachable_obj.$field;
            }
        };
    };
}

#[cfg(test)]
mod tests {
    use super::assert_has_field;

    #[allow(dead_code)]
    struct Point {
        x: u64,
        y: u64,
    }

    assert_has_field!(Point, x);
    assert_has_field!(Point, x : u64);
}
