use matematica_e_fisica_para_jogos::set::Conjunto;
fn main() {
    println!("Lista 1 - Matemática e Física para Jogos\n");
    println!("Item a: conjunto vazio");

    let mut a = Conjunto::<char>::new();
    a.print();
    println!();

    println!("Item b: cópia de conjunto");
    a.add('a');
    a.add('b');
    a.add('c');
    a.add('c'); // Não adiciona repetidos
    let b = a.clone();
    print!("a: ");
    a.print();
    print!("b: ");
    b.print();
    println!();

    println!("Item c: diferença de conjuntos");
    a.add('x');
    a.add('y');
    a.add('z');
    let mut c = a.clone() - b.clone();
    c.print();
    println!();

    println!("Item d: intersecção entre conjuntos");
    let d = a.intersection(b.clone());
    d.print();
    println!();

    println!("Itens e,f: união entre conjuntos");
    let e = a.union(b.clone());
    e.print();
    println!();

    println!("Item g: checar se um elemento pertence ao conjunto");
    let f = a.contains('x');
    let g = a.contains('h');
    println!("'x' pertence ao conjunto a: {}", f);
    println!("'h' pertence ao conjunto a: {}", g);
    println!();

    println!("Item h: checar se um conjunto é subconjunto próprio de outro");
    let h = b.is_subset(a.clone());
    c.add('h');
    let i = c.is_subset(a.clone());
    println!("Conjunto b é subconjunto de conjunto a: {}", h);
    println!("Conjunto c é subconjunto de conjunto a: {}", i);

    println!("Item i: gerar todos os subconjuntos próprios");
    let j = a.proper_subsets();
    for elem in j {
        elem.print();
    }
}
