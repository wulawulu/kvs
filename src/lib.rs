use clap::Parser;
use std::collections::HashMap;

#[derive(Debug, Parser)]
#[command(name = "kvs", version, author, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    /// The command to run
    pub command: Commands,
}

#[derive(Debug, Parser)]
pub enum Commands {
    #[command(name = "get", about = "Get the string value of a given string key")]
    Get(GetOpt),
    #[command(name = "set", about = "Set the value of a string key to a string")]
    Set(SetOpt),
    #[command(name = "rm", about = "Remove a given key")]
    Remove(RemoveOpt),
}

#[derive(Debug, Parser)]
pub struct GetOpt {
    pub key: String,
}

#[derive(Debug, Parser)]
pub struct SetOpt {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Parser)]
pub struct RemoveOpt {
    pub key: String,
}

pub struct KvStore {
    hash_map: HashMap<String, String>,
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            hash_map: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.hash_map.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<String> {
        self.hash_map.get(&key).cloned()
    }

    pub fn remove(&mut self, key: String) {
        self.hash_map.remove(&key);
    }
}
