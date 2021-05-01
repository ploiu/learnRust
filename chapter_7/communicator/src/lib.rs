#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// modules can be declared as exports this way, and exist in a file named moduleName.rs
mod client;
pub mod network;
