use std::{ops::RangeInclusive, io::{BufReader, BufRead}, fs::File};

use regex::Regex;

#[derive(Clone, PartialEq, Eq, Debug)]
struct CompositeCuboid {

    cuboids: Vec<Cuboid>
}



impl CompositeCuboid {

    fn add_cuboid(&mut self, cuboid: Cuboid) {

        let mut new_composition: Vec<Cuboid> = self.cuboids.iter().enumerate().flat_map(|(i, c)| 
        {
            c.cut(&cuboid).into_iter()
        }).collect();
        new_composition.push(cuboid);
        self.cuboids = new_composition;
    }

    fn cut_cuboid(&mut self, cuboid: Cuboid) {
        self.cuboids = self.cuboids.iter().enumerate().flat_map(|(i, c)| 
            {
                c.cut(&cuboid).into_iter()
            }).collect();
    }

    fn volume(&self) -> usize {
        self.cuboids.iter().map(|c| c.volume()).sum()
    }

}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Cuboid {

    Empty,
    Cube(Inclusive3DRange)
}

impl Cuboid {

    fn cut(&self, other: &Self) -> Vec<Self> {

        match self {
            Cuboid::Empty => vec![],
            Cuboid::Cube(range) => {
                match other {
                    Cuboid::Empty => vec![self.clone()],
                    Cuboid::Cube(other_range) => {
                        Inclusive3DRange::cut(range, other_range)
                        .into_iter().map(|r| Cuboid::Cube(r)).collect()
                    }
                }
            }
        }
    }

    fn union(&self, other: &Self) -> Vec<Self> {
        Self::merge(self, other)
    }

    fn merge(cube_a: &Self, cube_b: &Self) -> Vec<Self> {

        match cube_a {
            Cuboid::Empty => {
                match cube_b {
                    Cuboid::Empty => vec![],
                    Cuboid::Cube(range_b) => vec![Cuboid::Cube(range_b.clone())],
                }
            },
            Cuboid::Cube(range_a) => {
                match cube_b {
                    Cuboid::Empty => vec![Cuboid::Cube(range_a.clone())],
                    Cuboid::Cube(range_b) => 
                    Inclusive3DRange::merge(range_a, range_b).into_iter().map(|r| Cuboid::Cube(r)).collect(),
                }
            },
        }
    }

    fn intersect(&self, other: &Self) -> Self {
        Self::intersection(self, other)
    }

    fn intersection(cube_a: &Self, cube_b: &Self) -> Self {

        match cube_a {
            Cuboid::Empty => {
                match cube_b {
                    Cuboid::Empty => Cuboid::Empty,
                    Cuboid::Cube(_) => Cuboid::Empty,
                }
            },
            Cuboid::Cube(range_a) => {
                match cube_b {
                    Cuboid::Empty => Cuboid::Empty,
                    Cuboid::Cube(range_b) => Cuboid::Cube(range_a.intersect(range_b)),
                }
            },
        }

    }

