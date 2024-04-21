use std::time::Duration;

use anyhow::Context;
use gxhash::GxHashMap;
use ureq::Agent;

use crate::Solution;

pub struct InputCache {
    map: GxHashMap<(u16, u8), String>,
}

impl InputCache {
    fn new() -> Self {
        Self {
            map: GxHashMap::default(),
        }
    }

    fn get(&mut self, solution: Solution) -> anyhow::Result<&str> {
        Ok(self.map.entry(solution.get_datetuple()).or_insert_with(|| {
            get_input(&solution)
                .or_else(|_| {
                    std::thread::sleep(Duration::from_secs(5));
                    get_input(&solution)
                })
                .context("Failed to get input")
                .unwrap()
        }))
    }
}

pub fn get_input(day: &Solution) -> anyhow::Result<String> {
    dotenvy::dotenv()?;

    let url = format!(
        "https://adventofcode.com/{}/day/{}/input",
        day.info.year, day.info.day
    );

    let mut y = cookie_store::CookieStore::new(None);
    let x = ureq::Cookie::new("session", std::env::var("AOC_TOKEN")?);
    y.insert_raw(&x, &"https://adventofcode.com/".parse()?)?;

    let agent: Agent = ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(5))
        .timeout_write(Duration::from_secs(5))
        .cookie_store(y)
        .build();

    Ok(agent.get(&url).call()?.into_string()?)
}
