fn main() {
    let piece_specs = create_piece_specs();
    for piece_spec in piece_specs.iter() {
        let edges = piece_spec.edges();
        let outline = edges.find_outline();
        println!("{:#?}", outline);
    }
}

//--------------------------------------------------

// Create the pieces
// Positive x goes to the right, positive y goes down.  The board's upper left is 0, 0
// Pieces all define one square at 0,0 and must not extend to the left or up (-x or -y)

pub fn create_piece_specs() -> Vec<PieceSpec> {
    vec!(
        PieceSpec {
            squares: vec!(
                PieceSquare {x: 0, y: 0},
                PieceSquare {x: 1, y: 0},
                PieceSquare {x: 0, y: 1},
                PieceSquare {x: 1, y: 1},
            ),
        },
        PieceSpec {
            squares: vec!(
                PieceSquare {x: 0, y: 0},
                PieceSquare {x: 0, y: 1},
                PieceSquare {x: 1, y: 1},
                PieceSquare {x: 2, y: 1},
                PieceSquare {x: 2, y: 0},
            ),
        },
        PieceSpec {
            squares: vec!(
                PieceSquare {x: 0, y: 0},
                PieceSquare {x: 0, y: 1},
                PieceSquare {x: 0, y: 2},
                PieceSquare {x: 0, y: 3},
                PieceSquare {x: 1, y: 2},
            ),
        },
        PieceSpec {
            squares: vec!(
                PieceSquare {x: 0, y: 0},
                PieceSquare {x: 0, y: 1},
                PieceSquare {x: 0, y: 2},
                PieceSquare {x: 0, y: 3},
                PieceSquare {x: 1, y: 0},
            ),
        },
        PieceSpec {
            squares: vec!(
                PieceSquare {x: 0, y: 0},
                PieceSquare {x: 1, y: 0},
                PieceSquare {x: 2, y: 0},
                PieceSquare {x: 2, y: 1},
                PieceSquare {x: 3, y: 1},
            ),
        },
        PieceSpec {
            squares: vec!(
                PieceSquare {x: 0, y: 0},
                PieceSquare {x: 0, y: 1},
                PieceSquare {x: 1, y: 1},
                PieceSquare {x: 2, y: 1},
                PieceSquare {x: 2, y: 2},
            ),
        },
        PieceSpec {
            squares: vec!(
                PieceSquare {x: 0, y: 0},
                PieceSquare {x: 0, y: 1},
                PieceSquare {x: 0, y: 2},
                PieceSquare {x: 1, y: 2},
                PieceSquare {x: 2, y: 2},
            ),
        },
        PieceSpec {
            squares: vec!(
                PieceSquare {x: 0, y: 0},
                PieceSquare {x: 1, y: 0},
                PieceSquare {x: 2, y: 0},
                PieceSquare {x: 0, y: 1},
                PieceSquare {x: 1, y: 1},
                PieceSquare {x: 2, y: 1},
                PieceSquare {x: 3, y: 1},
            ),
        },
        PieceSpec {
            squares: vec!(
                PieceSquare {x: 0, y: 0},
            ),
        },
        PieceSpec {
            squares: vec!(
                PieceSquare {x: 0, y: 0},
            ),
        },
        PieceSpec {
            squares: vec!(
                PieceSquare {x: 0, y: 0},
                PieceSquare {x: 1, y: 0},
            ),
        },
    )
}

pub struct PieceSquare {
    pub x: i32,
    pub y: i32,
}

impl PieceSquare {
    pub fn edges(&self) -> Vec<Edge> {
        let ul = Point {x: self.x, y: self.y};
        let ur = Point {x: self.x + 1, y: self.y};
        let ll = Point {x: self.x, y: self.y + 1};
        let lr = Point {x: self.x + 1, y: self.y + 1};

        vec!(
            Edge {p1: ul, p2: ur},
            Edge {p1: ur, p2: lr},
            Edge {p1: lr, p2: ll},
            Edge {p1: ll, p2: ul},
        )
    }
}

pub struct PieceSpec {
    squares: Vec<PieceSquare>
}

impl PieceSpec {
    pub fn edges(&self) -> Edges {
        let mut ret = Vec::<Edge>::new();
        for square in &self.squares {
            ret.extend(&square.edges());
        }
        Edges {
            edges: ret,
        }
    }
}

//--------------------------------------------------

// Compute various forms of the pieces


