//! Auth (Login/Logout) utilities for MoodleTUI

mod creds;

pub use creds::*;

pub trait AuthCredentials {
    type Name;
    type Password;

    fn name(&self) -> &Self::Name;
    fn password(&self) -> &Self::Password;
}

pub trait AuthClient<N, P>
where
    N: Sized,
    P: Sized,
{
    type Err;
    type Creds;

    fn login(&self) -> Result<Self::Creds, Self::Err>;
    fn logout(&self) -> Result<(), Self::Err>;
    fn get_auth_creds(&self) -> Box<dyn AuthCredentials<Name = N, Password = P>>;
}

pub enum AuthError {}
