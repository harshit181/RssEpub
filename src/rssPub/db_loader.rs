use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use crate::rssPub::rss_fetcher::get_urls;

const DB_PATH: &str ="infoData.db";
pub fn init_data(){
    let conn = Connection::open(DB_PATH).expect("Failed to open database");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS rss_feeds (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            rss_link TEXT NOT NULL
        )", [],
    ).expect("Failed to create table");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS email_password (
            id INTEGER PRIMARY KEY,
            email TEXT NOT NULL,
            password TEXT NOT NULL,
            send_to_email TEXT NOT NULL
        )", [],
    ).expect("Failed to create table");

    conn.close().unwrap();

}

#[derive(Debug, Serialize, Deserialize)]
pub struct RssFeed {
    pub id: i32,
    pub name: String,
    pub rss_link: String,
}

pub fn load_rss_feeds() -> Vec<RssFeed> {
    let conn = Connection::open(DB_PATH).expect("Failed to open database");
    let mut stmt = conn
        .prepare("SELECT id, name, rss_link FROM rss_feeds")
        .expect("Failed to prepare statement");
    let rss_feed_iter = stmt
        .query_map([], |row| {
            Ok(RssFeed {
                id: row.get(0)?,
                name: row.get(1)?,
                rss_link: row.get(2)?,
            })
        })
        .expect("Failed to execute query");

    let mut rss_feeds = Vec::new();
    for rss_feed in rss_feed_iter {
        rss_feeds.push(rss_feed.expect("Failed to get row"));
    }
    rss_feeds
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailPassword {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub send_to_email: String,
}

pub fn load_email_password() -> Option<EmailPassword> {
    let conn = Connection::open(DB_PATH).expect("Failed to open database");
    let mut stmt = conn
        .prepare("SELECT id, email, password, send_to_email FROM email_password LIMIT 1")
        .expect("Failed to prepare statement");
    let mut email_password_iter = stmt
        .query_map([], |row| {
            Ok(EmailPassword {
                id: row.get(0)?,
                email: row.get(1)?,
                password: row.get(2)?,
                send_to_email: row.get(3)?,
            })
        })
        .expect("Failed to execute query");

    let email_password = email_password_iter.next();
    match email_password {
        Some(result) => Some(result.expect("Failed to get row")),
        None => None,
    }
}


pub fn insert_data(ompl:&str){
   let data= get_urls(ompl);
    let conn = Connection::open(DB_PATH).expect("Failed to open database");
    for (name,url) in data{
        conn.execute(
            "INSERT INTO rss_feeds (name, rss_link) VALUES (?1, ?2)",
            [&name, &url],
        ).expect("Failed to insert data");
    }
    conn.close().unwrap();

}

