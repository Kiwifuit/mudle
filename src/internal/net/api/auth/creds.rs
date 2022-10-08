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
