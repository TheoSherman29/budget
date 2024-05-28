#[macro_use]
use diesel::prelude::*;
use std::{io::stdin, io::Read, fmt, option::Option, string::String};
use chrono::naive::NaiveDate;
use diesel::ExpressionMethods;

use crate::models::{NewTransaction, Transaction};
use crate::db::establish_connection;

mod db;
mod schema;
mod models;
mod accounts;
mod categories;

pub fn main() {


    let prompt = "  ---------------------------- MAIN MENU ----------------------------\n";

    let choices = vec!["1", "2", "3",
    ];
    let mut choice: String = Default::default();

    stdin().read_line(&mut choice).expect("Can't read input");
    choice.pop();

    dbg!(choices.iter());
    dbg!(&choice);

    if choices.iter().any(|&i| i=="1") {
        match choice.as_str() {
            "1" => prompt_add_transaction(),
            "2" => show_ytd_transactions(),
            "3" => prompt_delete_transaction(),
            &_ => println!("Unknown value: {}", choice) ,
        }
    }
}

pub fn prompt_add_transaction() {
    let mut connection = &mut establish_connection();

    println!("Enter the following comma separated values:\n
        date: YYYY-MM-DD\n
        type: (Expense, Income, Transfer)\n
        amount:\n
        source:\n
        destination:\n
        category:\n
        description:\n
        earmark:");

    let mut input: String = Default::default(); 

    stdin().read_line(&mut input).unwrap();
    

    let fields = input.split(',').collect::<Vec<&str>>();
    let binding = fields[0].to_string();
    let date: &str = binding.trim();
    let binding = fields[1].to_string();
    let tpe: &str = binding.trim();
    let binding = fields[2].to_string();
    let amount: f32 = binding.trim().parse().unwrap();
    let binding = fields[3].to_string();
    let source: &str = binding.trim();
    let binding = fields[4].to_string();
    let destination: &str = binding.trim();
    let binding = fields[5].to_string();
    let category: &str = binding.trim();
    let binding = fields[6].to_string();
    let description: &str = binding.trim();
    let binding = fields[7].to_string();
    let earmark: &str = binding.trim();

    dbg!(&fields);
    add_transaction(&mut connection, &date, &tpe, &amount, &source, &destination, &category, &description, &earmark); 
}



pub fn add_transaction(
    conn:  &mut SqliteConnection, 
    date:  &str,
    tpe: &str,
    amount: &f32,
    source: &str,
    destination: &str,
    category: &str,
    description: &str,
    earmark: &str,) {
    use crate::schema::transactions::dsl::transactions;
    let new_transaction = NewTransaction { date, tpe, amount, source, destination, category, description, earmark };

    diesel::insert_into(transactions)
        .values(&new_transaction)
        .execute(conn)
        .expect("Error saving new transaction");
}


pub fn show_ytd_transactions() {
    use crate::schema::transactions::dsl::*;
    let mut conn = &mut establish_connection();
    let year = "2024";
    let results = transactions
        .select(Transaction::as_select())
        .load(conn)
        .expect("Error loading transactions");
    
    for transaction in results {
        println!("{}", transaction);
        //dbg!(transaction);
    }
}

pub fn prompt_delete_transaction() {
    let conn = &mut establish_connection();

    println!("Enter the ID of the transaction to delete.");

    let mut input: String = Default::default(); 

    stdin().read_line(&mut input).unwrap();

    let input_int: i32 = input.trim().parse().expect("ID should be an integer.");
    
    delete_transaction(conn, input_int)
}


pub fn delete_transaction(conn: &mut SqliteConnection, id: i32) {
    use crate::schema::transactions::dsl::transactions;
    
    let num_deleted = diesel::delete(transactions.find(id))
        .execute(conn)
        .expect("Error deleting posts");

}


pub fn prompt_add_csv() {
    let conn = &mut establish_connection();

}


pub fn add_csv(conn: &mut SqliteConnection, path: &String) {
    use crate::schema::transactions::dsl::transactions;

    
}


pub fn show_account_balances() {

}
