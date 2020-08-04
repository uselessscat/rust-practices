fn main() {
    let a: i32 = 5;
    let b: i32 = 18;

    let c: i32 = a + b;

    println!("i32 {}", c);

    // flotantes
    let d: f32 = 4.4;

    // para poder operar hay que convertir explicitamente los tipos de datos
    let e: f64 = f64::from(c) + (d as f64);

    println!("f64 {}", e);

    // booleanos
    let boolean: bool = true && false;

    println!("bool {}", boolean);

    // caracteres
    let z: char = 'z';
    // en rust los caracteres son unicode
    let gato: char = '😻';

    println!("chars {} {}", z, gato);

    // tuplas
    let mi_tupla: (i32, f32, char) = (20, 4.4, 'A');
    // acceder con <tupla>.n
    println!("tuple {} {} {}", mi_tupla.0, mi_tupla.1, mi_tupla.2);

    // destructuring
    let (x, y, z) = mi_tupla;
    println!("tuple {} {} {}", x, y, z);

    // arrays
    // se inicializa el array de strings
    let mi_array: [&str; 2] = ["hola", "mundo"];

    // se inicializa el array con cinco 0's
    let mi_array2: [i32; 5] = [0; 5];

    println!("array1 {}", mi_array[1]);
    println!("array2 {}", mi_array2[4]);
    println!("array2 len {}", mi_array2.len());
}
