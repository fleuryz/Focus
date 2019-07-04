#[derive(Debug)]
pub struct Par{
	pub nome: String,
	pub valor: String,
}

impl Par{
	pub fn new(nome: String)-> Par{
		Par{
			nome: nome,
			valor: String::from("null"),		
		}
	}

	pub fn new_vec(mut nomes: Vec<String>) -> Vec<Par>{
		let mut pares: Vec<Par> = Vec::new();
		while !nomes.is_empty(){
			pares.push(Par::new(nomes.pop().unwrap()));
		}
		pares
	}

	pub fn inserir(pares: &mut Vec<Par>, nome: &str, valor: String)-> bool{
		for par in pares.iter_mut(){
			if par.nome.eq(nome) {
				par.valor = valor;
				return true;
			}		
		}

		return false;
	}
	
	pub fn zerar(&mut self){
		self.valor = String::from("null");
	}
}
