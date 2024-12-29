/// Add a module with the provided name which contains the build information generated by `shadow-rs`.
///
/// # Example
///
/// ```ignore
/// use shadow_rs::shadow;
///
/// shadow!(my_build_information);
///
/// fn main() {
///     println!("I'm version {}!", my_build_information::VERSION);
/// }
/// ```
///
/// The convention, however, is to use `shadow!(build);`.
#[macro_export]
macro_rules! shadow {
    ($build_mod:ident) => {
        #[doc = r#"shadow-rs mod"#]
        pub mod $build_mod {
            include!(concat!(env!("OUT_DIR"), "/shadow.rs"));
        }
    };
}

pub use const_format::concatcp;