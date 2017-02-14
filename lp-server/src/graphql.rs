use juniper::{self, ExecutionError, GraphQLError, GraphQLType, RootNode, Value, Variables};
use rocket::data::{self, Data, FromData};
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::response::{self, content, Responder};
use rocket::request::{FromFormValue, Request};
use serde::ser::{self, SerializeMap};
use serde_json::error::Error as SerdeError;
use serde_json;
use std::io::Read;
use std::ops::Deref;

#[derive(Deserialize)]
struct InputValue(juniper::InputValue);

impl<'v> FromFormValue<'v> for InputValue {
    type Error = SerdeError;

    fn from_form_value(value: &'v str) -> Result<Self, Self::Error> {
        serde_json::from_str(value).map(|iv| InputValue(iv))
    }
}

impl Deref for InputValue {
    type Target = juniper::InputValue;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Deserialize, FromForm)]
pub struct GraphQLQuery {
    query: String,
    #[serde(rename = "operationName")]
    operation_name: Option<String>,
    variables: Option<InputValue>,
}

impl GraphQLQuery {
    pub fn execute<'a, QueryT, MutationT, CtxT>(
        &self,
        schema: &RootNode<QueryT, MutationT>,
        context: &CtxT,
    )
        -> GraphQLResult
        where QueryT: GraphQLType<Context=CtxT>,
              MutationT: GraphQLType<Context=CtxT>,
    {
        let operation_name = self.operation_name.as_ref().map(String::as_ref);
        let variables = self.variables();

        let result = WrappedGraphQLResult(
            juniper::execute(&self.query, operation_name, schema, &variables, context)
        );

        GraphQLResult(serde_json::to_string(&result))
    }

    pub fn variables(&self) -> Variables {
        self.variables.as_ref().and_then(|iv| {
            iv.to_object_value().map(|o| {
                o.into_iter().map(|(k, v)| (k.to_owned(), v.clone())).collect()
            })
        }).unwrap_or_default()
    }
}

const DATA_SIZE: u64 = 1048576; // 1 MiB

impl FromData for GraphQLQuery {
    type Error = SerdeError;

    fn from_data(request: &Request, data: Data) -> data::Outcome<Self, Self::Error> {
        if !request.content_type().map_or(false, |ct| ct.is_json()) {
            return Outcome::Forward(data);
        }

        let reader = data.open().take(DATA_SIZE);

        match serde_json::from_reader(reader) {
            Ok(v) => Outcome::Success(v),
            Err(e) => Outcome::Failure((Status::BadRequest, e)),
        }
    }
}

pub struct GraphQLResult(Result<String, SerdeError>);

impl<'r> Responder<'r> for GraphQLResult {
    fn respond(self) -> response::Result<'r> {
        self.0.map(|s| {
            content::JSON(s).respond().unwrap()
        }).map_err(|_| {
            Status::InternalServerError
        })
    }
}

struct WrappedGraphQLResult<'a>(Result<(Value, Vec<ExecutionError>), GraphQLError<'a>>);

impl<'a> ser::Serialize for WrappedGraphQLResult<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ser::Serializer,
    {
        match self.0 {
            Ok((ref res, ref err)) => {
                let mut map = try!(serializer.serialize_map(None));

                try!(map.serialize_key("data"));
                try!(map.serialize_value(res));

                if !err.is_empty() {
                    try!(map.serialize_key("errors"));
                    try!(map.serialize_value(err));
                }

                map.end()
            },
            Err(ref err) => {
                let mut map = try!(serializer.serialize_map(Some(1)));
                try!(map.serialize_key("errors"));
                try!(map.serialize_value(err));
                map.end()
            },
        }
    }
}
