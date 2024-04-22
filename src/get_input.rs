use std::time::Duration;

use anyhow::Context;
use gxhash::GxHashMap;
use ureq::Agent;

use crate::Solution;

pub struct InputCache {
    map: GxHashMap<(u16, u8), String>,
    agent: Agent,
}

impl InputCache {
    fn new() -> Self {
        dotenvy::dotenv()?;

        let mut cookies = cookie_store::CookieStore::new(None);
        let session = ureq::Cookie::new(
            "session",
            std::env::var("AOC_TOKEN").context("Please set the env-var AOC_TOKEN")?,
        );
        cookies.insert_raw(&session, &"https://adventofcode.com/".parse()?)?;

        let agent: Agent = ureq::AgentBuilder::new()
            .timeout_read(Duration::from_secs(5))
            .timeout_write(Duration::from_secs(5))
            .cookie_store(cookies)
            .build();

        Self {
            map: GxHashMap::default(),
            agent,
        }
    }

    fn get_input(&self, day: &Solution) -> anyhow::Result<String> {
        let url = format!(
            "https://adventofcode.com/{}/day/{}/input",
            day.info.year, day.info.day
        );

        Ok(self.agent.get(&url).call()?.into_string()?)
    }

    fn get(&mut self, solution: Solution) -> anyhow::Result<&str> {
        Ok(self.map.entry(solution.get_datetuple()).or_insert_with(|| {
            self.get_input(&solution)
                .or_else(|_| {
                    // retry after 5 seconds
                    std::thread::sleep(Duration::from_secs(5));
                    self.get_input(&solution)
                })
                .context("Failed to get input")
                .unwrap()
        }))
    }
}
