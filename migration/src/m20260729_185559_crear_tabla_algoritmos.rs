use sea_orm_migration::{prelude::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260729_185559_crear_tabla_algoritmos"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
             .create_table(
                Table::create()
                    .table(Algoritmos::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Algoritmos::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Algoritmos::Nombre)
                            .string_len(100)
                            .not_null()
                            .unique_key(),
                    )
                    .to_owned(),
            )
            .await?;

            manager
            .exec_stmt(
                Query::insert()
                    .into_table(Algoritmos::Table)
                    .columns([Algoritmos::Nombre])
                    .values_panic(["Bubble Sort".into()])
                    .values_panic(["Selection Sort".into()])
                    .values_panic(["Insertion Sort".into()])
                    .values_panic(["Merge Sort".into()])
                    .values_panic(["Quick Sort".into()])
                    .values_panic(["Heap Sort".into()])
                    .values_panic(["Counting Sort".into()])
                    .values_panic(["Radix Sort".into()])
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Algoritmos::Table).to_owned())
            .await
    }
}
#[derive(DeriveIden)]
enum Algoritmos {
    Table,
    Id,
    Nombre,
}