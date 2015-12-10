use std::io;

fn main() 
{
	//el tablero es un array de enteros
	//0 es vacio
	//1 es cruz
	//2 es circulo
	let mut tablero: [i32; 9] = [0,0,0,
							 0,0,0,
							 0,0,0];
    
    print_tablero(&tablero);

    let mut fin: bool = false;
    let mut jugador1_turno: bool = true;
    while !fin
    {
    	let mut opcion = 1;
    	let mut posicion = String::new();
    	/*println!("Jugador 1, elige tu movimiento: 1 o 2");
    	io::stdin().read_line(&mut opcion)
    		.ok()
    		.expect("Error al leer la linea");

    	let opcion: i32 = opcion.trim().parse()
    		.ok()
    		.expect("Debes introducir un numero, linea 27");
		*/
		if jugador1_turno
		{
			println!("Jugador 1, elige la posicion: 1 al 9");	
		}
		else 
		{
		    println!("Jugador 2, elige la posicion: 1 al 9");
		}
    	
    	io::stdin().read_line(&mut posicion)
    		.ok()
    		.expect("Error al leer la linea");

    	let posicion: i32 = posicion.trim().parse()
    		.ok()
    		.expect("Debes introducir un numero, linea 36");


    	if posicion >= 1 || posicion <= 9
    	{    		
    		if jugador1_turno
    		{
    			opcion = 1;
    		}
    		else
    		{
    			opcion = 2;
    		}

	    	if !calc_posicion_valida(posicion, &tablero)
	    	{
	    		println!("La posicion elegida ya ha sido ocupada");
	    	}
	    	else
	    	{
	    		modificar_tablero(opcion, posicion, &mut tablero);	    		
	    		jugador1_turno = !jugador1_turno;
	    	}
    	}
    	else 
    	{
    		println!("La posicion debe ser entre 1 y 9");
    	}
    	print_tablero(&tablero);
    	fin = true;
    	for x in tablero.iter()
    	{	    		
    		if *x == 0
    		{
    			fin = false;
    		}
    	}    	
    	fin = fin || check_ralla(&tablero);

    	if fin
    	{
    		if !jugador1_turno
    		{
    			println!("Enhorabuena Jugador 1, has ganado");
    		}
    		else 
    		{
    		 	println!("Enhorabuena Jugador 2, has ganado");   
    		}
    	}
    }
}

fn print_tablero(t: &[i32])
{	
	let mut i: i32 = 0;
	for x in t.iter()
	{
		if calc_resto(i,3) == 0
		{
			println!("");
		}
		print!("{:?}",x);
		i = i+1;
	}
	println!("");
}

//Rust no tiene la operacion resto, asi que creamos una
fn calc_resto(x: i32, y: i32) -> i32
{
	let resultado: i32 = ((x % y) + y) % y;
	return resultado;
}

fn calc_posicion_valida(posicion: i32, t: &[i32]) -> bool
{
	let mut resultado = false;	
	//Es decir que si la posicion esta vacia
	if t[(posicion-1) as usize] == 0
	{
		resultado = true;
	}
	return resultado
}

fn modificar_tablero(opcion: i32, posicion: i32,t: &mut [i32])
{
	t[(posicion-1) as usize] = opcion;
}

fn check_ralla(t: &[i32]) -> bool
{
	let mut resultado = false;
	//vamos a comprobar que existe alguna linea
	if t[0] != 0
	{
		if t[0] == t[1] && t[1] == t[2]
		{
			resultado = true
		}
		if t[0] == t[3] && t[3] == t[6]
		{
			resultado = true
		}
		if t[0] == t[4] && t[4] == t[8]
		{
			resultado = true
		}
	}

	if t[1] != 0
	{
		if t[1] == t[4] && t[4] == t[7]
		{
			resultado = true
		}		
	}

	if t[3] != 0
	{
		if t[3] == t[4] && t[4] == t[5]
		{
			resultado = true
		}		
	}

	if t[2] != 0
	{
		if t[2] == t[5] && t[5] == t[8]
		{
			resultado = true
		}
		if t[2] == t[4] && t[4] == t[6]
		{
			resultado = true
		}	
	}
	
	return resultado;
}
