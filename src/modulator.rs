use super::*;

pub struct FreqRange {
	base: f64,
	offset: f64,
}

impl FreqRange {
	pub fn new(base: f64, offset: f64) -> Self {
		Self { base, offset }
	}
}

pub struct Modulator {
	range: FreqRange,
	freq_gen: Arc<Mutex<FrequencyGenerator>>,
}

impl Modulator {
	pub fn new(range: FreqRange, freq_gen: Arc<Mutex<FrequencyGenerator>>) -> Self {
		Self { range, freq_gen }
	}

	pub fn set_sending(&mut self) {
		self.freq_gen
			.lock()
			.unwrap()
			.add_freq(
				self.range
					.base,
			);
	}

	pub fn set_not_sending(&mut self) {
		self.freq_gen
			.lock()
			.unwrap()
			.del_freq(
				self.range
					.base,
			);
	}

	pub fn send_byte(&mut self, byte: u8) {
		for i in 0..8 {
			if (byte >> i) % 2 == 1 {
				self.freq_gen
					.lock()
					.unwrap()
					.add_freq(
						self.range
							.base + (self
							.range
							.offset / 9.0 * (1.0 + i as f64)),
					)
			} else {
				self.freq_gen
					.lock()
					.unwrap()
					.del_freq(
						self.range
							.base + (self
							.range
							.offset / 9.0 * (1.0 + i as f64)),
					)
			}
		}
		std::thread::sleep(std::time::Duration::from_millis(BYTE_SEND_DURATION_MS));
		for i in 0..8 {
			self.freq_gen
				.lock()
				.unwrap()
				.del_freq(
					self.range
						.base + (self
						.range
						.offset / 9.0 * (1.0 + i as f64)),
				)
		}
	}
}

impl Write for Modulator {
	fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
		self.set_sending();
		let mut sent = 0;
		for (n, byte) in buf
			.iter()
			.enumerate()
		{
			self.send_byte(*byte);
			sent = n + 1;
		}
		self.set_not_sending();
		Ok(sent)
	}

	fn flush(&mut self) -> std::io::Result<()> {
		Ok(())
	}
}
