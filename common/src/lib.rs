pub mod reach;
pub mod task;

pub use task::*;

#[macro_export]
macro_rules! mat {
	($($e:expr),*) => { Vec::from(vec![$($e),*]) };
	($($e:expr,)*) => { Vec::from(vec![$($e),*]) };
	($e:expr; $d:expr) => { Vec::from(vec![$e; $d]) };
	($e:expr; $d:expr $(; $ds:expr)+) => { Vec::from(vec![mat![$e $(; $ds)*]; $d]) };
}

#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub enum Square {
    Empty,
    Block,
    Filled,
}

#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub enum Booster {
    Extension,
    Fast,
    Drill,
    X,
}

impl std::str::FromStr for Booster {
    type Err = ();

    fn from_str(s: &str) -> Result<Booster, ()> {
        match s {
            "B" => Ok(Booster::Extension),
            "F" => Ok(Booster::Fast),
            "L" => Ok(Booster::Drill),
            "X" => Ok(Booster::X),
            _ => Err(()),
        }
    }
}

pub fn apply_move((x, y): (usize, usize), dir: usize) -> (usize, usize) {
    match dir {
        0 => (x + 1, y),
        1 => (x, y - 1),
        2 => (x - 1, y),
        3 => (x, y + 1),
        _ => panic!("illegal dir: {}", dir)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Action {
    Move(usize),
    Nothing,
    TurnR,
    TurnL,
    Extension(i32, i32),
    Fast,
    Drill
}

pub fn actions_to_string(list: &Vec<Action>) -> String {
    let mut out = String::new();
    for mv in list {
        match mv {
            Action::Move(dir) => out += ["D", "S", "A", "W"][*dir],
            Action::Nothing => out += "Z",
            Action::TurnR => out += "E",
            Action::TurnL => out += "Q",
            Action::Extension(dx, dy) => out += &format!("B({},{})", dx, dy),
            Action::Fast => out += "F",
            Action::Drill => out += "L"
        }
    }
    out
}

pub struct PlayerState {
    x: usize,  //・今いる座標
    y: usize,
    dir: usize,  //・向いている向き
    unused_boosters: Vec<Square>,  //・持っている
    active_boosters: Vec<(Square, i32)>,  //・発動中の効果、残りターン
    manipulators: Vec<(i32, i32)>,  // マニピュレータたちの相対位置
}
