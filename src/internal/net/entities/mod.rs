// Copyright (c) 2022 Misery <mahkiwi123@gmail.com>
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! Structures and Enums that abstract the contents of a course

mod course;
mod resources;
mod status;

// Bring the rest of the entities into scope
pub use course::*;
pub use resources::*;
pub use status::*;

/// Represents a quiz, whether or not you've answered it and its score
pub struct Quiz {
    id: u16,
    answered: bool,
    score: [u8; 2],
}

/// Represents a lesson and its contents
pub struct Lesson {
    id: u16,
    content: String,
}

/// Reprensents a downloadable resource and its type
pub struct Resource {
    id: u16,
    uri: String,
    rstype: ResourceType,
}

/// Represents an assignment whose state you can query
pub struct Assignment {
    id: u16,
    state: SubmissionStatus,
}

/// Represents an entire course in the Moodle
pub struct Course {
    id: u16,
    items: Vec<CourseItem>,
}
