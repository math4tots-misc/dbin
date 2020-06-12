mod context;
mod spec;

pub use context::ContextValue;
pub use context::ReadContext;
pub use spec::DataSpec;
pub use spec::UInt;
pub use spec::SInt;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
