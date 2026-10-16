use megastore::Loja;
#[test]
fn fluxo_de_ponta_a_ponta() {
    let mut l=Loja::new();
    l.cadastrar(10,"A","C").unwrap(); l.cadastrar(20,"B","C").unwrap();
    l.comprar(1,10).unwrap(); l.comprar(2,10).unwrap(); l.comprar(2,20).unwrap();
    assert_eq!(l.recomendar_cliente(1,5),vec![(20,1)]);
    assert_eq!(l.recomendar_produto(10,5),vec![(20,1)]);
    assert!(l.recomendar_cliente(999,5).is_empty());
}
