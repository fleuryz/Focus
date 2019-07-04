use std::io;

pub enum RespostaSN {
    SIM,
    NAO,
}

impl RespostaSN {
    pub fn new(pergunta: &str) -> RespostaSN {
        let mut resposta = String::new();
        loop {
            println!("{}(S/n)", pergunta);
            io::stdin().read_line(&mut resposta)
                .expect("Erro ao ler pergunta de Sim ou Não.");
            {
                let resposta = resposta.trim();
                if resposta == "s" || resposta == "S"{
                    return RespostaSN::SIM;
                }else if resposta == "n" || resposta == "N"{
                    return RespostaSN::NAO;
                }   

                println!("Erro! A resposta deve ser S ou n. A resposta foi: ({}).", resposta);
            }
            resposta = String::from("");
        }
    }
}
