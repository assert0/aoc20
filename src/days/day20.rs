use std::fs;
use std::fmt;
use regex::Regex;
use itertools::iproduct;
use std::collections::HashMap;
use itertools::concat;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Edge {
    Top,
    Right,
    Bottom,
    Left
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image {
    image: Vec<Vec<char>>
}

impl Image {

    pub fn new(image: Vec<Vec<char>>) -> Image {
        Image { image }
    }

    pub fn parse(lines: &str) -> Image {
        let image = lines.lines().map(|l| l.chars().collect()).collect();
        Image { image }
    }

    pub fn height(&self) -> usize {
        self.image.len()
    }

    pub fn width(&self) -> usize {
        self.image[0].len()
    }

    pub fn get_edge(&self, edge: Edge) -> Vec<char> {
        match edge {
            Edge::Top => self.image.first().unwrap().to_vec(),
            Edge::Right => self.image.iter().map(|r| r.last().unwrap().clone()).collect(),
            Edge::Bottom => self.image.last().unwrap().to_vec(),
            Edge::Left => self.image.iter().map(|r| r.first().unwrap().clone()).collect(),
        }
    }

    pub fn get_edge_string(&self, edge: Edge) -> String {
        self.get_edge(edge).iter().collect::<String>()
    }

    pub fn orient(&self, rotations: u8, flipped: bool) -> Image {
        let (width, height) = (self.width(), self.height());
        assert_eq!(width, height);
        let mut out = self.image.clone();
        for _r in 0..rotations {
            let mut next = vec![vec![' '; width]; height];
            for (x, y) in iproduct!((0..height).enumerate(), (0..width).rev().enumerate()) {
                next[x.0][y.0] = out[y.1][x.1];
            }
            out = next;
        }
        if flipped {
            out = out.into_iter().map(|r| r.into_iter().rev().collect()).collect();
        }
        Image { image: out }
    }

    pub fn get_without_edges(&self) -> Image {
        let mut image = Vec::<Vec<char>>::new();
        for y in 1..self.height()-1 {
            image.push(self.image[y][1..self.width()-1].to_vec());
        }
        Image { image }
    }

    pub fn contains_image(&self, input: &Image, offset: (usize, usize)) -> bool {
        if offset.0 + input.height() > self.height() || offset.1 + input.width() > self.width() {
            return false;
        }
        //println!("{:?}", offset);
        for (y, x) in iproduct!(0..input.height(), 0..input.width()) {
            if input.image[y][x] == '#' {
                if self.image[offset.0+y][offset.1+x] != '#' {
                    return false;
                }
            }
        }
        true
    }

    pub fn find_image_positions(&self, find: &Image) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();
        for pos in iproduct!(0..self.height(), 0..self.width()) {
            if self.contains_image(find, pos) {
                positions.push(pos);
            }
        }
        positions
    }

    pub fn count_char(&self, match_char: char) -> usize {
        self.image.iter().flatten().filter(|&&c| c==match_char).count()
    }
        
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = Vec::new();
        for y in 0..self.height() {
            for x in 0..self.width() {
                output.push(format!("{}", self.image[y][x]));
            }
            output.push(String::from("\n"));
        }
        write!(f, "{}", output.join(""))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tile {
    id: usize,
    image: Image
}

impl Tile {

    pub fn new(id: usize, image: Image) -> Tile {
        Tile { id, image }
    }

    pub fn parse(lines: &str) -> Tile {
        // ex:
        //    Tile 2311:
        //    ..##.#..#.
        //    ##..#.....
        //    #...##..#.
        //    ####.#...#
        //    ##.##.###.
        //    ##...#.###
        //    .#.#.#..##
        //    ..#....#..
        //    ###...#.#.
        //    ..###..###
        lazy_static! {
            static ref ID: Regex = Regex::new(r"^Tile (\d+):$").unwrap();
        }
        if let Some((first, rows)) = lines.lines().collect::<Vec<&str>>().split_first() {
            assert!(ID.is_match(first));
            let caps = ID.captures(first).unwrap();
            let id = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let image = Image::parse(&rows.join("\n"));
            return Tile { id, image };
        }
        unreachable!("Failed to parse");
    }

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arrangement {
    tiles: Vec<Tile>,
    image: Vec<Vec<Option<Tile>>>,
}

impl Arrangement {

    pub fn new(tiles: Vec<Tile>) -> Arrangement {
        let width = (tiles.len() as f64).sqrt() as usize;
        let image = vec![vec![None; width]; width];
        Arrangement { tiles, image }
    }

    pub fn height(&self) -> usize {
        self.image.len()
    }

    pub fn width(&self) -> usize {
        self.image[0].len()
    }

    fn edge_counts(&self) -> HashMap::<String, usize> {
        let edges = vec![Edge::Top, Edge::Right, Edge::Bottom, Edge::Left];
        let rotations = 0..4;

        //count the occurances of each edge
        let mut counts = HashMap::<String, usize>::new();
        for (rotations, flipped, tile) in iproduct!(rotations, vec![false, true], &self.tiles) {
            let i = tile.image.orient(rotations, flipped);
            for edge in &edges {
                let e = i.get_edge_string(*edge);
                match counts.get_mut(&e) {
                    Some(c) => *c+=1,
                    None => match counts.insert(e, 1) {_=>()},
                };
            }
        }
        counts
    }

    fn outer_edges(&self) -> Vec<String> {
        let counts = self.edge_counts();
        counts.iter().filter(|&(_e, c)| *c == 4).map(|(e, _c)| e.clone()).collect::<Vec<String>>()
    }

    pub fn find_corners(&self) -> Vec<Tile> {
        let outer = self.outer_edges();

        // find the 4 tiles with two outer edges
        let mut corners = Vec::new();
        for tile in self.tiles.iter() {
            let count = vec![Edge::Top, Edge::Right, Edge::Bottom, Edge::Left]
                                .iter().map(|e| tile.image.get_edge_string(*e))
                                .filter(|e| outer.contains(e))
                                .count();
            
            if count == 2 {
                corners.push(tile.clone());
            }
        }
        corners
    }

    pub fn build_image(&mut self) {
        let mut first_corner = self.find_corners()[0].image.clone();
        let outer_edges = self.outer_edges();
        // orient the first corner in the top left with the outer edges in the correct position
        for (rotation, flipped) in iproduct!(0..4, vec![false, true]) {
            let c = first_corner.orient(rotation, flipped);
            let left = c.get_edge(Edge::Left).iter().collect::<String>();
            let top = c.get_edge(Edge::Top).iter().collect::<String>();
            if outer_edges.contains(&left) && outer_edges.contains(&top) {
                first_corner = c;
                break;
            }
        }
        //println!("{}", first_corner);
        let mut available_tiles = self.tiles.clone();
        let mut edge = Edge::Left;
        let mut matching_edge = first_corner.get_edge(Edge::Left);
        for (y, x) in iproduct!(0..self.height(), 0..self.width()) {
            //println!("{} {} {:?} {}", y, x, edge, matching_edge.iter().collect::<String>());
            let t = self.find_tile(edge, matching_edge, &available_tiles);
            let pos = available_tiles.iter().position(|n| n.id == t.as_ref().unwrap().id);
            if pos.is_some() {
                available_tiles.remove(pos.unwrap());
            }
            self.image[y][x] = t;
            if x == self.width()-1 {
                edge = Edge::Top;
                matching_edge = self.image[y][0].as_ref().unwrap().image.get_edge(Edge::Bottom);
            } else {
                edge = Edge::Left;
                matching_edge = self.image[y][x].as_ref().unwrap().image.get_edge(Edge::Right);
            }
        }
    }

    fn find_tile(&self, edge: Edge, matching_edge: Vec<char>, available_tiles: &Vec<Tile>) -> Option<Tile> {
        for (rotation, flipped, tile) in iproduct!(0..4, vec![false, true], available_tiles) {
            let i = tile.image.orient(rotation, flipped);
            if i.get_edge(edge) == matching_edge {
                return Some(Tile { id: tile.id, image: i });
            }
        }
        None
    }

    pub fn get_corners(&self) -> Vec<usize> {
        vec![
            self.image[0][0].as_ref().unwrap().id, 
            self.image[0][self.width()-1].as_ref().unwrap().id,
            self.image[self.height()-1][0].as_ref().unwrap().id, 
            self.image[self.height()-1][self.width()-1].as_ref().unwrap().id
        ]
    }

    pub fn get_result(&self) -> Image {
        let mut image = Vec::<Vec<char>>::new();
        for y in 0..self.height() {
            let row: Vec<Image> = (0..self.width()).map(|x| self.image[y][x].as_ref().unwrap().image.get_without_edges()).collect();
            for r in 0..row[0].height() {
                image.push(concat(row.iter().map(|i| i.image[r].clone()).collect::<Vec<Vec<char>>>()));
            }
        }
        Image { image }
    }

}

pub fn day20(args: &[String]) -> i32 {
    println!("Day 20");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let tiles: Vec<Tile> = contents.split("\n\n")
                                   .map(|s| Tile::parse(s))
                                   .collect();

    let mut arrangement = Arrangement::new(tiles.clone());
    println!("Part 1: {}", arrangement.find_corners().iter().map(|t| t.id).product::<usize>());

    arrangement.build_image();
    let result = arrangement.get_result();
    //println!("{}", result.orient(1, true));

    let monster = Image::parse(
        ["                  # ",
         "#    ##    ##    ###",
         " #  #  #  #  #  #   "].join("\n").as_str()
    );
    let monster_char_count = monster.count_char('#');

    for (rotations, flipped) in iproduct!(0..4, vec![false, true]) {
        let r = result.orient(rotations, flipped);
        let found_count = r.find_image_positions(&monster).len();
        if found_count > 0 {
            println!("Part 2: {:?}", r.count_char('#') - monster_char_count * found_count);
        }
    }

    0
}
