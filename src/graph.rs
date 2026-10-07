use std::collections::{HashMap, HashSet, VecDeque};
use crate::models::{Edge, Product, Vertex};

pub struct RecommendationGraph {
    adj_list: HashMap<Vertex, Vec<Edge>>,
    products: HashMap<u64, Product>,
}

impl RecommendationGraph {
    pub fn new() -> Self {
        Self {
            adj_list: HashMap::new(),
            products: HashMap::new(),
        }
    }

    pub fn add_product(&mut self, product: Product) {
        let p_vertex = Vertex::Product(product.id);
        let c_vertex = Vertex::Category(product.category.clone());

        self.products.insert(product.id, product);

        // Conecta produto à sua categoria com peso 2.0 (relação fraca/estrutural)
        self.add_edge(p_vertex, c_vertex, 2.0);
    }

    pub fn get_product(&self, id: u64) -> Option<&Product> {
        self.products.get(&id)
    }

    pub fn add_edge(&mut self, from: Vertex, to: Vertex, weight: f64) {
        self.adj_list
            .entry(from.clone())
            .or_default()
            .push(Edge { target: to.clone(), weight });

        // Grafo não-direcionado para permitir navegação em via dupla
        self.adj_list
            .entry(to)
            .or_default()
            .push(Edge { target: from, weight });
    }

    /// Algoritmo de Busca em Largura (BFS) com cálculo de relevância ponderada
    pub fn recommend_for_customer(&self, customer_id: u64, limit: usize) -> Vec<(Product, f64)> {
        let start_vertex = Vertex::Customer(customer_id);
        let mut visited = HashSet::new();
        let mut scores: HashMap<u64, f64> = HashMap::new();
        let mut queue = VecDeque::new();

        // HashSet para identificar produtos que o cliente JÁ comprou/interagiu diretamente
        let mut direct_interactions = HashSet::new();
        if let Some(edges) = self.adj_list.get(&start_vertex) {
            for edge in edges {
                if let Vertex::Product(pid) = edge.target {
                    direct_interactions.insert(pid);
                }
            }
        }

        visited.insert(start_vertex.clone());
        queue.push_back((start_vertex, 0, 1.0)); // (Vértice, Profundidade, Peso Acumulado)

        while let Some((curr, depth, weight_acc)) = queue.pop_front() {
            if depth >= 3 {
                continue; // Limita a profundidade a 3 saltos para evitar dispersão
            }

            if let Some(neighbors) = self.adj_list.get(&curr) {
                for edge in neighbors {
                    if !visited.contains(&edge.target) {
                        let new_weight = weight_acc * edge.weight;

                        if let Vertex::Product(pid) = edge.target {
                            // Prevenção de recomendação duplicada de itens já adquiridos pelo cliente
                            if !direct_interactions.contains(&pid) {
                                *scores.entry(pid).or_insert(0.0) += new_weight;
                            }
                        }

                        if depth + 1 < 3 {
                            visited.insert(edge.target.clone());
                            queue.push_back((edge.target.clone(), depth + 1, new_weight));
                        }
                    }
                }
            }
        }

        // Ordenação das recomendações por relevância
        let mut result: Vec<(Product, f64)> = scores
            .into_iter()
            .filter_map(|(pid, score)| self.products.get(&pid).cloned().map(|p| (p, score)))
            .collect();

        result.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        result.truncate(limit);
        result
    }
}