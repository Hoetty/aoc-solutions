use std::ops::{Deref, DerefMut, Range};

/// The flatgrid stores a two dimensional grid of a size known at **compile time** in 
/// a one dimensional vector to avoid the performance cost of indexing nested vectors.
/// 
/// A flatgrid can be created and pushed to
/// ```
/// let grid: FlatGrid<u32, 64, 64> = FlatGrid::new();
/// grid.push(1);
/// grid.push(2);
/// ```
/// 
/// A flatgrid also implements Deref and DerefMut to the underlying vector, meaning every
/// vector function is avalaible. Therefore a flatgrid can also be indixed.
/// 
/// ```
/// # let grid: FlatGrid<u32, 64, 64> = FlatGrid::new();
/// # grid.push(1);
/// # grid.push(2);
/// assert_eq!(1, grid[0]);
/// ```
/// 
/// A flatgrid is indexed by a onedimensional index of type usize, just like a normal vec,
/// but provides many methods to manipulate indizes to move them in 2D space
#[derive(Clone)]
pub struct FlatGrid<T, const W: usize, const H: usize> {
    data: Vec<T>
}

impl <T, const W: usize, const H: usize> FlatGrid<T, W, H> {
    /// Creates a new FlatGrid. The inner vector preallocates the neccessary capacity for width * height
    pub fn new() -> FlatGrid<T, W, H> {
        FlatGrid {
            data: Vec::with_capacity(W * H)
        }
    }

    /// Creates a new FlatGrid filled with the specified value
    pub fn filled(value: T) -> FlatGrid<T, W, H> where T: Clone {
        FlatGrid {
            data: vec![value; W * H]
        }
    }

    /// Gets the x coordiante of an index
    #[inline(always)]
    pub fn x_coordinate(&self, index: usize) -> usize {
        index % W
    }

    /// Gets the y coordinate of an index
    #[inline(always)]
    pub fn y_coordinate(&self, index: usize) -> usize {
        index / W
    }

    /// Splits the index into its x and y coordinates
    #[inline(always)]
    pub fn to_coordinates(&self, index: usize) -> (usize, usize) {
        (self.x_coordinate(index), self.y_coordinate(index))
    }

    /// Transforms x and y coordinates into an index
    #[inline(always)]
    pub fn to_index(&self, coords: (usize, usize)) -> usize {
        coords.0 + coords.1 * W
    }

    /// Calculates the distance to the left edge from the index
    #[inline(always)]
    pub fn distance_to_left_edge(&self, index: usize) -> usize {
        self.x_coordinate(index)
    }

    /// Calculates the distance to the bottom edge from the index
    #[inline(always)]
    pub fn distance_to_bottom_edge(&self, index: usize) -> usize {
        self.y_coordinate(index)
    }

    /// Calculates the distance to the right edge from the index
    #[inline(always)]
    pub fn distance_to_right_edge(&self, index: usize) -> usize {
        W - self.x_coordinate(index) - 1
    }

    /// Calculates the distance to the top edge from the index
    #[inline(always)]
    pub fn distance_to_top_edge(&self, index: usize) -> usize {
        H - self.y_coordinate(index) - 1
    }

    /// Checks if a horizontal move starting from the index will cross a border
    pub fn will_horizontal_move_cross_border(&self, index: usize, direction: isize) -> bool {
        if direction < 0 {
            direction.unsigned_abs() > self.distance_to_left_edge(index)
        } else {
            direction as usize > self.distance_to_right_edge(index)
        }
    }

    /// Checks if a vertical move starting from the index will cross a border
    pub fn will_vertical_move_cross_border(&self, index: usize, direction: isize) -> bool {
        if direction < 0 {
            direction.unsigned_abs() > self.distance_to_bottom_edge(index)
        } else {
            direction as usize > self.distance_to_top_edge(index)
        }
    }

    /// Returns the grids width
    #[inline(always)]
    pub fn width(&self) -> usize {
        W
    }

    /// Returns the grids height
    #[inline(always)]
    pub fn height(&self) -> usize {
        H
    }

    /// Returns the grids area, which should be equal to its length
    #[inline(always)]
    pub fn area(&self) -> usize {
        W * H
    }

    /// Gets the index movement for a horizontal movement
    #[inline(always)]
    pub fn horizontal_movement(&self, amount: isize) -> isize {
        amount
    }

    /// Gets the index movement for a vertical movement
    #[inline(always)]
    pub fn vertical_movement(&self, amount: isize) -> isize {
        amount * W as isize
    }

    /// Gets the index movement for a combined horizontal and vertical movement
    #[inline(always)]
    pub fn movement(&self, horizontal: isize, vertical: isize) -> isize {
        self.vertical_movement(vertical) + self.horizontal_movement(horizontal)
    }

    /// Moves an index horizontally
    #[inline(always)]
    pub fn moved_horizontally(&self, index: usize, amount: isize) -> usize {
        (index as isize + self.horizontal_movement(amount)) as usize
    }

    /// Moves an index vertically
    #[inline(always)]
    pub fn moved_vertically(&self, index: usize, amount: isize) -> usize {
        (index as isize + self.vertical_movement(amount)) as usize
    }

    /// Moves an index both horizontally and vertically
    #[inline(always)]
    pub fn moved(&self, index: usize, horizontal: isize, vertical: isize) -> usize {
        (index as isize + self.movement(horizontal, vertical)) as usize
    }

    /// Provices an iterator over the entire grids indices
    #[inline(always)]
    pub fn indices(&self) -> Range<usize> {
        0..self.area()
    }
}

impl <T, const W: usize, const H: usize> From<Vec<T>> for FlatGrid<T, W, H> {
    fn from(value: Vec<T>) -> Self {
        Self {
            data: value
        }
    }
}

impl <T, const W: usize, const H: usize> FromIterator<T> for FlatGrid<T, W, H> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            data: iter.into_iter().collect()
        }
    }
}

impl <T, const W: usize, const H: usize> Default for FlatGrid<T, W, H> where T: Default, T: Clone {
    fn default() -> Self {
        Self { data: vec![T::default(); W * H] }
    }
}

impl <T, const W: usize, const H: usize> Deref for FlatGrid<T, W, H> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl <T, const W: usize, const H: usize> DerefMut for FlatGrid<T, W, H> {
    fn deref_mut(&mut self) -> &mut Vec<T> {
        &mut self.data
    }
}