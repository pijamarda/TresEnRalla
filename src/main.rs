use std::io;

fn main() 
{
	//el tablero es un array de enteros
	//0 es vacio
	//1 es cruz
	//2 es circulo
	let mut tablero: [i32; 9] = 
		[0,0,0,
		 0,0,0,
		 0,0,0];
    
    print_tablero(&tablero);

    let mut fin: bool = false;
    let mut jugador1_turno: bool = true;
    while !fin
    {
    	let mut opcion = 1;
    	//hay que poner aqui la declaracion de la variable porque estamos haciendo
    	//mas abajo una reasignacion de tipo, de manera que si lo declaramos fuera
    	//al utilizar trim().parse() da un error
    	let mut posicion = String::new();
    	
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

    	//Convertimos a un entero el numero que se haya puesto
    	//si no puede convertirlo da un error y sale del programa
    	//TODO: Ver en la guia como hacer que el programa continue
    	let posicion: i32 = posicion.trim().parse()
    		.ok()
    		.expect("Debes introducir un numero, linea 36");

    	//Despues comprobamos que el numero introducido esta dentro de las posiciones validas
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

    		//Miramos a ver si la posicion donde queremos colocarla es un 0
	    	if !calc_posicion_valida(posicion, &tablero)
	    	{
	    		println!("La posicion elegida ya ha sido ocupada");
	    	}
	    	//Si no es asi, en principio ha pasado todas las validaciones y modificamos
	    	//el Array que es el tablero y cambiamos el turno
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

    	//Recorremos el Array para ver si hay algun cero, en cuanto encuentra 1 
    	// el fin se pone a false, de manera que si no hay ningun cero, es que ya
    	//tenemos el tablero completo
    	for x in tablero.iter()
    	{	    		
    		if *x == 0
    		{
    			fin = false;
    		}
    	}    	

    	//Comprobamos a ver si hay alguna ralla ya completada
    	if check_ralla(&tablero)
    	{
    		if !jugador1_turno
    		{
    			println!("Enhorabuena Jugador 1, has ganado");
    		}
    		else 
    		{
    		 	println!("Enhorabuena Jugador 2, has ganado");   
    		}
    		fin = true;
    	}
    	//Si no hay ninguna ralla comprobamos si acaba el programa porque
    	//esten todos los huecos ocupados
    	else if fin
    	{
    		println!("Tablas!");
    	}    	
    }
}

//ASCII del tablero
//TODO: Aqui es donde podriamos sacar en modo grafico el estado del tablero
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

//Rust no tiene la operacion resto, asi que creamos una que he visto en internet
//o si la tiene en alguna libreria no la he visto
fn calc_resto(x: i32, y: i32) -> i32
{
	let resultado: i32 = ((x % y) + y) % y;
	return resultado;
}

//Le pasamos el tablero y la posicion para validar que haya un hueco
fn calc_posicion_valida(posicion: i32, t: &[i32]) -> bool
{
	let mut resultado = false;	
	//Es decir que si la posicion esta vacia
	//restamos 1, puesto que el usuario utiliza valores entre 1..9
	//pero el array es desde 0..8
	if t[(posicion-1) as usize] == 0
	{
		resultado = true;
	}
	return resultado
}


fn modificar_tablero(opcion: i32, posicion: i32,t: &mut [i32])
{
	//Igual que antes restamos 1 porque el usuario trata con numero del 1..9
	t[(posicion-1) as usize] = opcion;
}

fn check_ralla(t: &[i32]) -> bool
{
	let mut resultado = false;
	//vamos a comprobar que existe alguna linea

	//Empezamos desde la esquina arriba izquierda
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

	//arriba centro
	if t[1] != 0
	{
		if t[1] == t[4] && t[4] == t[7]
		{
			resultado = true
		}		
	}

	//izquierda centro
	if t[3] != 0
	{
		if t[3] == t[4] && t[4] == t[5]
		{
			resultado = true
		}		
	}

	//esquina arriba derecha
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
