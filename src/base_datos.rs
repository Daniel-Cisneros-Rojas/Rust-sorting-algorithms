use crate::entities;
use sea_orm::*;
use crate::entities::*;
use tabled::{Table, Tabled};
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
use crate::Estadisticas;
pub async fn guardar_estadisticas(datos_ingresar: &mut Estadisticas, conexion: &DatabaseConnection) -> Result<(), DbErr> {
    
    
    let nueva_estadistica = estadisticas::ActiveModel {
        algoritmo_id: Set(datos_ingresar.algoritmo_id),
        tiempo: Set(datos_ingresar.tiempo.as_millis() as i64),
        comparaciones: Set(datos_ingresar.comparaciones as i64),
        intercambios: Set(datos_ingresar.intercambios as i64),
        escrituras: Set(datos_ingresar.escrituras as i64),
        cantidad_datos: Set(datos_ingresar.cantidad_datos),
        fecha: Set(chrono::Local::now().naive_local()),
        ..Default::default()
    };

    let resultado=entities::estadisticas::Entity::insert(nueva_estadistica).exec(conexion).await?;
    println!("\nguardado");
    Ok(())
}

pub async fn ver_estadisticas_todas(conexion: &DatabaseConnection)-> Result<(),DbErr>{
    let registros = estadisticas::Entity::find()
        .select_only()
        .column(estadisticas::Column::Id)
        .column(estadisticas::Column::Tiempo)
        .column(estadisticas::Column::Comparaciones)
        .column(estadisticas::Column::Intercambios)
        .column(estadisticas::Column::Escrituras)
        .column(estadisticas::Column::CantidadDatos)
        .column(estadisticas::Column::Fecha)
        .column_as(algoritmos::Column::Nombre, "algoritmo")
        .join(
            JoinType::InnerJoin,
            estadisticas::Relation::Algoritmos.def(),
        )
        .into_tuple::<(
            i32,
            i64,
            i64,
            i64,
            i64,
            i32,
            chrono::NaiveDateTime,
            String,
        )>()
        .all(conexion)
        .await?;

    let tabla: Vec<EstadisticaTabla> = registros
        .into_iter()
        .map(
            |(
                id,
                tiempo,
                comparaciones,
                intercambios,
                escrituras,
                cantidad,
                fecha,
                algoritmo,
            )| EstadisticaTabla {
                id,
                algoritmo,
                tiempo_us: tiempo,
                comparaciones,
                intercambios,
                escrituras,
                cantidad_datos: cantidad,
                fecha,
            },
        )
        .collect();

    println!("{}", Table::new(tabla));

    Ok(())
}