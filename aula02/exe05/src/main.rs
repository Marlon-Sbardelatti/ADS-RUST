mod aluno;
use std::collections::HashMap;

use aluno::Aluno;
use text_io::*;

fn main() {
    let mut map: HashMap<String, Aluno> = HashMap::new();

    println!("(1) - Cadastrar\n(2) - Listar\n(3) - Alterar\n(4) - Remover\n(5) - Finalizar\n");
    let mut res: u32 = read!();

    while res != 5 {
        match res {
            1 => {
                cadastrar(&mut map);
            }
            2 => {
                listar(&map);
            }
            3 => {
                alterar(&mut map);
            }
            4 => {
                remover(&mut map);
            }
            _ => println!("Operação inválida."),
        }
        println!(
            "\n(1) - Cadastrar\n(2) - Listar\n(3) - Alterar\n(4) - Remover\n(5) - Finalizar\n"
        );
        res = read!();
    }
}

fn cadastrar(map: &mut HashMap<String, Aluno>) -> bool {
    println!("Digite o nome: ");
    let nome: String = read!();
    println!("Digite a idade: ");
    let idade: u32 = read!();

    if !map.contains_key(&nome) {
        map.insert(nome.clone(), Aluno::new(nome, idade));
        return true;
    }

    println!("Não foi possivel adicionar o aluno");
    false
}

fn listar(map: &HashMap<String, Aluno>) {
    for e in map.values() {
        println!("{:?}", e);
    }
}

fn alterar(map: &mut HashMap<String, Aluno>) -> bool {
    println!("Digite o nome: ");
    let nome: String = read!();

    if map.contains_key(&nome) {
        match map.remove(&nome) {
            Some(_) => {
                println!("Digite o novo nome: ");
                let novo_nome: String = read!();
                println!("Digite a nova idade: ");
                let nova_idade: u32 = read!();
                map.insert(novo_nome.clone(), Aluno::new(novo_nome, nova_idade));
                println!("Aluno alterado com sucesso!");
                return true;
            }
            None => {
                println!("Não foi possivel localizar o aluno.");
                return false;
            }
        }
    }

    println!("Não foi possivel adicionar o aluno");
    false
}

fn remover(map: &mut HashMap<String, Aluno>) -> bool {
    println!("Digite o nome: ");
    let nome: String = read!();

    if map.contains_key(&nome) {
        // map.remove(&nome);

        match map.remove(&nome) {
            Some(_) => {
                println!("Aluno removido com sucesso!");
                return true;
            }
            None => {
                println!("Não foi possivel remover o aluno.");
                return false;
            }
        }
    }

    println!("Não foi possivel adicionar o aluno");
    false
}
