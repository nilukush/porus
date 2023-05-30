// tests.rs

#[cfg(test)]
mod tests {
    // Import the PocketSdk and models modules from the crate root
    use httpmock::Method;
    use httpmock::MockServer;
    use porus::pocket_sdk::PocketSdk;

    #[tokio::test]
    async fn test_obtain_request_token() {
        let pocket_sdk = PocketSdk::new("80908-b39061ed0999bb292f0fe716".to_string());
        let redirect_uri = "http://example.com"; // Specify the redirect URI here
        let result = pocket_sdk.obtain_request_token(redirect_uri).await;

        assert!(result.is_ok());
        let request_token_response = result.unwrap();
        println!("Request token response: {:?}", request_token_response);
    }

    #[tokio::test]
    async fn test_convert_request_token_to_access_token() {
        let pocket_sdk = PocketSdk::new("80908-b39061ed0999bb292f0fe716".to_string());

        // Step 1: Obtain the request token
        let result = pocket_sdk.obtain_request_token("http:://example.com").await;
        assert!(result.is_ok());
        let request_token_response = result.unwrap();
        println!("Request token response: {:?}", request_token_response);

        // Step 2: Build the authorization URL
        let request_token = request_token_response.code;
        let authorization_url =
            pocket_sdk.build_authorization_url(&request_token, "http:://example.com");
        println!("Authorization URL: {}", authorization_url);

        // Step 3: Instruct the user to open the authorization URL and authorize the application

        // Step 4: Convert the request token to an access token
        println!("Please authorize the application and press Enter to continue...");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let result = pocket_sdk
            .convert_request_token_to_access_token(&request_token)
            .await;
        assert!(result.is_ok());
        let access_token_response = result.unwrap();
        println!("Access token response: {:?}", access_token_response);
    }

    #[tokio::test]
    async fn test_get_tags_with_article_count() {
        // Initialize the PocketSdk with a dummy consumer key
        let pocket_sdk = PocketSdk::new("80908-b39061ed0999bb292f0fe716".to_string());

        // Mock the HTTP response body
        let response_body = r#"{"status":1,"complete":1,"list":{"30211549":{"item_id":"30211549","resolved_id":"30211549","given_url":"https:\/\/www.homeremediesweb.com","given_title":"","favorite":"0","status":"0","time_added":"1537696723","time_updated":"1537696723","time_read":"0","time_favorited":"0","sort_id":0,"resolved_title":"Good Health Starts At Home","resolved_url":"https:\/\/www.homeremediesweb.com\/","excerpt":"Welcome to Home Remedies Web, one of the premier online resources for finding free home remedies from around the world. Natural remedies, home remedies, and herbal supplements are growing in popularity as more and more people discover the magical benefits of these forms of alternative medicine.","is_article":"0","is_index":"1","has_video":"0","has_image":"1","word_count":"96","lang":"en","top_image_url":"https:\/\/www.homeremediesweb.com\/assets\/img\/logo_social.jpg","listen_duration_estimate":37}},"error":null,"search_meta":{"search_type":"normal"},"since":1685373830}"#;

        // Create a mock server
        let server = MockServer::start();

        // Create a mock response
        let _mock = server.mock(|when, then| {
            when.method(Method::POST)
                .path("/get")
                .header("Content-Type", "application/json; charset=UTF-8");
            then.status(200)
                .header("Content-Type", "application/json; charset=UTF-8")
                .body(response_body);
        });

        // Perform the test
        let access_token = "f52be2ce-e2e1-366d-0acb-b19670";
        let result = pocket_sdk.get_tags_with_article_count(access_token).await;

        if let Err(err) = &result {
            eprintln!("Error: {}", err);
        }

        assert!(result.is_ok());
        let tags_with_article_count = result.unwrap();
        println!("Tags with article count: {:?}", tags_with_article_count);
    }
}
