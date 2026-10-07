# ConectaStore - Sistema de Recomendação de Produtos em Rust

> **MegaStore** | Projeto da Disciplina: *Data Structure Strategy and Implementation*

## 1. Visão Geral do Projeto
O **ConectaStore** é um motor de recomendação colaborativo desenvolvido em **Rust** para superar as limitações de recomendações estáticas baseadas em categoria. Através do uso de **Grafos Heterogêneos Ponderados**, o sistema mapeia conexões complexas entre **Clientes**, **Produtos** e **Categorias**, permitindo sugerir itens altamente relevantes com base no comportamento de compra e navegação.

---

## 2. Estruturas de Dados Utilizadas & Decisões Arquiteturais

| Estrutura | Aplicação no Sistema | Justificação Téorica |
| :--- | :--- | :--- |
| `HashMap<Vertex, Vec<Edge>>` | **Lista de Adjacência do Grafo** | Acesso $O(1)$ aos nós. Ocupação de memória $O(V + E)$ (ideal contra a esparsidade de uma Matriz $O(V^2)$). |
| `HashMap<u64, Product>` | **Índice de Produtos** | Recuperação direta de metadados do produto em $O(1)$. |
| `HashSet<Vertex>` | **Controlo de Percurso** | Evita ciclos em tempo de execução e filtra produtos que o cliente já comprou. |
| `VecDeque<(Vertex, usize, f64)>` | **Fila do BFS** | Execução eficiente da **Busca em Largura** em camadas (limitada a 3 saltos). |

---

## 3. Instruções de Compilação e Execução

### Pré-requisitos
Ter o ambiente Rust instalado (`rustc` e `cargo`).

### Compilar e Executar a Aplicação Principal
```bash
cargo run