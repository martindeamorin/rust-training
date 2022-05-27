fn main() {

    /*
    FUNCIONES

     Las funciones se definene con el keyword fn seguido por el nombre y parentesis. El contenido de las funciones va entre corchetes. Las funciones se llaman con el nombre seguido por parentesis. Las funciones se pueden declarar antes o despues de ser llamadas, no tiene importancia.

     Las funciones pueden tener parametros queson variables que las funciones reciben. Los argumentos son los valores concretos que se le pasan a las funciones cuando se llaman. Se deben declarar el tipo de dato de los parametros.


     Los cuerpos de las funciones pueden tener declaraciones y pueden terminar en una expresion. Las declaraciones son instrucciones que realizan una accion pero que no retornan ningun resultado mientras que las expresiones si lo hacen. Las expresiones pueden ser parte de una declaracion. Las expresiones no llevan punto y coma al final.

     Las funciones que retornan un valor deben especificar el valor que retornan despues del nombre con una flecha (->)

    */
    println!("Hello, world!");

    another_function(5);

    println!("The return value of the funcion is {}", five());
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
    let y = {
        let x = 3;
        x + 1 //expresion
    }; //declaracion

    println!("The value of y is: {:?}", y);

}

fn five() -> i32 {
    5
}