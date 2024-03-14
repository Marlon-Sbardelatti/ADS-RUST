use queues::*;
use text_io::*;

fn main() {
    let mut q: Queue<String> = queue![];

    println!("(1) - Cadastrar\n(2) - Remover\n(3) - Listar\n(4) - Finalizar\n");
    let mut res: i32 = read!();

    while res != 4 {
        match res {
            1 => {
                println!("Digite um nome: ");
                let nome: String = read!(); // Read into a String
                cadastrar(&mut q, nome); // Pass ownership to the function
            }
            2 => {
                let res = remover(&mut q);
                if res {
                    println!("Elemento removido com sucesso!");
                } else {
                    println!("Não foi possível remover o elemento.")
                }
            }
            3 => {
                println!("{:?}", q)
            }
            _ => println!("Operação inválida."),
        }
        println!("(1) - Cadastrar\n(2) - Remover\n(3) - Listar\n(4) - Finalizar\n");
        res = read!();
    }
}

fn cadastrar(queue: &mut Queue<String>, nome: String) -> bool {
    if let Ok(_) = queue.add(nome) {
        return true;
    }
    false
}

fn remover(queue: &mut Queue<String>) -> bool {
    if let Ok(_) = queue.remove() {
        return true;
    }
    false
}
