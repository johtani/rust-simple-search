use search::Query;
use analysis::Tokenizer;

pub fn parseQuery<T: Tokenizer> (queryString: &str, analyzer: &T) -> Query {
    //FIXME there is no support query syntax, like Boolean query.
    let query = Query::new(queryString);
    query.parse_query(analyzer);
    return query;
}