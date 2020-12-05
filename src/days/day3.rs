use std::fs;

#[derive(Debug, Clone)]
pub struct Geology {
    map: Vec<Vec<char>>
}

impl Geology {

    pub fn new(input: &String) -> Geology {
        let map: Vec<Vec<char>> = input.lines()
                    .map(|l| l.chars().collect())
                    .collect();
        Geology { map }
    }

    pub fn width(&self) -> usize {
        self.map[0].len()
    }

    pub fn height(&self) -> usize {
        self.map.len()
    }

    pub fn is_tree(&self, y: usize, x: usize) -> bool {
        match self.map[y][x % self.width()] {
            '#' => true,
            _ => false,
        }
    }

    pub fn hit_trees(&self, ystep: usize, xstep: usize) -> usize {
        let yrange = (0..self.height()).step_by(ystep);
        let xrange = (0..).step_by(xstep);
        let positions = yrange.zip(xrange);

        positions.filter(|&(y, x)| self.is_tree(y, x)).count()
    }

}

pub fn day3(args: &[String]) -> i32 {
    println!("Day 3");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    
    let geo = Geology::new(&contents);
    
    println!("Part 1: {:}", geo.hit_trees(1, 3));
    // part 2
    let slopes = vec![
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];
    let result = slopes.iter()
            .map(|&(x, y)| geo.hit_trees(y, x))
            .product::<usize>();
    println!("Part 2: {:}", result);
    0
}
