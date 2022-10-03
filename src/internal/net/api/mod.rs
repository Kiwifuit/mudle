// Copyright (c) 2022 Misery <mahkiwi123@gmail.com>
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! Basic Moodle API utilities

mod auth;
mod course;

pub struct MoodleClient {
    cookie: String,
}

impl auth::AuthClient<String, String> for MoodleClient {
    type Err = auth::AuthError;
    type Creds = auth::MoodleCreds;

    fn login(&self) -> Result<Self::Creds, Self::Err> {
        todo!()
    }

    fn logout(&self) -> Result<(), Self::Err> {
        todo!()
    }

    fn get_auth_creds(&self) -> Box<dyn auth::AuthCredentials<Name = String, Password = String>> {
        todo!()
    }
}
