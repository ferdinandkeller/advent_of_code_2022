/// Represents the visibility of a tree.
/// If a tree is visible, then it can be seen from the border of the forest, either from the north, east, south, or west.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TreeVisibility {
    Visible,
    Hidden
}

/// Represents a tree in the forest.
/// 
/// # Fields
/// 
/// * `height` - The height of the tree.
/// * `visibility` - The visibility of the tree.
/// * `north_view_distance` - The number of trees that can be seen from the top of the tree when looking north.
/// * `east_view_distance` - The number of trees that can be seen from the top of the tree when looking east.
/// * `south_view_distance` - The number of trees that can be seen from the top of the tree when looking south.
/// * `west_view_distance` - The number of trees that can be seen from the top of the tree when looking west.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tree {
    pub height: usize,
    pub visibility: TreeVisibility,
    pub north_view_distance: usize,
    pub east_view_distance: usize,
    pub south_view_distance: usize,
    pub west_view_distance: usize,
}

impl Tree {
    /// Creates a new tree with the given height.
    /// Every other field is set to its default value.
    pub fn new(height: usize) -> Tree {
        Tree {
            height,
            visibility: TreeVisibility::Hidden,
            north_view_distance: 0,
            east_view_distance: 0,
            south_view_distance: 0,
            west_view_distance: 0,
        }
    }

    /// Returns the scenic score of the tree.
    pub fn scenic_score(&self) -> usize {
        self.north_view_distance * self.east_view_distance * self.south_view_distance * self.west_view_distance
    }
}

impl PartialOrd for Tree {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.height.cmp(&other.height))
    }
}

impl Ord for Tree {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.height.cmp(&other.height)
    }
}
