//! Request Validation Structs.
//! Define structures that allow us to deserialize and validate the request bodies.
//! Validation ensures that the necessary fields are present in the request body and that they have the correct data types.

use serde::{Deserialize, Serialize};

/// Specify filtering and pagination options for the results
/// Avoids sending large payloads in the JSON response when dealing with databases that contain a large number of note elements.
#[derive(Debug, Deserialize, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

/// It is used as a type for an extractor through which the id of a specific article is sent.
#[derive(Debug, Deserialize)]
pub struct ParamsOptions {
    pub id: String,
}

/// It is used as the type for an extractor through which data is sent to create a note.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNoteSchema {
    pub title: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<bool>,
}

/// Used as the type for an extractor through which data is sent to update a note.
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateNoteSchema {
    pub title: Option<String>,
    pub content: Option<String>,
    pub category: Option<String>,
    pub published: Option<bool>,
}
