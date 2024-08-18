use crate::prelude::*;

#[derive(Resource, Debug, Reflect)]
pub struct GameStats {
    // count of total dropped requests,
    pub dropped_requests: usize,
    // count of how many requests we successfully processed
    pub handled_requests: usize,
    // how fast we handled each request
    pub response_times: Vec<f32>,
    // Average response time
    pub avg_response_time: f32,
}

impl GameStats {
    pub fn update_avg_response_time(&mut self) {
        let total: f32 = self.response_times.len() as f32;
        let sum: f32 = self
            .response_times
            .clone()
            .into_iter()
            .reduce(|acc, curr| acc + curr)
            .unwrap();
        self.avg_response_time = sum / total;
    }
}

impl Default for GameStats {
    fn default() -> Self {
        Self {
            dropped_requests: 0,
            handled_requests: 0,
            response_times: vec![],
            avg_response_time: 0.0,
        }
    }
}
