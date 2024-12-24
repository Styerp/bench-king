#[derive(Debug, Default, Clone)]
pub struct BenchKingReport {
    pub owner_name: String,
    pub optimal_points: f32,
    pub actual_points: f32,
}
impl BenchKingReport {
    pub fn difference(&self) -> f32 {
        self.optimal_points - self.actual_points
    }
}
impl std::fmt::Display for BenchKingReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"
        Team: {}
        Acutal: {}, Optimal: {}, Delta: {}"#,
            self.owner_name,
            round_to_two_decimals(self.actual_points),
            round_to_two_decimals(self.optimal_points),
            round_to_two_decimals(self.difference())
        )
    }
}

fn round_to_two_decimals(f: f32) -> f32 {
    (f * 100.0).round() / 100.0
}
