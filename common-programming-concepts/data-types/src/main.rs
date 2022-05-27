fn main() {
    /* Todos los valores de Rust son de un tipo de dato determinado. Rust necesita saber el tipo de dato de todas sus variables en tiempo de compilación. En la mayoria de los casos Rust puede deducir los tipos de datos, pero hay casoss en lo que tenemos que especificarselo, por ejemplo, cuando el tipo de dato del valor que devuelve una función puede ser diverso */

    let guess: u32 = "42".parse().expect("Not a number!");

    /*
        En Rust hay dos tipos de datos: scalar y compound.
    */

    //SCALAR
    /*
    
    Los scalar represetan un valor único. Pueden ser:

    
        INTEGER

        Length	Signed	Unsigned
        8-bit	i8	    u8
        16-bit	i16	    u16
        32-bit	i32	    u32
        64-bit	i64	    u64
        128-bit	i128	u128
        arch	isize	usize

        Un integer es un valor numerico sin fracción. Signed quiere decir que puede ser negativo y unsigned quiere decir que debe ser positivo. Cada signed puede almacenar -(2^n-1) a (2^n-1) - 1  inclusive, donde n es el numero de bits que puede almacenar. CAda unsigned puede almacenar 0 a 2^n-1. Arch depende de la arquitectura del sistema operativo.

        Los valores validos como numeros pueden ser:

        Number literals	Example
        Decimal	        98_222
        Hex	            0xff
        Octal	        0o77
        Binary	        0b1111_0000
        Byte (u8 only)	b'A'

        Floating point values

        Valores numericos fraccionarios. Son todos signed.

        Booolean

        Dos valores posibles, true y false. Ocupan un solo byte. El tipo es bool

        Character Type

        Los valores char se escriben con comillas simples, mientras que los string con comillas dobles. Ocupa cuatrp bytes y representa un valor unicode único.

    */

    /*
        COMPUND TYPES

        Los valores compound puede agrupar multiples valores en un grupo. Hay dos valores compound primiitvos: tuplas y arrays. Si la tupla no tiene ningun valor () entonces devuelve un tipo unit.

        TUPLE
        Las tuplas pueden agrupar diferentes tipos de valores. Tienen una longitud deteminada. Podemos acceder a los valores con .index

    */

    let x: (i64, char, f64) = (30000, 'a', 3.60);

    println!("The value of the first element is {}", x.0);


    //Las tuplas se pueden desetructurar.

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    /*
    
    ARRAY

    Los arrays permiten agrupar elementos con el mismo tipo de datos. Los arrays tienen una longitud fija. 
    
    */

    //declarar un array

    let a = [1, 2, 3, 4, 5];

    //declaracion explicita

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    //declaracion con elementos repetidos

    let a = [3; 5];



    //acceder datos de un array

    let first = a[0];
    let second = a[1];


}
