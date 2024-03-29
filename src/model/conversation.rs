use leptos::server_fn::serde;
use serde::{Deserialize, Serialize};         // To serialize and deserialize

#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct Conversation{
    pub messages:Vec<Message>
}

impl Conversation{
    pub fn new() -> Conversation{
        Conversation{
            messages: Vec::new()
        }
    }
}

#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct Message{
    pub user:bool,
    pub text:String
}