#[derive(GraphQLQuery)]
#[graphql(
  schema_path = "./schema.docs.graphql",
  query_path = "./query/view_repo.graphql",
  response_derives = "Debug"
)]
pub struct ViewRepoQuery;
