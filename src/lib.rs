#![no_std]
#![doc = include_str!("../README.md")]

#[doc(hidden)]
pub mod secret {
    // Source: https://github.com/WorldSEnder/type-equalities-rs/tree/0a1aac50899ae966147ac6c917dbcc07da6a3626
    pub trait AliasSelf {
        /// Always set to `Self`, but the type checker doesn't reduce `T::Alias` to `T`.
        type Alias: ?Sized;
    }
    impl<T: ?Sized> AliasSelf for T {
        type Alias = T;
    }

    // Source: https://github.com/WorldSEnder/type-equalities-rs/tree/0a1aac50899ae966147ac6c917dbcc07da6a3626
    pub trait IsEqual<U: ?Sized>: AliasSelf<Alias = U> {}
    impl<T: ?Sized, U: ?Sized> IsEqual<U> for T where T: AliasSelf<Alias = U> {}

    // Source: https://stackoverflow.com/a/70978292/8341513
    // The function cannot be const at the time of writing because
    // destructor of `T` cannot be evaluated at compile-time and const drop is unstable
    pub fn ty_must_eq<T, U>(_: T)
    where
        T: IsEqual<U>,
    {
    }
}

/// This macro performs a compile-time check if a struct has a specific field.
///
/// ## Syntax
///
/// The macro offers three syntaxes for checking if a struct has a field
///
/// 1. `assert_has_field!(Struct, field);` - checks if the struct has a field with the given name.
/// 2. `assert_has_field!(Struct, field: Type);` - checks if the struct has a field with the given name and type.
/// 3. `assert_has_field!(Struct, field :~ Type);` - checks if the struct has a field with the given name and type that can be coerced to the specified type `Type`.
///
/// ## Examples
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
/// Note, however, that `:` syntax in this macro asserts the *exact* type of the field,
/// preventing any coercion to minimize the human error.
///
/// The following code will not compile:
///
/// ```rust,compile_fail
/// use assert_has_field::assert_has_field;
///
/// struct Wrapper<T>(T);
///
/// impl<T> core::ops::Deref for Wrapper<T> {
///   type Target = T;
///
///  fn deref(&self) -> &Self::Target {
///      &self.0
///  }
/// }
///
/// #[allow(dead_code)]
/// struct Point2 {
///    x: &'static Wrapper<u64>,
///   y: u64,
/// }
///
/// // This will cause a compile-time error because `Point2`'s field `x` is of type
/// // `&'static Wrapper<u64>`, not `&'static u64`.
/// assert_has_field!(Point2, x: &'static u64);
/// ```
///
/// Additionally, you can use the made-up `:~` syntax to assert that the field
/// can be coerced to the specified type.
///
/// ```rust
/// use assert_has_field::assert_has_field;
///
/// #[allow(dead_code)]
/// struct Wrapper<T>(T);
///
/// impl<T> core::ops::Deref for Wrapper<T> {
///    type Target = T;
///
///   fn deref(&self) -> &Self::Target {
///       &self.0
///   }
/// }
///
/// #[allow(dead_code)]
/// struct Point2 {
///     x: &'static Wrapper<u64>,
///     y: u64,
/// }
///
/// // This will compile because `Point2` has a field `x` that can be coerced to `&'static u64`.
/// assert_has_field!(Point2, x :~ &'static u64);
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
    (@ASSERT $unreachable_obj:ident: $struct:ty, $field:ident) => {
        // Here, it is only checked that the field exists.
        let _: _ = $unreachable_obj.$field;
    };
    (@ASSERT $unreachable_obj:ident: $struct:ty, $field:ident : $field_ty:ty) => {
        // We define a dummy function instead of calling the function directly
        // because the function call would be non-constant
        //
        // At the moment of writing, a non-constant function call falsly compiled but oh well
        fn dummy(v: $struct) {
            $crate::secret::ty_must_eq::<_, $field_ty>(
                // Here, the validation that the field exists is performed
                v.$field
            );
        }
    };
    (@ASSERT $unreachable_obj:ident: $struct:ty, $field:ident :~ $field_ty:ty) => {
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
                assert_has_field!(@ASSERT unreachable_obj: $struct, $field $($rest)*);
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

    struct Wrapper<T>(T);

    impl<T> core::ops::Deref for Wrapper<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    #[allow(dead_code)]
    struct Point2 {
        x: &'static Wrapper<u64>,
        y: u64,
    }

    assert_has_field!(Point2, x :~ &'static u64);
}
