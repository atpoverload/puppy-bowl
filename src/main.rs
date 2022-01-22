use std::vec::Vec;

use gloo_timers::callback::Interval;
use rand::random;
use yew::html::Scope;
use yew::{classes, html, Component, Context, Html};
use rand::seq::SliceRandom;

use puppy::Puppy;

mod puppy;

// TODO: decide if we want to modularize the non-yew bits further
type Position = (usize, usize);
type Dimensions = (usize, usize);

pub struct App {
    puppies: Vec<Puppy>,
    ball: Position,
    goal: Position,
    dimensions: Dimensions,
    _interval: Interval,
}

fn adjacent(p: Position, puppy: &Puppy) -> bool {
    return (p.0 as isize - puppy.x).abs() <= 1 || (p.1 as isize - puppy.y).abs() <= 1
}

fn new_position(dimensions: Dimensions) -> Position {
    let pos = random::<(usize, usize)>();
    ((pos.0 % dimensions.0 / 2) + dimensions.0 / 4, (pos.1 % dimensions.1 / 2) + dimensions.1 / 4)
}

impl App {
    fn step(&mut self) {
        let mut order = (0..self.puppies.len()).collect::<Vec<usize>>();
        order.shuffle(&mut rand::thread_rng());
        for i in order {
            let mut occupied: Vec<(usize, usize)> = self.puppies.iter().map(|p| (p.x as usize, p.y as usize)).collect();
            let mut puppy = self.puppies.get_mut(i).unwrap();

            // TODO: this is ugly as sin
            let pos = (puppy.x as usize, puppy.y as usize);
            if pos == self.ball {
                if self.ball == self.goal {
                    puppy.points += 1;
                    self.ball = new_position(self.dimensions);
                    loop {
                        let goal = new_position(self.dimensions);
                        if goal != self.ball {
                            self.goal = goal;
                            break;
                        }
                    };
                } else {
                    puppy.step(self.goal, &occupied);
                    self.ball = (puppy.x as usize, puppy.y as usize);
                }
            } else {
                occupied.push(self.goal);
                if !puppy.step(self.ball, &occupied) && adjacent(self.ball, puppy) && (random::<usize>() % 5) == 0 {
                    puppy.steals += 1;
                    self.ball = (puppy.x as usize, puppy.y as usize);
                }
            }
        }
    }

    fn view_cellule(&self, pos: (usize, usize), idx: usize, _: &Scope<Self>) -> Html {
        let puppies: Vec<String> = self.puppies.iter()
            .filter(|p| (p.x as usize, p.y as usize) == pos)
            .map(|p| p.get_image())
            .collect();
        if let Some(name) = puppies.get(0) {
            if pos == self.ball {
                html! {
                    <div key={idx} class={classes!("game-cell")}>
                        <img src="field.png" style="position: relative;"/>
                        <img src={name.to_owned()} style="position: absolute; z-index: 1;"/>
                        <img src="ball.png" style="position: absolute; z-index: 2;"/>
                    </div>
                }
            } else {
                html! {
                    <div key={idx} class={classes!("game-cell")}>
                        <img src="field.png" style="position: relative;"/>
                        <img src={name.to_owned()} style="position: absolute; z-index: 1;"/>
                    </div>
                }
            }
        } else if pos == self.ball {
            html! {
                <div key={idx} class={classes!("game-cell")}>
                    <img src="field.png" style="position: relative;"/>
                    <img src="ball.png" style="position: absolute; z-index: 1;"/>
                </div>
            }
        } else if pos == self.goal {
            html! {
                <div key={idx} class={classes!("game-cell")}>
                    <img src="field.png" style="position: relative;"/>
                    <img src="goal.png" style="position: absolute; z-index: 1;"/>
                </div>
            }
        } else {
            html! {
                <div key={idx} class={classes!("game-cell")}>
                    <img src="field.png"/>
                </div>
            }
        }
    }
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(|_| ());
        let interval = Interval::new(100, move || callback.emit(()));

        // abstract field creation
        let width = 30;
        let height = 25;
        let dimensions = (width, height);

        let ball = new_position(dimensions);
        let mut goal = new_position(dimensions);
        while goal == ball {
            goal = new_position(dimensions);
        };

        // abstract puppy creation
        let puppies = 10;
        let puppies = (0..puppies).map(|_| {
            let pos = new_position(dimensions);
            Puppy::at_pos(pos.0, pos.1)
        }).collect();

        Self {
            puppies,
            ball,
            goal,
            dimensions,
            _interval: interval,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        self.step();
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // abstract ui rendering
        let ui_rows = self.puppies.iter().map(|puppy| puppy.view());

        // abstract field rendering
        let (width, height) = self.dimensions;
        let cell_rows = (0..height)
            .map(|y| (0..width).map(move |x| self.view_cellule((x, y), y * width + x, ctx.link())))
            .enumerate()
            .map(|(x, cells)| {
                html! {
                    <div key={x} class="game-row">
                        { for cells }
                    </div>
                }
            });

        html! {
            <div>
                <section class="game-container">
                    <header class="app-header">
                        <h1 class="app-title">{ "Puppy Bowl Livestream" }</h1>
                    </header>
                    <section class="game-area">
                        <div class="game-ui">
                            { for ui_rows }
                        </div>
                        <div class="game-field">
                            { for cell_rows }
                        </div>
                    </section>
                </section>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::trace!("Initializing yew...");
    yew::start_app::<App>();
}
