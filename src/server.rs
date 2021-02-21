

    use tonic::{transport::Server, Request, Response, Status};
    use quelle::search_provider_server::{SearchProvider, SearchProviderServer};
    use quelle::{SearchResult, SearchRequest};
    use quelle::{FetchResult, FetchRequest};
    use quelle::{PageResult, PageRequest};
    use std::collections::HashMap;

    mod quelle;

    // defining a struct for our service
    #[derive(Default)]
    pub struct QuelleSearchProvider {}

    // implementing rpc for service defined in .proto
    #[tonic::async_trait]
    impl SearchProvider for QuelleSearchProvider {
        // our rpc impelemented as function
        async fn search(&self,request:Request<SearchRequest>)->Result<Response<SearchResult>,Status>{
            // returning a response as SayResponse message as defined in .proto
            Ok(Response::new(SearchResult{
                success:false,
                cursor: 0,
                remainder: 0,
                summary: String::from(""),
                session: String::from(""),
                enriched_request: None,
                records: HashMap::new(),
                errors: vec![ String::from("not implemented")],
                warnings: vec![],
            }))
        }
        // our rpc impelemented as function
        async fn fetch(&self,request:Request<FetchRequest>)->Result<Response<FetchResult>,Status>{
            // returning a response as SayResponse message as defined in .proto
            Ok(Response::new(FetchResult{
                success:false,
                cursor: 0,
                remainder: 0,
                session: String::from(""),
                records: HashMap::new(),
                errors: vec![ String::from("not implemented")],
                warnings: vec![],
            }))
        }
        // our rpc impelemented as function
        async fn page(&self,request:Request<PageRequest>)->Result<Response<PageResult>,Status>{
            // returning a response as SayResponse message as defined in .proto
            Ok(Response::new(PageResult{
                success:false,
                result: String::from(""),
                request: None,
                errors: vec![ String::from("not implemented")],
                warnings: vec![],
            }))
        }
    }


    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // defining address for our service
        let addr = "[::1]:50051".parse().unwrap();
    // creating a service
        let provider = QuelleSearchProvider::default();
        println!("Server listening on {}", addr);
    // adding our service to our server.
        Server::builder()
            .add_service(SearchProviderServer::new(provider))
            .serve(addr)
            .await?;
        Ok(())
    }

