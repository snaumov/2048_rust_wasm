use game;

#[cfg(test)]
mod tests {
    #[test]
    fn squeze_2048_test() {
        assert_eq!(game::squeze_2048_test([1, 2, 3], Direction::Down), [1, 2, 3]);
        assert_eq!(game::squeze_2048_test([1, 2, 2, 3], Direction::Down), [1, 4, 3, 0]);
        assert_eq!(game::squeze_2048_test([4, 4, 4, 8], Direction::Down), [8, 4, 8, 0]);
        assert_eq!(game::squeze_2048_test([4, 4, 4, 8], Direction::Up), [0, 4, 8, 8]);

        assert_eq!(game::squeze_2048_test([4, 4, 0, 4, 0, 8, 0, 0, 0], Direction::Up), [0, 0, 0, 0, 0, 0, 4, 8, 8]);
        assert_eq!(game::squeze_2048_test([4, 4, 0, 4, 0, 8, 0, 0, 0], Direction::Down), [8, 4, 8, 0, 0, 0, 0, 0, 0]);
    }
}