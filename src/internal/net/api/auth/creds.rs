// Copyright (c) 2022 Misery <mahkiwi123@gmail.com>
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use super::AuthCredentials;

pub struct MoodleCreds {
    user: String,
    password: String,
}

impl AuthCredentials for MoodleCreds {
    type Name = String;
    type Password = String;

    fn name(&self) -> &Self::Name {
        &self.user
    }

    fn password(&self) -> &Self::Password {
        &self.password
    }
}
