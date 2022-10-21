fn main() {
    /*  Las variables se declaran de la siguiente manera:
        nombre:tipo = valor;    */
    
    //Las variables pueden se de los siguientes tipos.

    //Booleana
    let logical: bool = true;

    /*Flotante y Entero
    variable:tamaño del apuntador
    Declaración regular*/
    let regular_float: f64 = 1.0; //f32, f64
    let regular_integer: i32 = 5; //i8, i16, i32, i64, i128 and isize

    //Declaracion por Sujifo
    let suffix_float = 1.0f64; 
    let suffix_integer = 5i32; 

    // Valores por defecto para flotante y entero (f64 y i32)
    let default_float   = 3.0; // f64
    let default_integer = 7;   // i32
    
    // El valor de una variable mutable se puede modificar
    let mut mutable = 12; // Mutable por defecto i32 (Entero)
    mutable = 21;
    
    // ¡ERROR!, el tipo de variable no puede cambiar
    mutable = true;
    
    // Shadowing permite sobreescribir las variables.
    let mutable = true;
}