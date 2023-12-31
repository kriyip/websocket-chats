use std::collections::{HashMap, HashSet};
use actix::prelude::*;
use uuid::Uuid;

struct ChatRoom {
    id: usize,
    name: String,
}

pub struct ChatServer {
    user_sessions: HashMap<usize, Recipient<Message>>,
    chat_rooms: HashMap<String, usize>,
}

impl ChatServer {
    pub fn new() -> Self {
        Self {
            user_sessions: HashMap::new(),
            chat_rooms: HashMap::new(),
        }
    }

    pub fn send_message(&self, chatroom: String, message: &str, sender_id: usize) {
        // iterate through 
    }
}