pub struct Connection {
    pub target: String,
    _private_key: String,
}

fn init_connection(_target: &String) -> String {
    // Connect to target, get secret key
    "000000".to_string()
}
impl Connection {
    pub fn new(target: String) -> Self {
        Self {
            _private_key: init_connection(&target),
            target: target,
        }
    }
    pub fn graceful_shutdown(&self) {
        println!("Shutted down conncection gracefully.");
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        self.graceful_shutdown();
    }
}

fn main() {
    let _my_connect = Connection::new("127.0.0.1".to_string());
}
