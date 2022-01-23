use pathfinding::directed::astar::astar;
use yew::{html, Html};

#[derive(Clone, Copy, PartialEq)]
pub struct Puppy {
    pub x: isize,
    pub y: isize,
    pub points: usize,
    pub steals: usize,
    name: usize,
}

const NAMES: [&str; 10] = [
    "Bella",
    "Bailey",
    "Max",
    "Lucy",
    "Molly",
    "Charlie",
    "Daisy",
    "Buddy",
    "Maggie",
    "Sophie",
];
static mut COUNTER: usize = 0;

fn get_name(name_idx: usize) -> String {
    NAMES[name_idx].to_string()
}

fn neighbors(x: isize, y: isize, occupied: &Vec<(isize, isize)>) -> Vec<((isize, isize), usize)> {
    let mut n1: Vec<((isize, isize), usize)> = vec![
        (x + 1, y),
        (x - 1, y),
        (x, y - 1),
        (x, y + 1)
    ].into_iter().map(|p| (p, 2)).collect();
    let mut n2: Vec<((isize, isize), usize)> = vec![
        (x + 1, y - 1),
        (x - 1, y + 1),
        (x + 1, y - 1),
        (x - 1, y + 1)
    ].into_iter().map(|p| (p, 2)).collect();
    n1.append(&mut n2);
    let n1: Vec<((isize, isize), usize)> = n1.into_iter().filter(|p| !occupied.contains(&p.0)).collect();
    if n1.len() == 0 {
        return vec![((-1, -1), 1)]
    } else {
        n1
    }
}

impl Puppy {
    pub fn at_pos(x: usize, y: usize) -> Puppy {
        // this is because i'm being lazy with a static variable
        unsafe {
            COUNTER += 1;
            Puppy {x: x as isize, y: y as isize, points: 0, steals: 0, name: COUNTER - 1}
        }
    }

    pub fn step(&mut self, goal: (usize, usize), occupied: &Vec<(usize, usize)>) -> bool {
        let occupied2: Vec<(isize, isize)> = occupied.iter()
            .filter(|p| **p != goal)
            .map(|p| (p.0 as isize, p.1 as isize))
            .collect();
        let goal = (goal.0 as isize, goal.1 as isize);
        if let Some(path) = astar(
            &(self.x, self.y),
            |(x, y)| neighbors(*x, *y, &occupied2),
            |(x, y)| ((goal.0 - x) * (goal.0 - x) + (goal.1 - y) * (goal.1 - y)) as usize,
            |pos| *pos == goal || *pos == (-1, -1)
        ) {
            if !path.0.contains(&(-1, -1)) {
                if let Some(&pos) = path.0.get(1) {
                    if !occupied.contains(&(pos.0 as usize, pos.1 as usize)) {
                        self.x = pos.0;
                        self.y = pos.1;
                        return true;
                    }
                }
            }
        }
        return false;
    }

    pub fn view(&self, i: usize) -> Html {
        html! {
            <div class="ui-row">
                { format!("{:<2}", i) }
                <img src={format!("generated/puppy-{}.png", self.name)}/>
                { format!("{:<10} |", get_name(self.name)) }
                <span>
                { format!(
                        "P:{:0>3}  S:{:0>4}",
                        self.points,
                        self.steals
                ) }
                </span>
            </div>
        }
    }

    pub fn get_image(&self) -> String {
        format!("generated/puppy-{}.png", self.name)
    }
}