#[derive(Debug, Clone, PartialEq, Copy, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn origin() -> Point {
        Point {x: 0, y: 0}
    }

    // Returns true if this point is in line with the other two Points.  Only handles vertical and horizontal lines
    pub fn is_inline(&self, p1: &Point, p2: &Point) -> bool {
        (self.x == p1.x && self.x == p2.x) || (self.y == p1.y && self.y == p2.y)
    }
}

#[derive(Debug, Clone, PartialEq, Copy, Eq)]
pub struct Edge {
    pub p1: Point,
    pub p2: Point,
}

#[derive(Debug, Clone, PartialEq, Copy, Eq)]
pub enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl Dir {
    // Returns the search order for dirs that are the "most left" of this dir
    pub fn leftmost_dirs(&self) -> Vec<Dir> {
        match self {
            Self::Left => vec!(Self::Down, Self::Left, Self::Up),
            Self::Down => vec!(Self::Right, Self::Down, Self::Left),
            Self::Right => vec!(Self::Up, Self::Right, Self::Down),
            Self::Up => vec!(Self::Left, Self::Up, Self::Right),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Path {
    pub points: Vec<Point>
}

impl Path {
    pub fn new() -> Path {
        Path {
            points: Vec::<Point>::new(),
        }
    }
    
    // Adds the given point.  If the point is inline with the previous two points, then the previous point is removed before adding
    pub fn add_point(&mut self, point: &Point) {
        if self.points.len() >= 2 &&
            let Some(m2) = self.points.get(self.points.len() - 2) &&
            let Some(m1) = self.points.get(self.points.len() - 1) &&
            point.is_inline(m1, m2) {
                self.points.pop();
            }
        self.points.push(*point);
    }
    
    // Combine points that are along the same line
    pub fn coalesce(&self) -> Path {
        let mut ret = Path::new();
        for p in self.points.iter() {
            ret.add_point(p);
        }

        // Add the first point back in, to coalesce to the first point
        if let Some(first) = self.points.first() {
            ret.add_point(first);
            ret.points.pop();
        }

        ret
    }
}

impl Edge {
    pub fn dir(&self) -> Dir {
        // Vertical
        if self.p1.x == self.p2.x {
            if self.p1.y < self.p2.y {
                Dir::Down
            }
            else {
                Dir::Up
            }
        }
        // Horizontal
        else {
            if self.p1.x < self.p2.x {
                Dir::Right
            }
            else {
                Dir::Left
            }
        }
    }

}

#[derive(Debug, Clone)]
pub struct Edges {
    edges: Vec<Edge>
}

impl Edges {
    // Searches for all edges that contain the given point, and returns those edges such that the first point is p
    pub fn find_edges_with_point(&self, p: Point) -> Vec<Edge> {
        let mut ret = Vec::<Edge>::new();
        for e in self.edges.iter() {
            if e.p1 == p {
                ret.push(Edge {
                    p1: p,
                    p2: e.p2,
                })
            }
            else if e.p2 == p {
                ret.push(Edge {
                    p1: p,
                    p2: e.p1,
                })
            }
        }
        ret
    }

    // Searches for the edge that connects to the given edge, that points most to the left
    pub fn find_leftmost_edge(&self, e: &Edge) -> Option<Edge> {
        // Get all the edges that attach to this edge's last point
        let edges = self.find_edges_with_point(e.p2);
        // Run through the dirs from leftmost to rightmost
        for dir in e.dir().leftmost_dirs() {
            for edge in edges.iter() {
                if edge.dir() == dir {
                    return Some(*edge)
                }
            }
        }
        None
    }

    // Search for an edge that starts at 0,0
    pub fn find_initial_edge(&self) -> Option<Edge> {
        let edges = self.find_edges_with_point(Point::origin());
        match edges.get(0) {
            Some(e) => Some(*e),
            None => None
        }
    }

    pub fn find_outline(&self) -> Option<Path> {
        let mut edges = Vec::<Edge>::new();
        if let Some(initial_edge) = self.find_initial_edge() {
            edges.push(initial_edge);

            // From the end of the outline, look for the edge that is "turning left" the most, and follow that
            while let Some(last) = edges.last() {
                if last.p2 == Point::origin() {
                    break
                }
                if let Some(e) = self.find_leftmost_edge(last) {
                    edges.push(e);
                }
                else {
                    return None
                }
            }

            // Map to the points
            let path = Path {points: edges.iter().map(|e| e.p1).collect()}.coalesce();
            Some(path)
        }
        else {
            None
        }
    }
}
