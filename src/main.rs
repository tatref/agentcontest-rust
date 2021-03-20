#![allow(dead_code)]
#![allow(unused_imports)]

//https://multiagentcontest.org/2020/
//https://github.com/agentcontest/massim_2020
//https://github.com/agentcontest/massim_2020/blob/master/docs/scenario.md
//https://github.com/agentcontest/massim_2020/blob/master/docs/protocol.md

use core::time;
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

mod agent;
mod messages;

use messages::*;

#[derive(Debug, Deserialize)]
pub struct InitialPercept {
    name: String,
    team: String,
    #[serde(rename = "teamSize")]
    team_size: u32,
    steps: u32,
    vision: u32,
}

#[derive(Debug, Deserialize)]
struct Thing {
    x: u32,
    y: u32,
    details: String,
    #[serde(rename = "type")]
    _type: String,
}

#[derive(Default, Debug, Deserialize)]
struct Terrain {
    #[serde(default)]
    goal: Vec<(u32, u32)>,
    #[serde(default)]
    obstacle: Vec<(u32, u32)>,
}

#[derive(Debug, Deserialize)]
struct TaskRequirement {
    x: i32,
    y: i32,
    details: String,
    #[serde(rename = "type")]
    _type: String,
}

#[derive(Debug, Deserialize)]
struct Task {
    name: String,
    deadline: u64,
    reward: u32,
    requirements: Vec<TaskRequirement>,
}

#[derive(Debug, Deserialize)]
struct StepPercept {
    score: u32,
    #[serde(rename = "lastAction")]
    last_action: String,
    #[serde(rename = "lastActionResult")]
    last_action_result: String,
    #[serde(rename = "lastActionParams")]
    last_action_params: Vec<String>,
    energy: u32,
    disabled: bool,
    task: String,
    things: Vec<Thing>,
    terrain: Terrain,
    tasks: Vec<Task>,
    attached: Vec<(u32, u32)>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut agent =
        agent::AgentAuthenticator::auth("192.168.0.13:12300", "agentA1", "1")?.wait_for_start()?;

    eprintln!("STARTED!");
    //agent.send(&message)?;
    let m = agent.recv()?;
    dbg!(m);
    let m = agent.recv()?;
    dbg!(m);

    println!("Hello, world!");
    Ok(())
}
