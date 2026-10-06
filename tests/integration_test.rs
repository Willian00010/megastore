use megastore::graph::ConcurrentConectaStore;
use megastore::models::{NodeType, Product};

#[test]
fn test_recommendation_filtering() {
    let store = ConcurrentConectaStore::new();
    
    let p1 = store.add_product(Product { id: 1, name: "A".into(), category_id: 1, price: 10.0 }).unwrap();
    let p2 = store.add_product(Product { id: 2, name: "B".into(), category_id: 1, price: 20.0 }).unwrap();
    let c1 = store.add_node("c1".into(), NodeType::Client);
    let c2 = store.add_node("c2".into(), NodeType::Client);

    store.add_edge(c1, p1, 5.0).unwrap();
    store.add_edge(c2, p1, 5.0).unwrap();
    store.add_edge(c2, p2, 4.0).unwrap();

    let recs = store.recommend_for_client("c1", 10).unwrap();
    
    // Valida que o cliente não recebe o produto que já comprou (p1)
    assert_eq!(recs.len(), 1);
    assert_eq!(recs[0].0.id, 2);
}