    fn volume(&self) -> usize {
        match self {
            Cuboid::Empty => 0,
            Cuboid::Cube(r) => r.volume(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Inclusive3DRange {

    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
    z: RangeInclusive<i32>    
}

impl Inclusive3DRange {

    fn cut(&self, cut_range: &Self) -> Vec<Self> {

        if cut_range.x.start() > self.x.end() || cut_range.x.end() < self.x.start() ||
           cut_range.y.start() > self.y.end() || cut_range.y.end() < self.y.start() || 
           cut_range.z.start() > self.z.end() || cut_range.z.end() < self.z.start() {

            return vec![self.clone()];
        }

        let mut result = Vec::new();
        let mut residue = self.clone();

        if cut_range.x.start() > residue.x.start() {

            let (left, right) = residue.cut_x(*cut_range.x.start() - 1).unwrap();
            result.push(left);
            residue = right;
        }

        if cut_range.x.end() < residue.x.end() {

            let (left, right) = residue.cut_x(*cut_range.x.end()).unwrap();
            result.push(right);
            residue = left;
        }

        if cut_range.y.start() > residue.y.start() {

            let (left, right) = residue.cut_y(*cut_range.y.start() - 1).unwrap();
            result.push(left);
            residue = right;
        }

        if cut_range.y.end() < residue.y.end() {

            let (left, right) = residue.cut_y(*cut_range.y.end()).unwrap();
            result.push(right);
            residue = left;
        }

        if cut_range.z.start() > residue.z.start() {

            let (left, right) = residue.cut_z(*cut_range.z.start() - 1).unwrap();
            result.push(left);
            residue = right;
        }

        if cut_range.z.end() < residue.z.end() {

            let (_, right) = residue.cut_z(*cut_range.z.end()).unwrap();
            result.push(right);
        }

        return result
    }

    fn cut_x(&self, cut_plane: i32) -> Result<(Inclusive3DRange, Inclusive3DRange), ()> {

        if cut_plane < *self.x.start() || cut_plane >= *self.x.end() {
            return Err(());
        }

        Ok((
            Inclusive3DRange{
                x: *self.x.start()..=cut_plane,
                y: self.y.clone(),
                z: self.z.clone()},
            Inclusive3DRange{
                x: cut_plane + 1..=*self.x.end(),
                y: self.y.clone(),
                z: self.z.clone()
            }))
    }

    fn cut_y(&self, cut_plane: i32) -> Result<(Inclusive3DRange, Inclusive3DRange), ()> {

        if cut_plane < *self.y.start() || cut_plane >= *self.y.end() {
            return Err(());
        }

        Ok((
            Inclusive3DRange{
                x: self.x.clone(),
                y: *self.y.start()..=cut_plane,
                z: self.z.clone()},
            Inclusive3DRange{
                x: self.x.clone(),
                y: cut_plane + 1..=*self.y.end(),
                z: self.z.clone()
            }))
    }

    fn cut_z(&self, cut_plane: i32) -> Result<(Inclusive3DRange, Inclusive3DRange), ()> {

        if cut_plane < *self.z.start() || cut_plane >= *self.z.end() {
            return Err(());
        }

        Ok((
            Inclusive3DRange{
                x: self.x.clone(),
                y: self.y.clone(),
                z: *self.z.start()..=cut_plane}, 
            Inclusive3DRange{
                x: self.x.clone(),
                y: self.y.clone(),
                z: cut_plane + 1..=*self.z.end()
            }))
    }

    fn merge(range_a: &Self, range_b: &Self) -> Vec<Self> {
    
        let mut res = range_a.cut(range_b);
        res.push(range_b.clone());
        res
    }

    fn intersect(&self, other: &Self) -> Self {
        Self::intersection(self, other)
    }

    fn intersection(range_a: &Self, range_b: &Self) -> Self {
        todo!()
    }

    fn volume(&self) -> usize {
        (self.x.end() - self.x.start() + 1) as usize *
        (self.y.end() - self.y.start() + 1) as usize*
        (self.z.end() - self.z.start() + 1) as usize
    }
}

struct Instruction {
    action: bool,
    range: Inclusive3DRange
}

fn main() {

    let mut instructions: Vec<Instruction> = vec![];

    let lines = BufReader::new(File::open("input.txt").unwrap()).lines().map(|l| l.unwrap());

    let re = Regex::new(r"-?\d+").unwrap();

    for line in lines {

        let mut cap_it = re.captures_iter(&line);

        let range = Inclusive3DRange {
            x: cap_it.next().unwrap()[0].parse::<i32>().unwrap()..=cap_it.next().unwrap()[0].parse::<i32>().unwrap(),
            y: cap_it.next().unwrap()[0].parse::<i32>().unwrap()..=cap_it.next().unwrap()[0].parse::<i32>().unwrap(),
            z: cap_it.next().unwrap()[0].parse::<i32>().unwrap()..=cap_it.next().unwrap()[0].parse::<i32>().unwrap()
        };

        instructions.push(Instruction { action: &line[..2] == "on", range });
    }

    let mut all_cubes = CompositeCuboid{cuboids: vec![]};

    for instruction in instructions {

        if instruction.action {
            all_cubes.add_cuboid(Cuboid::Cube(instruction.range.clone()));
        } else {
            all_cubes.cut_cuboid(Cuboid::Cube(instruction.range.clone()));
        }

    }

    println!("total volume: {}", all_cubes.volume());
}
