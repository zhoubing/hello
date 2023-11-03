use std::{env, thread};
use crossbeam::{select, unbounded};
use libc::select;
use svg::Document;
use svg::node::element::path::{Command, Data, Position};
use svg::node::element::{Path, Rectangle};
use crate::threads::render_hex::Operation::{Forward, Home, Noop, TurnLeft, TurnRight};
use crate::threads::render_hex::Orientation::{East, North, South, West};
use rayon::prelude::*;
use crate::threads::render_hex::ConnectivityCheck::Ping;
use crate::threads::render_hex::Work::{Finished, Task};

const WIDTH: isize = 420;
const HEIGHT: isize = WIDTH;

const HOME_X: isize = WIDTH / 2;
const HOME_Y: isize = HEIGHT / 2;

const STROKE_WIDTH: isize = 5;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Forward(isize),
    TurnLeft,
    TurnRight,
    Home,
    Noop(usize),
}

#[derive(Debug, Clone, Copy)]
enum Orientation {
    North,
    East,
    West,
    South,
}

#[derive(Debug)]
struct Drawer {
    x: isize,
    y: isize,
    heading: Orientation,
}

impl Drawer {
    pub fn new() -> Drawer {
        Drawer {
            x: HOME_X,
            y: HOME_Y,
            heading: North,
        }
    }

    fn home(&mut self) {
        self.x = HOME_X;
        self.y = HOME_Y;
    }

    fn forward(&mut self, distance: isize) {
        match self.heading {
            North => self.y += distance,
            South => self.y -= distance,
            West => self.x += distance,
            East => self.x -= distance,
        }
    }

    fn turn_right(&mut self) {
        self.heading = match self.heading {
            North => East,
            South => West,
            West => North,
            East => South,
        }
    }

    fn turn_left(&mut self) {
        self.heading = match self.heading {
            North => West,
            South => East,
            West => South,
            East => North,
        }
    }

    fn wrap(&mut self) {
        if self.x < 0 {
            self.x = HOME_X;
            self.heading = West;
        } else if self.x > WIDTH {
            self.x = HOME_X;
            self.heading = East;
        }

        if self.y < 0 {
            self.y = HOME_Y;
            self.heading = North;
        } else if self.y > HEIGHT {
            self.x = HOME_Y;
            self.heading = South;
        }
    }
}

pub fn run() {
    let args = env::args().collect::<Vec<String>>();
    let input = args.get(1).unwrap();
    println!("{}", input);

    let input = args.get(0).unwrap();
    println!("{}", input);

    let ops = parse("31b0bf8da2ab461f357190cdd224bdf076ca85cf");
    let path_data = convert(&ops);
    let document = generate_svg(path_data);
    svg::save("/home/rust/hello/1.svg", &document).unwrap();

    channel();
    channels_complex();
}

fn parse(input: &str) -> Vec::<Operation> {
    println!("input size is {:?}", input.len());
    let mut steps = Vec::<Operation>::new();
    for b in input.bytes() {
        println!("byte is {:?}", b);
        let step = match b {
            b'0' => Home,
            b'1'..=b'9' => Forward((b - 0x30) as isize * HEIGHT / 10),
            b'a' | b'b' | b'c' => TurnLeft,
            b'd' | b'e' | b'f' => TurnRight,
            _ => {
                Noop(b as usize)
            }
        };
        steps.push(step)
    }
    steps
}

fn functional_parse(input: &str) -> Vec::<Operation> {
    input.as_bytes()
        .par_iter()
        .map(|b| {
        match b {
            b'0' => Home,
            b'1'..=b'9' => Forward((b - 0x30) as isize * HEIGHT / 10),
            b'a' | b'b' | b'c' => TurnLeft,
            b'd' | b'e' | b'f' => TurnRight,
            _ => {
                Noop(*b as usize)
            }
        }
    }).collect()
}

