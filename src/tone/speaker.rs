use super::*;

pub struct Speaker {
	sound_source: Arc<Mutex<FrequencyGenerator>>,
}

impl Speaker {
	pub fn new(sound_source: Arc<Mutex<FrequencyGenerator>>) -> Self {
		Self { sound_source }
	}

	fn step(&mut self) -> f32 {
		self.sound_source
			.lock()
			.unwrap()
			.step()
	}
}

impl Iterator for Speaker {
	type Item = f32;
	fn next(&mut self) -> Option<f32> {
		Some(self.step())
	}
}

impl Source for Speaker {
	fn current_frame_len(&self) -> Option<usize> {
		Some(1)
	}

	fn channels(&self) -> u16 {
		1
	}

	fn sample_rate(&self) -> u32 {
		self.sound_source
			.lock()
			.unwrap()
			.sample_rate() as u32
	}

	fn total_duration(&self) -> Option<std::time::Duration> {
		None
	}
}