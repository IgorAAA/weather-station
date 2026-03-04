use crate::config::DbConfig;
use crate::error::Result;
use influxdb::{Client, InfluxDbWriteable};
use log::debug;

/// Influx Client wrapper
pub struct InfluxClient {
    client: Client,
}

impl InfluxClient {
    pub fn new(config: DbConfig) -> Self {
        Self {
            client: Client::new(
                format!("http://{}:{}", config.host, config.port),
                config.database,
            )
            .with_token(config.token),
        }
    }

    pub async fn write_to_influx_db<A: InfluxDbWriteable>(
        &self,
        query_name: &str,
        payload: A,
    ) -> Result<()> {
        let query = payload.into_query(query_name);
        let res = self.client.query(query).await?;
        debug!("Response: {:?}", res);
        Ok(())
    }
}
