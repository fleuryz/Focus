use variavel::Variavel;

use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Cenario{
    pub nome: String,
    pub variaveis: Vec<Variavel>,
}

impl Cenario{
    pub fn new(nome: &str) -> Cenario {
        Cenario{
            nome: String::from(nome),
            variaveis: Vec::new(),
        }
    }

    pub fn add(mut self, variaveis_iter: &Vec<String>) -> Cenario {
        Variavel::get_variaveis(&mut self.variaveis, variaveis_iter);
        self
    }

    pub fn fill(mut self, variaveis_iter: &Vec<String>, valores_iter: &[String]) -> Cenario {
        Variavel::fill_variaveis(&mut self.variaveis, variaveis_iter, valores_iter);
        self
    }

    pub fn teste_variaveis(&self, outro_cenario: &Cenario) -> Result< (), String >{
        if self.nome.eq(&outro_cenario.nome) {
            return Err(String::from("Cenários tem o mesmo nome!"));
        }

        let mut variavel2_iter = outro_cenario.variaveis.iter();
        for variavel in self.variaveis.iter() {
            if let Some(variavel2) = variavel2_iter.next(){
                let erro = String::from("Cenarios tem tipos de variáveis diferentes!");
                match variavel {
                    &Variavel::Int(_) => if !(*variavel2).is_int() {return Err(erro); },
                    &Variavel::Float(_) => if !(*variavel2).is_float() {return Err(erro); },
                    &Variavel::Booleano(_) => if !(*variavel2).is_bool() {return Err(erro);},
                    &Variavel::Texto(_) => if !(*variavel2).is_texto() {return Err(erro);},
                }    
            }
        }
        Ok( () )
    }

    pub fn escrever(&self, mut arquivo: &File){

        arquivo.write(self.nome.as_bytes()).unwrap();
        arquivo.write(b"\n").unwrap();
        for valores in self.variaveis.iter(){
            valores.escrever(arquivo);
            arquivo.write(b"\n").unwrap();
        }
    }
}