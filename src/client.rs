use quelle::search_provider_client::SearchProviderClient;
use quelle::SearchRequest;
use quelle::SearchClause;

    mod quelle;

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // creating a channel ie connection to server
        let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
        .connect()
        .await?;
    // creating gRPC client from channel
        let mut client = SearchProviderClient::new(channel);
    // creating a new Request
        let clause1 = SearchClause{
            syntax: String::from("QuelleSearchProvider"),
            segment:String::from("eternal power godhead"),
            fragments: Vec::new(),
            polarity:1,
        };
        let clause2 = SearchClause{
            syntax: String::from("QuelleSearchProvider"),
            segment:String::from("#run&/v/|/adj/"),
            fragments: Vec::new(),
            polarity:1,
        };
        let mut searchReq = SearchRequest{
            controls: None,
            clauses: vec! [ clause1, clause2 ],
            count: 10,
        };

        let request = tonic::Request::new(searchReq);

    // sending request and waiting for response
        let response = client.search(request).await?.into_inner();
        println!("RESPONSE={:?}", response);
        Ok(())
    }
