mod produto;
use std::fmt::format;

use produto::Produto;
use text_io::*;

fn main() {
    let mut produtos: Vec<Produto> = Vec::new();
    println!("(1) - Cadastrar\n(2) - Listar\n(3) - Busca por segmento\n(4) - Alterar\n(5) - Remover\n(6) - Finalizar\n");
    let mut res: u32 = read!();

    while res != 6 {
        match res {
            1 => {
                cadastrar(&mut produtos);
            }
            2 => {
                listar(&produtos);
            }
            3 => {
                pesquisa_segmento(&mut produtos);
            }
            4 => {
                alterar(&mut produtos);
            }
            5 => {
                remover(&mut produtos);
            }
            _ => println!("Operação inválida."),
        }

        println!("\n(1) - Cadastrar\n(2) - Listar\n(3) - Busca por segmento\n(4) - Alterar\n(5) - Remover\n(6) - Finalizar\n");
        res = read!();
    }
}

fn cadastrar<'a>(produtos: &'a mut Vec<Produto>) {
    println!("Digite o nome do produto: ");
    let nome: String = read!();
    println!("Digite o segmento do produto: ");
    let segmento: String = read!();
    produtos.push(Produto::new(nome, segmento));
    println!("Produto inserido com sucesso!");
}

fn listar(produtos: &Vec<Produto>) {
    for p in produtos {
        println!("{:?}", p);
    }
}

fn alterar(produtos: &mut Vec<Produto>) {
    println!("Digite o nome do produto: ");
    let nome: String = read!();

    let mut found = false;
    for p in produtos {
        if p.name == nome {
            println!("Digite o novo nome do produto: ");
            let novo_nome: String = read!();
            println!("Digite o novo segmento do produto: ");
            let novo_segmento: String = read!();
            p.name = novo_nome;
            p.segmento = novo_segmento;
            println!("Produto alterado com sucesso!");
            found = true;
        }
    }

    if !found {
        println!("Produto não localizado");
    }
}

fn remover(produtos: &mut Vec<Produto>) {
    println!("Digite o nome do produto: ");
    let nome: String = read!();
    let mut found = false;

    let _res = produtos.retain(|p| {
        // p.name != nome;
        found = true;
        p.name != nome
    });

    if !found {
        println!("Produto não localizado");
    }
}
fn pesquisa_segmento(produtos: &mut Vec<Produto>) {
    let mut temp: Vec<String> = Vec::new();
    for p in &mut *produtos {
        if !temp.contains(&p.segmento) {
            temp.push(p.segmento.clone());
        }
    }

    let mut dados = String::new();
    for seg in temp {
        let mut count = 0;
        let mut dado_seg = format!("Segmento: {}\n", seg);
        for p in &mut *produtos {
            if seg == p.segmento {
                dado_seg += &format!("{:?}\n", p);
                count += 1;
            }
        }
        dado_seg += &format!("Total: {}\n\n", count);
        dados += &dado_seg;
    }
    println!("{}", dados);
}
