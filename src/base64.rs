use std::io;
const BASE64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";


pub fn escolhendo_palavra() {


    println!("\nDigite o que deseja que ser codificado e descodificado: ");
    let mut original_text = String::new();
    match io::stdin().read_line(&mut original_text) {
        Ok(_) => {
            println!("Você digitou: {}", original_text);
        }
        Err(error) => {
            println!("Erro ao ler entrada: {}", error);
        }
    }
    original_text = original_text.trim().to_string();

    // Codificando para Base64
    let encoded_text = encode_base64(original_text.as_bytes());
    println!("Texto codificado: {}", encoded_text);

    // Decodificando de Base64
    let decoded_text = base64_decode(&encoded_text);
    println!("Texto decodificado: {}", decoded_text);
}


fn encode_base64(input: &[u8]) -> String {
    let mut result = String::new();
    let mut i = 0;

    while i < input.len() {
        let first = (input[i] >> 2) & 0x3F;
        result.push(BASE64_CHARS[first as usize] as char);
        /*       -- Aqui, o primeiro byte é processado --
                    Em (input[i] >> 2) eu pego os 6 bits mais significativos, depois
                movo eles para a direita em 2 posições.
                    Em & 0x3F, uso ele para pegar os 6 bits menos significativos/os 6 primeiros e
                guardo/mantenho eles, no caso esses bits vão ser usados para buscar o
                caractere correspondente em BASE64_CHARS e então é adicionado ao resultado(result).
                    Usando "gael" como exemplo:
                      -  O byte correspondente a 'g' em ASCII é 103, que em binário é 01100111.
                      -  Deslocando para a direita por 2 posições, temos 00011001.
                      -  Aplicando a máscara & 0x3F, obtemos 000110, que é 6 em decimal.
                      -  O caractere Base64 correspondente ao valor 6 é 'G'.
        */
        let second = ((input[i] & 0x03) << 4) | ((input.get(i + 1).unwrap_or(&0) >> 4) & 0x0F);
        result.push(BASE64_CHARS[second as usize] as char);
        /*      .unwrap_or(&0) -> Se não houver byte seguinte, ele retorna 0
                    Em (input[i] & 0x03)<< 4, basicamente estou pegando os 2 bits restantes do processo anterior e
                os movendo 4 posições para a esquerda.
                    Os 4 bits mais significativos do segundo byte são extraídos e posicionado a esquerda dos 2 bits
                obtidos anteriormente. (feito pela mascára 0x0F), depois disso descolamos esses 4 bits 4 posições
                a direita. ((input.get(i + 1).unwrap_or(&0) >> 4) & 0x0F).
                    Esses 6 bits são usados para buscar o indice na tabela Base64(BASE64_CHARS)

        */

        let third = ((input.get(i + 1).unwrap_or(&0) & 0x0F) << 2) | ((input.get(i + 2).unwrap_or(&0) >> 6) & 0x03);
        result.push(BASE64_CHARS[third as usize] as char);
        /*
            .unwrap_or(&0) -> Se não houver byte seguinte, ele retorna 0
            Em (input.get(i + 1).unwrap_or(&0) & 0x0F), acesso o segundo byte após o indice, e pego os 4 bits mais
        significativos (& 0x0F) e depois movo eles 2 posições para a esquerda, para dar espaço aos próximos bits
        do próximo byte.
            Em (input.get(i + 2).unwrap_or(&0) >> 6  & 0x03 acesso o  byte seguinte, o terceiro Byte, e então movo eles para
        a direita em 6 posições, deixando somente os 2 bits mais significativos(& 0x03).
            Então combinamos os dois grupinhos de bits.
         */


        let fourth = input.get(i + 2).unwrap_or(&0) & 0x3F;
        result.push(BASE64_CHARS[fourth as usize] as char);
        /*
            Pegamos o terceiro byte após o indice, e então pegamos somente os 6 bits mais significativos.
            Depois adicionamos ele ao result.
         */
        i += 3; // i vai de 3 em 3, pois sempre trabalhamos com 3 bytes em cada iteração;
    }

    let remainder = input.len() % 3; // verifica se temos bits sobrando
    if remainder == 1 { // se tiver 1 sobrando
        result.pop();
        result.push('=');
        result.push('=');
    } else if remainder == 2 { // se tiver 2 sobrando
        result.push('=');
    }

    result
}


fn base64_decode(input: &str) -> String {
    let mut result = String::new();
    let mut buffer: u32 = 0; // pega temporariamente os bits decodificados
    let mut buffer_length = 0; // usado para saber quantos bits estão no buffer

    for ch in input.chars() {
        if ch == '=' { // quando tem o excesso de bits no encode
            break;
        }

        let value = BASE64_CHARS.iter().position(|&c| c == ch as u8).unwrap() as u32; // Procura na tabela de Base64 a posição da letra e então a converte em u32;
        buffer = (buffer << 6) | value; // Base64 usando 6 bits, então desloca eles para a esquerda e depois são adicionados os bits do novo valor
        buffer_length += 6; // atualiza o comprimento

        if buffer_length >= 8 { // se tiver pelo menos 8 bits no buffer, então podemos retirar 1 byte dele
            let offset = buffer >> (buffer_length - 8); // calcula o byte a ser extraido e mandando os bits para a direita, deixando somente os 8 mais significativos
            result.push(char::from(offset as u8)); // converte o offset para u8 e manda esse byte para o resultado.
            buffer_length -= 8; // diminuí o comprimento do buffer
            buffer &= (1 << buffer_length) - 1; //  tira os bits extraídos do buffer e deixa apenas os bits restantes. Define os bits extraídos como zero.
            /*
                (1 << buffer_length) - 1 cria uma mascara de bits com buffer_length bits.
                        - cria um valor que os bits são definidos como zero, com excessão do bit na posição buffer_length, que é definido como 1.
                        - depois subtrair 1  e deixa todos os bits até a posição buffer_length - 1 definidos como 1 e mantém os bits posteriores como 0.
                &= ele pega o valor atual de buffer e faz um AND bit a bit com o valor resultante da expressão acima.
             */
        }
    }

    result
}
