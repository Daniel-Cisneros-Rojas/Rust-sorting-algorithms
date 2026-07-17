use rand::Rng;
fn main() {
    println!("Algoritmos de ordenamiento");
    let mut numeros = numeros_aleatorios(20);
    println!("Números generados aleatoriamente : {:?}", numeros);
    //let mut num=vec![5, 3, 8, 4, 2];
    bubble_sort(&mut numeros);
    println!("{:?}", numeros);

}

fn numeros_aleatorios(tam:i32)-> Vec<i32> {
   let mut rng = rand::thread_rng();
   let mut numeros: Vec<i32> = Vec::new();
   for _ in 0..tam {
       let numero = rng.gen_range(1..=100);
       numeros.push(numero);
   }
   return numeros;
}

fn bubble_sort<T: Ord + std::fmt::Debug>(datos: &mut[T]){
   let tam= datos.len();
    for i in 0..tam-1{
        for j in 0..tam-1-i{
            if datos[j]>datos[j+1]{
                datos.swap(j,j+1);
            }
            println!("{:?} vuelta {}",datos, i)
        }

    }
}

