use crate::prelude::*;

pub const LOAD_BALANCER_STARTING_MILLIS: u64 = 50;

pub enum LoadBalancingAlgorithm {
    RoundRobin,
}

#[derive(Component)]
pub struct LoadBalancer {
    pub processing_power: usize,
    pub queued_requests: VecDeque<Entity>,
    pub max_queue_size: usize,
    pub available_targets: Vec<Entity>,
    pub algorithm: LoadBalancingAlgorithm,
    pub next_target_index: usize,
    pub current_progress: Timer,
}

impl LoadBalancer {
    pub fn add_request(&mut self, request: Entity) {
        self.queued_requests.push_back(request);
    }
    pub fn has_requests(&mut self) -> bool {
        self.queued_requests.len() > 0
    }
    pub fn is_queue_full(&mut self) -> bool {
        let len = self.queued_requests.len();
        len >= self.max_queue_size
    }
    pub fn get_request(&mut self) -> Option<Entity> {
        self.queued_requests.pop_front()
    }
    pub fn chose_next_target(&mut self) -> Option<Entity> {
        let num_targets = self.available_targets.len();
        let ret = self.available_targets.get(self.next_target_index).copied();
        self.next_target_index += 1;
        if self.next_target_index > num_targets - 1 {
            self.next_target_index = 0;
        }
        ret
    }
    pub fn reset_progress(&mut self) {
        self.current_progress = Timer::new(
            Duration::from_millis(LOAD_BALANCER_STARTING_MILLIS),
            TimerMode::Once,
        );
    }
}
