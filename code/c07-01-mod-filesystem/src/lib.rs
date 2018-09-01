pub mod client; // client.rs
pub mod network; // network.rs

#[cfg(test)]
mod tests {
    use super::client;
    #[test]
    fn it_works() {
        client::connect();
    }
}
