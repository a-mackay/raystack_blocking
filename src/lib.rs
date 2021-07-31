use chrono::Utc;
pub use raystack::{
    is_tag_name, skyspark_tz_string_to_tz, BasicNumber, Coord, Date, DateTime, Error,
    FromHaysonError, Grid, Hayson, HisReadRange, Marker, Na, NewSkySparkClientError, Number,
    ParseJsonGridError, ParseRefError, ParseTagNameError, Ref, RemoveMarker, ScientificNumber,
    Symbol, TagName, Time, Uri, ValueExt, Xstr,
};
use std::sync::Arc;
use tokio::runtime::Runtime;
use url::Url;

pub mod auth;

type Result<T> = std::result::Result<T, Error>;

/// A client for interacting with a SkySpark server.
#[derive(Debug)]
pub struct SkySparkClient {
    client: raystack::SkySparkClient,
    rt: Arc<Runtime>,
}

impl SkySparkClient {
    /// Create a new `SkySparkClient`.
    ///
    /// # Example
    /// ```rust,no_run
    /// # fn run() {
    /// use raystack_blocking::SkySparkClient;
    /// use url::Url;
    /// let url = Url::parse("https://skyspark.company.com/api/bigProject/").unwrap();
    /// let mut client = SkySparkClient::new(url, "username", "p4ssw0rd").unwrap();
    /// # }
    /// ```
    pub fn new(
        project_api_url: Url,
        username: &str,
        password: &str,
    ) -> std::result::Result<Self, NewSkySparkClientError> {
        let rt = Runtime::new().expect("could not create a new Tokio runtime");
        Self::new_with_runtime(project_api_url, username, password, Arc::new(rt))
    }

    /// Create a new `SkySparkClient` using an existing Tokio runtime.
    pub fn new_with_runtime(
        project_api_url: Url,
        username: &str,
        password: &str,
        rt: Arc<Runtime>,
    ) -> std::result::Result<Self, NewSkySparkClientError> {
        let rclient = reqwest::Client::new();
        let client = rt.block_on(raystack::SkySparkClient::new_with_client(
            project_api_url,
            username,
            password,
            rclient,
        ))?;
        Ok(Self { client, rt })
    }

    /// Return the project name for this client.
    pub fn project_name(&self) -> &str {
        self.client.project_name()
    }

    /// Return the project API url being used by this client.
    pub fn project_api_url(&self) -> &Url {
        self.client.project_api_url()
    }
}

impl SkySparkClient {
    /// Returns a grid containing basic server information.
    pub fn about(&mut self) -> Result<Grid> {
        self.rt.block_on(self.client.about())
    }

    /// Returns a grid describing what MIME types are available.
    pub fn formats(&mut self) -> Result<Grid> {
        self.rt.block_on(self.client.formats())
    }

    /// Returns a grid of history data for a single point.
    pub fn his_read(&mut self, id: &Ref, range: &HisReadRange) -> Result<Grid> {
        self.rt.block_on(self.client.his_read(id, range))
    }

    /// Writes boolean values to a single point.
    pub fn his_write_bool(&mut self, id: &Ref, his_data: &[(DateTime, bool)]) -> Result<Grid> {
        self.rt.block_on(self.client.his_write_bool(id, his_data))
    }

    /// Writes numeric values to a single point. `unit` must be a valid
    /// Haystack unit literal, such as `L/s` or `celsius`.
    pub fn his_write_num(&mut self, id: &Ref, his_data: &[(DateTime, Number)]) -> Result<Grid> {
        self.rt.block_on(self.client.his_write_num(id, his_data))
    }

    /// Writes string values to a single point.
    pub fn his_write_str(&mut self, id: &Ref, his_data: &[(DateTime, String)]) -> Result<Grid> {
        self.rt.block_on(self.client.his_write_str(id, his_data))
    }

    /// Writes boolean values with UTC timestamps to a single point.
    /// `time_zone_name` must be a valid SkySpark timezone name.
    pub fn utc_his_write_bool(
        &mut self,
        id: &Ref,
        time_zone_name: &str,
        his_data: &[(chrono::DateTime<Utc>, bool)],
    ) -> Result<Grid> {
        self.rt
            .block_on(self.client.utc_his_write_bool(id, time_zone_name, his_data))
    }

    /// Writes numeric values with UTC timestamps to a single point.
    /// `unit` must be a valid Haystack unit literal, such as `L/s` or
    /// `celsius`.
    /// `time_zone_name` must be a valid SkySpark timezone name.
    pub fn utc_his_write_num(
        &mut self,
        id: &Ref,
        time_zone_name: &str,
        his_data: &[(chrono::DateTime<Utc>, Number)],
    ) -> Result<Grid> {
        self.rt
            .block_on(self.client.utc_his_write_num(id, time_zone_name, his_data))
    }

    /// Writes string values with UTC timestamps to a single point.
    /// `time_zone_name` must be a valid SkySpark timezone name.
    pub fn utc_his_write_str(
        &mut self,
        id: &Ref,
        time_zone_name: &str,
        his_data: &[(chrono::DateTime<Utc>, String)],
    ) -> Result<Grid> {
        self.rt
            .block_on(self.client.utc_his_write_str(id, time_zone_name, his_data))
    }

    /// The Haystack nav operation.
    pub fn nav(&mut self, nav_id: Option<&Ref>) -> Result<Grid> {
        self.rt.block_on(self.client.nav(nav_id))
    }

    /// Returns a grid containing the operations available on the server.
    pub fn ops(&mut self) -> Result<Grid> {
        self.rt.block_on(self.client.ops())
    }

    /// Returns a grid containing the records matching the given Axon
    /// filter string.
    pub fn read(&mut self, filter: &str, limit: Option<u64>) -> Result<Grid> {
        self.rt.block_on(self.client.read(filter, limit))
    }

    /// Returns a grid containing the records matching the given id
    /// `Ref`s.
    pub fn read_by_ids(&mut self, ids: &[Ref]) -> Result<Grid> {
        self.rt.block_on(self.client.read_by_ids(ids))
    }
}

impl SkySparkClient {
    pub fn eval(&mut self, axon_expr: &str) -> Result<Grid> {
        self.rt.block_on(self.client.eval(axon_expr))
    }
}
