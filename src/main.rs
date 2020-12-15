use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
//use arrow::array::{Array,BooleanArray};

fn main() -> Result<(), Box<dyn Error>> {
    let raw = File::open("./input.txt")?;
    let buf = BufReader::new(raw);
    let mut seats: Vec<Option<bool>> = Vec::new();
    let mut w = 0;
    for line in buf.lines() {
        let line = line.unwrap();
        w = line.len();
        seats.extend(line.chars().map(|c| match c {
            'L' => Some(false),
            '#' => Some(true),
            '.' => None,
            _ => unreachable!(),
        }));
    }

    let h = seats.len() / w;

    let mut buffer = seats.clone();
    let mut forwards = true;
    let mut changed = true;
    let mut a;
    let mut b;

    while changed {
        changed = false;

        if forwards {
            a = &mut seats;
            b = &mut buffer;
        } else {
            a = &mut buffer;
            b = &mut seats;
        }
        forwards = !forwards;

        for i in 0..seats.len() {
            let mut count = 0;
            let x = i % w;
            let y = i / w;
            for (dx, dy) in [
                (-1, -1),
                (0, -1),
                (1, -1),
                (-1, 0),
                (1, 0),
                (-1, 1),
                (0, 1),
                (1, -1),
            ]
            .iter()
            {
                let dx = *dx;
                let dy = *dy;
                if (dx == -1 && x == 0)
                    || (dy == -1 && y == 0)
                    || (dx == 1 && x == w - 1)
                    || (dx == 1 && y == h - 1)
                {
                    continue;
                }

            }
        }
    }
    println!("{:#?}", seats);
    Ok(())
}
