mod vetor;
use vetor::{Produto, Vetor};

fn main() {
    // Example of using the Vetor struct
    let mut vetor = Vetor::new();

    // Calling the cadastrar method to add elements

    vetor.cadastrar(Produto::new("Item1", 42.0));
    vetor.cadastrar(Produto::new("Item2", 15.0));
}

