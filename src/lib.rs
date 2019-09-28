mod model;
pub use model::*;

#[cfg(feature = "mysql")]
pub mod sql;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
