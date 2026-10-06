use std::fmt;

pub type NodeIndex = usize;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NodeType {
    Client,
    Product,
    Category,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub category_id: u32,
    pub price: f64,
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub target: NodeIndex,
    pub weight: f32,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum StoreError {
    NodeNotFound(String),
    InvalidOperation(String),
}

impl std::error::Error for StoreError {}

impl fmt::Display for StoreError {
    // Corrigido: adicionado 'fn' antes de fmt
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StoreError::NodeNotFound(msg) => write!(f, "Nó não encontrado: {}", msg),
            StoreError::InvalidOperation(msg) => write!(f, "Operação inválida: {}", msg),
        }
    }
}