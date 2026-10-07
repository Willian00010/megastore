use megastore::benchmark::run_benchmarks;
use megastore::graph::RecommendationGraph;
use megastore::models::{Product, Vertex};

fn main() {
    println!("=== ConectaStore: Sistema de Recomendação MegaStore ===");

    let mut graph = RecommendationGraph::new();

    // Cadastrando Produtos
    let p1 = Product { id: 101, name: "Notebook Gamer".to_string(), category: "Eletrônicos".to_string(), price: 4500.0 };
    let p2 = Product { id: 102, name: "Mouse Sem Fio".to_string(), category: "Eletrônicos".to_string(), price: 150.0 };
    let p3 = Product { id: 103, name: "Teclado Mecânico".to_string(), category: "Eletrônicos".to_string(), price: 350.0 };
    let p4 = Product { id: 104, name: "Cadeira Ergonômica".to_string(), category: "Móveis".to_string(), price: 1200.0 };

    graph.add_product(p1);
    graph.add_product(p2);
    graph.add_product(p3);
    graph.add_product(p4);

    // Registrando Histórico de Compras e Interações
    // Cliente 1 comprou Notebook Gamer (101)
    graph.add_edge(Vertex::Customer(1), Vertex::Product(101), 5.0);

    // Cliente 2 comprou Notebook Gamer (101) e Mouse Sem Fio (102)
    graph.add_edge(Vertex::Customer(2), Vertex::Product(101), 5.0);
    graph.add_edge(Vertex::Customer(2), Vertex::Product(102), 5.0);

    // Cliente 2 também comprou Teclado Mecânico (103)
    graph.add_edge(Vertex::Customer(2), Vertex::Product(103), 5.0);

    println!("\nGerando Recomendações para o Cliente 1...");
    let recs = graph.recommend_for_customer(1, 3);

    for (prod, score) in recs {
        println!(" -> Recomendado: {} (Categoria: {}) - Relevância: {:.2}", prod.name, prod.category, score);
    }

    // Executando Testes de Carga
    run_benchmarks();
}