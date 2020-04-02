use serde::Deserialize;
use std::fmt;
use getset::Getters;

#[derive(Debug, Clone)]
pub enum NodeType {
    Tezedge,
    TezedgeMaster,
    Ocaml,
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Branch {
    Master,
    Modified,
    Ocaml,
}

impl fmt::Display for Branch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Deserialize, Debug, Getters, Clone)]
pub struct WrkResult {
    #[get = "pub(crate)"]
    duration: f32,

    #[get = "pub(crate)"]
    requests: f32,

    #[get = "pub(crate)"]
    latency_max: f32,

    #[get = "pub(crate)"]
    latency_min: f32,

    #[get = "pub(crate)"]
    latency_mean: f32,

    #[get = "pub(crate)"]
    latency_stdev: f32,
}