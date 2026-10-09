# ConectaStore - Sistema de Recomendação de Produtos Baseado em Grafos

> **MegaStore** | Projeto Integrador — *Data Structure Strategy and Implementation*  
> **Instituição:** UniFECAF

---

## 1. Objetivo e Funcionamento do Sistema
O **ConectaStore** é um motor de recomendação desenvolvido para solucionar o problema de recomendações genéricas no e-commerce **MegaStore**. 

O sistema substitui abordagens estáticas (como listas de mais vendidos ou filtros simples de categoria) por um **Grafo Heterogêneo Ponderado**. Ele conecta **Clientes**, **Produtos** e **Categorias** por meio de relações comportamentais (como histórico de compras e interações). 

Através do algoritmo de **Busca em Largura (BFS)**, o sistema percorre as conexões no grafo até uma profundidade controlada (níveis de salto), calcula a relevância acumulada das arestas e sugere produtos altamente personalizados para cada cliente, prevenindo recomendações de itens já adquiridos.

---

## 2. Tecnologias e Estruturas Utilizadas
- **Linguagem de Programação:** Rust (Edição 2021)
- **Estruturas de Dados Principais:**
  - `HashMap<Vertex, Vec<Edge>>`: **Lista de Adjacência** para representação do Grafo, garantindo acesso aos nós em O(1) e complexidade de espaço O(V + E).
  - `HashMap<u64, Product>`: Índice direto para consulta e recuperação de metadados dos produtos por ID em O(1).
  - `HashSet<Vertex>`: Controle de nós visitados durante as travessias (evitando loops infinitos) e filtragem de itens que o cliente já comprou.
  - `VecDeque<(Vertex, usize, f64)>`: Fila de prioridade de níveis utilizada na execução da Busca em Largura (BFS).

---

## 3. Instruções para Compilação e Execução

### Pré-requisitos
Ter o ambiente de desenvolvimento Rust e o gerenciador de pacotes `cargo` instalados em sua máquina:
```bash
rustc --version
cargo --version
```

## 4. Instruções para Execução dos Testes

O projeto conta com suítes de testes unitários e de integração para garantir a integridade das conexões do grafo, a validação das buscas e a prevenção de recomendações duplicadas.

Para rodar todos os testes automatizados do repositório:
```bash
cargo test
```
Para rodar os testes e visualizar as saídas detalhadas do console:
```bash
cargo test -- --nocapture
```
### **Exemplos de Uso**

```markdown
## 5. Exemplos de Uso

Abaixo está um exemplo prático de como instanciar o grafo, cadastrar produtos, registrar conexões comportamentais e gerar recomendações:

```rust
use megastore::graph::RecommendationGraph;
use megastore::models::{Product, Vertex};

fn main() {
    let mut graph = RecommendationGraph::new();

    // 1. Cadastrando Produtos
    let p1 = Product { id: 101, name: "Notebook Gamer".to_string(), category: "Eletrônicos".to_string(), price: 4500.0 };
    let p2 = Product { id: 102, name: "Mouse Sem Fio".to_string(), category: "Eletrônicos".to_string(), price: 150.0 };

    graph.add_product(p1);
    graph.add_product(p2);

    // 2. Registrando Conexões (Cliente 1 comprou Notebook Gamer)
    graph.add_edge(Vertex::Customer(1), Vertex::Product(101), 5.0);

    // 3. Registrando histórico do Cliente 2 (Comprou Notebook e Mouse)
    graph.add_edge(Vertex::Customer(2), Vertex::Product(101), 5.0);
    graph.add_edge(Vertex::Customer(2), Vertex::Product(102), 5.0);

    // 4. Gerando Recomendações para o Cliente 1 (Retorna até 3 produtos)
    let recomendacoes = graph.recommend_for_customer(1, 3);

    for (produto, pontuacao) in recomendacoes {
        println!("Recomendado: {} | Relevância: {:.2}", produto.name, pontuacao);
    }
}
```
### **Arquitetura da Solução**

```markdown
## 6. Arquitetura da Solução

A estrutura do repositório foi modulada de forma limpa e idiomática em Rust, garantindo separação clara de responsabilidades:

```text
megastore/
├── src/
│   ├── main.rs        # Ponto de entrada (demonstração interativa e execução)
│   ├── lib.rs         # Exportação e declaração dos módulos do projeto
│   ├── models.rs      # Definição das structs (Product, Edge) e Enums (Vertex)
│   ├── graph.rs       # Implementação da Lista de Adjacência e do algoritmo BFS
│   └── benchmark.rs   # Módulo de medição de desempenho e carga
├── tests/
│   └── integration_test.rs # Testes de integração automatizados
├── Cargo.toml         # Gerenciamento de dependências e metadata do Rust
└── README.md          # Documentação oficial do projeto
```


## 7. Resultados dos Testes de Desempenho

A medição do desempenho foi realizada utilizando o módulo `benchmark.rs`, variando a quantidade $N$ de produtos e conexões para avaliar a escalabilidade da solução em tempo de execução:

| Quantidade de Produtos ($N$) | Carga do Grafo / Inserção | Resposta da Consulta (BFS) |
| :--- | :--- | :--- |
| **$N = 1.000$** | `~0.8 ms` | `< 100 µs` |
| **$N = 10.000$** | `~8.2 ms` | `~1.2 ms` |
| **$N = 50.000$** | `~45.0 ms` | `~8.5 ms` |

*Os resultados demonstram que o uso da **Lista de Adjacência** em conjunto com o controle de profundidade do BFS mantêm o tempo de resposta extremamente baixo, mesmo com o crescimento expressivo da base de dados.*


## 8. Link do Vídeo Pitch

- **Link do Vídeo no YouTube:** [https://youtu.be/HisClAIKzwc]

*(O vídeo demonstra o problema enfrentado pela MegaStore, a modelagem em grafo, a execução real do código no terminal e a análise das decisões de desempenho).*
