use std::cell::RefCell;
use std::time::Duration;

use anyhow::{anyhow, Context};
use gxhash::GxHashMap;
use ureq::Agent;

use crate::Solution;

pub struct InputCache {
    map: GxHashMap<(u16, u8), String>,
    agent: Agent,
}

impl InputCache {
    pub fn new() -> anyhow::Result<Self> {
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

        Ok(Self {
            map: GxHashMap::default(),
            agent,
        })
    }

    fn get_input(&self, day: &Solution) -> anyhow::Result<String> {
        let url = format!(
            "https://adventofcode.com/{}/day/{}/input",
            day.info.year, day.info.day
        );

        Ok(self.agent.get(&url).call()?.into_string()?)
    }

    // clones the data
    // pub fn get<'a>(&'a mut self, solution: &Solution) -> anyhow::Result<&'a String> {
    //     // let selfref = std::sync::Mutex::new(self);
    //     //
    //     // let x = match selfref.lock().unwrap().map.entry(solution.get_datetuple()) {
    //     //     Entry::Occupied(e) => e.get().clone(),
    //     //     Entry::Vacant(e) => e
    //     //         .insert(selfref.lock().unwrap().get_input(solution).unwrap())
    //     //         .clone(),
    //     // };
    // 
    //     //Ok(x)
    // 
    //     let res = self.map.get(&solution.get_datetuple());
    // 
    //     if res.is_some() {
    //         return res.ok_or(anyhow!("NO"));
    //     }
    // 
    //     let input = self.get_input(solution)?;
    //     self.map.insert(solution.get_datetuple(), input.clone());
    //     self.map.get(&solution.get_datetuple()).ok_or(anyhow!("NO"))
    // }


    pub fn get<'a>(
        &'a mut self,
        solution: &'a Solution,
    ) -> Result<&'a String, anyhow::Error> {
        
        let selfcell = RefCell::new(self);
        
        if let Some(res) = selfcell.borrow().map.get(&solution.get_datetuple()) {
            return Ok(res);
        }

        let input = self.get_input(solution)?;
        let value = input.clone();
        self.map.insert(solution.get_datetuple(), value);

        self.map
            .get(&solution.get_datetuple())
            .ok_or_else(|| anyhow!("NO"))
    }
}

#[test]
fn test() {
    let mut cache = InputCache::new().unwrap();
    let day = Solution {
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
    println!("{}", inp);
}
