use serde::{ser::SerializeMap, Deserialize, Serialize, Serializer};
use serde_json::{json, Value};

use crate::InitialPercept;
use crate::StepPercept;

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthRequest {
    user: String,
    pw: String,
}

#[derive(Debug)]
pub enum AuthResponse {
    Ok,
    Fail,
}
#[derive(Debug, Deserialize)]
pub struct RequestAction {
    id: u32,
    time: u64,
    deadline: u64,
    step: u32,
    percept: StepPercept,
}

#[derive(Debug)]
pub enum Message {
    AuthRequest {
        user: String,
        pw: String,
    },
    AuthResponse(AuthResponse),
    SimStart {
        time: u64,
        initial_percept: InitialPercept,
    },
    RequestAction(RequestAction),
}

impl Serialize for Message {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let (message_type, content) = match self {
            Message::AuthRequest { user, pw } => {
                let message_type = "auth-request";
                let content = json!({
                    "user": user,
                    "pw": pw,
                });
                (message_type, content)
            }
            Message::AuthResponse(_) => unreachable!(),
            Message::SimStart {
                time: _time,
                initial_percept: _percept,
            } => unreachable!(),
            Message::RequestAction(_) => unreachable!(),
        };

        let message = json!(
        {
          "type": message_type,
          "content": content
        }
        );

        message.serialize(serializer)
    }
}
