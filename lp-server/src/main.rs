use juniper::RootNode;
use lp::graphql::{Context, MutationRoot, QueryRoot};
use warp::Filter;

type Schema = RootNode<'static, QueryRoot, MutationRoot>;

fn schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot)
}

fn main() {
    let state = warp::any().map(Context::new);

    let graphql_filter = juniper_warp::make_graphql_filter(schema(), state.boxed());
    let graphql = warp::path("graphql").and(graphql_filter);

    let graphiql_filter = juniper_warp::graphiql_filter("/graphql");
    let graphiql = warp::path("graphiql").and(graphiql_filter);

    let log = warp::log("lp-server");

    let routes = warp::get2().and(graphiql).or(graphql).with(log);

    warp::serve(routes).run(([127, 0, 0, 1], 3030));
}
