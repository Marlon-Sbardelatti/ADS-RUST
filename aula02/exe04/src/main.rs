use std::collections::LinkedList;
use text_io::*;
mod user;
use user::User;

fn main() {
    let mut list: LinkedList<User> = LinkedList::new();
    println!("(1) - Cadastrar\n(2) - Listar\n(3) - Finalizar\n");
    let mut res: i32 = read!();

    while res != 3 {
        match res {
            1 => {
                cadastrar(&mut list);
            }
            2 => {
                listar(&mut list);
            }
            _ => println!("Operação inválida."),
        }
        println!("\n(1) - Cadastrar\n(2) - Listar\n(3) - Finalizar\n");
        res = read!();
    }
}

fn cadastrar(list: &mut LinkedList<User>) {
    println!("Digite o nome: ");
    let nome: String = read!();
    println!("Digite a idade: ");
    let idade: u32 = read!();
    list.push_back(User::new(nome, idade));
}

fn listar(list: &mut LinkedList<User>) {
    for e in list {
        println!("{:?}", e);
    }
}
