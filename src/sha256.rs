use std::io;
pub fn escolhendo_palavra() {
    println!("\nDigite o que deseja que ser codificado: ");
    let mut message = String::new();
    match io::stdin().read_line(&mut message) {
        Ok(_) => {
            println!("Você digitou: {}", message);
        }
        Err(error) => {
            println!("Erro ao ler entrada: {}", error);
        }
    }
    message = message.trim().to_string();

    let hash = sha256(&message);
    println!("SHA-256 hash of '{}' is: {}", message, hash);
}

fn sha256(input: &str) -> String {
    let mut bytes = input.as_bytes().to_vec();

    let initial_len = bytes.len() as u64 * 8;

    bytes.push(0x80); // appending a single '1' bit

    while (bytes.len() * 8) % 512 != 448 {
        bytes.push(0x00); // padding with zeros until length ≡ 448 (mod 512)
    }

    let mut length_bytes = initial_len.to_be_bytes().to_vec();
    bytes.append(&mut length_bytes);

    let mut h: [u32; 8] = [ // 8 primeiros termos, que são a raiz quadrada dos 8 primeiros primos;
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
        0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
    ];

    const K: [u32; 64] = [ // constantes que são a raiz cubica dos números primos
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
        0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
        0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
        0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
        0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
    ];

    let mut w: [u32; 64] = [0; 64];
    let mut chunk = [0; 64];

    for chunk_start in (0..bytes.len()).step_by(64) {
        chunk.copy_from_slice(&bytes[chunk_start..chunk_start + 64]);

        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                chunk[i * 4],
                chunk[i * 4 + 1],
                chunk[i * 4 + 2],
                chunk[i * 4 + 3]
            ]);

        }

        for i in 16..64 {//  a partir de 16 até 63, utilizando de 0 até 15(palvras originais),  são calculadas novas palavras para w[i], com base nas palavras anteriores.
            let s0 = (w[i - 15].rotate_right(7)) ^ (w[i - 15].rotate_right(18)) ^ (w[i - 15] >> 3);
            //println!("s0 {}: {:032b}", i, s0);

            // pego a palvra, rotaciono ela(00001111.rotate_right(2) = 11000011), depois faço um XOR, que basicamente comparo,
            let s1 = (w[i - 2].rotate_right(17)) ^ (w[i - 2].rotate_right(19)) ^ (w[i - 2] >> 10);
            //println!("s1 {}: {:032b}", i, s1);

            // acidiona a palavra no w[i]
            w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]) .wrapping_add(s1);

            // println!("w[i]: {:08b}", w[i]);
        }

        let mut a = h[0];
        let mut b = h[1];
        let mut c = h[2];
        let mut d = h[3];
        let mut e = h[4];
        let mut f = h[5];
        let mut g = h[6];
        let mut hh = h[7];

        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            //compara os bits de e e f, e só o que for 1 em ambos recebe 1, caso contrario é 0
            // (!e) inverte os bits e depois faz a mesma coisa do e e f
            // por fim compara os bits XOR
            let temp1 = hh.wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);

            /*
            este loop principal do algoritmo sha256, onde as variáveis a, b, c, d, e, f, g e hh são atualizadas iterativamente com base nos cálculos dos blocos de mensagem e nas constantes K, segue a especificação do algoritmo sha256.
             */
        }

        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
        //atualiza os valores do hash com os valores temporários calculados.
    }

    let mut hash = String::new();
    for &val in &h {
        hash.push_str(&format!("{:08x}", val));
    }
    hash

    //converte os valores do hash em uma representação hexadecimal e os concatena para formar a string final do hash.
}






