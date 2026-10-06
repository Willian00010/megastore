# ConectaStore — Sistema de Recomendação Baseado em Grafos

## 1. Objetivo e Funcionamento
O ConectaStore é um motor de recomendação desenvolvido em Rust para a MegaStore, utilizando modelagem por grafos para superar as limitações das recomendações estáticas.

## 2. Arquitetura e Estruturas de Dados
- **Grafo Multipartido Ponderado:** Representado via Lista de Adjacência (`Vec<Vec<Edge>>`) para otimização de memória $O(\vert{}V\vert{} + \vert{}E\vert{})$.
- **Indexação Rápidas:** Utilização de `HashMap` para pesquisas $O(1)$ de nós e produtos.
- **Concorrência Segura:** Utilização de `Arc<RwLock<GraphCore>>` para permitir leitura simultânea multi-thread sem *data races*.

## 3. Instruções de Compilação e Execução
```bash
# Compilar e executar o projeto
cargo run

# Executar a suíte de testes automatizados
cargo test

