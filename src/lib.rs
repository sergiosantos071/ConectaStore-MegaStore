use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Produto { pub id: u32, pub nome: String, pub categoria: String }

#[derive(Debug, Default)]
pub struct Loja {
    produtos: HashMap<u32, Produto>,
    // Grafo bipartido: cliente -> produtos adquiridos; produto -> clientes compradores.
    compras: HashMap<u32, HashSet<u32>>,
    compradores: HashMap<u32, HashSet<u32>>,
}

impl Loja {
    pub fn new() -> Self { Self::default() }
    pub fn cadastrar(&mut self, id: u32, nome: &str, categoria: &str) -> Result<(), String> {
        if self.produtos.contains_key(&id) { return Err("ID de produto duplicado".into()); }
        self.produtos.insert(id, Produto { id, nome: nome.into(), categoria: categoria.into() });
        Ok(())
    }
    pub fn consultar(&self, id: u32) -> Option<&Produto> { self.produtos.get(&id) }
    pub fn comprar(&mut self, cliente: u32, produto: u32) -> Result<(), String> {
        if !self.produtos.contains_key(&produto) { return Err("Produto inexistente".into()); }
        self.compras.entry(cliente).or_default().insert(produto);
        self.compradores.entry(produto).or_default().insert(cliente);
        Ok(())
    }
    // BFS de ate 3 arestas: cliente -> produto -> outro cliente -> produto.
    // Pontuacao = numero de caminhos colaborativos distintos; desempate por ID.
    pub fn recomendar_cliente(&self, cliente: u32, limite: usize) -> Vec<(u32, usize)> {
        let adquiridos = self.compras.get(&cliente).cloned().unwrap_or_default();
        let mut fila: VecDeque<(bool, u32, usize)> = VecDeque::new();
        let mut vistos_produtos = HashSet::new();
        let mut vistos_clientes = HashSet::new();
        let mut pontuacoes: HashMap<u32, usize> = HashMap::new();
        fila.push_back((true, cliente, 0));
        vistos_clientes.insert(cliente);
        while let Some((eh_cliente, id, distancia)) = fila.pop_front() {
            if distancia >= 3 { continue; }
            if eh_cliente {
                if let Some(produtos) = self.compras.get(&id) {
                    for &p in produtos {
                        if distancia == 2 && !adquiridos.contains(&p) { *pontuacoes.entry(p).or_default() += 1; }
                        if vistos_produtos.insert(p) { fila.push_back((false, p, distancia + 1)); }
                    }
                }
            } else if let Some(clientes) = self.compradores.get(&id) {
                for &c in clientes {
                    if vistos_clientes.insert(c) { fila.push_back((true, c, distancia + 1)); }
                }
            }
        }
        let mut resultado: Vec<_> = pontuacoes.into_iter().collect();
        resultado.sort_by(|a,b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        resultado.truncate(limite);
        resultado
    }
    pub fn recomendar_produto(&self, produto: u32, limite: usize) -> Vec<(u32, usize)> {
        let mut pontuacoes: HashMap<u32, usize> = HashMap::new();
        if let Some(clientes) = self.compradores.get(&produto) {
            for cliente in clientes {
                if let Some(produtos) = self.compras.get(cliente) {
                    for &p in produtos { if p != produto { *pontuacoes.entry(p).or_default() += 1; } }
                }
            }
        }
        let mut resultado: Vec<_> = pontuacoes.into_iter().collect();
        resultado.sort_by(|a,b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        resultado.truncate(limite);
        resultado
    }
    pub fn total_produtos(&self) -> usize { self.produtos.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn cadastro_e_consulta() {
        let mut l=Loja::new(); assert!(l.cadastrar(1,"Mouse","Eletronicos").is_ok());
        assert_eq!(l.consultar(1).unwrap().nome,"Mouse"); assert!(l.cadastrar(1,"Outro","X").is_err());
    }
    #[test] fn compra_invalida() { assert!(Loja::new().comprar(1,99).is_err()); }
    #[test] fn recomenda_sem_repetir_ou_comprados() {
        let mut l=Loja::new(); for i in 1..=3 {l.cadastrar(i,"Item","X").unwrap();}
        for (c,p) in [(1,1),(2,1),(2,2),(3,1),(3,2),(3,3)] {l.comprar(c,p).unwrap();}
        assert_eq!(l.recomendar_cliente(1,10),vec![(2,2),(3,1)]);
    }
}
