use megastore::graph::RecommendationGraph;
use megastore::models::{Product, Vertex};

#[test]
fn test_no_duplicate_recommendations() {
    let mut graph = RecommendationGraph::new();

    let p1 = Product { id: 1, name: "Item A".to_string(), category: "Cat1".to_string(), price: 10.0 };
    let p2 = Product { id: 2, name: "Item B".to_string(), category: "Cat1".to_string(), price: 20.0 };

    graph.add_product(p1);
    graph.add_product(p2);

    // Cliente 1 comprou Item A
    graph.add_edge(Vertex::Customer(1), Vertex::Product(1), 5.0);

    let recs = graph.recommend_for_customer(1, 10);

    // Item A não deve estar nas recomendações do Cliente 1 pois ele já comprou
    assert!(recs.iter().all(|(p, _)| p.id != 1));
}