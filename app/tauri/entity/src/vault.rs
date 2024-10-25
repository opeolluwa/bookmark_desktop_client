//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "vault")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::vault_entry::Entity")]
    VaultEntry,
}

impl Related<super::vault_entry::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::VaultEntry.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
