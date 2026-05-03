fn main() {
    let mut board = Board::new(7, 7);
    board.set_blocked(6, 0);
    board.set_blocked(6, 1);
    board.set_blocked(3, 6);
    board.set_blocked(4, 6);
    board.set_blocked(5, 6);
    board.set_blocked(6, 6);

    let piece_specs = create_piece_specs();
    let pieces: Vec<Piece> = piece_specs.iter().map(|ps| Piece::from_piece_spec(ps)).collect();

    let mut placed_pieces = PlacedPieces::new();

//     for piece_spec in piece_specs.iter() {
//         let edges = piece_spec.edges();
//         let outline = edges.find_outline();
//         let piece = Piece::from_piece_spec(piece_spec);
// //        println!("{:#?}", outline);
//         println!("{:#?} = {:?}", piece.square_count, piece.orientations.len());
// //        println!("{:#?}", piece);
//     }
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

#[derive(Debug, Clone, PartialEq, Copy, Eq, PartialOrd, Ord)]
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

    pub fn orient(self, o: &Orientation) -> PieceSquare {
        let p: Point = self.into();
        p.orient(o).into()
    }

    pub fn translate(&self, t: &Translation) -> Self {
        PieceSquare {
            x: self.x + t.dx,
            y: self.y + t.dy,
        }
    }
}

impl From<Point> for PieceSquare {
    fn from(item: Point) -> PieceSquare {
        PieceSquare {x: item.x, y: item.y }
    }
}

impl From<PieceSquare> for Point {
    fn from(item: PieceSquare) -> Point {
        Point {x: item.x, y: item.y }
    }
}

pub struct PieceSpec {
    squares: Vec<PieceSquare>
}

impl PieceSpec {
    pub fn edges(&self) -> Edges {
        edges_from_piece_squares(&self.squares)
    }
}

pub fn edges_from_piece_squares(squares: &Vec<PieceSquare>) -> Edges {
    let mut ret = Vec::<Edge>::new();
    for square in squares {
        ret.extend(&square.edges());
    }
    Edges {
        edges: ret,
    }
}

//--------------------------------------------------

// Compute various forms of the pieces


