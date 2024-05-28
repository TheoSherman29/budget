use diesel::prelude::*;
use crate::schema::transactions;
use std::fmt;

#[derive(Insertable)]
#[diesel(table_name = transactions)]
pub struct NewTransaction<'a> {
    pub date: &'a str,
    pub tpe: &'a str,
    pub amount: &'a f32,
    pub source: &'a str,
    pub destination: &'a str,
    pub category: &'a str,
    pub description: &'a str,
    pub earmark: &'a str,
}


#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = transactions)]
pub struct Transaction {
    pub id: i32,
    pub date: String,
    pub tpe: String,
    pub amount: f32,
    pub source: Option<String>,
    pub destination: Option<String>,
    pub category: String,        
    pub description: Option<String>,
    pub earmark: Option<String>,
}


impl fmt::Display for Transaction {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{0: <8} | {1: <8} | {2: <8} | {3: <8} | {4: <8} | {5: <8} | {6: <8} | {7: <8} | {8: <8} |", 
              &self.id, 
              &self.date, 
              &self.tpe, 
              &self.amount, 
              &self.source.as_ref().unwrap_or(&"N/A".to_string()), 
              &self.destination.as_ref().unwrap_or(&"N/A".to_string()), 
              &self.category, 
              &self.description.as_ref().unwrap_or(&"N/A".to_string()), 
              &self.earmark.as_ref().unwrap_or(&"N/A".to_string()))
    }
}
