#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate juniper;
extern crate juniper_rocket;
#[macro_use] extern crate lazy_static;
extern crate lp;
extern crate rocket;

use juniper::RootNode;
use lp::graphql::{Context, MutationRoot, QueryRoot};
use rocket::response::content;

lazy_static! {
    static ref QUERY_ROOT: QueryRoot = QueryRoot;
    static ref MUTATION_ROOT: MutationRoot = MutationRoot;
    static ref SCHEMA: RootNode<'static, &'static QueryRoot, &'static MutationRoot> = {
        RootNode::new(&QUERY_ROOT, &MUTATION_ROOT)
    };
}

#[get("/graphql?<query>")]
fn graphql_get(query: juniper_rocket::GraphQLRequest) -> juniper_rocket::GraphQLResponse {
    let context = Context::new();
    query.execute(&SCHEMA, &context)
}

#[post("/graphql", data = "<query>")]
fn graphql_post(query: juniper_rocket::GraphQLRequest) -> juniper_rocket::GraphQLResponse {
    let context = Context::new();
    query.execute(&SCHEMA, &context)
}

#[get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

fn main() {
    rocket::ignite()
        .mount("/", routes![graphql_get, graphql_post, graphiql])
        .launch();
}
