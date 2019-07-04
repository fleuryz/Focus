extern crate focus;

use focus::teste::Teste;
use focus::respostaSN::RespostaSN;
use focus::gui::Gui;

fn main() {
    let novo = Gui::new();
    novo.run();
/*
    let mut teste = match  RespostaSN::new("Existe um arquivo de teste a ser carregado?"){
        RespostaSN::SIM => Teste::load(),
        RespostaSN::NAO => Teste::new(),
    };

    //TODO: Iniciar teste.
    teste.iniciar();
*/
}
