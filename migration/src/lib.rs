pub use sea_orm_migration::prelude::*;


mod m20260728_193609_crear_tabla_estadisticas;
mod m20260729_185559_crear_tabla_algoritmos;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260728_193609_crear_tabla_estadisticas::Migration),
            Box::new(m20260729_185559_crear_tabla_algoritmos::Migration),
        ]
    }
}
