use std::{str::FromStr, usize};

#[derive(Debug)]
pub struct CharGrid {
    cols: usize,
    cells: Vec<char>,
}

impl CharGrid {
    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn at(&self, pos: usize) -> Option<char> {
        self.cells.get(pos).copied()
    }

    pub fn adj_cells(&self, pos: usize) -> Vec<char> {
        let cols = self.cols;
        let len = self.len();

        let has_left = pos % cols != 0;
        let has_top = pos >= cols;
        let has_right = pos + 1 % cols != 0;
        let has_bottom = pos < len - cols;

        let mut adj_pos = Vec::with_capacity(8);

        if has_top {
            if has_left {
                adj_pos.push(pos - cols - 1);
            }

            adj_pos.push(pos - cols);

            if has_right {
                adj_pos.push(pos - cols + 1);
            }
        }

        if has_left {
            adj_pos.push(pos - 1);
        }

        if has_right {
            adj_pos.push(pos + 1);
        }

        if has_bottom {
            if has_left {
                adj_pos.push(pos + cols - 1);
            }

            adj_pos.push(pos + cols);

            if has_right {
                adj_pos.push(pos + cols + 1);
            }
        }

        adj_pos.into_iter().filter_map(|p| self.at(p)).collect()
    }

    pub fn cells(&self) -> std::slice::Iter<'_, char> {
        self.cells.iter()
    }
}

impl FromStr for CharGrid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cols = s.find('\n').unwrap_or(s.len());
        let cells = s.chars().filter(|c| *c != '\n').collect();

        Ok(CharGrid { cols, cells })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_grid() {
        let source = "\
abc
123
def
456
";

        let grid = CharGrid::from_str(source).unwrap();

        assert_eq!(12, grid.cells.len(), "cell count");
        assert_eq!(3, grid.cols(), "column count");

        assert_eq!(Some('a'), grid.at(0), "first cell");
        assert_eq!(Some('c'), grid.at(2), "end of first line");
        assert_eq!(Some('1'), grid.at(3), "start of second line");
        assert_eq!(Some('6'), grid.at(11), "last cell");
        assert_eq!(None, grid.at(12), "after last cell");

        assert_eq!(
            vec!['b', '1', '2'],
            grid.adj_cells(0),
            "adjacent cells of pos 0"
        );

        assert_eq!(Some('e'), grid.at(7));

        assert_eq!(
            vec!['1', '2', '3', 'd', 'f', '4', '5', '6'],
            grid.adj_cells(7),
            "adjacent cells of pos 0"
        );

        assert_eq!(Some('5'), grid.at(10));

        assert_eq!(
            vec!['d', 'e', 'f', '4', '6'],
            grid.adj_cells(10),
            "adjacent cells of pos 0"
        );
    }
}
