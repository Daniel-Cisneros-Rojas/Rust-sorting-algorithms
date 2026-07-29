use dotenvy::dotenv;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbErr, Statement};
use std::env;

const CREATE_DATABASE: bool = true;

pub async fn obtener_conexion() -> Result<DatabaseConnection, DbErr> {
  
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("La variable DATABASE_URL no existe en el archivo .env");

    if CREATE_DATABASE {
        // Obtener la URL sin el nombre de la base de datos
        let (server_url, db_name) = database_url
            .rsplit_once('/')
            .expect("DATABASE_URL tiene un formato inválido");

        let server_url = format!("{}/", server_url);

        let conexion = Database::connect(&server_url).await?;

        let query = format!("CREATE DATABASE {}", db_name);

        let stmt = Statement::from_string(conexion.get_database_backend(), query);

        
        let _ = conexion.execute(stmt).await;
    }

    let conexion = Database::connect(&database_url).await?;

    Ok(conexion)
}