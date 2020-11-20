fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq, Debug)]
struct Cell {
    x: i32,
    y: i32,
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
}
