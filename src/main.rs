use rand::Rng;
fn main() {
    println!("Algoritmos de ordenamiento");
    let  numeros_originales = numeros_aleatorios(10);
    println!("Números generados aleatoriamente : {:?}", numeros_originales);
    //let mut num=vec![5, 3, 8, 4, 2];
    //num.swap(0, 0);

    let mut numeros_para_procesar=numeros_originales.clone();
    bubble_sort( &mut numeros_para_procesar);

    let mut numeros_para_procesar=numeros_originales.clone();
    selection_sort(&mut numeros_para_procesar);

    let mut numeros_para_procesar=numeros_originales.clone();
    insertion_sort(&mut numeros_para_procesar);

    println!("{:?}", numeros_para_procesar);

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
    println!("Algoritmo ordenamiento burbuja\n");
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


fn selection_sort<T: Ord + std::fmt::Debug>(datos: &mut[T]){
    println!("Algoritmo ordenamiento selección\n");
    let tam= datos.len();
    for i in 0..tam-1{
        let mut index_menor_numero=i;
        for j in i+1..tam{
            if datos[index_menor_numero]>datos[j]{
                index_menor_numero=j;
            }
        }
        datos.swap(i, index_menor_numero);
        println!("{:?} vuelta {}", datos, i);
    }
}

fn insertion_sort<T: Ord + std::fmt::Debug>(datos: &mut[T]){
    println!("Algoritmo ordenamiento inserción\n");
    let tam= datos.len();
    for i in 1..tam{
       for j in (0..i).rev(){
           println!("{:?} dato analizado {:?} comparado contra {:?}", datos, datos[j+1], datos[j]);
           if datos[j+1]<datos[j]{
            datos.swap(j+1, j);
           }else{
            break;
           }
           
       }
       
    }
}