fn main() {
    let piece_specs = create_piece_specs();
    for piece_spec in piece_specs.iter() {
        let mut edges = piece_spec.edges();
        edges.remove_and_combine_edges();
        println!("{:#?}", edges);
    }
}

//--------------------------------------------------

// Create the pieces
// Positive x goes to the right, positive y goes down.  The board's upper left is 0, 0
// Pieces all define one square at 0,0 and must not extend to the left or up (-x or -y)

pub fn create_piece_specs() -> Vec<PieceSpec> {
    vec!(
        // PieceSpec {
        //     squares: vec!(
        //         PieceSquare {x: 0, y: 0},
        //         PieceSquare {x: 1, y: 0},
        //         PieceSquare {x: 0, y: 1},
        //         PieceSquare {x: 1, y: 1},
        //     ),
        // },
        // PieceSpec {
        //     squares: vec!(
        //         PieceSquare {x: 0, y: 0},
        //         PieceSquare {x: 0, y: 1},
        //         PieceSquare {x: 1, y: 1},
        //         PieceSquare {x: 2, y: 1},
        //         PieceSquare {x: 2, y: 0},
        //     ),
        // },
        // PieceSpec {
        //     squares: vec!(
        //         PieceSquare {x: 0, y: 0},
        //         PieceSquare {x: 0, y: 1},
        //         PieceSquare {x: 0, y: 2},
        //         PieceSquare {x: 0, y: 3},
        //         PieceSquare {x: 1, y: 2},
        //     ),
        // },
        // PieceSpec {
        //     squares: vec!(
        //         PieceSquare {x: 0, y: 0},
        //         PieceSquare {x: 0, y: 1},
        //         PieceSquare {x: 0, y: 2},
        //         PieceSquare {x: 0, y: 3},
        //         PieceSquare {x: 1, y: 0},
        //     ),
        // },
        // PieceSpec {
        //     squares: vec!(
        //         PieceSquare {x: 0, y: 0},
        //         PieceSquare {x: 1, y: 0},
        //         PieceSquare {x: 2, y: 0},
        //         PieceSquare {x: 2, y: 1},
        //         PieceSquare {x: 3, y: 1},
        //     ),
        // },
        // PieceSpec {
        //     squares: vec!(
        //         PieceSquare {x: 0, y: 0},
        //         PieceSquare {x: 0, y: 1},
        //         PieceSquare {x: 1, y: 1},
        //         PieceSquare {x: 2, y: 1},
        //         PieceSquare {x: 2, y: 2},
        //     ),
        // },
        // PieceSpec {
        //     squares: vec!(
        //         PieceSquare {x: 0, y: 0},
        //         PieceSquare {x: 0, y: 1},
        //         PieceSquare {x: 0, y: 2},
        //         PieceSquare {x: 1, y: 2},
        //         PieceSquare {x: 2, y: 2},
        //     ),
        // },
        // PieceSpec {
        //     squares: vec!(
        //         PieceSquare {x: 0, y: 0},
        //         PieceSquare {x: 1, y: 0},
        //         PieceSquare {x: 2, y: 0},
        //         PieceSquare {x: 0, y: 1},
        //         PieceSquare {x: 1, y: 1},
        //         PieceSquare {x: 2, y: 1},
        //         PieceSquare {x: 3, y: 1},
        //     ),
        // },
        // PieceSpec {
        //     squares: vec!(
        //         PieceSquare {x: 0, y: 0},
        //     ),
        // },
        // PieceSpec {
        //     squares: vec!(
        //         PieceSquare {x: 0, y: 0},
        //     ),
        // },
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

#[derive(Debug, Clone, PartialEq, Copy, Eq)]
pub struct Edge {
    pub p1: Point,
    pub p2: Point,
}

#[derive(Debug, Clone, PartialEq, Copy, Eq)]
pub enum EdgeOrientation {
    Horizontal,
    Vertical,
}

impl Edge {
    // Return true if these are the same edge, even with points flipped
    pub fn same_edge(&self, e2: &Edge) -> bool {
        (self.p1 == e2.p1 && self.p2 == e2.p2) ||
            (self.p1 == e2.p2 && self.p2 == e2.p1)
    }

    // Return if the edge is oriented horizontally or vertically
    pub fn orientation(&self) -> EdgeOrientation {
        if self.p1.x == self.p2.x {EdgeOrientation::Vertical}
        else {EdgeOrientation::Horizontal}
    }

    // If this edge and the given edge are the same direction and share a common endpoint, return a single edge that encompasses both
    pub fn combine(&self, e2: &Edge) -> Option<Edge> {
        if self.orientation() != e2.orientation() {
            None
        }
        else {
            if self.p1 == e2.p1 {
                Some(Edge { p1: self.p2, p2: e2.p2 })
            }
            else if self.p1 == e2.p2 {
                Some(Edge { p1: self.p2, p2: e2.p1 })
            }
            else if self.p2 == e2.p1 {
                Some(Edge { p1: self.p1, p2: e2.p2 })
            }
            else if self.p2 == e2.p2 {
                Some(Edge { p1: self.p1, p2: e2.p1 })
            }
            else {
                None
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Edges {
    edges: Vec<Edge>
}

impl Edges {
    // Searches all the edges, except for ix, looking for an edge that is the same as the edge at ix.  If found, returns the index of the found edge
    pub fn find_same_edge(&self, e: &Edge, ix: usize) -> Option<usize> {
        for (i, e2) in self.edges.iter().enumerate() {
            if ix != i && e.same_edge(e2) {
                return Some(ix)
            }
        }
        None
    }

    // Removes one pair of common edges.  Returns true if any were removed
    pub fn remove_common_edges_one_pass(&mut self) -> bool {
        for (i, e) in self.edges.iter().enumerate() {
            // See if an edge is found
            if let Some(ix) = self.find_same_edge(e, i) {
                if i < ix {
                    self.edges.remove(ix);
                    self.edges.remove(i);
                }
                else {
                    self.edges.remove(i);
                    self.edges.remove(ix);
                }
                return true
            }
        }
        false
    }

    // Keeps removing common edges until none remain.  Returns true if any were removed
    pub fn remove_common_edges(&mut self) -> bool {
        let mut ret = false;
        loop {
            if self.remove_common_edges_one_pass() {
                ret = true;
            }
            else {
                return ret
            }
        }
    }

    // Searches all the edges, except for ix, looking for an edge that can be combined with the edge at ix.  If found, returns the index of the found edge and the new combined edge
    pub fn find_combinable_edge(&self, e: &Edge, ix: usize) -> Option<(usize, Edge)> {
        for (i, e2) in self.edges.iter().enumerate() {
            if ix != i && let Some(ec) = e.combine(e2) {
                return Some((ix, ec))
            }
        }
        None
    }

    // Combines one pair of edges.  Returns true if any were combined
    pub fn combine_edges_one_pass(&mut self) -> bool {
        for (i, e) in self.edges.iter().enumerate() {
            // See if an edge is found
            if let Some((ix, ec)) = self.find_combinable_edge(e, i) {
                if i < ix {
                    self.edges.remove(ix);
                    self.edges.remove(i);
                }
                else {
                    self.edges.remove(i);
                    self.edges.remove(ix);
                }
                self.edges.push(ec);
                return true
            }
        }
        false
    }

    // Keeps combining edges until none remain.  Returns true if any were combined
    pub fn combine_edges(&mut self) -> bool {
        let mut ret = false;
        loop {
            if self.combine_edges_one_pass() {
                println!("combined edges!!");
                println!("{:#?}", self);
                ret = true;
            }
            else {
                return ret
            }
        }
    }

    pub fn remove_and_combine_edges_one_pass(&mut self) -> bool {
        self.remove_common_edges() || self.combine_edges()
    }

    pub fn remove_and_combine_edges(&mut self) {
        while self.remove_and_combine_edges_one_pass() {}
    }
}
