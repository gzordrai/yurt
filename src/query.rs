use std::fmt::{Display, Formatter, Result};

use serde::Serialize;

/// Query parameters used when requesting build orders from the API
///
/// This struct is serialized to URL query parameters using Serde
/// Each field is optional — absent fields are omitted from the query string
#[derive(Debug, Serialize)]
pub struct Query {
    /// Civilization filter (`?civ=FRE`)
    ///
    /// When `None`, the request applies to *all* civilizations
    #[serde(skip_serializing_if = "Option::is_none")]
    civ: Option<String>,

    /// Sorting criterion (`?orderBy=score`)
    ///
    /// Corresponds to [`SortBy`]. When `None`, the API uses its default sort order
    #[serde(rename = "orderBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    order_by: Option<String>,

    /// Whether to include overlay-specific data in the response (`?overlay=true`)
    ///
    /// The parameter is omitted when `false`, making `false` the implicit default
    #[serde(skip_serializing_if = "is_false")]
    overlay: bool,
}

impl Query {
    /// Builds a [`Query`] from high-level enum values
    ///
    /// Automatically skips `"Any"` civilizations and converts enums to
    /// their API string representations.
    ///
    /// # Example
    /// ```
    /// use yurt::{Civilization, Query, SortBy};
    ///
    /// let query = Query::from_parts(Civilization::Mon, Some(SortBy::Score), false);
    /// // => ?civ=FRE&orderBy=score
    /// ```
    pub fn from_parts(civ: Civilization, order_by: Option<SortBy>, overlay: bool) -> Self {
        Self {
            civ: match civ {
                Civilization::Any => None,
                other => Some(other.to_string()),
            },
            order_by: order_by.map(|o| o.to_string()),
            overlay,
        }
    }
}

/// Helper function used by Serde to omit `overlay` when `false`.
fn is_false(b: &bool) -> bool {
    !*b
}

/// Enumeration of available civilizations recognized by the API
///
/// Each variant corresponds to a civilization code used in `/builds` queries
/// and build order metadata
#[derive(Debug, Clone)]
pub enum Civilization {
    /// Any civilization (no filter)
    Any,

    /// Abbasid Dynasty
    Abb,

    /// Ayyubids
    Ayy,

    /// Byzantines
    Byz,

    /// Chinese
    Chi,

    /// Delhi Sultanate
    Del,

    /// English
    Eng,

    /// French
    Fre,

    /// Golden Horde
    Gol,

    /// House of Lancaster
    Hol,

    /// Holy Roman Empire
    Hre,

    /// Japanese
    Jap,

    /// Jeanne d'Arc
    Jda,

    /// Knights Templar
    Kte,

    /// Macedonian Dynasty
    Mac,

    /// Malians
    Mal,

    /// Mongols
    Mon,
    /// Order of the Dragon
    Dra,

    /// Ottomans
    Ott,

    /// Rus
    Rus,

    /// Sengoku Daimyo
    Sen,

    /// Tughlaq Dynasty
    Tug,

    /// Zhu Xi's Legacy
    Zxl,
}

impl Display for Civilization {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let s = match self {
            Civilization::Any => "ANY",
            Civilization::Abb => "ABB",
            Civilization::Ayy => "AYY",
            Civilization::Byz => "BYZ",
            Civilization::Chi => "CHI",
            Civilization::Del => "DEL",
            Civilization::Eng => "ENG",
            Civilization::Fre => "FRE",
            Civilization::Gol => "GOL",
            Civilization::Hol => "HOL",
            Civilization::Hre => "HRE",
            Civilization::Jap => "JAP",
            Civilization::Jda => "JDA",
            Civilization::Kte => "KTE",
            Civilization::Mac => "MAC",
            Civilization::Mal => "MAL",
            Civilization::Mon => "MON",
            Civilization::Dra => "DRA",
            Civilization::Ott => "OTT",
            Civilization::Rus => "RUS",
            Civilization::Sen => "SEN",
            Civilization::Tug => "TUG",
            Civilization::Zxl => "ZXL",
        };

        f.write_str(s)
    }
}

/// Available sorting criteria for build order queries.
///
/// Determines the order in which build orders are returned by the API
/// Example: `?orderBy=score`
#[derive(Debug)]
pub enum SortBy {
    /// Sort by dynamic score
    Score,

    /// Sort by creation time (newest first)
    TimeCreated,

    /// Sort by total number of views.
    Views,

    /// Sort by number of likes.
    Likes,
}

impl Display for SortBy {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let s = match self {
            SortBy::Score => "score",
            SortBy::TimeCreated => "timeCreated",
            SortBy::Views => "views",
            SortBy::Likes => "likes",
        };

        f.write_str(s)
    }
}
