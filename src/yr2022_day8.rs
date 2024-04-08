pub fn part1() -> anyhow::Result<u32> {
    let data = get_data();

    todo!()
}

struct Tree(u8);

fn parse(data: &str) -> Vec<Vec<Tree>> {
    todo!()
}

fn get_data() -> &'static str {
    if cfg!(debug_assertions) {
        include_str!("../inputs/day8-test.txt")
    } else {
        todo!()
    }
}
