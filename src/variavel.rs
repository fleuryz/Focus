use std::fs::File;
use std::io;
use std::string::ParseError;
use std::str::FromStr;
use std::io::prelude::*;

#[derive(Debug)]
pub enum Variavel{
    Int(i32),
    Float(f64),
    Booleano(bool),
    Texto(String),
}

impl PartialEq for Variavel {
    fn eq(&self, other: &Variavel) -> bool {
        match self {
            &Variavel::Int(ref valor) => {
                if let &Variavel::Int(ref valor2) = other {
                    return valor == valor2
                }
            },
            &Variavel::Float(_) => {
                if let &Variavel::Float(_) = other {
                    return false
                }
            },
            &Variavel::Booleano(ref valor) => {
                if let &Variavel::Booleano(ref valor2) = other {
                    return valor == valor2
                }
            },
            &Variavel::Texto(ref valor) => {
                if let &Variavel::Texto(ref valor2) = other {
                    return valor == valor2
                }
            },
        }
        false
    }
}
impl Eq for Variavel {}

impl FromStr for Variavel{
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i32>() {
            Ok(valor) => Ok(Variavel::Int(valor)),
            Err(_) => {
                match s.parse::<f64>() {
                    Ok(valor) => Ok(Variavel::Float(valor)),
                    Err(_) => {
                        match s.parse::<bool>(){
                            Ok(valor) => Ok(Variavel::Booleano(valor)),
                            Err(_) => s.parse(),
                        }
                    },
                }
            },
        }
    }
}

impl Variavel{

    fn add(nome: &str, valor: &str) -> Variavel{
        let valor_receb = valor.trim();
        match valor_receb.parse::<i32>() {
            Ok(valor) => Variavel::Int(valor),
            Err(_) => {
                match valor_receb.parse::<f64>() {
                    Ok(valor) => Variavel::Float(valor),
                    Err(_) => {
                        match valor_receb.parse::<bool>(){
                            Ok(valor) => Variavel::Booleano(valor),
                            Err(_) => Variavel::Texto(String::from(valor_receb)),
                        }
                    },
                }
            },
        }
    }

    fn add_variavel(nome: &str) -> Variavel {
        let mut valor_receb = String::new();
        println!("Digite o valor da variável {}:", nome);
        io::stdin().read_line(&mut valor_receb)
            .expect("Erro ao ler valor de variável.");
        let valor_receb = valor_receb.trim();
        match valor_receb.parse::<i32>() {
            Ok(valor) => Variavel::Int(valor),
            Err(_) => {
                match valor_receb.parse::<f64>() {
                    Ok(valor) => Variavel::Float(valor),
                    Err(_) => {
                        match valor_receb.parse::<bool>(){
                            Ok(valor) => Variavel::Booleano(valor),
                            Err(_) => Variavel::Texto(String::from(valor_receb)),
                        }
                    },
                }
            },
        }
    }
    
    pub fn get_variaveis(variaveis: &mut Vec<Variavel>, variaveis_iter: &Vec<String>) {
        for nome in variaveis_iter.iter() {
            (*variaveis).push(Variavel::add_variavel(nome))
        }
    }

    pub fn fill_variaveis(variaveis: &mut Vec<Variavel>, variaveis_iter: &Vec<String>, valores_iter: &[String]) {
        let mut valor = valores_iter.iter();
        for nome in variaveis_iter.iter() {
            (*variaveis).push(Variavel::add(nome,valor.next().unwrap()))
        }
    }

    pub fn is_bool(&self) -> bool{
        if let Variavel::Booleano(_) = *self{
            return true
        }
        false
    }

    pub fn is_int(&self) -> bool{
        if let Variavel::Int(_) = *self{
            return true
        }
        false
    }

    pub fn is_float(&self) -> bool{
        if let Variavel::Float(_) = *self{
            return true
        }
        false
    }
    
    pub fn is_texto(&self) -> bool{
        if let Variavel::Texto(_) = *self{
            return true
        }
        false
    }

    pub fn escrever(&self, mut arquivo: &File){
        match self {
            &Variavel::Int(ref valor) => {arquivo.write(format!("{}", valor).as_bytes()).unwrap();},
            &Variavel::Float(ref valor) => {arquivo.write(format!("{}", valor).as_bytes()).unwrap();},
            &Variavel::Booleano(ref valor) => {arquivo.write(format!("{}", valor).as_bytes()).unwrap();},
            &Variavel::Texto(ref valor) => {arquivo.write(format!("{}", valor).as_bytes()).unwrap();},
        }
    }

    pub fn as_string(&self) -> String{

        match self {
                &Variavel::Int(ref valor) => format!("{}", valor),
                &Variavel::Float(ref valor) => format!("{}", valor),
                &Variavel::Booleano(ref valor) => format!("{}", valor),
                &Variavel::Texto(ref valor) =>format!("{}", valor),
            }
    }
}