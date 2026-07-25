use rand::{Rng};
fn main() {
    println!("Algoritmos de ordenamiento");
    let  numeros_originales = numeros_aleatorios(8);
    println!("Números generados aleatoriamente : {:?}", numeros_originales);
    //let mut num=vec![5, 3, 8, 4, 2];
    //num.swap(0, 0);

    let mut numeros_para_procesar=numeros_originales.clone();
    bubble_sort( &mut numeros_para_procesar);

    let mut numeros_para_procesar=numeros_originales.clone();
    selection_sort(&mut numeros_para_procesar);

    let mut numeros_para_procesar=numeros_originales.clone();
    insertion_sort(&mut numeros_para_procesar);

    let mut numeros_para_procesar=numeros_originales.clone();
    println!("Algoritmo ordenamiento por mezcla (Merge sort)\n");
    merge_sort(&mut numeros_para_procesar);

    let mut numeros_para_procesar=numeros_originales.clone();
    
    println!("Algoritmo ordenamiento Quick sort\n");
    quick_sort(&mut numeros_para_procesar);
    println!("Datos ordenados {:?}", numeros_para_procesar);

    let mut numeros_para_procesar=numeros_originales.clone();
    heap_sort(&mut numeros_para_procesar);

    let mut numeros_para_procesar=numeros_originales.clone();
    counting_sort(&mut numeros_para_procesar);

    let mut numeros_para_procesar=numeros_originales.clone();
    radix_sort(&mut numeros_para_procesar);

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

fn merge_sort<T: Ord + std::fmt::Debug+ std::clone::Clone>(datos: &mut[T]){
    
    let tam=datos.len();
    
    if tam<=1{
        return;
    }
        let mitad=tam/2;
        
        merge_sort(&mut datos[..mitad]);
        merge_sort(&mut datos[mitad..]);
        
        merge(datos,mitad);

}

fn merge<T: Ord + std::fmt::Debug+ std::clone::Clone>(datos: &mut[T],mitad:usize){
    let (mut i,mut j,mut k)=(0,0,0);
    let izquierda=datos[..mitad].to_vec();
    let derecha=datos[mitad..].to_vec();
    println!("datos a ordenar {:?}", datos);
    while i<izquierda.len() && j<derecha.len()
    {
        if izquierda[i]<=derecha[j]
        {
            datos[k]=izquierda[i].clone();
            i+=1;
        }else{
            datos[k]=derecha[j].clone();
            j+=1;
        }
        k+=1;
    }

    while i<izquierda.len(){
        datos[k]=izquierda[i].clone();
        i+=1;
        k+=1;
    }
    while j<derecha.len(){
        datos[k]=derecha[j].clone();
        j+=1;
        k+=1;
    }

    println!("datos ordenados {:?}", datos);
}

fn quick_sort<T: Ord+ std::fmt::Debug>(datos: &mut [T]) {
    let len = datos.len();
    if len <= 1 {
        return;
    }

    let pivote_index = partition(datos);

    let (left, right) = datos.split_at_mut(pivote_index);
    
 
    quick_sort(left);
    
    quick_sort(&mut right[1..]);
}

fn partition<T: Ord + std::fmt::Debug>(datos: &mut [T]) -> usize {
    let len = datos.len();
    let pivote_index = len - 1; 
    let mut i = 0;
    println!("pivote {:?}", datos[pivote_index]);
    for j in 0..pivote_index {
        
        if datos[j] <= datos[pivote_index] {
            datos.swap(i, j);
            i += 1;
        }
    }

    datos.swap(i, pivote_index);
    println!("datos {:?}", datos);
    i
}

fn heap_sort<T: Ord + std::fmt::Debug>(datos: &mut [T]){
    println!("Algoritmo Heap sort\n");
   let len = datos.len();
    if len <= 1 {
        return;
    }
 
    for i in (0..len / 2).rev() {
        heapify(datos, len, i);
        println!("Datos {:?}", datos);
    }

    for i in (1..len).rev() {
        
        datos.swap(0, i);
        
        heapify(datos, i, 0);
    }
    println!("Datos ordenados {:?}", datos);
}

fn heapify<T: Ord>(datos: &mut [T], n: usize, i: usize) {
    let mut mayor = i;       // Inicializar el más grande como la raíz
    let hijo_izquierdo = 2 * i + 1;      
    let hijo_derecho = 2 * i + 2;     

  
    if hijo_izquierdo < n && datos[hijo_izquierdo] > datos[mayor] {
        mayor = hijo_izquierdo;
    }

    if hijo_derecho < n && datos[hijo_derecho] > datos[mayor] {
        mayor = hijo_derecho;
    }

    
    if mayor != i {
        datos.swap(i, mayor);
        heapify(datos, n, mayor);
    }
}


fn counting_sort<T: Ord + std::fmt::Debug>(datos: &mut [T]){
   println!("Algoritmo ordenamiento por conteo\n");
}

fn radix_sort<T: Ord + std::fmt::Debug>(datos: &mut [T]){
    println!("Algoritmo ordenamiento por digitos\n");

}
