#![recursion_limit = "256"]
use gloo::timers::callback::{Interval, Timeout};
use gloo_console::log;
use rand::{rngs::ThreadRng, seq::IteratorRandom, thread_rng, Rng};
use std::ops::{Add, AddAssign, Sub};
use yew::prelude::*;

#[derive(Debug, Copy, Clone)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Vec2 {
        Vec2 { x, y }
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn as_pair(self: Self) -> (i32, i32) {
        match self {
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
        }
    }

    fn build_traversal(self) -> Vec<Position> {
        let i_traversal: Vec<usize> = match self {
            Direction::Down => (0..4).rev().collect(),
            _ => (0..4).collect(),
        };

        let j_traversal: Vec<usize> = match self {
            Direction::Right => (0..4).rev().collect(),
            _ => (0..4).collect(),
        };

        i_traversal
            .iter()
            .flat_map(|i| j_traversal.iter().map(move |j| Position::new(*i, *j)))
            .collect()
    }
}

impl From<Vec2> for Direction {
    fn from(vec: Vec2) -> Self {
        if vec.x.abs() > vec.y.abs() {
            if vec.x > 0 {
                Direction::Right
            } else {
                Direction::Left
            }
        } else {
            if vec.y > 0 {
                Direction::Down
            } else {
                Direction::Up
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Position {
    i: usize,
    j: usize,
}

impl Position {
    pub fn new(i: usize, j: usize) -> Position {
        Position { i, j }
    }

    pub fn from_index(index: usize) -> Position {
        Position {
            i: index / 4,
            j: index % 4,
        }
    }

    pub fn index(self) -> usize {
        self.i * 4 + self.j
    }

    pub fn is_out_of_bounds(self) -> bool {
        self.i >= 4 || self.j >= 4
    }
}

impl Add<Direction> for Position {
    type Output = Position;

    fn add(self, direction: Direction) -> Self::Output {
        let (i, j) = direction.as_pair();

        Position {
            i: (self.i as i32 + i) as usize,
            j: (self.j as i32 + j) as usize,
        }
    }
}

impl AddAssign<Direction> for Position {
    fn add_assign(&mut self, direction: Direction) {
        *self = *self + direction
    }
}

#[derive(Debug, Copy, Clone, Eq)]
struct Tile {
    number: i32,
    state: TileState,
    previous_position: Option<Position>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum TileState {
    New,
    Static,
    Merged,
}

impl TileState {
    fn to_string(&self) -> &str {
        match self {
            TileState::New => "new",
            TileState::Static => "static",
            TileState::Merged => "merged",
        }
    }
}

impl Tile {
    fn new(number: i32) -> Tile {
        Tile {
            number,
            state: TileState::New,
            previous_position: None,
        }
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Tile) -> bool {
        self.number == other.number
    }
}

type Cell = Option<Tile>;

#[derive(Debug, Copy, Clone)]
struct Grid {
    cells: [Cell; 16],
    rng: ThreadRng,
    enable_new_tiles: bool,
}

impl Default for Grid {
    fn default() -> Self {
        let mut grid = Grid::new([None; 16]);

        for _ in 0..2 {
            grid.add_random_tile();
        }

        grid
    }
}

impl PartialEq for Grid {
    fn eq(&self, other: &Grid) -> bool {
        self.cells == other.cells
    }
}

impl Grid {
    pub fn new(cells: [Cell; 16]) -> Grid {
        Grid {
            cells,
            rng: thread_rng(),
            enable_new_tiles: true,
        }
    }

    pub fn disable_new_tiles(&mut self) {
        self.enable_new_tiles = false;
    }

    fn get(&self, position: Position) -> Option<Tile> {
        self.cells.get(position.index()).and_then(|tile| *tile)
    }

    fn prepare_for_move(&mut self) {
        for i in 0..16 {
            self.cells
                .get_mut(i)
                .and_then(|cell| cell.as_mut())
                .map(|tile| {
                    tile.state = TileState::Static;
                    tile.previous_position = Some(Position::from_index(i));
                });
        }
    }

    pub fn move_in(&mut self, direction: Direction) {
        self.prepare_for_move();

        let traversal = direction.build_traversal();

        let mut moved = false;

        for start_position in traversal {
            moved |= self.traverse_from(start_position, direction);
        }

        if moved {
            self.add_random_tile()
        }
    }

    fn traverse_from(&mut self, start_position: Position, in_direction: Direction) -> bool {
        let mut start_tile = match self.get(start_position) {
            Some(tile) => tile,
            None => return false,
        };

        let mut new_position = start_position;

        loop {
            let next_position = new_position + in_direction;

            if next_position.is_out_of_bounds() {
                break;
            }

            if let Some(tile) = self.get(next_position) {
                if tile == start_tile && tile.state != TileState::Merged {
                    start_tile.number *= 2;
                    start_tile.state = TileState::Merged;
                    new_position = next_position;
                }

                break;
            }

            new_position = next_position;
        }

        if start_position == new_position {
            return false;
        }

        self.cells[start_position.index()] = None;
        self.cells[new_position.index()] = Some(start_tile);

        return true;
    }

    fn add_random_tile(&mut self) {
        if !self.enable_new_tiles {
            return;
        }

        let rng = &mut self.rng;

        let empty_cells = self.cells.iter_mut().filter(|x| x.is_none());

        if let Some(empty) = empty_cells.choose(rng) {
            let number = match self.rng.gen::<f64>() {
                x if x > 0.9 => 4,
                _ => 2,
            };

            *empty = Some(Tile::new(number));
        }
    }

    fn tiles(&self) -> impl Iterator<Item = (Position, Tile)> + '_ {
        self.cells
            .iter()
            .enumerate()
            .filter_map(|(i, cell)| match cell {
                None => None,
                Some(tile) => Some((Position::from_index(i), *tile)),
            })
            .flat_map(|(position, tile)| match tile.state {
                TileState::Merged => vec![
                    (position, tile),
                    (
                        position,
                        Tile {
                            state: TileState::Static,
                            previous_position: tile.previous_position,
                            number: tile.number / 2,
                        },
                    ),
                ],
                _ => vec![(position, tile)],
            })
    }
}

struct Model {
    grid: Grid,
    current_render: i32,
    touch_start: Option<TouchEvent>,
}

impl Model {
    fn move_in(&mut self, direction: Direction) {
        self.grid.move_in(direction);
    }
}

enum Msg {
    KeyboardEvent(KeyboardEvent),
    TouchStart(TouchEvent),
    TouchEnd(TouchEvent),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            grid: Grid::default(),
            touch_start: None,
            current_render: 0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::KeyboardEvent(e) => {
                log!(e.key_code());
                match e.key_code() {
                    37 => self.move_in(Direction::Left),
                    38 => self.move_in(Direction::Up),
                    39 => self.move_in(Direction::Right),
                    40 => self.move_in(Direction::Down),
                    _ => return false,
                }
            }
            Msg::TouchStart(e) => {
                e.prevent_default();

                self.touch_start = Some(e);

                return false;
            }
            Msg::TouchEnd(touches_end) => {
                let touch_start = self
                    .touch_start
                    .as_ref()
                    .and_then(|e| e.changed_touches().item(0))
                    .map(|touch| Vec2::new(touch.client_x(), touch.client_y()));

                let touch_end = touches_end
                    .changed_touches()
                    .item(0)
                    .map(|touch| Vec2::new(touch.client_x(), touch.client_y()));

                match (touch_start, touch_end) {
                    (Some(start), Some(end)) => self.move_in((end - start).into()),
                    _ => return false,
                };
            }
        };

        self.current_render += 1;

        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let touch_start = ctx.link().callback(Msg::TouchStart);
        let touch_end = ctx.link().callback(Msg::TouchEnd);
        let onkeypress = ctx
            .link()
            .callback(|event: KeyboardEvent| Msg::KeyboardEvent(event));

        html! {
            <div class="grid-wrapper" ontouchstart={touch_start} ontouchend={touch_end} onkeydown={onkeypress} tabIndex="0">
                <div class="grid" key={self.current_render}>
                { for (0..16).map(|_| { html! { <div class="cell"></div> }}) }
                { for self.grid.tiles().map(|(position, tile)| html! { <TileComponent position={position} tile={tile} />} ) }
                </div>
            </div>
        }
    }
}

struct TileComponent {
    tile: Tile,
    position: Position,
    pre_pos: Option<Position>,
    h: Option<Timeout>,
}

impl TileComponent {
    fn class_name(&self) -> String {
        format!(
            "tile tile-{} tile-{}-{} tile-{}",
            if self.tile.number <= 2048 {
                self.tile.number.to_string()
            } else {
                "super".to_string()
            },
            self.position.index() % 4,
            self.position.index() / 4,
            self.tile.state.to_string(),
        )
    }
}

#[derive(Properties, Clone, PartialEq)]
struct TileComponentProps {
    tile: Tile,
    position: Position,
}

enum TileMsg {
    ActualPosition(Position),
}

impl Component for TileComponent {
    type Message = TileMsg;
    type Properties = TileComponentProps;

    fn create(ctx: &Context<Self>) -> Self {
        let mut position = ctx.props().position;
        log!("create!", position.i, " ", position.j);
        let mut pre_pos = None;

        match (ctx.props().tile.state, ctx.props().tile.previous_position) {
            (TileState::Merged, _) => {}
            (_, Some(previous_position)) => {
                position = previous_position;

                let actual_position = ctx.props().position;
                pre_pos = Some(actual_position);
            }
            _ => {}
        }

        Self {
            tile: ctx.props().tile,
            position,
            pre_pos,
            h: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TileMsg::ActualPosition(position) => {
                self.position = position;
            }
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.tile = ctx.props().tile;
        self.position = ctx.props().position;

        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }

        if let Some(pre_pos) = self.pre_pos {
            let cb = ctx.link().callback(TileMsg::ActualPosition);
            self.h = Some(Timeout::new(50, move || cb.emit(pre_pos)));
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={self.class_name()}>
                <div class="tile-inner">
                    { self.tile.number.to_string() }
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
