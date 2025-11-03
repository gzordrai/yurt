use serde::Deserialize;

/// Simple status payload returned by health/check endpoints
#[derive(Debug, Deserialize)]
pub struct Status {
    pub status: String,
}

/// Represents a complete build order as returned by the API
#[derive(Debug, Deserialize)]
pub struct BuildOrder {
    /// The auto-generated id of the build order
    pub id: Option<String>,

    /// The title of the build order
    pub title: Option<String>,

    /// The description of the build order
    pub description: Option<String>,

    /// The youtube url of the build order
    pub video: Option<String>,

    /// The author name
    pub author: Option<String>,

    /// The author's id
    #[serde(rename = "authorUid")]
    pub author_uid: String,

    /// The civilization this build order is designed for
    pub civ: Option<String>,

    /// The number of comments left by users
    pub comments: Option<i64>,

    /// The number of likes the build order received
    pub likes: Option<i64>,

    /// The map this build order targets (if specified)
    pub map: Option<String>,

    /// The original ranked ladder season the build order was created for
    pub season: Option<String>,

    /// The current dynamic score assigned by the platform
    pub score: f64,

    /// The all-time cumulative score
    #[serde(rename = "scoreAllTime")]
    pub score_all_time: f64,

    /// A sortable title string used for ranking or display order
    #[serde(rename = "sortTitle")]
    pub sort_title: String,

    /// The ordered sequence of build steps that make up this build.
    pub steps: Vec<BuildOrderStep>,

    /// The general strategic focus of the build order
    pub strategy: Option<String>,

    /// Timestamp when the build order was created
    #[serde(rename = "timeCreated")]
    pub time_created: Timestamp,

    /// Timestamp when the build order was last updated.
    #[serde(rename = "timeUpdated")]
    pub time_updated: Timestamp,

    /// Number of upvotes
    pub upvotes: Option<i64>,

    /// The total number of views for this build order
    pub views: i64,

    /// Whether the build order is still a draft (unpublished)
    #[serde(rename = "isDraft")]
    pub is_draft: Option<bool>,
}

/// A timestamp object used by the API
#[derive(Debug, Deserialize)]
pub struct Timestamp {
    /// The Unix timestamp in seconds.
    #[serde(rename = "_seconds")]
    pub seconds: i64,

    /// The fractional part of the timestamp in nanoseconds
    #[serde(rename = "_nanoseconds")]
    pub nanoseconds: i64,
}

/// A sequence of steps representing a single phase of a build order
#[derive(Debug, Deserialize)]
pub struct BuildOrderStep {
    /// A short description or label summarizing this step’s strategy
    pub gameplan: Option<String>,

    /// Age indicator. 0 if feature is not used. 1-4 otherwise
    pub age: Option<u8>,

    #[serde(rename = "type")]
    /// age when "in age" or ageUp when "aging up"
    pub step_type: Option<String>,

    /// Build order steps
    pub steps: Option<Vec<DetailStep>>,
}

/// Container for the actual build steps
#[derive(Debug, Deserialize)]
pub struct DetailStep {
    /// Number of villagers
    pub villagers: Option<String>,

    /// Number of builders
    pub builders: Option<String>,

    /// Number of villagers assigned to food
    pub food: Option<String>,

    /// Number of villagers assigned to wood
    pub wood: Option<String>,

    /// Number of villagers assigned to stone
    pub stone: Option<String>,

    /// Number of villagers assigned to gold
    pub gold: Option<String>,

    /// Textual timestamp
    pub time: Option<String>,

    /// Textual description of the build step including images
    pub description: Option<String>,
}

/// A list of build orders returned by the API
pub type BuildOrders = Vec<BuildOrder>;
