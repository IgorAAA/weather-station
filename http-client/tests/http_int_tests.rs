#[cfg(test)]
mod integration_tests {
    use http_client::HttpClient;
    use http_client::WeatherClient;
    use http_client::config::WeatherApiConfig;
    use http_client::error::{Result, WeatherResponseError};
    use serde::Deserialize;
    use std::net::TcpListener;
    use wiremock::MockServer;
    use wiremock::matchers::query_param;
    use wiremock::{
        Mock, ResponseTemplate,
        matchers::{method, path},
    };

    #[tokio::test]
    async fn test_weather_response_success() {
        let listener = TcpListener::bind("127.0.0.1:8082").unwrap();
        // start mock server
        let server = MockServer::builder().listener(listener).start().await;

        #[derive(Deserialize, Debug, PartialEq)]
        struct TestResp {
            hello: String,
            value: i32,
        }

        let body = r#"{"hello":"world","value":42}"#;
        Mock::given(method("GET"))
            .and(path("/v1/current.json"))
            .and(query_param("key", "aaa"))
            .and(query_param("q", "London"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(body, "application/json"))
            .mount(&server)
            .await;

        let config = WeatherApiConfig {
            scheme: "http".to_string(),
            host: "localhost:8082".to_string(),
            coords: "London".to_string(),
            weather_api_key: "aaa".to_string(),
            poll_interval_secs: 0,
        };

        let client = WeatherClient::new(config).unwrap();
        let resp: TestResp = client
            .weather_response()
            .await
            .expect("weather_response failed");
        assert_eq!(
            resp,
            TestResp {
                hello: "world".into(),
                value: 42
            }
        );
    }

    #[tokio::test]
    async fn weather_response_json_error() {
        // server returns 200 but invalid JSON -> should map to WeatherResponseError::Json
        let listener = TcpListener::bind("127.0.0.1:8083").unwrap();
        let server = MockServer::builder().listener(listener).start().await;

        Mock::given(method("GET"))
            .and(path("/v1/current.json"))
            .respond_with(ResponseTemplate::new(200).set_body_string("not valid json"))
            .mount(&server)
            .await;

        let config = WeatherApiConfig {
            scheme: "http".to_string(),
            host: "localhost:8083".to_string(),
            coords: "".to_string(),
            weather_api_key: "".to_string(),
            poll_interval_secs: 0,
        };

        let client = WeatherClient::new(config).unwrap();
        let res: Result<serde_json::Value> = client.weather_response().await;
        assert!(res.is_err());
        match res.unwrap_err() {
            WeatherResponseError::Json(_) => (),
            other => panic!("expected Json error, got {:?}", other),
        }
    }
}
