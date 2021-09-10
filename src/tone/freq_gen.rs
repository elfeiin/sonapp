use super::*;

pub struct FrequencyGenerator {
	sample_rate: f64,
	frequencies: HashMap<u64, f64>,
}

impl FrequencyGenerator {
	pub fn new(sample_rate: f64) -> Self {
		Self {
			sample_rate,
			frequencies: HashMap::new(),
		}
	}

	pub fn step(&mut self) -> f32 {
		let mut output = 0.0;
		let mut set = false;
		for (freq, theta) in self
			.frequencies
			.iter_mut()
		{
			let tmp = theta.sin() as f32;
			*theta += f64::from_bits(*freq) * (TAU / self.sample_rate);
			if *theta > TAU {
				*theta = 0.0;
			}
			output += tmp;
			if !set {
				output /= 2.0;
				set = true;
			}
		}
		output
	}

	pub fn add_freq(&mut self, freq: f64) {
		self.frequencies
			.insert(freq.to_bits(), 0.0);
	}

	pub fn del_freq(&mut self, freq: f64) {
		self.frequencies
			.remove(&freq.to_bits());
	}
}

impl Iterator for FrequencyGenerator {
	type Item = f32;
	fn next(&mut self) -> Option<f32> {
		Some(self.step())
	}
}

impl Source for FrequencyGenerator {
	fn current_frame_len(&self) -> Option<usize> {
		Some(1)
	}

	fn channels(&self) -> u16 {
		1
	}

	fn sample_rate(&self) -> u32 {
		self.sample_rate as u32
	}

	fn total_duration(&self) -> Option<std::time::Duration> {
		None
	}
}
