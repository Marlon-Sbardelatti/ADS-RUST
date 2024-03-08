mod produto;
mod vetor;
use produto::Produto;
use text_io::*;
use vetor::Vetor;

fn main() {
    let mut vetor = Vetor::new();

    print!("(1) - Cadastrar\n(2) - Listar\n(3) - Pesquisar\n(4) - Alterar\n(5) - Remover\n(6) - Finalizar\n");
    let mut res: i32 = read!();

    while res != 6 {
        match res {
            1 => {
                println!("Nome do produto: ");
                let nome: String = read!();
                println!("Valor do produto: ");
                let valor: f64 = read!();
                vetor.cadastrar(Produto::new(nome, valor));
            }
            2 => {
                vetor.listar();
            }
            3 => {
                println!("Nome do produto: ");
                let nome: String = read!();
                vetor.pesquisar(nome);
            }
            4 => {
                println!("Nome do produto: ");
                let nome: String = read!();
                vetor.alterar(nome);
            }
            5 => {
                println!("Nome do produto: ");
                let nome: String = read!();
                vetor.remover(nome);
            }
            _ => println!("Operação inválida."),
        }

        print!("\n(1) - Cadastrar\n(2) - Listar\n(3) - Pesquisar\n(4) - Alterar\n(5) - Remover\n(6) - Finalizar\n");
        res = read!();
    }
}
