extern crate clap;

#[macro_export]
macro_rules! bin_prelude {
    ($(($($dec:tt)*))*) => {
        $(
            bin_prelude_decl!($($dec)*);
        )*
    }
}

#[macro_export]
macro_rules! bin_prelude_decl {
    (print $s:expr) => {
        println!($s);
    }
}
