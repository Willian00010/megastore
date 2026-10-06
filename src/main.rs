use megastore::graph::ConcurrentConectaStore;
use megastore::models::{NodeType, Product};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = ConcurrentConectaStore::new();

    let p1 = store.add_product(Product { id: 101, name: "Laptop Gaming".into(), category_id: 1, price: 7500.0 })?;
    let p2 = store.add_product(Product { id: 102, name: "Rato Vertical Ergonómico".into(), category_id: 1, price: 250.0 })?;
    let p3 = store.add_product(Product { id: 103, name: "Teclado Mecânico RGB".into(), category_id: 1, price: 450.0 })?;

    let c1 = store.add_node("cli_1001".into(), NodeType::Client);
    let c2 = store.add_node("cli_1002".into(), NodeType::Client);

    store.add_edge(c1, p1, 5.0)?;
    store.add_edge(c2, p1, 4.5)?;
    store.add_edge(c2, p2, 5.0)?;
    store.add_edge(c2, p3, 4.0)?;

    let start = Instant::now();
    let recommendations = store.recommend_for_client("cli_1001", 5)?;
    let elapsed = start.elapsed();

    println!("=== ConectaStore Engine | Módulo de Recomendação ===");
    println!("Recomendações geradas para 'cli_1001':");
    for (product, score) in recommendations {
        println!("  - ID: {} | Item: {} | Score Relevância: {:.2}", product.id, product.name, score);
    }
    println!("\n[Benchmark] Tempo de execução: {:.3?} ({} µs)", elapsed, elapsed.as_micros());

    Ok(())
}