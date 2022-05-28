use puffin_http::{Client, Server};

use crate::*;

pub struct PuffinHttpContext {
    pub server: Server,
    _client: Client,
}

impl PuffinHttpContext {
    pub fn new() -> Result<Self, anyhow::Error> {
        let server_result = Server::new("0.0.0.0:8585");
        if let Ok(server) = server_result {
            let client = Client::new("127.0.0.1:8585".to_owned());
            Ok(Self {
                server,
                _client: client,
            })
        } else {
            Err(server_result.err().unwrap())
        }
    }
}

impl Subcontext for PuffinHttpContext {}
