/// Structs
///
/// A simple macro to wrap your structs and their content
///
/// Example:
/// ```rs
/// structs! {
///     User { name: String }
/// }
/// ```
#[macro_export]
macro_rules! structs {
    ($($name:ident {$($arg:ident: $type:ty),*})*) => {
        $(
            pub struct $name {
                $($arg: $type,)*
            }
        )*
    };
}

/// Functions
///
/// A simple macro to wrap your functions and their body
///
/// Example:
/// ```rs
/// functions! {
///     create_user(name: String) -> User { User { name } }
/// }
/// ```
#[macro_export]
macro_rules! functions {
    ($($name:ident($($arg:ident: $type:ty),*) -> $rtype:ty { $($code:tt)* })*) => {
        $(
            pub fn $name($($arg: $type),*) -> $rtype {
                { $($code)* }
            }
        )*
    };
}