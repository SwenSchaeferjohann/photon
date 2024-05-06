//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "state_trees")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub tree: Vec<u8>,
    #[sea_orm(primary_key, auto_increment = false)]
    pub node_idx: i64,
    pub leaf_idx: Option<i64>,
    pub level: i64,
    pub hash: Vec<u8>,
    pub seq: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