#[derive(Debug, Clone, PartialEq, Copy, Eq, PartialOrd, Ord)]
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

    pub fn rot_90_ccw(&self) -> Point {
        Point {x: self.y, y: -self.x}
    }

    pub fn rot_180_ccw(&self) -> Point {
        self.rot_90_ccw().rot_90_ccw()
    }

    pub fn rot_270_ccw(&self) -> Point {
        self.rot_90_ccw().rot_90_ccw().rot_90_ccw()
    }

    pub fn flip_h(&self) -> Point {
        Point {x: -self.x, y: self.y}
    }

    pub fn orient(&self, o: &Orientation) -> Point {
        match o {
            Orientation::Original => *self,
            Orientation::Rot90CCW => self.rot_90_ccw(),
            Orientation::Rot180CCW => self.rot_180_ccw(),
            Orientation::Rot270CCW => self.rot_270_ccw(),
            Orientation::FlipH => self.flip_h(),
            Orientation::FlipHRot90CCW => self.flip_h().rot_90_ccw(),
            Orientation::FlipHRot180CCW => self.flip_h().rot_180_ccw(),
            Orientation::FlipHRot270CCW => self.flip_h().rot_270_ccw(),
        }
    }

    pub fn translate(&self, t: &Translation) -> Point {
        Point {
            x: self.x + t.dx,
            y: self.y + t.dy,
        }
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

pub enum Orientation {
    Original,
    Rot90CCW,
    Rot180CCW,
    Rot270CCW,
    FlipH,
    FlipHRot90CCW,
    FlipHRot180CCW,
    FlipHRot270CCW,
}

impl Orientation {
    pub fn all() -> Vec<Self> {
        vec!(
            Self::Original,
            Self::Rot90CCW,
            Self::Rot180CCW,
            Self::Rot270CCW,
            Self::FlipH,
            Self::FlipHRot90CCW,
            Self::FlipHRot180CCW,
            Self::FlipHRot270CCW,
            )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
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

    pub fn orient(&self, o: &Orientation) -> Path {
        Path {
            points: self.points.iter().map(|p| p.orient(o)).collect(),
        }
    }

    pub fn translate(&self, t: &Translation) -> Path {
        Path {
            points: self.points.iter().map(|p| p.translate(t)).collect(),
        }
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

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PieceSquaresBuilder {
    pub extents: Option<Extents>,
    pub squares: Vec<PieceSquare>,
}

impl PieceSquaresBuilder {
    pub fn add(&mut self, s: PieceSquare) {
        match &mut self.extents {
            Some(e) => e.add(&s),
            None => self.extents = Some(Extents::new(&s)),
        }
        self.squares.push(s);
    }

    pub fn from_spec(spec: &PieceSpec, o: &Orientation) -> PieceSquares {
        let mut ps1 = PieceSquaresBuilder::default();
        for &s in spec.squares.iter() {
            ps1.add(s.orient(o));
        }
        let extents = ps1.extents.unwrap_or_default();
        let translation = extents.to_translation();
        let translated_extents = extents.translate(&translation);
        let mut squares:Vec<PieceSquare> = ps1.squares.iter().map(|s| s.translate(&translation)).collect();
        // Sort the squares so we can compare against other sets of squares
        squares.sort();

        let outline = edges_from_piece_squares(&squares).find_outline().unwrap_or_default();

        PieceSquares {
            extents: translated_extents,
            squares,
            outline,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PieceSquares {
    pub extents: Extents,
    pub squares: Vec<PieceSquare>,
    pub outline: Path,
}

#[derive(Debug, Clone, PartialEq, Copy, Eq, PartialOrd, Ord, Default)]
pub struct Extents {
    l: i32,
    r: i32,
    t: i32,
    b: i32,
}

impl Extents {
    pub fn new(s: &PieceSquare) -> Extents {
        Extents {
            l: s.x,
            r: s.x,
            t: s.y,
            b: s.y,
        }
    }

    pub fn add(&mut self, s: &PieceSquare) {
        if s.x < self.l {
            self.l = s.x;
        }
        if s.x > self.r {
            self.r = s.x;
        }
        if s.y < self.t {
            self.t = s.y;
        }
        if s.y > self.b {
            self.b = s.y;
        }
    }

    // Returns the translate that moves l, t to 0, 0
    pub fn to_translation(&self) -> Translation {
        Translation {
            dx: -self.l,
            dy: -self.t,
        }
    }

    pub fn translate(&self, t: &Translation) -> Extents {
        Extents {
            l: self.l + t.dx,
            r: self.r + t.dx,
            t: self.t + t.dy,
            b: self.b + t.dy,
        }
    }
}

pub struct Translation {
    pub dx: i32,
    pub dy: i32,
}


#[derive(Debug, Clone)]
pub struct Piece {
    pub square_count: usize,
    pub orientations: Vec<PieceSquares>,
}

impl Piece {
    pub fn from_piece_spec(spec: &PieceSpec) -> Piece {
        let square_count = spec.squares.len();
        let mut orientations: Vec<PieceSquares> = Orientation::all().iter().map(|o| PieceSquaresBuilder::from_spec(&spec, o)).collect();
        // Remove duplicate orientations
        orientations.sort();
        orientations.dedup();
        Piece {square_count, orientations}
    }
}

//--------------------------------------------------

pub struct PlacedPieces {
    pub next_piece_num: usize,
    pub placed_pieces: Vec<PlacedPiece>,
}

impl PlacedPieces {
    pub fn new() -> Self {
        Self {
            next_piece_num: 0,
            placed_pieces: vec!(),
        }
    }
}

pub struct PlacedPiece {
    pub piece_num: usize,
    pub placement: Placement,
}

pub enum Placement {
    // The piece is deliberately left off
    SetAside,
    // Successfully placed
    Placed(PiecePlacement),
}

pub struct PiecePlacement {
    pub orientation_num: usize,
    pub x: i32,
    pub y: i32,
}

impl PiecePlacement {
    pub fn new() -> Self {
        PiecePlacement {
            orientation_num: 0,
            x: 0,
            y: 0,
        }
    }
    
    pub fn can_place(&self, board: &Board, piece: &Piece) -> bool {
        let piece_squares = &piece.orientations[self.orientation_num];
        board.can_place_squares_at(self.x, self.y, piece_squares)
    }
    
    pub fn place(&self, board: &mut Board, piece: &Piece) {
        let piece_squares = &piece.orientations[self.orientation_num];
        board.place_squares_at(self.x, self.y, piece_squares)
    }
    
    pub fn unplace(&self, board: &mut Board, piece: &Piece) {
        let piece_squares = &piece.orientations[self.orientation_num];
        board.unplace_squares_at(self.x, self.y, piece_squares)
    }
    
    pub fn next_to_try(&self, board: &Board, piece: &Piece) -> Option<Self> {
        let piece_squares = &piece.orientations[self.orientation_num];
        if self.x < (board.column_count as i32) - piece_squares.extents.r {
            Some(Self {
                orientation_num: self.orientation_num,
                x: self.x + 1,
                y: self.y,
            })
        }
        else if self.y < (board.row_count as i32) - piece_squares.extents.b {
            Some(Self {
                orientation_num: self.orientation_num,
                x: 0,
                y: self.y + 1,
            })
        }
        else if self.orientation_num < piece.orientations.len() {
            Some(Self {
                orientation_num: self.orientation_num + 1,
                x: 0,
                y: 0,
            })
        }
        else {
            None
        }
    }

    pub fn next_placeable(&mut self, board: &Board, piece: &Piece) -> Option<Self> {
        while let Some(p) = self.next_to_try(board, piece) {
            if p.can_place(board, piece) {
                Some(p)
            }
        }
        None
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Board {
    pub column_count: usize,
    pub row_count: usize,
    pub rows: Vec<BoardRow>,
    pub empty_count: usize,
    pub set_aside_count: usize,
}

impl<'a> Board {
    pub fn new(row_count: usize, column_count: usize) -> Self {
        let mut rows = Vec::<BoardRow>::new();
        for _ in 0..row_count {
            rows.push(BoardRow::new(column_count));
        }
        Board {
            row_count,
            column_count,
            rows,
            empty_count: row_count * column_count,
            set_aside_count: 0,
        }
    }
    
    pub fn square_at(&'a self, x: i32, y: i32) -> &'a BoardSquare {
        let xu:usize = usize::try_from(x).expect("x cannot be negative");
        let yu:usize = usize::try_from(y).expect("y cannot be negative");
        &self.rows[yu].squares[xu]
    }

    pub fn square_at_mut(&'a mut self, x: i32, y: i32) -> &'a mut BoardSquare {
        let xu:usize = usize::try_from(x).expect("x cannot be negative");
        let yu:usize = usize::try_from(y).expect("y cannot be negative");
        &mut self.rows[yu].squares[xu]
    }
    
    pub fn set_status(&mut self, x: i32, y: i32, status: BoardSquareStatus) {
        let square = self.square_at_mut(x, y);
        let status_before = square.status;
        square.status = status;
        if status_before.is_empty() && !status.is_empty() {
            self.empty_count -= 1;
        }
        else if !status_before.is_empty() && status.is_empty() {
            self.empty_count += 1;
        }
    }
    
    pub fn set_blocked(&mut self, x: i32, y: i32) {
        self.set_status(x, y, BoardSquareStatus::Blocked)
    }

    pub fn can_place_at(&self, x: i32, y: i32) -> bool {
        self.square_at(x, y).status.can_place()
    }

    pub fn place_at(&mut self, x: i32, y: i32) {
        self.square_at_mut(x, y).status = BoardSquareStatus::Placed;
    }

    pub fn unplace_at(&mut self, x: i32, y: i32) {
        self.square_at_mut(x, y).status = BoardSquareStatus::Empty;
    }

    pub fn can_place_squares_at(&self, x: i32, y: i32, squares: &PieceSquares) -> bool {
        for &square in &squares.squares {
            if !self.can_place_at(x + square.x, y + square.y) {
                return false
            }
        }
        true
    }

    pub fn place_squares_at(&mut self, x: i32, y: i32, squares: &PieceSquares) {
        for &square in &squares.squares {
            self.place_at(x + square.x, y + square.y)
        }
    }

    pub fn unplace_squares_at(&mut self, x: i32, y: i32, squares: &PieceSquares) {
        for &square in &squares.squares {
            self.unplace_at(x + square.x, y + square.y)
        }
    }

    pub fn can_set_aside(&self, count: usize) -> bool {
        // At most 2 squares can be left set aside
        self.set_aside_count + count <= 2
    }

    pub fn set_aside(&mut self, count: usize) {
        self.set_aside_count -= count
    }

    pub fn unset_aside(&mut self, count: usize) {
        self.set_aside_count += count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoardRow {
    pub squares: Vec<BoardSquare>
}

impl BoardRow {
    pub fn new(column_count: usize) -> Self {
        let mut squares = Vec::<BoardSquare>::new();
        for _ in 0..column_count {
            squares.push(BoardSquare::default());
        }
        BoardRow {
            squares
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct BoardSquare {
    status: BoardSquareStatus
}

impl BoardSquare {
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BoardSquareStatus {
    // One of the squares that's supposed to remain open as a target
    Target,
    // Permanently blocked square on the board
    Blocked,
    // Occupied by a piece
    Placed,
    // Available for a piece
    Empty,
}

impl BoardSquareStatus {
    pub fn can_place(&self) -> bool {
        self.is_empty()
    }

    pub fn is_empty(&self) -> bool {
        *self == Self::Empty
    }
}

impl Default for BoardSquareStatus {
    fn default() -> Self {
        Self::Empty
    }
}
