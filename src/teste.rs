extern crate time;

use cenario::Cenario;
use par::Par;
use sessao::Sessao;
use dados::TipoDados;
use dados::Dado;
use sessao::TipoSessao;
use respostaSN::RespostaSN;
use variavel::Variavel;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io;

#[derive(Debug)]
pub struct Teste{
    pub nome: String,
    pub executavel: String,
    pub variaveis: Vec<String>,
    pub tipo: Tipos,
    pub sessao: u32,
}

impl Teste{
    pub fn criar_linha_comando() -> Teste{
        let mut nome = String::new();
        let mut executavel = String::new();
        let mut variaveis: Vec<String> = Vec::new();
        let mut tipo: Tipos;

        println!("Qual o nome teste?");
        io::stdin().read_line(&mut nome)
            .expect("Erro ao ler nome do arquivo a ser executado.");
        

        println!("Qual o nome do arquivo a ser executado?");
        io::stdin().read_line(&mut executavel)
            .expect("Erro ao ler nome do arquivo a ser executado.");
        
        loop{
            match RespostaSN::new("Deseja adicionar uma variável?") {
                RespostaSN::SIM => {
                    let mut nome_var = String::new();
                    println!("Qual o nome da variável?");
                    io::stdin().read_line(&mut nome_var)
                        .expect("Erro ao ler nome de variável.");
                    variaveis.push(String::from(nome_var.trim()));
                },
                RespostaSN::NAO => break,
            }
        }
        
        if variaveis.len() > 0 {
            tipo = match RespostaSN::new("Deseja criar cenários?") {
                RespostaSN::SIM => Tipos::Cenarios(Vec::new()),
                RespostaSN::NAO => Tipos::Leve,
            };
            if let Tipos::Cenarios(mut cenarios) = tipo {
                loop{
                    match Teste::criar_cenario(&mut cenarios, &variaveis) {
                        Ok(_) => break,
                        Err(erro) => match RespostaSN::new(&erro){
                            RespostaSN::SIM => continue,
                            RespostaSN::NAO => panic!("Encerrando."),
                        }
                    };
                }
                tipo = Tipos::Cenarios(cenarios)
            }
        } else {
            tipo = Tipos::Vazio;
        }
        
        let teste = Teste{
            nome: String::from(nome.trim()),
            executavel: String::from(executavel.trim()),
            variaveis,
            tipo,
            sessao: 0,
        };
        teste.escrever();
        teste
    }

    pub fn carregar(nome: &str, nome_arquivo: &str) -> Teste{
        
        let arquivo = File::open(nome_arquivo)
            .expect("Erro ao abrir arquivo de teste.");
        let mut buf_leitor = BufReader::new(arquivo);
        let mut linhas = String::new();

        buf_leitor.read_to_string(&mut linhas).unwrap();

        let mut linhas = linhas.lines();
        let executavel = linhas.next().unwrap();
        let mut variavel = linhas.next().unwrap();
        let mut variaveis: Vec<String> = Vec::new();
        let mut tipo:Tipos;

        if !variavel.eq("Vazio") {
            while !variavel.eq("Cenarios:") && !variavel.eq("Sessoes:") {
                variaveis.push(format!("{}", variavel));
                variavel = linhas.next().unwrap();
            }
            tipo = Tipos::Leve;
        }else{
            tipo = Tipos::Vazio;
            variavel = linhas.next().unwrap();
        }
        let mut cenarios: Vec<Cenario> = Vec::new();
        if variavel.eq("Cenarios:") {
            let mut nome_cenario = linhas.next().unwrap();
            let mut cenario:Cenario;
            
            while !nome_cenario.eq("Sessoes:") {
                cenario = Cenario::new(nome_cenario);
                let mut valores_variaveis:Vec<Variavel> = Vec::new();
                for i in 0..variaveis.len() {
                    valores_variaveis.push(linhas.next().unwrap().parse().unwrap());
                    
                }
                cenario.variaveis = valores_variaveis;
                cenarios.push(cenario);
                nome_cenario = linhas.next().unwrap();
            }
            tipo = Tipos::Cenarios(cenarios);
            
        }

        let teste = linhas.next().unwrap();
        let sessao = teste.parse().unwrap();
        Teste{
            nome: String::from(nome),
            executavel: String::from(executavel),
            variaveis,
            tipo,
            sessao,
        }
    }
    
