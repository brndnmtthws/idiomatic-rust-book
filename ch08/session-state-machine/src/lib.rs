use std::collections::HashMap;
use std::marker::PhantomData;
use uuid::Uuid;

pub trait SessionState {}

#[derive(Debug)]
pub struct Session<State: SessionState = Initial> {
    session_id: Uuid,
    props: HashMap<String, String>,
    phantom: PhantomData<State>,
}

#[derive(Debug, Default)]
pub struct Initial;
#[derive(Debug, Default)]
pub struct Anonymous;
#[derive(Debug, Default)]
pub struct Authenticated;
#[derive(Debug, Default)]
pub struct LoggedOut;

impl SessionState for Initial {}
impl SessionState for Anonymous {}
impl SessionState for Authenticated {}
impl SessionState for LoggedOut {}

#[derive(Debug)]
pub enum ResumeResult {
    Invalid(Session<Initial>),
    Anonymous(Session<Anonymous>),
    Authenticated(Session<Authenticated>),
}

impl Session<Initial> {
    /// Returns a new session, defaulting to the anonymous state
    pub fn new() -> Session<Anonymous> {
        Session::<Anonymous> {
            session_id: Uuid::new_v4(),
            props: HashMap::new(),
            phantom: PhantomData,
        }
    }
    /// Returns the result of resuming this session from an existing ID.
    pub fn resume_from(session_id: Uuid) -> ResumeResult {
        // Here we'd have to check the session_id against a database,
        // and return the result accordingly. For this example we'll
        // just return a new authenticated session for test purposes.
        ResumeResult::Authenticated(Session::<Authenticated> {
            session_id,
            props: HashMap::new(),
            phantom: PhantomData,
        })
    }
}

impl Session<Anonymous> {
    pub fn authenticate(
        self,
        username: &str,
        password: &str,
    ) -> Result<Session<Authenticated>, Session<Anonymous>> {
        // Here we would perform the authentication process,
        // but we're just simulating that in this example.
        if !username.is_empty() && !password.is_empty() {
            Ok(Session::<Authenticated> {
                session_id: self.session_id,
                props: HashMap::new(),
                phantom: PhantomData,
            })
        } else {
            Err(self)
        }
    }
}

impl Session<Authenticated> {
    fn update_property(&mut self, key: &str, value: &str) {
        if let Some(prop) = self.props.get_mut(key) {
            *prop = value.to_string();
        } else {
            self.props.insert(key.to_string(), value.to_string());
        }
        // Store props in DB
    }
    fn logout(self) -> Session<LoggedOut> {
        // Delete session from DB
        Session {
            session_id: Uuid::nil(),
            props: HashMap::new(),
            phantom: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Session;

    #[test]
    fn new_session() {
        let session = Session::new();
        println!("{:?}", session);
    }

    #[test]
    fn test_transitions() {
        let session = Session::new();
        println!("{:?}", session);
        if let Ok(mut session) =
            session.authenticate("username", "password")
        {
            session.update_property("some.preference.bool", "true");
            println!("{:?}", session);
            let session = session.logout();
            println!("{:?}", session);
        }
    }
}
