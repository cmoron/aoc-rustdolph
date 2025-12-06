/// Structure pour stocker les résultats d'un jour
#[derive(Debug)]
pub struct DayResult {
    pub day: u8,
    pub part1_result: Option<String>,
    pub part1_time: Option<f64>,
    pub part2_result: Option<String>,
    pub part2_time: Option<f64>,
}

impl DayResult {
    pub fn total_time(&self) -> f64 {
        self.part1_time.unwrap_or(0.0) + self.part2_time.unwrap_or(0.0)
    }
}

/// Parse une partie (Part 1 ou Part 2) de la sortie
pub fn parse_part(output: &str, part_name: &str) -> (Option<String>, Option<f64>) {
    let mut result = None;
    let mut time = None;

    for line in output.lines() {
        if line.starts_with(part_name) {
            // Format: "Part 1: 12345"
            if let Some(value) = line.split(':').nth(1) {
                result = Some(value.trim().to_string());
            }
        } else if line.starts_with("Time:") && result.is_some() && time.is_none() {
            // Format: "Time: 0.1234ms"
            if let Some(time_str) = line.split(':').nth(1) {
                let time_str = time_str.trim().trim_end_matches("ms");
                if let Ok(t) = time_str.parse::<f64>() {
                    time = Some(t);
                }
            }
        }
    }

    (result, time)
}
