pub struct Message {
    pub content: String,
    pub user: String
}

impl Message {
  pub fn new(ms: String, u: String) -> Message {
    Message{
      content:ms,
      user:u
    }
  }
  pub fn send_ms(&self) -> Option<&str> {
    if self.content.is_empty() || self.content.contains("stupid") {
      None
    } else {
      Some(&self.content)
    }
  }
}

pub fn check_ms(ms: &Message) -> (bool, &str) {
    match ms.send_ms() {
        Some(ms) => (true, ms),
        None => (false, "ERROR: illegal")
    }
}

