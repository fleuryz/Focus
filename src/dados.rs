extern crate time;

use variavel::Variavel;
use sessao::Sessao;

use std::fs::File;
use std::io::prelude::*;
use std::cmp::Ordering;

#[derive(Debug)]
#[derive(Eq)]
pub struct Dado {
    pub data: time::Tm,
    pub variavel: String,
    pub valor: Variavel,
}

impl Ord for Dado {
    fn cmp(&self, other: &Dado) -> Ordering {
        self.data.cmp(&other.data)
    }
}

impl PartialOrd for Dado {
    fn partial_cmp(&self, other: &Dado) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Dado {
    fn eq(&self, other: &Dado) -> bool {
        self.data == other.data
    }
}

impl Dado {
    pub fn new(data: time::Tm, variavel: String, valor: Variavel) -> Dado {
        Dado{
            data,
            variavel,
            valor,
        }
    }

    pub fn get_dados(linha: &str) -> Dado{
        let valores:Vec<_> = linha.split('-').collect();
        let valor_variavel = match valores[2].parse(){
            Ok(valor) => valor,
            Err(erro) => panic!(erro),
        };
        Dado::new(Sessao::to_tm(valores[0]), String::from(valores[1]), valor_variavel)
        
    }

    pub fn escrever(&self, arquivo:&mut File){
        Sessao::escrever_data(arquivo, self.data);
        arquivo.write(b"-").unwrap();
        arquivo.write(self.variavel.as_bytes()).unwrap();
        arquivo.write(b"-").unwrap();
        self.valor.escrever(arquivo);
        arquivo.write(b"\n").unwrap();
    }

    pub fn copiar(&self) -> Dado {
        Dado{
            data: self.data,
            variavel: format!("{}",self.variavel),
            valor: match self.valor {
                Variavel::Int(ref valor) => Variavel::Int(*valor),
                Variavel::Float(ref valor) => Variavel::Float(*valor),
                Variavel::Booleano(ref valor) => Variavel::Booleano(*valor),
                Variavel::Texto(ref valor) => Variavel::Texto(format!("{}",valor)),
            },
        }
    }

}

#[derive(Debug)]
pub enum TipoDados {
    DadosProcessados (Vec<Dado>),
    DadosBrutos (Vec<Dado>),
}

impl TipoDados{
    pub fn escrever(&self, arquivo:&mut File){
        match self{
            &TipoDados::DadosProcessados(ref dados) => {
                arquivo.write(b"Processado\n").unwrap();
                for dado in dados.iter(){
                    dado.escrever(arquivo);
                }
            },

            &TipoDados::DadosBrutos(ref dados) => {
                arquivo.write(b"Bruto\n").unwrap();
                for dado in dados.iter(){
                    dado.escrever(arquivo);
                }
            },
        }
    }

	pub fn nomes_variaveis(&self)-> Vec<String>{
		let mut nomes:Vec<String> = Vec::new();
		match self{
            &TipoDados::DadosProcessados(ref dados) => {
                for dado in dados.iter(){
					if !nomes.contains(&dado.variavel) {
						nomes.push(dado.variavel.clone());					
					}                    
                }
            },

            &TipoDados::DadosBrutos(ref dados) => {
                for dado in dados.iter(){
					if !nomes.contains(&dado.variavel) {
						nomes.push(dado.variavel.clone());					
					}                    
                }
            },
        }
		nomes
	}

}