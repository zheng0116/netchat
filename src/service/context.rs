use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::RwLock;
const MAX_CONTEXT_LENGTH: usize = 10;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone)]
pub struct ChatContext {
    history: VecDeque<Message>,
}

impl ChatContext {
    pub fn new() -> Self {
        Self {
            history: VecDeque::with_capacity(MAX_CONTEXT_LENGTH),
        }
    }

    pub fn add_message(&mut self, role: &str, content: &str) {
        if self.history.len() >= MAX_CONTEXT_LENGTH {
            self.history.pop_front();
        }
        self.history.push_back(Message {
            role: role.to_string(),
            content: content.to_string(),
            timestamp: chrono::Utc::now().timestamp(),
        });
    }

    pub fn get_history(&self) -> Vec<Message> {
        self.history.iter().cloned().collect()
    }

    pub fn clear(&mut self) {
        self.history.clear();
    }
}

#[derive(Clone)]
pub struct ChatManager {
    contexts: Arc<RwLock<HashMap<String, ChatContext>>>,
}

impl ChatManager {
    pub fn new() -> Self {
        Self {
            contexts: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get_or_create_context(&self, conversation_id: &str) -> ChatContext {
        let contexts = self.contexts.read().await;
        match contexts.get(conversation_id) {
            Some(context) => context.clone(),
            None => {
                drop(contexts);
                let mut contexts = self.contexts.write().await;
                let context = ChatContext::new();
                contexts.insert(conversation_id.to_string(), context.clone());
                context
            }
        }
    }

    pub async fn update_context(&self, conversation_id: &str, role: &str, content: &str) {
        let mut contexts = self.contexts.write().await;
        if let Some(context) = contexts.get_mut(conversation_id) {
            context.add_message(role, content);
        }
    }

    pub async fn clear_context(&self, conversation_id: &str) {
        let mut contexts = self.contexts.write().await;
        if let Some(context) = contexts.get_mut(conversation_id) {
            context.clear();
        }
    }

    pub async fn get_conversation_history(&self, conversation_id: &str) -> Option<Vec<Message>> {
        let contexts = self.contexts.read().await;
        contexts.get(conversation_id).map(|ctx| ctx.get_history())
    }
}
