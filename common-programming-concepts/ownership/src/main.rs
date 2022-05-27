fn main() {

    /*
        Memoria heap y memoria stack. La memoria stack es la memoria mas rapida y eficiente. Es mas rapida para guardar datos y mas rapida para buscarlos. Esto es porque la memoria stack guarda datos cuyo peso es fijo, por lo tanto, ya sabe donde va a ubicar el dato pues se sabe su valor de antemano. La memoria heap se encarga de guardar todos los datos cuyo valor no es fijo. Por lo tanto al guardar un dato debe buscar un lugar libre y disponible. Y para buscarlo tiene que guardar una referencia a ese lugar para encontrarlo. El ownership viene a hacer que el uso de la memoria heap sea lo mas eficiente posible.


        Ownership sigue tres reglas:

        -cada valor tiene una variable que es su dueña.
        -solo puede haber una variable dueña al mismo tiempo.
        -cuando la variable dueña sale de scope el valor se pierde.


        En la memoria stack se guardan los valores "primitivos", aquellos cuyo valor de memoria se conoce de antemano. Los otros tipos de datos como los String se guardan en la memoria heap pues su contenido puede cambiar, su peso es dinamico.
    
    */

    // Rust declara los valores por referencia en los valores no primitivos, los que se guardan en la memoria heap. En vez de crear una alocacion de memoria nueva lo que se hace es guardar la referencia de memoria anterior, el que tenia la variable s. Pero en vez de dejar el accesso a esa referencia tanto en s como en s_new lo que se se hace es MOVER (Move trait) la referencia a la nueva variables (sNew). Si trato de acceder o usar s voy a tener un error de compilacion. En los valores primitivos (memoria stack) se crea una copia profunda (Copy trait) por que no es tan costoso hacerlo y Rust lo hace.

    let s = String::from("Hello mai friend");
    let s_new = s;

    println!("Esto es legal y correcto, el ownership se movio aquì {}", s_new);
    //println!("Esto no es legal ni correcto :(, esta variable ya no es dueña {}", s);

    //Lo mismo pasa si paso esa variable como argumento a una función.
    
    let v = String::from("Hello mai friend");
    print_s(v);

    //println!("Esto falla tambien debido a que se cedió el ownership a la función", v);


    //RECORDAR LA PRIMER REGLA, SOLO UN DUEÑO.


    //Respecto a la tercera regla:
    {
        let out_of_scope = "testing";
        println!("Dentro del scope esta todo bien, responde a la tercera regla, sigue vivo el ownership {}", out_of_scope)
    }
    //println!("Aqui ya se perdio el ownership, error de compilacion {}", out_of_scope)

    /*
    Entonces, los valores que implementan el trait Copy crean una copia del valor EN LA MEMORIA STACK, y por lo tanto no entran en las reglas del ownership. EN cambio los valores que implementan el trait Move no crea una copia profundo, ni comparten el valor por referencia, sino que se MUEVE el valor por referencia en la memoria HEAP y la nueva variable o funcion es la nueva dueña del valor.x1º
    */

    //para hacer una copia profunda de un trait Move se puede usar el metodo clone.

    let original = "hola";
    let copia_original = original.clone();
    println!("Podemos imprimir los dos: el original {} y la copia {}", original, copia_original);

    // PRESTAME Y REFERENCIA

    /*
    
    Como puedo pasar el valor de una variable sin que esa variable pierda el ownership? Hacer una copìa profunda no es algo deseable debido a que es algo sumamente costosoa nivel de rendimiento. Para estos casos lo que implementa Rust son los prestamos. Los prestamos son puntero que nos permiten acceder al valor de la variable en heap sin crear una nueva ownership.
    
    */

        let s1 = String::from("hello");
    
        let len = calculate_length(&s1);
    
        println!("The length of '{}' is {}.", s1, len);

    // Ahora al igual que las variables las referencias son inmutables. No se puede dar un nuevo valor a un referencia.
    
    let s5 = String::from("hello");

    //change(&s5); ESto va a tirar error


    // REFERENCIAS MUTABLES

    /*
    
    Las referencias mutables se declaran con &mut, es importante hacer notar que la variable original debe ser mutable tambien. Una restriccion grande es que una variable dueña SOLO PUEDE TENER MAXIMO UNA REFERENCIA MUTABLE AL MISMO TIEMPO mientras que puede tener un numero ilimitado de referencias inmutables. 
    
    */

    let mut s6 = String::from("hello");

    change(&mut s6);

    // A su vez no puedo tener referencias mutables e inmutables conviviendo puesto que una referencia mutable podria estar cambiando el valor de una inmutable, lo que es indeseable. Puedo usar una referencia mutable siempre que las referencias inmutables esten en otro scope o hayan dejado de estar utilizadas.

    let mut s7 = String::from("hello");

    let r1 = &s7; // no problem
    let r2 = &s7; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s7; // no problem
    println!("{}", r3);

    //Dangling references: este tipo de referencias son aquellas referencias que se hacen de una variable dueña que ha dejado de tener efecto. Por lo tanto se hace referencia de algo que ya no existe. Rust nos avisara en tiempo de compilacion de este error.

    fn dangle() -> &String { // dangle returns a reference to a String

        let s = String::from("hello"); // s is a new String
    
        &s // we return a reference to the String, s
    } // Here, s goes out of scope, and is dropped. Its memory goes away.
      // Danger!
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}



fn print_s(s: String){
    println!("S es {}", s);    
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

