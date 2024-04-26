use std::cell::RefCell;
use std::fs;
use std::path::Path;
use std::time::Duration;

use anyhow::Context;
use gxhash::GxHashMap;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use ureq::Agent;

use crate::types::DateProvider;

pub struct InputCache {
    map: GxHashMap<(u16, u8), String>,
    agent: Agent,
}

impl InputCache {
    pub fn new() -> anyhow::Result<Self> {
        dotenvy::dotenv()?;

        let session_cookie = ureq::Cookie::new(
            "session",
            std::env::var("AOC_TOKEN").context("Please set the env-var AOC_TOKEN")?,
        );
        let mut cookies = cookie_store::CookieStore::new(None);
        cookies.insert_raw(&session_cookie, &"https://adventofcode.com/".parse()?)?;

        let agent: Agent = ureq::AgentBuilder::new()
            .timeout_read(Duration::from_secs(5))
            .timeout_write(Duration::from_secs(5))
            .cookie_store(cookies)
            .build();

        Ok(Self {
            map: Self::retrieve_local_cache().unwrap_or_default(),
            agent,
        })
    }

    fn get_web_input(&self, day: &dyn DateProvider) -> anyhow::Result<String> {
        let url = format!(
            "https://adventofcode.com/{}/day/{}/input",
            day.year(),
            day.day(),
        );

        Ok(self.agent.get(&url).call()?.into_string()?)
    }

    pub fn get(&mut self, solution: &dyn DateProvider) -> Result<String, anyhow::Error> {
        let self_ = RefCell::new(self);

        if let Some(res) = self_.borrow().map.get(&solution.get_datetuple()) {
            return Ok(res.clone());
        }

        let value = self_.borrow().get_web_input(solution)?;
        //  let value = input;

        self_
            .borrow_mut()
            .map
            .insert(solution.get_datetuple(), value.clone());

        debug_assert_ne!(
            "Puzzle inputs differ by user.  Please log in to get your puzzle input.",
            value
        );

        Ok(value)
    }

    fn retrieve_local_cache() -> Option<GxHashMap<(u16, u8), String>> {
        if let Ok(ser) = std::fs::read_to_string(*SAVED_LOCATION) {
            Some(serde_json::from_str::<SerdeMap>(&ser).ok()?.0)
        } else {
            None
        }
    }
}

static SAVED_LOCATION: Lazy<&Path> = Lazy::new(|| Path::new("./.cache/aoc_input.json"));

#[serde_as]
#[derive(Serialize, Deserialize)]
struct SerdeMap(#[serde_as(as = "Vec<(_, _)>")] GxHashMap<(u16, u8), String>);

impl Drop for InputCache {
    fn drop(&mut self) {
        let serde_map = &SerdeMap(std::mem::take(&mut self.map));

        if let Some(parent) = SAVED_LOCATION.parent() {
            let _ = fs::create_dir_all(parent);
        }

        if let Ok(ser) = serde_json::to_string(serde_map) {
            let _ = std::fs::write(*SAVED_LOCATION, ser)
                .inspect_err(|err| eprintln!("could not save map, err: {err}"));
        }
    }
}

#[test]
fn test() {
    let mut cache = InputCache::new().unwrap();
    let day = crate::Solution {
        info: crate::types::Info {
            name: "Rucksack Reorganization",
            day: 3,
            year: 2022,
            bench: crate::types::BenchTimes::Default,
        },
        other: &[],
        part1: |_| todo!(),
        part2: Some(|_| todo!()),
    };
    let inp = cache.get(&day).unwrap();
    println!("{inp}");
}
