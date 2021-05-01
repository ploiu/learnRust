// modules can be declared as exports this way, and exist in a file named moduleName.rs
pub mod client;
pub mod network;

#[cfg(test)]
mod tests {
    use super::client;
    #[test]
    fn it_works() {
        client::connect();
    }
}
