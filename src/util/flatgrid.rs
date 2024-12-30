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
    pub const fn x_coordinate(index: usize) -> usize {
        index % W
    }

    /// Gets the y coordinate of an index
    #[inline(always)]
    pub const fn y_coordinate(index: usize) -> usize {
        index / W
    }

    /// Splits the index into its x and y coordinates
    #[inline(always)]
    pub const fn to_coordinates(index: usize) -> (usize, usize) {
        (Self::x_coordinate(index), Self::y_coordinate(index))
    }

    /// Transforms x and y coordinates into an index
    #[inline(always)]
    pub const fn to_index(x: usize, y: usize) -> usize {
        x + y * W
    }

    /// Calculates the distance to the left edge from the index
    #[inline(always)]
    pub const fn distance_to_left_edge(index: usize) -> usize {
        Self::x_coordinate(index)
    }

    /// Calculates the distance to the bottom edge from the index
    #[inline(always)]
    pub const fn distance_to_bottom_edge(index: usize) -> usize {
        Self::y_coordinate(index)
    }

    /// Calculates the distance to the right edge from the index
    #[inline(always)]
    pub const fn distance_to_right_edge(index: usize) -> usize {
        W - Self::x_coordinate(index) - 1
    }

    /// Calculates the distance to the top edge from the index
    #[inline(always)]
    pub const fn distance_to_top_edge(index: usize) -> usize {
        H - Self::y_coordinate(index) - 1
    }

    /// Checks if a horizontal move starting from the index will cross a border
    pub const fn will_horizontal_move_cross_border(index: usize, direction: isize) -> bool {
        if direction < 0 {
            direction.unsigned_abs() > Self::distance_to_left_edge(index)
        } else {
            direction as usize > Self::distance_to_right_edge(index)
        }
    }

    /// Checks if a vertical move starting from the index will cross a border
    pub const fn will_vertical_move_cross_border(index: usize, direction: isize) -> bool {
        if direction < 0 {
            direction.unsigned_abs() > Self::distance_to_bottom_edge(index)
        } else {
            direction as usize > Self::distance_to_top_edge(index)
        }
    }

    /// Returns the grids width
    #[inline(always)]
    pub const fn width() -> usize {
        W
    }

    /// Returns the grids height
    #[inline(always)]
    pub const fn height() -> usize {
        H
    }

    /// Returns the grids area, which should be equal to its length
    #[inline(always)]
    pub const fn area() -> usize {
        W * H
    }

    /// Gets the index movement for a horizontal movement
    #[inline(always)]
    pub const fn horizontal_movement(amount: isize) -> isize {
        amount
    }

    /// Gets the index movement for a vertical movement
    #[inline(always)]
    pub const fn vertical_movement(amount: isize) -> isize {
        amount * W as isize
    }

    /// Gets the index movement for a combined horizontal and vertical movement
    #[inline(always)]
    pub const fn movement(horizontal: isize, vertical: isize) -> isize {
        Self::vertical_movement(vertical) + Self::horizontal_movement(horizontal)
    }

    /// Moves an index horizontally
    #[inline(always)]
    pub const fn moved_horizontally(index: usize, amount: isize) -> usize {
        (index as isize + Self::horizontal_movement(amount)) as usize
    }

    /// Moves an index vertically
    #[inline(always)]
    pub const fn moved_vertically(index: usize, amount: isize) -> usize {
        (index as isize + Self::vertical_movement(amount)) as usize
    }

    /// Moves an index both horizontally and vertically
    #[inline(always)]
    pub const fn moved(index: usize, horizontal: isize, vertical: isize) -> usize {
        (index as isize + Self::movement(horizontal, vertical)) as usize
    }

    /// Provices an iterator over the entire grids indices
    #[inline(always)]
    pub const fn indices() -> Range<usize> {
        0..Self::area()
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