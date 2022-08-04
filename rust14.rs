fn quadrado(x:u32) -> u32{
	return x*x;
}

fn main (){
	let num :u32 = 3;
	println!("o quadrado do numero {} se refere a {}",num, quadrado(num));
}
