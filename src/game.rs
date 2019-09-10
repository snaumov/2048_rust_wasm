use rand::seq::SliceRandom; 

#[derive(Clone, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub enum GameStatus {
    InProgress,
    Finished,
}

pub struct Game {
    pub field: Vec<Vec<i32>>,
    pub status: GameStatus,
}


impl Game {

    pub fn make_move(&mut self, direction: Direction) {
        match direction {
            Direction::Down => {make_move_vertical(&mut self.field, Direction::Down)},
            Direction::Up => {make_move_vertical(&mut self.field, Direction::Up)},
            Direction::Left => {make_move_horizontal(&mut self.field, Direction::Left)},
            Direction::Right => {make_move_horizontal(&mut self.field, Direction::Right)},
        }
        if self.can_spawn() {
            self.spawn();
        }
    }

    fn can_spawn(&self) -> bool {
        for row in self.field.iter() {
            for col in row {
                if *col == 0 {
                    return true;
                };
            }
        }
        false
    }

    fn spawn(&mut self) {
        let mut free_cells = vec![];

        for (row_idx, row) in self.field.iter().enumerate() {
            for (col_idx, col) in row.iter().enumerate() {
                if *col == 0 {
                    free_cells.push((row_idx, col_idx));
                }
            }
        }

        let random_free_cell = free_cells.choose(&mut rand::thread_rng());
        match random_free_cell {
            Some(rand_cell) => self.field[(*rand_cell).0][(*rand_cell).1] = 2,
            None => {},
        }
    }

   
}

 fn make_move_vertical(game: &mut Vec<Vec<i32>>, direction: Direction) {

    for (col_idx, col_val) in game[0].clone().iter().enumerate() {
        let mut array_to_squeze = vec![];
        
        for row in &game.clone() {
            array_to_squeze.push(row[col_idx]);
        }
        
        let squeezed_array = squeze_2048(array_to_squeze, if direction == Direction::Up {Direction::Down} else {Direction::Up});

        for (idx, val) in squeezed_array.iter().enumerate() {
            game[idx][col_idx] = *val;
        }
    }

}

fn make_move_horizontal(game: &mut Vec<Vec<i32>>, direction: Direction) {
    for (row_idx, row) in game.clone().iter().enumerate() {
        let squeezed_array = squeze_2048(row.clone(), if direction == Direction::Left {Direction::Down} else {Direction::Up});

        game[row_idx] = squeezed_array;
    }
}

fn squeze_2048(arr: Vec<i32>, direction: Direction) -> Vec<i32> {
    let mut res_arr = vec![];
    let mut last_was_merged = false;
    let arr_len = arr.len();

    for val in if direction == Direction::Down { arr } else { let mut arr_clone = arr.clone(); arr_clone.reverse(); arr_clone } {
        if val == 0 {
            continue;
        }
        let res_arr_len = res_arr.len();
        if (res_arr_len > 0) && (res_arr[res_arr_len - 1] == val) && !last_was_merged {
            res_arr[res_arr_len - 1] = val * 2;
            last_was_merged = true;
        } else {
            res_arr.push(val);
            last_was_merged = false;
        }
    }
    
    if direction == Direction::Up {
        res_arr.reverse();
    }

    if direction == Direction::Up {
        return [vec![0; arr_len - res_arr.len()], res_arr].concat();
    }
    let res_arr_len = res_arr.len();
    [res_arr, vec![0; arr_len - res_arr_len]].concat()
}

#[cfg(test)]
mod tests {
    #[test]
    fn squeze_2048_test() {
        assert_eq!(super::squeze_2048(vec![1, 2, 3], super::Direction::Down), [1, 2, 3]);
        assert_eq!(super::squeze_2048(vec![1, 2, 2, 3], super::Direction::Down), [1, 4, 3, 0]);
        assert_eq!(super::squeze_2048(vec![4, 4, 4, 8], super::Direction::Down), [8, 4, 8, 0]);
        assert_eq!(super::squeze_2048(vec![4, 4, 4, 8], super::Direction::Up), [0, 4, 8, 8]);

        assert_eq!(super::squeze_2048(vec![4, 4, 0, 4, 0, 8, 0, 0, 0], super::Direction::Up), [0, 0, 0, 0, 0, 0, 4, 8, 8]);
        assert_eq!(super::squeze_2048(vec![4, 4, 0, 4, 0, 8, 0, 0, 0], super::Direction::Down), [8, 4, 8, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn move_down_test() {
        let mut game = vec![vec![2, 4, 8], vec![8, 4, 2], vec![0, 0, 0]];

        let expected_game = vec![vec![0, 0, 0], vec![2, 0, 8], vec![8, 8, 2]];

        super::make_move_down(&mut game);

        assert_eq!(game, expected_game);
    }
}