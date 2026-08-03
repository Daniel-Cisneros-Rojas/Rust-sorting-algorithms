# Sorting Algorithms Analyzer

Aplicación desarrollada en **Rust** para analizar y comparar el rendimiento de distintos algoritmos de ordenamiento. El programa genera conjuntos de datos aleatorios, ejecuta los algoritmos seleccionados, mide su desempeño y almacena las estadísticas en una base de datos **PostgreSQL** mediante **SeaORM**.
<img width="1219" height="377" alt="image" src="https://github.com/user-attachments/assets/2a69525b-5110-4a77-90d0-3a9b420b2dc8" />

##  Características

* Implementación de 8 algoritmos de ordenamiento:

  * Bubble Sort
  * Selection Sort
  * Insertion Sort
  * Merge Sort
  * Quick Sort
  * Heap Sort
  * Counting Sort
  * Radix Sort
* Medición de tiempo de ejecución, comparaciones, intercambios y escrituras.
* Ejecución individual o comparativa desde un menú interactivo.
* Almacenamiento y consulta del historial de ejecuciones en PostgreSQL.
* Base de datos gestionada mediante migraciones de SeaORM.

## Estructura base de datos 

<img width="551" height="665" alt="image" src="https://github.com/user-attachments/assets/5dec5adb-9005-4474-bcd7-cfb49199bdf4" />

##  Requisitos

* Rust y Cargo
* PostgreSQL

##  Configuración

Crea un archivo `.env` en la raíz del proyecto:

```env
DATABASE_URL=postgres://usuario:contraseña@localhost:5432/algoritmos_db
```

##  Ejecutar las migraciones

Desde la carpeta `migration`:

```bash
cargo run -- up
```

## Ejecutar el proyecto

Desde la raíz del proyecto:

```bash
cargo run
```

##  Estadísticas registradas

Cada ejecución almacena:

* Algoritmo utilizado.
* Tiempo de ejecución (μs).
* Comparaciones.
* Intercambios.
* Escrituras.
* Cantidad de datos procesados.
* Fecha y hora de ejecución.