    pub fn carregar_linha_comando() -> Teste{
        let mut nome = String::new();
        println!("Qual o nome do teste?");
        io::stdin().read_line(&mut nome)
                .expect("Erro ao ler nome de teste.");

        nome = format!("{}", nome.trim());
        let nome_arquivo = format!("./Testes/{}.kans", nome.trim());
        let arquivo = File::open(nome_arquivo)
            .expect("Erro ao abrir arquivo de teste.");
        let mut buf_leitor = BufReader::new(arquivo);
        let mut linhas = String::new();

        buf_leitor.read_to_string(&mut linhas).unwrap();

        let mut linhas = linhas.lines();
        let executavel = linhas.next().unwrap();
        let mut variavel = linhas.next().unwrap();
        let mut variaveis: Vec<String> = Vec::new();
        let mut tipo:Tipos;

        if !variavel.eq("Vazio") {
            while !variavel.eq("Cenarios:") && !variavel.eq("Sessoes:") {
                variaveis.push(format!("{}", variavel));
                variavel = linhas.next().unwrap();
            }
            tipo = Tipos::Leve;
        }else{
            tipo = Tipos::Vazio;
            variavel = linhas.next().unwrap();
        }
        let mut cenarios: Vec<Cenario> = Vec::new();
        if variavel.eq("Cenarios:") {
            let mut nome_cenario = linhas.next().unwrap();
            let mut cenario:Cenario;
            
            while !nome_cenario.eq("Sessoes:") {
                cenario = Cenario::new(nome_cenario);
                let mut valores_variaveis:Vec<Variavel> = Vec::new();
                for i in 0..variaveis.len() {
                    valores_variaveis.push(linhas.next().unwrap().parse().unwrap());
                    
                }
                cenario.variaveis = valores_variaveis;
                cenarios.push(cenario);
                nome_cenario = linhas.next().unwrap();
            }
            tipo = Tipos::Cenarios(cenarios);
            
        }

        let teste = linhas.next().unwrap();
        let sessao = teste.parse().unwrap();
        Teste{
            nome,
            executavel: String::from(executavel),
            variaveis,
            tipo,
            sessao,
        }
    }

