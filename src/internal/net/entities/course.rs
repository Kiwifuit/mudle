// Copyright (c) 2022 Misery <mahkiwi123@gmail.com>
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use super::{Assignment, Lesson, Quiz, Resource};

/// Represents the possible things a course can have
pub enum CourseItem {
    Assignment,
    Lesson,
    Quiz,
    Resource,
}

impl From<Assignment> for CourseItem {
    fn from(_: Assignment) -> Self {
        Self::Assignment
    }
}

impl From<Quiz> for CourseItem {
    fn from(_: Quiz) -> Self {
        Self::Quiz
    }
}

impl From<Lesson> for CourseItem {
    fn from(_: Lesson) -> Self {
        Self::Lesson
    }
}

impl From<Resource> for CourseItem {
    fn from(_: Resource) -> Self {
        Self::Resource
    }
}

impl From<String> for CourseItem {
    fn from(s: String) -> Self {
        if let Some(_) = s.find("quiz") {
            Self::Quiz
        } else if let Some(_) = s.find("page") {
            Self::Lesson
        } else if let Some(_) = s.find("assign") {
            Self::Assignment
        } else {
            Self::Resource
        }
    }
}
