pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20240701_104404_add_table_expences;
mod m20240906_142826_addd_user_table;
mod m20240906_142837_addd_receipts_table;
mod m20240906_143558_add_user_id_to_expense;
mod m20240906_150646_addd_session_table;
mod m20240906_175111_add_exp_rec_view;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20240701_104404_add_table_expences::Migration),
            Box::new(m20240906_142826_addd_user_table::Migration),
            Box::new(m20240906_142837_addd_receipts_table::Migration),
            Box::new(m20240906_143558_add_user_id_to_expense::Migration),
            Box::new(m20240906_150646_addd_session_table::Migration),
            Box::new(m20240906_175111_add_exp_rec_view::Migration),
        ]
    }
}
