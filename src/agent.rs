use std::net::{TcpStream, ToSocketAddrs};
use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};
use std::{
    io::{prelude::*, BufWriter},
    u32,
};

use serde::{ser::SerializeMap, Deserialize, Serialize, Serializer};
use serde_json::{json, Value};

use crate::messages::*;
use crate::*;

struct AgentStreams {
    stream: TcpStream,
    //writer: BufWriter<&'a TcpStream>,
    //reader: BufReader<&'a TcpStream>,
}

impl AgentStreams {
    fn send(&mut self, message: &Message) -> std::io::Result<()> {
        let mut serialized = serde_json::to_vec(&message).unwrap();
        serialized.push(0x00);

        //eprintln!("writting...");
        let mut writer = BufWriter::new(&self.stream);
        writer.write_all(&serialized)?;
        writer.flush()?;
        //eprintln!("ok");
        Ok(())
    }

    fn recv(&mut self) -> Result<Message, Box<dyn std::error::Error>> {
        //eprint!("reading...");
        let mut buf = Vec::new();
        let mut reader = BufReader::new(&self.stream);
        reader.read_until(b'\0', &mut buf)?;
        //eprint!("read ok");
        buf.pop(); // 0x00

        let message: serde_json::Value = serde_json::from_slice(&buf)?;
        //dbg!(&message);

        let content = message.get("content").ok_or("No content")?;
        let message: Message = match message
            .get("type")
            .ok_or("No type")?
            .as_str()
            .ok_or("Not a str")?
        {
            "auth-request" => unimplemented!(),
            "auth-response" => {
                let result = content
                    .get("result")
                    .ok_or("No result")?
                    .as_str()
                    .ok_or("Not a str")?;
                let result = match result {
                    "ok" => AuthResponse::Ok,
                    "fail" => AuthResponse::Fail,
                    _ => unreachable!(),
                };
                Message::AuthResponse(result)
            }
            "sim-start" => {
                let time = content
                    .get("time")
                    .ok_or("no time key")?
                    .as_u64()
                    .ok_or("not a number")?;
                let percept: InitialPercept =
                    serde_json::from_value(content.get("percept").ok_or("No percept")?.clone())?;

                Message::SimStart {
                    time,
                    initial_percept: percept,
                }
            }
            "request-action" => {
                let request_action: RequestAction = serde_json::from_value(content.clone())?;
                Message::RequestAction(request_action)
            }
            //"action" => unimplemented!(),
            //"sim-end" => (),
            //"sim-bye" => (),
            //"status-request" => unimplemented!(),
            //"status-response" => (),
            x => unreachable!(x),
        };

        Ok(message)
    }
}

pub struct AgentStarter {
    streams: AgentStreams,
}

pub struct AgentAuthenticator {
    streams: AgentStreams,
}

impl AgentAuthenticator {
    pub fn auth<A: ToSocketAddrs>(
        addr: A,
        user: &str,
        pw: &str,
    ) -> Result<AgentStarter, Box<dyn std::error::Error>> {
        let tcp_stream = TcpStream::connect(addr).unwrap();
        //.set_read_timeout(Some(std::time::Duration::from_millis(1000)))
        let mut streams = AgentStreams { stream: tcp_stream };

        let message = Message::AuthRequest {
            user: user.into(),
            pw: pw.into(),
        };
        streams.send(&message)?;

        let m = streams.recv()?;
        if let Message::AuthResponse(AuthResponse::Ok) = m {
            println!("Auth OK!");
            Ok(AgentStarter { streams })
        } else {
            Err("Auth failed".into())
        }
    }
}

impl AgentStarter {
    pub fn wait_for_start(mut self) -> Result<Agent, Box<dyn std::error::Error>> {
        if let Message::SimStart {
            time,
            initial_percept,
        } = self.streams.recv()?
        {
            Ok(Agent {
                streams: self.streams,
                time,
                initial_percept,
            })
        } else {
            Err("wrong message type for start".into())
        }
    }
}

pub struct Agent {
    streams: AgentStreams,
    time: u64,
    initial_percept: InitialPercept,
}

impl Agent {
    pub fn send(&mut self, message: &Message) -> std::io::Result<()> {
        self.streams.send(message)
    }

    pub fn recv(&mut self) -> Result<Message, Box<dyn std::error::Error>> {
        self.streams.recv()
    }
}
