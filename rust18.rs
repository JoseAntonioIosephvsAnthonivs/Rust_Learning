enum Animal {
	Crocodilo,
	Serpente,
	Lagarto,
}

fn main (){
	let meu_pet: Animal = Animal::Crocodilo;
	match meu_pet{
		Animal::Crocodilo =>println!("Animal Crocodilo"),
		Animal::Serpente => println!("Animal Serpente"),
		Animal::Lagarto => println!("Animal Lagarto"),
		_ => println!("Nao conheco seu animal"),
		
	}
}
