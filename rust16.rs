fn main() {
	let minha_casa = Casa{
		largura: 6,
		comprimento: 35,
	};
	println!("A minha casa tem  {} metros quadradados", minha_casa.area());
}


impl Casa{
	fn area(&self) -> u32{
		self.largura * self.comprimento
	}
}

struct Casa{
	largura: u32,
	comprimento: u32,
}
