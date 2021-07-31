use crate::SkySparkClient;
use std::error::Error;
use url::Url;

/// A shortcut function to quickly create a `SkySparkClient`.
///
/// # Example
/// ```rust,no_run
/// # fn run() {
/// let mut client = raystack_blocking::new_client(
///     "https://skyspark.company.com/api/bigProject/",
///     "username",
///     "p4ssw0rd"
/// );
/// # }
/// ```
pub fn new_client(
    project_api_url: &str,
    username: &str,
    password: &str,
) -> Result<SkySparkClient, Box<dyn Error>> {
    let url = Url::parse(project_api_url)?;
    let client = SkySparkClient::new(url, username, password)?;
    Ok(client)
}
