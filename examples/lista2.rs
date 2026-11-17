use matematica_e_fisica_para_jogos::base_transformer::BaseTransformer;

fn main() {
    println!("Lista 2 - Matemática e Física para Jogos\n");
    println!("Questão 01");
    println!("Item a: 197 para a base 2");
    let mut conversor = BaseTransformer::new();
    conversor.set_base(10);
    conversor.set_numero("197".to_string());
    conversor.to_base(2);
    println!("{}\n", conversor.get_numero());

    println!("Item b: 197 para a base 3");
    conversor.to_base(3);
    println!("{}\n", conversor.get_numero());

    println!("Item c: 197 para a base 5");
    conversor.to_base(5);
    println!("{}\n", conversor.get_numero());

    println!("Item d: 197 para a base 4");
    conversor.to_base(4);
    println!("{}\n", conversor.get_numero());

    println!("Item e: 197 para a base 16");
    conversor.to_base(16);
    println!("{}\n", conversor.get_numero());
}
