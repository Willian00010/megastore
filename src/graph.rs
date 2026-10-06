use crate::models::{Edge, NodeIndex, NodeType, Product, StoreError};
use parking_lot::RwLock;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;

pub struct GraphCore {
    node_map: HashMap<String, NodeIndex>,
    node_types: Vec<NodeType>,
    products: HashMap<NodeIndex, Product>,
    adjacency_list: Vec<Vec<Edge>>,
}

#[derive(Clone)]
pub struct ConcurrentConectaStore {
    inner: Arc<RwLock<GraphCore>>,
}

impl ConcurrentConectaStore {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(GraphCore {
                node_map: HashMap::new(),
                node_types: Vec::new(),
                products: HashMap::new(),
                adjacency_list: Vec::new(),
            })),
        }
    }

    pub fn add_node(&self, external_key: String, node_type: NodeType) -> NodeIndex {
        let mut core = self.inner.write();
        if let Some(&idx) = core.node_map.get(&external_key) {
            return idx;
        }

        let idx = core.node_types.len();
        core.node_map.insert(external_key, idx);
        core.node_types.push(node_type);
        core.adjacency_list.push(Vec::new());
        idx
    }

    pub fn add_product(&self, product: Product) -> Result<NodeIndex, StoreError> {
        let key = format!("prod_{}", product.id);
        let idx = self.add_node(key, NodeType::Product);
        
        let mut core = self.inner.write();
        core.products.insert(idx, product);
        Ok(idx)
    }

    pub fn add_edge(&self, source: NodeIndex, target: NodeIndex, weight: f32) -> Result<(), StoreError> {
        let mut core = self.inner.write();
        let len = core.node_types.len();
        if source >= len || target >= len {
            return Err(StoreError::NodeNotFound("Índice de nó inválido ao criar aresta".into()));
        }

        core.adjacency_list[source].push(Edge { target, weight });
        core.adjacency_list[target].push(Edge { target: source, weight });
        Ok(())
    }

    pub fn recommend_for_client(&self, client_key: &str, limit: usize) -> Result<Vec<(Product, f32)>, StoreError> {
        let core = self.inner.read();
        
        let start_node = *core.node_map.get(client_key)
            .ok_or_else(|| StoreError::NodeNotFound(client_key.to_string()))?;

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut scores: HashMap<NodeIndex, f32> = HashMap::new();
        let mut purchased_products = HashSet::new();

        for edge in &core.adjacency_list[start_node] {
            if core.node_types[edge.target] == NodeType::Product {
                purchased_products.insert(edge.target);
            }
        }

        visited.insert(start_node);
        queue.push_back((start_node, 0, 1.0f32));

        while let Some((current_node, depth, current_weight)) = queue.pop_front() {
            if depth >= 3 {
                continue;
            }

            for edge in &core.adjacency_list[current_node] {
                if !visited.contains(&edge.target) {
                    let next_weight = current_weight * edge.weight;
                    
                    if core.node_types[edge.target] == NodeType::Product && !purchased_products.contains(&edge.target) {
                        *scores.entry(edge.target).or_insert(0.0) += next_weight;
                    }

                    visited.insert(edge.target);
                    queue.push_back((edge.target, depth + 1, next_weight));
                }
            }
        }

        let mut ranked_products: Vec<(Product, f32)> = scores
            .into_iter()
            .filter_map(|(idx, score)| {
                core.products.get(&idx).map(|prod| (prod.clone(), score))
            })
            .collect();

        ranked_products.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        ranked_products.truncate(limit);

        Ok(ranked_products)
    }
}