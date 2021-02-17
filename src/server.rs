

    use tonic::{transport::Server, Request, Response, Status};
    use quelle::search_provider_server::{SearchProvider, SearchProviderServer};
    use quelle::{QuelleSearchResult, QuelleSearchRequest};
    use quelle::{QuelleFetchResult, QuelleFetchRequest};
    use quelle::{QuellePageResult, QuellePageRequest};
    mod quelle;

    // defining a struct for our service
    #[derive(Default)]
    pub struct QuelleSearchProvider {}

    // implementing rpc for service defined in .proto
    #[tonic::async_trait]
    impl SearchProvider for QuelleSearchProvider {
        // our rpc impelemented as function
        async fn search(&self,request:Request<QuelleSearchRequest>)->Result<Response<QuelleSearchResult>,Status>{
            // returning a response as SayResponse message as defined in .proto
            Ok(Response::new(QuelleSearchResult{
                // reading data from request which is awrapper around our SayRequest message defined in .proto
                 message:format!("hello {}",request.get_ref().name),
            }))
        }
        // our rpc impelemented as function
        async fn fetch(&self,request:Request<QuelleFetchRequest>)->Result<Response<QuelleFetchResult>,Status>{
            // returning a response as SayResponse message as defined in .proto
            Ok(Response::new(QuelleFetchResult{
                // reading data from request which is awrapper around our SayRequest message defined in .proto
                 message:format!("hello {}",request.get_ref().name),
            }))
        }
        // our rpc impelemented as function
        async fn page(&self,request:Request<QuellePageRequest>)->Result<Response<QuellePageResult>,Status>{
            // returning a response as SayResponse message as defined in .proto
            Ok(Response::new(QuellePageResult{
                // reading data from request which is awrapper around our SayRequest message defined in .proto
                 message:format!("hello {}",request.get_ref().name),
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

