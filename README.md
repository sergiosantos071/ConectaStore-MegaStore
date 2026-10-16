# ConectaStore — MegaStore

Protótipo de recomendação colaborativa em Rust, sem dependências externas. Representa clientes e produtos como vértices de um grafo bipartido não direcionado; compras são arestas. A força de uma recomendação é o número de clientes similares que compraram o item.

## Requisitos
Rust e Cargo instalados: https://rustup.rs/

## Executar
```bash
cargo run
cargo test
cargo run --release -- bench
```

## Exemplo
O modo padrão cadastra 5 produtos, registra compras, consulta o produto 2 e recomenda itens para o cliente 10 e similares ao produto 1. O modo `bench` mede o tempo médio de 100 consultas com 100, 1.000 e 10.000 produtos. **Os números são medidos no computador que executar o projeto; não há resultados pré-calculados.**

## Arquitetura
- `src/lib.rs`: `Loja`, cadastro, consulta, relações e recomendações.
- `src/main.rs`: demonstração e benchmark.
- `tests/integracao.rs`: testes ponta a ponta.
- `Cargo.toml`: configuração Cargo.

`HashMap<u32, Produto>` indexa produtos; `HashMap<u32, HashSet<u32>>` representa listas de adjacência nas duas direções. `HashSet` evita arestas e resultados duplicados. `VecDeque` implementa a fila BFS. A busca percorre até três arestas cliente-produto-cliente-produto. Produtos já comprados são excluídos. A ordenação final usa pontuação decrescente e ID crescente.

## Complexidade e escalabilidade
Armazenamento: O(P+C+E), com P produtos, C clientes e E compras. Cadastro e consulta têm custo médio O(1). BFS tem custo O(V_r+E_r) sobre a sub-rede visitada; ordenar R candidatos custa O(R log R). A busca é limitada a três saltos, mas clientes e produtos muito populares podem ampliar a vizinhança. Para milhões de produtos, considerar persistência, índices e cache.

## Resultados de desempenho
Execute `cargo run --release -- bench` e registre os tempos exibidos (microssegundos por recomendação) no relatório. Variam conforme CPU, compilador e carga da máquina.

## Vídeo pitch
Adicione aqui o link público ou não listado depois de publicar o vídeo: **[LINK PENDENTE]**.

## Publicação
Crie um repositório público no GitHub e envie os arquivos desta pasta. O upload/publicação não é feito automaticamente por este projeto.
