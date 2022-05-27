fn main() {

    /*
    
        IF
    
    */
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    //if para expresion

    //let number = if condition { 5 } else { 6 };

    // LOOPS

    /*
    
    TRES LOOPS: 

    LOOP, WHILE Y FOR.

    Loop no lleva ninguna condicion. Para salir de un loop podemos utilizar la keyword break. Para saltar a la proxima iteracion podemos utilziar la keyword continue. Tambien se pueden usar etiquetas para volver a un punto determinado donde esa etiqueta se declaro. 
    
    */

    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);



    // los loops pueden retornar un valor poniendo luego de la keywoard break la expresion que se quiere devolver.

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // WHILE - loops condicionales

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    //FOR - Iterar sobre colecciones. Los for pueden hacerse sobre una coleccion o sobre un rango.

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

}