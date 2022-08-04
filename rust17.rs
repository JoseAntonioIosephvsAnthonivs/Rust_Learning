fn main() {
	let fruta: &str = "goiaba";
	match fruta {
		"manga" => println!("Fruta manga"),
		"banana" => println!("Fruta banana "),
		"goiaba" => println!("Fruta goiaba"),
		_ => println!("Nao conheco sua fruta"),
	}
}