fn convert(operations: &Vec<Operation>) -> Vec<Command> {
    let mut turtle = Drawer::new();
    let mut path_data = Vec::<Command>::with_capacity(operations.len());
    let starting_point = Command::Move(Position::Absolute, (HOME_X, HOME_Y).into()); //???
    path_data.push(starting_point);
    for op in operations {
        match *op {
            Forward(distance) => turtle.forward(distance),
            TurnLeft => turtle.turn_left(),
            TurnRight => turtle.turn_right(),
            Home => turtle.home(),
            Noop(byte) => {
                eprintln!("illegal byte encountered! {:?}", byte)
            }
        }
        let path_segment = Command::Line(Position::Absolute, (turtle.x, turtle.y).into());
        path_data.push(path_segment);
        turtle.wrap();
    }
    path_data
}

fn generate_svg(path_data: Vec<Command>) -> Document {
    let bg = Rectangle::new()
        .set("x", 0)
        .set("y", 0)
        .set("width", WIDTH)
        .set("height", HEIGHT)
        .set("fill", "#ffffff");

    let border = bg.clone().set("fill-opacity", "0.0")
        .set("stroke", "#cccccc").set("stroke-width", 3 * STROKE_WIDTH);

    let sketch = Path::new().set("fill", "none")
        .set("stroke", "#2f2f2f").set("stroke-width", STROKE_WIDTH)
        .set("stroke-opacity", "0.9").set("d", Data::from(path_data));

    let document = Document::new().set("viewBox", (0, 0, HEIGHT, WIDTH))
        .set("height", HEIGHT).set("width", WIDTH).set("style", "style=\"outline: 5px solid #800000;    \"")
        .add(bg).add(sketch).add(border);

    document
}

fn channel() {
    let (tx, rx) = unbounded();
    thread::spawn(move || {
       tx.send(42).unwrap();
    });

    select! {
        recv(rx) -> msg => println!("{:?}", msg.unwrap()),
    }
}

#[derive(Debug)]
enum ConnectivityCheck {
    Ping,
    Pong,
    Pang
}
fn channels_complex() {
    let (req_tx, req_rx) = unbounded();
    let (resp_tx, resp_rx) = unbounded();
    thread::spawn(move || loop {
        match req_rx.recv().unwrap() {
            ConnectivityCheck::Pong => eprintln!("unexpected pong response"),
            ConnectivityCheck::Ping => {
                println!("ConnectivityCheck::Ping");
                resp_tx.send(ConnectivityCheck::Pong).unwrap()
            },
            ConnectivityCheck::Pang => return,
        }
    });
    req_tx.send(Ping).unwrap();
    req_tx.send(ConnectivityCheck::Pang).unwrap();

    select! {
        recv(resp_rx) -> msg => {
            match msg {
                Ok(msg) => println!("recv {:?}", msg),
                Err(e) => eprintln!("{:?}", e)
            }
        },
    }
}

enum Work {
    Task(usize, u8),
    Finished
}
fn task_queue() {

}

fn task_queue_parse_byte(byte: u8) -> Operation {
    match byte {
        b'0' => Home,
        b'1'..=b'9' => Forward((byte - 0x30) as isize * HEIGHT / 10),
        b'a' | b'b' | b'c' => TurnLeft,
        b'd' | b'e' | b'f' => TurnRight,
        _ => Noop(byte as usize)
    }
}
fn task_queue_parse(input: &str) -> Vec<Operation> {
    let n_threads = 2;
    let (todo_tx, todo_rx) = unbounded();
    let (results_tx, results_rx) = unbounded();
    let mut n_bytes = 0;
    for (i, bytes) in input.bytes().enumerate() {
        todo_tx.send(Task(i, bytes)).unwrap();
        n_bytes += 1;
    }
    for _ in 0..n_threads {
        todo_tx.send(Finished).unwrap();
    }
    for _ in 0..n_threads {
        let to_do = todo_rx.clone();
        let results = results_tx.clone();
        thread::spawn(move || {
            loop {
                let task = to_do.recv();
                let result = match task {
                    Err(_) => break,
                    Ok(Finished) => break,
                    Ok(Task(i, byte)) => (i, task_queue_parse_byte(byte))
                };
                results.send(result).unwrap();
            }
        });
    }
    let mut ops = vec![Noop(0); n_bytes];
    for _ in 0..n_bytes {
        let (i, op) = results_rx.recv().unwrap();
        ops[i] = op;
    }
    ops
}

