use crate::{uri_builder::URIBuilder, Config, Error, ProjectHeader, WorkspaceHeader};

use hyper::{body::Buf, client::HttpConnector, Body, Method, Request, StatusCode, Uri};
use hyper_tls::HttpsConnector;

/// A single connection to clockify.
pub struct Client {
    /// The configuration for the client to
    config: Config,

    /// The URI builder for the clockify REST API Urls
    uri_builder: URIBuilder,

    /// The http client
    http_client: hyper::Client<HttpsConnector<HttpConnector>>,
}

impl Client {
    /// Creates a new clockify client
    pub async fn new(config: Config) -> Result<Self, Error> {
        let connector = HttpsConnector::new();
        let http_client = hyper::Client::builder().build::<_, hyper::Body>(connector);

        let endpoint: Uri = config.endpoint.parse()?;
        let uri_builder = URIBuilder::new(endpoint)?;

        Ok(Self {
            config,
            uri_builder,
            http_client,
        })
    }

    /// Returns a list of the existing workspaces.
    pub async fn get_workspaces(&self) -> Result<Vec<WorkspaceHeader>, Error> {
        let uri = self.uri_builder.get_workspaces();
        self.get_resource(uri).await
    }

    /// Returns the list of existing projects within the specified workspace.
    ///
    /// # Arguments
    /// * `workspace_id` - The workspace whose projects will be returned.
    pub async fn get_projects(&self, workspace_id: &str) -> Result<Vec<ProjectHeader>, Error> {
        let uri = self.uri_builder.get_projects(workspace_id)?;
        self.get_resource(&uri).await
    }

    /// Sends a GET request to the specified URI to deserialize a json afterwards.
    ///
    /// # Arguments
    /// * `uri` - The URI to which the request will be sent.
    pub async fn get_resource<T>(&self, uri: &Uri) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        let req = match Request::builder()
            .method(Method::GET)
            .uri(uri)
            .header("x-api-key", &self.config.api_key)
            .body(Body::empty())
        {
            Ok(req) => req,
            Err(err) => {
                return Err(Error::Internal(format!(
                    "Failed to build body due to {}",
                    err
                )));
            }
        };

        let response = match self.http_client.request(req).await {
            Ok(response) => response,
            Err(err) => {
                return Err(Error::IO(format!(
                    "Failed to send request due to '{}'",
                    err
                )));
            }
        };

        if response.status() != StatusCode::OK {
            return Err(Error::RestAPI(format!(
                "Returned status code {}",
                response.status()
            )));
        }

        let body_bytes = hyper::body::to_bytes(response.into_body()).await?;

        let s: T = serde_json::from_reader(body_bytes.reader())?;

        Ok(s)
    }
}
