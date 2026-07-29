use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260728_193609_crear_tabla_estadisticas"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Estadisticas::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Estadisticas::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Estadisticas::Nombre)
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Estadisticas::Tiempo)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Estadisticas::Comparaciones)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Estadisticas::Intercambios)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Estadisticas::Escrituras)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Estadisticas::CantidadDatos)
                            .integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
         manager
            .drop_table(Table::drop().table(Estadisticas::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Estadisticas {
    Table,
    Id,
    Nombre,
    Tiempo,
    Comparaciones,
    Intercambios,
    Escrituras,
    CantidadDatos,
}