    fn criar_cenario<'a>(cenarios: &mut Vec<Cenario>, variaveis: &Vec<String>) -> Result< (), String>{
        (*cenarios).clear();
        loop{
            let mut nome = String::new();
            println!("Qual o nome deste cenário?");
            io::stdin().read_line(&mut nome)
                .expect("Erro ao ler nome de cenário.");
            cenarios.push(Cenario::new(nome.trim()).add(variaveis));
            if cenarios.len() > 1{
                if let Err(erro) = cenarios[0].teste_variaveis(&cenarios[cenarios.len()-1]) {
                    return Err(format!("{} Deseja começar a criar os cenários novamente?", erro));
                }
            }
        
            match RespostaSN::new("Deseja criar outro cenário?"){
                RespostaSN::SIM => continue,
                RespostaSN::NAO => break,
            };
        }
        Ok( () )
    }

    pub fn iniciar_linha_comando(&mut self) {
        let mut sessoes_novas:Vec<Sessao> = Vec::new(); 
        loop{
            match RespostaSN::new("Deseja iniciar uma sessao?"){
                RespostaSN::SIM => {
                    self.sessao += 1;
                    let tipo = match self.tipo {
                        Tipos::Cenarios(_) => Some(TipoSessao::Cenario(String::new())),
                        Tipos::Leve => Some(TipoSessao::Leve(Vec::new())),
                        Tipos::Vazio => None,
                    };
                    let mut sessao = Sessao::new(&self.nome, self.sessao, tipo);
                    
                    sessao.iniciar(self, None);
                    sessoes_novas.push(sessao);
                },
                RespostaSN::NAO => break,
            }
        }
        let mut sessoes = self.ler_sessoes();
        let mut arquivo = self.escrever();
        
        sessoes.append(&mut sessoes_novas);	

        match RespostaSN::new("Deseja processar as sessoes agora?"){
            RespostaSN::SIM => {
                for cada_sessao in sessoes.iter_mut(){
                    cada_sessao.processar_video();
                }    
            },
            RespostaSN::NAO => {  
            },
        }
        
        match RespostaSN::new("Deseja exportar as sessoes agora?(Isso pode levar alguns minutos)"){
            RespostaSN::SIM => {
                let mut resposta = String::new();
                println!("Que tipo de saida deseja?\n (1) Tabela de todas as sessoes\n (2) Grafico de uma sessao\n (3) Video com Grafico de uma sessao");
                io::stdin().read_line(&mut resposta);
                let resposta = resposta.trim();
                if resposta == "1"{
                    for cada_sessao in sessoes.iter_mut(){
                        cada_sessao.processar_video();
                        cada_sessao.escrever(&mut arquivo);
                    }   
                    if sessoes.len() > 0{
	                    self.exportar(&sessoes);  
				    }
                } else if resposta == "2"{
                    let mut resposta = String::new();
                    println!("Qual sessao deseja exportar?");
                    io::stdin().read_line(&mut resposta);
                    resposta.pop();
                    let indice:usize = resposta.parse().unwrap();
                    for cada_sessao in sessoes.iter(){
                        cada_sessao.escrever(&mut arquivo);
                    }  
                    drop(arquivo);
                    sessoes[indice-1].exportar_grafico(0xFF, Vec::new());
                    return;
                } else if resposta == "3"{
                    let mut resposta = String::new();
                    println!("Qual sessao deseja exportar?");
                    io::stdin().read_line(&mut resposta);
                    resposta.pop();
                    let indice:usize = resposta.parse().unwrap();
                    sessoes[indice-1].exportar_video();
                }
	        },
            RespostaSN::NAO => (),
        }

        for cada_sessao in sessoes.iter(){
            cada_sessao.escrever(&mut arquivo);
        }    
        
    }

    pub fn escrever(&self)-> File {
        let nome = format!("./Testes/{}.kans", self.nome);
        let mut arquivo = File::create(nome)
            .expect("Erro ao criar arquivo.");
        
        arquivo.write(self.executavel.as_bytes()).unwrap();
        arquivo.write(b"\n").unwrap();
        for variavel in self.variaveis.iter(){
            arquivo.write(variavel.as_bytes()).unwrap();
            arquivo.write(b"\n").unwrap();
        }

        match self.tipo {
            Tipos::Vazio => {arquivo.write(b"Vazio\n").expect("Falha ao escrever.");},
            Tipos::Leve => (),
            Tipos::Cenarios(ref cenarios) => {
                arquivo.write(b"Cenarios:\n").expect("Falha ao escrever.");
                for cenario in cenarios {
                    cenario.escrever(&arquivo);
                }
            },
        }
        arquivo.write(b"Sessoes:\n").unwrap();
        arquivo.write(format!("{}\n.", self.sessao).as_bytes()).unwrap();

        arquivo
    }

    pub fn ler_sessoes(&self) -> Vec<Sessao>{
        let mut sessoes:Vec<Sessao> = Vec::new();
        let nome_arquivo = format!("./Testes/{}.kans", self.nome);
        let arquivo = File::open(nome_arquivo)
            .expect("Erro ao abrir arquivo de teste.");
        let mut buf_leitor = BufReader::new(arquivo);
        let mut linhas = String::new();

        buf_leitor.read_to_string(&mut linhas).unwrap();

        let mut linhas = linhas.lines();
        let mut controle = linhas.next().unwrap();
        while !controle.eq(".Sessao:") {
            controle = linhas.next().unwrap();
            if controle.eq("."){
                return Vec::new();
            }
        }
        while !controle.eq(".") {
            let num_sessao = linhas.next().unwrap();
            let tipo = linhas.next().unwrap();
            let data_inicio:time::Tm;
            let mut tipo_sessao:Option<TipoSessao> = None;
            if tipo.eq("Leve:") {
                let mut variaveis:Vec<String> = Vec::new();
                for i in 0 .. self.variaveis.len() {
                    variaveis.push(String::from(linhas.next().unwrap()));
                }
                tipo_sessao = Some(TipoSessao::Leve(variaveis));
                data_inicio = Sessao::to_tm(linhas.next().unwrap());
            }else if tipo.eq("Cenario:"){
                tipo_sessao = Some(TipoSessao::Cenario(String::from(linhas.next().unwrap())));
                data_inicio = Sessao::to_tm(linhas.next().unwrap());
            }else {
                data_inicio = Sessao::to_tm(tipo);
                let tipo_sessao:Option<TipoSessao> = None;
            }
            let mut sessao = Sessao::new(&self.nome, num_sessao.parse().unwrap(), tipo_sessao);
            sessao.data_inicio = data_inicio;
            sessao.data_conclusao = Sessao::to_tm(linhas.next().unwrap());
            let nome_video = linhas.next().unwrap();
            let tipo_dados = linhas.next().unwrap();
            let mut dados:Vec<Dado> = Vec::new();
            controle = linhas.next().unwrap();
            while !controle.eq(".") && !controle.eq(".Sessao:") {
                dados.push(Dado::get_dados(controle));
                controle = linhas.next().unwrap();
            }
            sessao.arquivo_video = String::from(nome_video);
            
            let dado:TipoDados;
            if tipo_dados.contains("Bruto"){
                dado = TipoDados::DadosBrutos(dados);
            }else{
                dado = TipoDados::DadosProcessados(dados);
            }
            sessao.dados = dado;
            sessoes.push(sessao);
        }
        sessoes
    }

    pub fn exportar(&self, sessoes: &Vec<Sessao>) {
    
        let nome = format!("./Saida/Tabelas/{}.csv", self.nome);
        let mut arquivo = File::create(nome)
            .expect("Erro ao criar arquivo.");
		let mut variaveis = sessoes[0].dados.nomes_variaveis();
		variaveis.push(String::from("Sessao"));
		let mut num_sessao = 1;
		let mut pares = Par::new_vec(variaveis);

		arquivo.write("Tempo".as_bytes()).unwrap();
		for par in pares.iter_mut(){
			arquivo.write(",".as_bytes()).unwrap();
			arquivo.write(par.nome.as_bytes()).unwrap();
		}
		arquivo.write("\n".as_bytes()).unwrap();

		for sessao in sessoes.iter(){
		    if let TipoDados::DadosProcessados(ref dados) = sessao.dados {
				let mut tempo_anterior:String = Sessao::data_string(dados[0].data);
		        for dado in dados.iter(){
					let tempo = Sessao::data_string(dado.data);
					if tempo.eq(&tempo_anterior) {
						Par::inserir(&mut pares, &dado.variavel, dado.valor.as_string());
					}else {
						Par::inserir(&mut pares, "Sessao", format!("{}", num_sessao));
						arquivo.write(tempo_anterior.as_bytes()).unwrap();
						for par in pares.iter_mut(){
							arquivo.write(",".as_bytes()).unwrap();
							arquivo.write(par.valor.as_bytes()).unwrap();
							par.zerar();
						}
						arquivo.write("\n".as_bytes()).unwrap();
						tempo_anterior = tempo;
						Par::inserir(&mut pares, &dado.variavel, dado.valor.as_string());
					}
		        }
		    }
			num_sessao += 1;
        }
        
        //sessoes
    
    }

}

#[derive(Debug)]
pub enum Tipos{
    Cenarios(Vec<Cenario>),
    Leve,
    Vazio,
}
