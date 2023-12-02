extern crate petstore_api;

#[cfg(test)]
mod tests {
    use petstore_api::apis::configuration::Configuration;
    use petstore_api::apis::pets_api::{create_pets, list_pets, show_pet_by_id};

    fn test_config() -> Configuration {
        // mocking the server with prism

        let client = reqwest::ClientBuilder::new();
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        // prefer:curl localhost:4010/pets/1 -H "Prefer: dynamic=true"
        headers.insert(
            reqwest::header::HeaderName::from_static("prefer"),
            reqwest::header::HeaderValue::from_static("dynamic=true"),
        );

        let client = client
            .default_headers(headers)
            .build()
            .expect("failed to build reqwest client");

        Configuration {
            base_path: "http://localhost:4010".to_string(),
            client,
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn test_create_pets() {
        let config = test_config();
        let result = create_pets(&config).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_pets() {
        let config = test_config();
        let result = list_pets(&config, None).await;

        assert!(result.is_ok());

        let pets = result.unwrap();
        assert_eq!(pets.len(), 100);

        let pet = &pets[0];
        assert!(!pet.name.is_empty());
    }

    #[tokio::test]
    async fn test_show_pet_by_id() {
        let config = test_config();
        let result = show_pet_by_id(&config, "1").await;

        assert!(result.is_ok());

        let pet = result.unwrap();
        assert!(!pet.name.is_empty());
    }
}
