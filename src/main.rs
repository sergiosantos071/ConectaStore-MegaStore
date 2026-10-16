use megastore::Loja;
use std::time::Instant;
fn demonstracao() {
    let mut loja=Loja::new();
    for (id,nome,cat) in [(1,"Notebook","Eletronicos"),(2,"Mouse","Acessorios"),(3,"Teclado","Acessorios"),(4,"Monitor","Eletronicos"),(5,"Livro Rust","Livros")] {
        loja.cadastrar(id,nome,cat).unwrap();
    }
    for (c,p) in [(10,1),(20,1),(20,2),(20,3),(30,1),(30,2),(30,4),(40,5)] {loja.comprar(c,p).unwrap();}
    println!("=== ConectaStore / MegaStore ===");
    println!("Produtos cadastrados: {}",loja.total_produtos());
    println!("Consulta ID 2: {:?}",loja.consultar(2));
    println!("Recomendacoes para cliente 10 (ID, pontuacao): {:?}",loja.recomendar_cliente(10,5));
    println!("Relacionados ao produto 1: {:?}",loja.recomendar_produto(1,5));
}
fn benchmark() {
    println!("Volume de produtos | Tempo de recomendacao (microssegundos)");
    for n in [100usize,1000,10000] {
        let mut l=Loja::new();
        for p in 0..n {l.cadastrar(p as u32,"Produto","Categoria").unwrap();}
        for c in 0..(n/10) { for k in 0..5 {l.comprar(c as u32,((c*7+k)%n) as u32).unwrap();} }
        let inicio=Instant::now();
        for c in 0..100 {std::hint::black_box(l.recomendar_cliente((c%(n/10)) as u32,10));}
        println!("{:>17} | {:>10.2}",n,inicio.elapsed().as_secs_f64()*1e6/100.0);
    }
}
fn main() {
    let args: Vec<String>=std::env::args().collect();
    if args.get(1).map(String::as_str)==Some("bench") {benchmark();} else {demonstracao();}
}
