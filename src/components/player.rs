use crate::core::types::Class;

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub class: Class,
    pub is_online: bool,
    pub connection_quality: f32,
}

impl Player {
    pub fn new(name: &str, class: Class) -> Self {
        Self {
            name: name.to_string(),
            class,
            is_online: true,
            connection_quality: 1.0,
        }
    }
}