use diesel::prelude::*;
use diesel::{PgConnection, QueryResult, RunQueryDsl};
use rocket_contrib::json::JsonValue;
use uuid;

use crate::app::core::bool_expression::expression_option;
use crate::app::model::item::Item;
use crate::schema::variables;

#[derive(Identifiable, Queryable, Insertable, Associations, PartialEq)]
#[belongs_to(Item)]
pub struct Variable {
    id: uuid::Uuid,
    name: String,
    value: String,
    item_id: uuid::Uuid,
}

impl Variable {
    pub fn insert(
        item: &Item,
        variable_option: &expression_option::Variable<i32>,
        conn: &PgConnection,
    ) -> QueryResult<Variable> {
        let variable = Variable {
            id: uuid::Uuid::new_v4(),
            name: variable_option.get_name().to_owned(),
            value: variable_option.get_value().to_string(),
            item_id: item.get_id(),
        };

        diesel::insert_into(variables::table)
            .values(&variable)
            .get_result(conn)
    }

    pub fn get(id: uuid::Uuid, conn: &PgConnection) -> QueryResult<Variable> {
        variables::table.find(id).get_result::<Variable>(conn)
    }

    pub fn to_json(&self) -> JsonValue {
        json!({
            "name": self.name,
            "value": self.value,
        })
    }

    pub fn get_id(&self) -> uuid::Uuid {
        self.id
    }

    pub fn get_item_id(&self) -> uuid::Uuid {
        self.item_id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }
}
