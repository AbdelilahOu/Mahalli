//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::{entity::prelude::*, Set};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "quotes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub client_id: String,
    pub is_deleted: bool,
    pub is_archived: bool,
    pub created_at: String,
    pub identifier: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::clients::Entity",
        from = "Column::ClientId",
        to = "super::clients::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Clients,
    #[sea_orm(has_one = "super::orders::Entity")]
    Orders,
    #[sea_orm(has_many = "super::quote_items::Entity")]
    QuoteItems,
}

impl Related<super::clients::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Clients.def()
    }
}

impl Related<super::orders::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Orders.def()
    }
}

impl Related<super::quote_items::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::QuoteItems.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(ulid::Ulid::new().to_string()),
            ..ActiveModelTrait::default()
        }
    }
}
