struct Game {
    frame: i64,
    score: i64,
    frame_scores: Vec<i64>,
}

impl Game {
    fn roll(&mut self, pins: i64) {
        self.frame_scores.push(pins);

        self.score += pins;
    }

    fn get_score(&self) -> i64 {
        self.score
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_game() {
        let game = Game { score: 0 };
    }

    #[test]
    fn test_can_roll() {
        let mut game = Game { score: 0 };
        game.roll(0);
    }

    #[test]
    fn test_calc_score() {
        let mut game = Game { score: 0 };
        for _ in 0..20 {
            game.roll(0);
        }
        let score = game.get_score();
        assert_eq!(score, 0);
    }

    #[test]
    fn test_calc_score_all_1() {
        let mut game = Game { score: 0 };
        for _ in 0..20 {
            game.roll(1);
        }
        let score = game.get_score();
        assert_eq!(score, 20);
    }

    #[test]
    fn test_calc_score_2_spares() {
        let mut game = Game { score: 0 };
        game.roll(5);
        game.roll(5);

        game.roll(5);
        game.roll(5);
        let score = game.get_score();
        assert_eq!(score, 25);
    }
}
