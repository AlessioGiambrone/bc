use clap::{Arg, ArgMatches};
use clap::App as ClapApp;
use serde::Serialize;
use std::sync::Mutex;
use uuid::Uuid;

use bc::blockchain::*;
use bc::block::*;
use bc::transaction::*;

static ADDRESS: &str = "0.0.0.0:8088";

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

struct AppState{
    // Mutex is necessary to mutate safely across threads
    blockchain: Mutex<Blockchain>,
}

async fn mine_block(data: web::Data<AppState>) -> impl Responder {
    let mut bc = data.blockchain.lock().unwrap();
    HttpResponse::Ok().json(bc.mine_block())
}

async fn get_blockchain_length(data: web::Data<AppState>) -> impl Responder {
    let bc = data.blockchain.lock().unwrap();
    HttpResponse::Ok().json(&bc.chain.len())
}

async fn get_blockchain(data: web::Data<AppState>) -> impl Responder {
    let bc = data.blockchain.lock().unwrap();
    HttpResponse::Ok().json(&bc.chain)
}

async fn get_is_valid(data: web::Data<AppState>) -> impl Responder {
    let bc = data.blockchain.lock().unwrap();
    let is_valid = Blockchain::is_chain_valid(&bc.chain);
    HttpResponse::Ok().json(is_valid)
}

async fn add_transaction(
        data: web::Data<AppState>,
        transaction: web::Json<Transaction>) -> impl Responder {
    let mut bc = data.blockchain.lock().unwrap();
    let tr = transaction.clone();
    HttpResponse::Ok().json(
        bc.add_transaction(
            tr.sender,tr.receiver,tr.amount)
    )
}

async fn connect_nodes(
        data: web::Data<AppState>,
        nodes: web::Json<Vec<String>>) -> impl Responder {
    let mut bc = data.blockchain.lock().unwrap();
    for node in nodes.clone() {
        match bc.add_node(node) {
            Err(e) => return HttpResponse::BadRequest().json(e.to_string()),
            Ok(()) => (),
        }
    }
    println!("Blockchain now contains {} nodes", bc.nodes.len());
    HttpResponse::Ok().json(bc.nodes.clone())
}

#[derive(Serialize)]
struct ReplaceChainResponse{
    is_replaced: bool,
    blockchain: Vec<Block>,
}

async fn replace_chain(data: web::Data<AppState>) -> impl Responder {
    let mut bc = data.blockchain.lock().unwrap();
    let is_replaced = bc.replace_chain();
    HttpResponse::Ok().json( ReplaceChainResponse{
        is_replaced: is_replaced,
        blockchain: bc.chain.clone(),
    })
}

fn parse_args() -> ArgMatches {
    ClapApp::new("BC")
        .version("1.0")
        .about(
            "Basic Blockchain and cryptocurrency with love from Rust",
        )
        .author("Alessio Giambrone")
        .arg(
            Arg::with_name("name")
                .short('n')
                .long("name")
                .value_name("USERNAME")
                .about("the name of who is running the node"),
        ).get_matches()
}
 

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let matches = parse_args();
    // is it right to panic if there isn't a valid username
    let username = matches.value_of("name").expect("a valid username is needed");
    println!("username: {}", username);

    let blockchain = Blockchain::new(
        Uuid::new_v4().to_simple().to_string(),
        username.to_string(),
        1,
        );
    // blockchain.add_node("http://".to_string()+ADDRESS);
    
    let bc = web::Data::new(AppState {
        blockchain: Mutex::new(blockchain),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(bc.clone())
            .route("/mine_block", web::post().to(mine_block))
            .route("/blockchain", web::get().to(get_blockchain))
            .route("/blockchain_length", web::get().to(get_blockchain_length))
            .route("/is_valid", web::get().to(get_is_valid))
            .route("/add_transaction", web::post().to(add_transaction))
            .route("/nodes", web::post().to(connect_nodes))
            .route("/replace_chain", web::get().to(replace_chain))
    })
    .bind(ADDRESS)?
    .run()
    .await
}
