use std::io;

mod base64;
mod sha256;

fn main() {

    while true {
        println!("Deseja utilizar qual método? \n[1] Base64\n[2] Sha256");
        let numero: i32 = {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Falha ao ler a linha");
            input.trim().parse().expect("Falha ao converter para inteiro")
        };

        match numero{
            1 => base64::escolhendo_palavra(),
            2 => sha256::escolhendo_palavra(),
            _ => println!("Falha ao escolher método!")
        }

        println!("\nDeseja continuar?\n[1] Sim\n[2] Sair\n");
        let escolha: i32 = {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Falha ao ler a linha");
            input.trim().parse().expect("Falha ao converter para inteiro")
        };

        if escolha==2 { break; }

    }
}

