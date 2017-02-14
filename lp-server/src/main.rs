#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate juniper;
#[macro_use] extern crate lazy_static;
extern crate lp;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

use graphql::{GraphQLQuery, GraphQLResult};
use juniper::RootNode;
use lp::graphql::{Context, MutationRoot, QueryRoot};
use rocket::response::NamedFile;
use std::io;

mod graphql;

lazy_static! {
    static ref QUERY_ROOT: QueryRoot = QueryRoot;
    static ref MUTATION_ROOT: MutationRoot = MutationRoot;
    static ref SCHEMA: RootNode<'static, &'static QueryRoot, &'static MutationRoot> = {
        RootNode::new(&QUERY_ROOT, &MUTATION_ROOT)
    };
}

#[get("/graphql?<query>")]
fn graphql_get(query: GraphQLQuery) -> GraphQLResult {
    let context = Context::new();
    query.execute(&SCHEMA, &context)
}

#[post("/graphql", data = "<query>")]
fn graphql_post(query: GraphQLQuery) -> GraphQLResult {
    let context = Context::new();
    query.execute(&SCHEMA, &context)
}

#[get("/")]
fn graphiql() -> io::Result<NamedFile> {
    NamedFile::open("static/graphiql.html")
}

/* #[route(OPTIONS, "/graphql")]
fn graphql_options() -> Option<()> {
    Some(())
} */

fn main() {
    rocket::ignite()
        .mount("/", routes![graphql_get, graphql_post, graphiql])
        .launch();
}
