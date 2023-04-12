use hyper::Uri;

use crate::Error;

/// The URI builder is creating all uris for clockify REST API.
pub struct URIBuilder {
    base_uri: Uri,
    workspaces_uri: Uri,
}

impl URIBuilder {
    /// Returns a new URI builder based on the provided base API-URI.
    ///
    /// # Arguments
    /// * `base_uri` - The based URI to the clockify API, e.g., https://api.clockify.me/api
    pub fn new(base_uri: Uri) -> Result<Self, Error> {
        if base_uri.host().is_none() {
            return Err(Error::InvalidURI(format!("URI must have a host")));
        }

        let workspaces_uri = Self::build_uri(&base_uri, "workspaces")?;

        Ok(Self {
            base_uri,
            workspaces_uri,
        })
    }

    /// Returns the server address, i.e., ${HOST}:${PORT}
    pub fn get_server_address(&self) -> String {
        let host = self.base_uri.host().expect("uri has no host");
        let port = self.base_uri.port_u16().unwrap_or(80);

        let address = format!("{}:{}", host, port);

        address
    }

    pub fn get_workspaces(&self) -> &Uri {
        &self.workspaces_uri
    }

    pub fn get_projects(&self, workspace_id: &str) -> Result<Uri, Error> {
        let sub_path = format!("workspaces/{}/projects", workspace_id);
        Self::build_uri(&self.base_uri, &sub_path)
    }

    /// Builds full api uri from the given base uri.
    ///
    /// # Arguments
    /// * `base_uri` - The base uri to the clockify.
    /// * `path` - The path being specified.
    fn build_uri(base_uri: &Uri, path: &str) -> Result<Uri, Error> {
        // get authority
        let authority = match base_uri.authority() {
            Some(a) => a.to_string(),
            None => "".to_owned(),
        };

        // retrieve full path and query
        let base_path = base_uri.path();
        let full_path = if base_path.ends_with('/') {
            format!("{}v1/{}", base_path, path)
        } else {
            format!("{}/v1/{}", base_path, path)
        };

        let full_path_and_query = match base_uri.query() {
            Some(q) => {
                format!("{}?{}", full_path, q)
            }
            None => full_path,
        };

        let full_uri = match Uri::builder()
            .scheme(base_uri.scheme_str().unwrap())
            .authority(authority)
            .path_and_query(full_path_and_query)
            .build()
        {
            Ok(uri) => uri,
            Err(err) => {
                return Err(Error::Internal(format!("Failed building URI: {}", err)));
            }
        };

        Ok(full_uri)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_uri_builder() {
        let base_uri: Uri = "https://hello.world".parse().unwrap();

        let uri_builder = URIBuilder::new(base_uri.clone()).unwrap();
        assert_eq!(
            uri_builder.get_workspaces().to_string(),
            "https://hello.world/v1/workspaces"
        );

        let base_uri: Uri = "https://hello.world/".parse().unwrap();

        let uri_builder = URIBuilder::new(base_uri.clone()).unwrap();
        assert_eq!(
            uri_builder.get_workspaces().to_string(),
            "https://hello.world/v1/workspaces"
        );

        assert_eq!(
            uri_builder.get_server_address().to_string(),
            "hello.world:80"
        );
    }
}
