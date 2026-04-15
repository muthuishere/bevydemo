// SRP: Only handles score calculation, stars, and streaks
#[derive(Debug, Clone, Default)]
pub struct SessionScore {
    pub correct: u32,
    pub incorrect: u32,
    pub streak: u32,
    pub max_streak: u32,
    pub time_bonus: u32,
}

impl SessionScore {
    pub fn record_answer(&mut self, correct: bool) {
        if correct {
            self.correct += 1;
            self.streak += 1;
            if self.streak > self.max_streak {
                self.max_streak = self.streak;
            }
        } else {
            self.incorrect += 1;
            self.streak = 0;
        }
    }

    pub fn accuracy(&self) -> f32 {
        let total = self.correct + self.incorrect;
        if total == 0 {
            0.0
        } else {
            self.correct as f32 / total as f32
        }
    }

    pub fn stars(&self, mastery_threshold: f32) -> u8 {
        let acc = self.accuracy();
        if acc >= 0.95 {
            3
        } else if acc >= mastery_threshold {
            2
        } else if acc >= mastery_threshold * 0.7 {
            1
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_score_gives_3_stars() {
        let mut s = SessionScore::default();
        for _ in 0..10 {
            s.record_answer(true);
        }
        assert_eq!(s.stars(0.8), 3);
        assert_eq!(s.accuracy(), 1.0);
        assert_eq!(s.max_streak, 10);
    }

    #[test]
    fn test_partial_score() {
        let mut s = SessionScore::default();
        for _ in 0..8 {
            s.record_answer(true);
        }
        for _ in 0..2 {
            s.record_answer(false);
        }
        assert_eq!(s.correct, 8);
        assert_eq!(s.incorrect, 2);
        assert!((s.accuracy() - 0.8).abs() < 0.01);
        assert_eq!(s.stars(0.8), 2);
    }

    #[test]
    fn test_streak_resets_on_wrong() {
        let mut s = SessionScore::default();
        s.record_answer(true);
        s.record_answer(true);
        s.record_answer(false);
        assert_eq!(s.streak, 0);
        assert_eq!(s.max_streak, 2);
    }
}
