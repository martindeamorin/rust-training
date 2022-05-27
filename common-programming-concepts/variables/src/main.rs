fn main() {
    /* let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x); 
    
    Rust por defecto hace que sus variables sean inmutables. Lo que quiere decir que su valor no puede cambiar. ESte codigo no va a funcionar.
    */


    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x); 

    //Para resolver esto y avisarle al compilador que el valor de la variable puede ser reasignado tenemos que agregarle la keyword mut adelante del nombre de la variable

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    //Las constantes como las variables tambien son inmutables por defecto. La diferencia es que las constantes no se pueden convertir en mutables. Por otra parte, es necesario que las constantes tengan su tipo de dato explicitamente anotado. Por ultimo, el valor de una constante no puede ser el resultado de una operacion de una variable normal.

    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of x in the inner scope is: {}", y);
    }

    println!("The value of x is: {}", y);



}
