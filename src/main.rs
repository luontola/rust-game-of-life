fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq, Debug, Clone)]
struct Cell {
    x: i32,
    y: i32,
}

fn neighbours( cell: Cell ) -> Vec<Cell> {
    vec![Cell { x: cell.x - 1, y: cell.y + 1 },
        Cell { x: cell.x , y: cell.y + 1 },
        Cell { x: cell.x + 1, y: cell.y + 1 },
        Cell { x: cell.x - 1, y: cell.y },
        Cell { x: cell.x + 1, y: cell.y },
        Cell { x: cell.x - 1, y: cell.y - 1 },
        Cell { x: cell.x , y: cell.y - 1 },
        Cell { x: cell.x + 1, y: cell.y - 1 }]
}

fn next_generation( alive_cells: &Vec<Cell> ) -> Vec<Cell> {
    alive_cells.clone()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_cell_equality() {
        let cell1 = Cell { x: 1, y: 2 };
        let cell2 = Cell { x: 1, y: 2 };
        assert_eq!(cell1, cell2);
    }

    #[test]
    fn test_cell_neighbours() {
        assert_eq!(vec![
            Cell { x: 4, y: 16 },
            Cell { x: 5, y: 16 },
            Cell { x: 6, y: 16 },
            Cell { x: 4, y: 15 },
            Cell { x: 6, y: 15 },
            Cell { x: 4, y: 14 },
            Cell { x: 5, y: 14 },
            Cell { x: 6, y: 14 }
            ], 
            neighbours(Cell { x: 5, y: 15 }));
    }

    #[test]
    fn test_next_generation_square() {
        let square = vec![
            Cell { x: 0, y: 0 },
            Cell { x: 0, y: 1 }, 
            Cell { x: 1, y: 0 }, 
            Cell { x: 1, y: 1 }
        ];
        assert_eq!(square, next_generation(&square));
    }
}
