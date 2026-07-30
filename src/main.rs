use rand::{Rng};
use std::println;
use std::time::Duration;
use std::time::Instant;

mod entities;
use entities::prelude::*;
use entities::*;
use sea_orm::*;
use sea_orm::prelude::*;
use tabled::{Table, Tabled};

mod base_datos;

struct Estadisticas {
    pub algoritmo_id: i32,
    pub tiempo: Duration,
    pub comparaciones: usize,
    pub intercambios: usize,
    pub escrituras: usize,
    pub cantidad_datos:i32,
}

#[derive(Tabled)]
struct EstadisticaTabla {
    id: i32,
    algoritmo: String,
    tiempo_us: i64,
    comparaciones: i64,
    intercambios: i64,
    escrituras: i64,
    cantidad_datos: i32,
    fecha: chrono::NaiveDateTime,
}

#[repr(i32)]
enum Algoritmo {
    Bubble = 1,
    Selection = 2,
    Insertion = 3,
    Merge = 4,
    Quick = 5,
    Heap = 6,
    Counting = 7,
    Radix = 8,
}

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{
    
    
    let mut stats = Estadisticas {
        algoritmo_id: 0,
        tiempo: Duration::ZERO,
        comparaciones: 0,
        intercambios: 0,
        escrituras:0,
        cantidad_datos:0,
        
    };
    let cantidad_datos:i32=90;
    stats.cantidad_datos=cantidad_datos;
    println!("\nAlgoritmos de ordenamiento");
    let  numeros_originales = numeros_aleatorios(cantidad_datos);
    //let numeros_originales=vec![5, 3, 8, 4, 2,-1];
    println!("\nNúmeros generados aleatoriamente : {:?}\n", numeros_originales);
    
    //bubble sort
    let mut numeros_para_procesar=numeros_originales.clone();
    let inicio = Instant::now();
    bubble_sort( &mut numeros_para_procesar,&mut stats);
    stats.tiempo = inicio.elapsed();
    imprimir_stats(&mut stats);

    //selection sort 
    let mut numeros_para_procesar=numeros_originales.clone();
    let inicio = Instant::now();
    selection_sort(&mut numeros_para_procesar, &mut stats);
    stats.tiempo = inicio.elapsed();
    imprimir_stats(&mut stats);

    //insertion sort
    let mut numeros_para_procesar=numeros_originales.clone();
    let inicio = Instant::now();
    insertion_sort(&mut numeros_para_procesar, &mut stats);
    stats.tiempo = inicio.elapsed();
    imprimir_stats(&mut stats);

    //merge sort
    let mut numeros_para_procesar=numeros_originales.clone();
    println!("Algoritmo ordenamiento por mezcla (Merge sort)\n");
    stats.algoritmo_id = Algoritmo::Merge as i32;
    stats.comparaciones = 0;
    stats.intercambios = 0;
    stats.escrituras=0;
    let inicio = Instant::now();
    merge_sort(&mut numeros_para_procesar, &mut stats);
    stats.tiempo = inicio.elapsed();
    imprimir_stats(&mut stats);

    //quick sort
    let mut numeros_para_procesar=numeros_originales.clone(); 
    println!("Algoritmo ordenamiento Quick sort\n");
    stats.algoritmo_id = Algoritmo::Quick as i32;
    stats.comparaciones = 0;
    stats.intercambios = 0;
    let inicio = Instant::now();
    quick_sort(&mut numeros_para_procesar, &mut stats);
    println!("Datos ordenados {:?}", numeros_para_procesar);
    stats.tiempo = inicio.elapsed();
    imprimir_stats(&mut stats);

    //heap sort
    let mut numeros_para_procesar=numeros_originales.clone();
    let inicio = Instant::now();
    heap_sort(&mut numeros_para_procesar, &mut stats);
    stats.tiempo = inicio.elapsed();
    imprimir_stats(&mut stats);

    //counting sort
    let mut numeros_para_procesar=numeros_originales.clone();
    let inicio = Instant::now();
    counting_sort(&mut numeros_para_procesar, &mut stats);
    stats.tiempo = inicio.elapsed();
    imprimir_stats(&mut stats);

    //radix sort
    let mut numeros_para_procesar=numeros_originales.clone();
    let inicio = Instant::now();
    radix_sort(&mut numeros_para_procesar, &mut stats);
    stats.tiempo = inicio.elapsed();
    imprimir_stats(&mut stats);

    let mut conexion=algorithms::obtener_conexion().await?;
    println!("conectado");
    //base_datos::guardar_estadisticas(&mut stats, &mut conexion).await?;
    base_datos::ver_estadisticas_todas(&mut conexion).await?;
    Ok(())
    
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

fn bubble_sort<T: Ord + std::fmt::Debug>(datos: &mut[T], stats: &mut Estadisticas){
    println!("Algoritmo ordenamiento burbuja\n");
    stats.algoritmo_id = Algoritmo::Bubble as i32;
   let tam= datos.len();
    for i in 0..tam-1{
        for j in 0..tam-1-i{
            stats.comparaciones += 1;
            if datos[j]>datos[j+1]{
                datos.swap(j,j+1);
                stats.intercambios += 1;
            }
            println!("{:?} vuelta {}",datos, i)
        }

    }
}


fn selection_sort<T: Ord + std::fmt::Debug>(datos: &mut[T],stats: &mut Estadisticas){
    println!("Algoritmo ordenamiento selección\n");
    stats.algoritmo_id = Algoritmo::Selection as i32;
    stats.comparaciones = 0;
    stats.intercambios = 0;
    let tam= datos.len();
    for i in 0..tam-1{
        let mut index_menor_numero=i;
        for j in i+1..tam{
            stats.comparaciones += 1;
            if datos[index_menor_numero]>datos[j]{
                index_menor_numero=j;
            }
        }
        datos.swap(i, index_menor_numero);
        stats.intercambios += 1;
        println!("{:?} vuelta {}", datos, i);
    }
}

fn insertion_sort<T: Ord + std::fmt::Debug>(datos: &mut[T],stats: &mut Estadisticas){
    println!("Algoritmo ordenamiento inserción\n");
    stats.algoritmo_id = Algoritmo::Insertion as i32;
    stats.comparaciones = 0;
    stats.intercambios = 0;
    let tam= datos.len();
    for i in 1..tam{
       for j in (0..i).rev(){
           println!("{:?} dato analizado {:?} comparado contra {:?}", datos, datos[j+1], datos[j]);
           stats.comparaciones += 1;
           if datos[j+1]<datos[j]{
            datos.swap(j+1, j);
            stats.intercambios += 1;
           }else{
            break;
           }
           
       }
       
    }
}

fn merge_sort<T: Ord + std::fmt::Debug+ std::clone::Clone>(datos: &mut[T],stats: &mut Estadisticas){
    
    let tam=datos.len();
    
    if tam<=1{
        return;
    }
        let mitad=tam/2;
        
        merge_sort(&mut datos[..mitad], stats);
        merge_sort(&mut datos[mitad..],stats);
        
        merge(datos,mitad,stats);

}

fn merge<T: Ord + std::fmt::Debug+ std::clone::Clone>(datos: &mut[T],mitad:usize, stats: &mut Estadisticas){
    let (mut i,mut j,mut k)=(0,0,0);
    let izquierda=datos[..mitad].to_vec();
    let derecha=datos[mitad..].to_vec();
    println!("datos a ordenar {:?}", datos);
    while i<izquierda.len() && j<derecha.len()
    {
        stats.comparaciones += 1;
        if izquierda[i]<=derecha[j]
        {
            datos[k]=izquierda[i].clone();
            stats.escrituras += 1;
            i+=1;
        }else{
            datos[k]=derecha[j].clone();
            stats.escrituras += 1;
            j+=1;
        }
        k+=1;
    }

    while i<izquierda.len(){
        stats.escrituras += 1; //escritura no intercambio
        datos[k]=izquierda[i].clone();
        i+=1;
        k+=1;
    }
    while j<derecha.len(){
        stats.escrituras += 1;
        datos[k]=derecha[j].clone();
        j+=1;
        k+=1;
    }

    println!("datos ordenados {:?}", datos);
}

fn quick_sort<T: Ord+ std::fmt::Debug>(datos: &mut [T],stats: &mut Estadisticas) {
    let len = datos.len();
    if len <= 1 {
        return;
    }

    let pivote_index = partition(datos, stats);

    let (left, right) = datos.split_at_mut(pivote_index);
    
 
    quick_sort(left,stats);
    
    quick_sort(&mut right[1..],stats);
}

fn partition<T: Ord + std::fmt::Debug>(datos: &mut [T], stats: &mut Estadisticas) -> usize {
    let len = datos.len();
    let pivote_index = len - 1; 
    let mut i = 0;
    println!("pivote {:?}", datos[pivote_index]);
    for j in 0..pivote_index {
        
        stats.comparaciones += 1;
        if datos[j] <= datos[pivote_index] {
            datos.swap(i, j);
            stats.intercambios +=1;
            i += 1;
        }
    }

    datos.swap(i, pivote_index);
    stats.intercambios += 1;
    println!("datos {:?}", datos);
    i
}

fn heap_sort<T: Ord + std::fmt::Debug>(datos: &mut [T], stats: &mut Estadisticas){
    println!("Algoritmo Heap sort\n");
    stats.algoritmo_id = Algoritmo::Heap as i32;
    stats.comparaciones = 0;
    stats.intercambios = 0;
   let len = datos.len();
    if len <= 1 {
        return;
    }
 
    for i in (0..len / 2).rev() {
        heapify(datos, len, i,stats);
        println!("Datos {:?}", datos);
    }

    for i in (1..len).rev() {
        
        datos.swap(0, i);
        stats.intercambios += 1;
        heapify(datos, i, 0,stats);
    }
    println!("Datos ordenados {:?}", datos);
}

fn heapify<T: Ord>(datos: &mut [T], n: usize, i: usize, stats: &mut Estadisticas) {
    let mut mayor = i;       // Inicializar el más grande como la raíz
    let hijo_izquierdo = 2 * i + 1;      
    let hijo_derecho = 2 * i + 2;     

    stats.comparaciones += 1;
    if hijo_izquierdo < n && datos[hijo_izquierdo] > datos[mayor] {
        mayor = hijo_izquierdo;
    }
    stats.comparaciones += 1;
    if hijo_derecho < n && datos[hijo_derecho] > datos[mayor] {
        mayor = hijo_derecho;
    }

    
    if mayor != i {
        datos.swap(i, mayor);
        stats.intercambios += 1;
        heapify(datos, n, mayor,stats);
    }
}


fn counting_sort(datos: &mut [i32], stats: &mut Estadisticas){
    let tam=datos.len();
   println!("Algoritmo ordenamiento por conteo\n");
   stats.algoritmo_id = Algoritmo::Counting as i32;
   stats.comparaciones = 0;
   stats.intercambios = 0;
   stats.escrituras=0;
   let (indice_maximo,indice_minimo)=maximo_minimo(datos, stats);
   println!("minimo {:?} maximo {:?}", datos[indice_minimo], datos[indice_maximo]);
   let mut conteo=vec![0; (datos[indice_maximo] - datos[indice_minimo] + 1)as usize];

   for i in 0..tam{
         let indice=(datos[i]-datos[indice_minimo]) as usize;
          conteo[indice]+=1;
   }
   println!("conteo {:?} ", conteo);
   
   let mut k=0;
   let numero_minimo=datos[indice_minimo];
   for i in 0..conteo.len(){
       for _ in 0..conteo[i]{
          stats.escrituras +=1;
          datos[k]=i as i32 + numero_minimo;
          k+=1;
       }
   }
   println!("Datos ordenados {:?} ", datos);
}

fn maximo_minimo(datos: &mut [i32], stats: &mut Estadisticas)->(usize,usize){
    let mut maximo=0;
    let mut minimo=0;
    for i in 0..datos.len(){
        stats.comparaciones +=1;
        if datos[i]>datos[maximo]{
            maximo=i;
        }
        stats.comparaciones +=1;
        if datos[i]<datos[minimo]{
            minimo=i;
        }
    }
    return (maximo,minimo);
}


fn radix_sort(datos: &mut [i32], stats: &mut Estadisticas){
    println!("Algoritmo ordenamiento por digitos\n");
    stats.algoritmo_id = Algoritmo::Radix as i32;
    stats.comparaciones = 0;
    stats.intercambios = 0;
    stats.escrituras=0;
    if datos.len() <= 1 {
        return;
    }

    let mut maximo = datos[0];
    for &valor in datos.iter().skip(1) {
        if valor > maximo {
            maximo = valor;
        }
        if valor<0{
            println!("Algoritmo solo para numeros positivos");
            return;
        }
    }

    let mut exp = 1;

    while maximo / exp > 0 {
        counting_sort_por_digito(datos, exp, stats);
        exp *= 10;
        println!("Datos {:?}", datos);
    }
    println!("Datos ordenados {:?}", datos);
}

fn counting_sort_por_digito(datos: &mut [i32], exp: i32, stats: &mut Estadisticas) {
    let n = datos.len();

    let mut salida = vec![0; n];

    let mut conteo = vec![0; 10];

    for &numero in datos.iter() {
        let digito = ((numero / exp) % 10) as usize;
        conteo[digito] += 1;
    }

    for i in 1..10 {
        conteo[i] += conteo[i - 1];
    }

    for i in (0..n).rev() {
        let digito = ((datos[i] / exp) % 10) as usize;
        conteo[digito] -= 1;
        let posicion = conteo[digito];
        salida[posicion] = datos[i];
        stats.escrituras +=1;
    }

    stats.escrituras +=1;
    datos.copy_from_slice(&salida);
}

fn imprimir_stats(estadisticas: &mut Estadisticas){
    println!("Id algoritmo : {:?}", estadisticas.algoritmo_id);
    println!("Comparaciones: {}", estadisticas.comparaciones);
    println!("Intercambios: {}", estadisticas.intercambios);
    println!("Escrituras: {}", estadisticas.escrituras);
    println!("Tiempo (milisegundos) : {:?}", estadisticas.tiempo.as_millis());
    println!("Cantidad datos: {}", estadisticas.cantidad_datos);
    println!("Hora: {:?}", chrono::Local::now());
}
