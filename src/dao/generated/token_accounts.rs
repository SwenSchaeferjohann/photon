//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "token_accounts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub hash: Vec<u8>,
    pub address: Option<Vec<u8>>,
    pub owner: Vec<u8>,
    pub mint: Vec<u8>,
    pub amount: i64,
    pub delegate: Option<Vec<u8>>,
    pub frozen: bool,
    pub is_native: Option<i64>,
    pub delegated_amount: i64,
    pub close_authority: Option<Vec<u8>>,
    pub spent: bool,
    pub slot_updated: i64,
    pub created_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::accounts::Entity",
        from = "Column::Hash",
        to = "super::accounts::Column::Hash",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Accounts,
}

impl Related<super::accounts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Accounts.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
