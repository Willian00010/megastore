use std::time::Instant;
use crate::graph::RecommendationGraph;
use crate::models::{Product, Vertex};

pub fn run_benchmarks() {
    println!("\n=== MEDIÇÃO DE DESEMPENHO E ESCALABILIDADE ===");
    let tamanhos = vec![1_000, 10_000, 50_000];

    for &n in &tamanhos {
        let mut graph = RecommendationGraph::new();

        // 1. Inserção de Produtos
        let start_insert = Instant::now();
        for i in 0..n {
            graph.add_product(Product {
                id: i,
                name: format!("Produto {}", i),
                category: format!("Categoria {}", i % 10),
                price: 10.0 + (i as f64),
            });
        }
        let duration_insert = start_insert.elapsed();

        // 2. Conexão de Clientes e Compras
        let start_edges = Instant::now();
        for c in 0..(n / 10) {
            let customer_v = Vertex::Customer(c);
            let product_v = Vertex::Product(c % n);
            graph.add_edge(customer_v, product_v, 5.0); // Conexão Compra
        }
        let duration_edges = start_edges.elapsed();

        // 3. Execução da Recomendação (BFS)
        let start_rec = Instant::now();
        let _recs = graph.recommend_for_customer(0, 5);
        let duration_rec = start_rec.elapsed();

        println!(
            "N = {:6} | Carga: {:8.2?} | Conexões: {:8.2?} | Consulta BFS: {:8.2?}",
            n, duration_insert, duration_edges, duration_rec
        );
    }
    println!("===============================================\n");
}