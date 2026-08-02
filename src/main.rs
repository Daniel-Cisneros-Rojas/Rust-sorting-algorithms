use rand::{Rng};
use std::println;
use std::time::Duration;
use std::time::Instant;

mod entities;
use entities::prelude::*;
use sea_orm::*;
use tabled::{Tabled};
use std::io::{self, Write};

mod base_datos;

struct Estadisticas {
    pub algoritmo_id: i32,
    pub tiempo: Duration,
    pub comparaciones: usize,
    pub intercambios: usize,
    pub escrituras: usize,
    pub cantidad_datos:i32,
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
    
    
    let mut conexion = algorithms::obtener_conexion().await?;

    loop {

        println!("\n==============================");
        println!(" SISTEMA DE ORDENAMIENTOS");
        println!("==============================");
        println!("1. Ejecutar un algoritmo");
        println!("2. Ejecutar todos");
        println!("3. Ver estadísticas");
        println!("4. Salir");
        println!("==============================");

        let opcion = leer_numero();

        match opcion {

            1 => ejecutar_un_algoritmo(&mut conexion).await?,

            2 => ejecutar_todos(&mut conexion).await?,

            3 => menu_estadisticas(&mut conexion).await?,

            4 => break,

            _ => println!("Opción inválida")

        }
    }

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

//menus ejecuciones 
fn leer_numero() -> i32 {

    loop {

        print!("> ");

        io::stdout().flush().unwrap();

        let mut entrada = String::new();

        io::stdin().read_line(&mut entrada).unwrap();

        match entrada.trim().parse() {

            Ok(numero) => return numero,

            Err(_) => println!("Ingrese un número válido.")

        }

    }

}

async fn ejecutar_un_algoritmo(
    conexion: &DatabaseConnection,
) -> Result<(), DbErr> {

    println!();
    println!("Seleccione algoritmo");
    println!("1 Bubble");
    println!("2 Selection");
    println!("3 Insertion");
    println!("4 Merge");
    println!("5 Quick");
    println!("6 Heap");
    println!("7 Counting");
    println!("8 Radix");

    let algoritmo = leer_numero();

    println!("Cantidad de datos:");

    let cantidad = leer_numero();

    println!("Número de repeticiones:");

    let repeticiones = leer_numero();

    for _ in 0..repeticiones {

        ejecutar_algoritmo(
            algoritmo,
            cantidad,
            conexion,
        ).await?;

    }

    Ok(())
}

async fn ejecutar_todos(
    conexion: &DatabaseConnection,
) -> Result<(), DbErr> {

    println!("Cantidad de datos:");

    let cantidad = leer_numero();

    println!("Número de repeticiones:");

    let repeticiones = leer_numero();

    for _ in 0..repeticiones {

        for algoritmo in 1..=8 {

            ejecutar_algoritmo(
                algoritmo,
                cantidad,
                conexion,
            ).await?;

        }

    }

    Ok(())
}

async fn ejecutar_algoritmo(

    algoritmo: i32,

    cantidad: i32,

    conexion: &DatabaseConnection,

) -> Result<(), DbErr> {

    let numeros = numeros_aleatorios(cantidad);

    let mut copia = numeros.clone();

    let mut stats = Estadisticas {

        algoritmo_id: algoritmo,

        tiempo: Duration::ZERO,

        comparaciones: 0,

        intercambios: 0,

        escrituras: 0,

        cantidad_datos: cantidad,

    };

    let inicio = Instant::now();

    match algoritmo {

        1 => bubble_sort(&mut copia, &mut stats),

        2 => selection_sort(&mut copia, &mut stats),

        3 => insertion_sort(&mut copia, &mut stats),

        4 => merge_sort(&mut copia, &mut stats),

        5 => quick_sort(&mut copia, &mut stats),

        6 => heap_sort(&mut copia, &mut stats),

        7 => counting_sort(&mut copia, &mut stats),

        8 => radix_sort(&mut copia, &mut stats),

        _ => {
            println!("Algoritmo inválido.");
            return Ok(());
        }

    }

    stats.tiempo = inicio.elapsed();

    imprimir_stats(&mut stats);

    base_datos::guardar_estadisticas(
        &mut stats,
        conexion,
    )
    .await?;

    Ok(())
}

async fn menu_estadisticas(
    conexion: &DatabaseConnection,
) -> Result<(), DbErr> {

    loop {

        println!();
        println!("1 Mostrar todas");

        println!("2 Mostrar por algoritmo");

        println!("3 Regresar");

        let opcion = leer_numero();

        match opcion {

            1 => {

                base_datos::mostrar_estadisticas_todas(conexion)
                    .await?;

            }

            2 => {

                println!("Ingrese el id del algoritmo:");

                let algoritmo = leer_numero();

                base_datos::mostrar_estadisticas_algoritmo(
                    conexion,
                    algoritmo,
                )
                .await?;

            }

            3 => break,

            _ => println!("Opción inválida")

        }

    }

    Ok(())
}