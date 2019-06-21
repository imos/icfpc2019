use common::*;

fn main() {
    let t = read_task("/Users/akiba/Downloads/part-1-initial/prob-001.desc");
    //let t = read_task("/Users/akiba/Downloads/part-2-teleports/prob-151.desc");

    let a = PlayerState::new(t.2, t.3);

    let mut bfs = BFS::new(t.0.len(), t.0[0].len());
    let actions = bfs.search_fewest_actions_to_move(&t.0, &a, 2, 2);
    dbg!(&actions);

    let actions = bfs.search_fewest_actions_to_move(&t.0, &a, 3, 3);
    dbg!(&actions);

    let actions = bfs.search_fewest_actions_to_move(&t.0, &a, 2, 2);
    dbg!(&actions);
}
