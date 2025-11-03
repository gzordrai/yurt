use reqwest::{Client, Error};

use crate::{
    Civilization, SortBy,
    query::Query,
    types::{BuildOrder, BuildOrders, Status},
};

const BASE_URI: &str = "https://aoe4guides.com/api";

pub struct OrdaClient {
    http: Client,
}

impl OrdaClient {
    /// Creates a new [`OrdaClient`] instance using the default API base URL
    ///
    /// This constructor initializes a reusable `reqwest::Client`
    /// that maintains connection pools and reduces overhead
    ///
    /// # Example
    /// ```
    /// use orda::OrdaClient;
    ///
    /// let client = OrdaClient::new();
    /// ```
    pub fn new() -> Self {
        Self {
            http: Client::new(),
        }
    }

    /// Fetches basic API status information
    ///
    /// This endpoint (`/status`) is a simple health indicator that currently **always returns**
    /// a [`Status`] with `status = "running"`
    ///
    /// # Returns
    /// A [`Status`] struct containing a single field:
    /// ```json
    /// { "status": "running" }
    /// ```
    ///
    /// # Example
    /// ```
    /// use orda::OrdaClient;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = OrdaClient::new();
    /// let status = client.get_status().await?;
    ///
    /// assert_eq!(status.status, "running");
    /// println!("API is {}", status.status);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_status(&self) -> Result<Status, Error> {
        self.http
            .get(format!("{BASE_URI}/status"))
            .send()
            .await?
            .json::<Status>()
            .await
    }

    /// Fetches a list of build orders (maximum 10 results per request)
    ///
    /// This method calls the `/builds` endpoint, which returns at most **10 build orders**
    /// per query
    ///
    /// - `civ`: civilization filter (use [`Civilization::Any`] for no filter)
    /// - `order_by`: optional sorting criterion (e.g. [`SortBy::Score`], [`SortBy::Views`])
    /// - `overlay`: if `true`, requests overlay-friendly data (used by stream overlays)
    ///
    /// # Returns
    /// A [`Vec`] of up to 10 [`BuildOrder`] objects.
    ///
    /// # Example
    /// ```
    /// use orda::{OrdaClient, Civilization, SortBy};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = OrdaClient::new();
    ///
    /// // Fetch the top 10 French builds sorted by score
    /// let builds = client.get_builds(Civilization::Fre, Some(SortBy::Score), false).await?;
    ///
    /// println!("Fetched {} builds (API max = 10)", builds.len());
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_builds(
        &self,
        civ: Civilization,
        order_by: Option<SortBy>,
        overlay: bool,
    ) -> Result<BuildOrders, Error> {
        let query = Query::from_parts(civ, order_by, overlay);

        self.http
            .get(format!("{BASE_URI}/builds"))
            .query(&query)
            .send()
            .await?
            .json::<BuildOrders>()
            .await
    }

    /// Fetches a single build order by its unique ID
    ///
    /// - `build_id`: the identifier string for the build
    /// - `overlay`: if `true`, requests overlay-friendly data (used by stream overlays)
    ///
    /// # Returns
    /// A single [`BuildOrder`] object
    ///
    /// # Example
    /// ```
    /// use orda::OrdaClient;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = OrdaClient::new();
    ///
    /// let build = client.get_build("00I7J47dv26cPbKmXYkO", false).await?;
    /// println!("Build title: {}", build.title.unwrap_or_default());
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_build(
        &self,
        build_id: impl Into<String>,
        overlay: bool,
    ) -> Result<BuildOrder, Error> {
        let build_id = build_id.into();
        let query = Query::from_parts(Civilization::Any, None, overlay);

        self.http
            .get(format!("{BASE_URI}/builds/{build_id}"))
            .query(&query)
            .send()
            .await?
            .json::<BuildOrder>()
            .await
    }

    /// Fetches the favorite build orders for a given user
    ///
    /// - `user_id`: the platform user ID
    /// - `civ`: civilization filter (use [`Civilization::Any`] for no filter)
    /// - `order_by`: optional sorting criterion (e.g. [`SortBy::Score`], [`SortBy::Views`])
    /// - `overlay`: if `true`, requests overlay-friendly data (used by stream overlays)
    ///
    /// # Returns
    /// A [`Vec`] of [`BuildOrder`] objects representing the user’s favorites
    ///
    /// # Example
    /// ```no_run
    /// use orda::{OrdaClient, Civilization, SortBy};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = OrdaClient::new();
    ///
    /// let favorites = client
    ///     .get_favorites("vOiAUO06vkMXuPuYb92APdyLDUO2", Civilization::Any, None, false)
    ///     .await?;
    ///
    /// println!("User has {} favorite builds", favorites.len());
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_favorites(
        &self,
        user_id: impl Into<String>,
        civ: Civilization,
        order_by: Option<SortBy>,
        overlay: bool,
    ) -> Result<BuildOrders, Error> {
        let user_id = user_id.into();
        let query = Query::from_parts(civ, order_by, overlay);

        self.http
            .get(format!("{BASE_URI}/favorites/{user_id}"))
            .query(&query)
            .send()
            .await?
            .json::<BuildOrders>()
            .await
    }
}

impl Default for OrdaClient {
    fn default() -> Self {
        Self::new()
    }
}
