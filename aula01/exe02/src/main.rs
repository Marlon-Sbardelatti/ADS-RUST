use text_io::*;

fn main() {
    let mut arr: [String; 5] = Default::default();
    let mut index: usize = 0;

    println!("(1) - Cadastrar\n(2) - Listar\n(3) - Alterar\n(4) - Remover\n(5) - Finalizar");
    let mut res: i32 = read!();

    while res != 5 {
        match res {
            1 => {
                cadastrar(&mut arr, &mut index);
            }
            2 => {
                listar(&arr);
            }
            3 => {
                alterar(&mut arr);
            }
            4 => {
                remover(&mut arr, &mut index);
            }
            _ => println!("Operação inválida."),
        }
        println!("(1) - Cadastrar\n(2) - Listar\n(3) - Alterar\n(4) - Remover\n(5) - Finalizar");
        res = read!();
    }
}

fn cadastrar(arr: &mut [String; 5], index: &mut usize) {
    println!("Digite o nome: ");
    let nome: String = read!();
    arr[*index] = nome;
    *index += 1;
}

fn listar(arr: &[String; 5]) {
    for e in arr {
        if e != "" {
            println!("{}", e)
        }
    }
}

fn alterar(arr: &mut [String; 5]) {
    println!("Digite o index: ");
    let index: usize = read!();
    println!("Digite o nome: ");
    let nome: String = read!();
    arr[index] = nome;
}

fn remover(arr: &mut [String; 5], index: &mut usize) {
    println!("Digite o index: ");
    for (i, e) in arr.clone().iter().enumerate() {
        if e != "" {
            println!("{} - {}", i, e);
        }
    }

    let idx: usize = read!();
    arr[idx] = Default::default();

    let mut temp_arr: [String; 5] = Default::default();
    let mut count: usize = 0;
    for e in &mut *arr {
        if e != "" {
            temp_arr[count] = e.to_string();
            count += 1;
        }
    }

    *arr = temp_arr;
    *index -= 1;
}
