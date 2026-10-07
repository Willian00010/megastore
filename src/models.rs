#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Vertex {
    Customer(u64),
    Product(u64),
    Category(String),
}

#[derive(Debug, Clone)]
pub struct Product {
    pub id: u64,
    pub name: String,
    pub category: String,
    pub price: f64,
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub target: Vertex,
    pub weight: f64,
}