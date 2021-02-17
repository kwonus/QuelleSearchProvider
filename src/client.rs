use quelle::search_provider_client::SearchProviderClient;
use quelle::QuelleSearchRequest;

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
        let request = tonic::Request::new(
            QuelleSearchRequest {
               name:String::from("anshul")
            },
        );
    // sending request and waiting for response
        let response = client.send(request).await?.into_inner();
        println!("RESPONSE={:?}", response);
        Ok(())
    }
