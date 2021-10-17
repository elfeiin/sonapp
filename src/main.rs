use cpal::traits::HostTrait;
use rodio::DeviceTrait;
use rodio::{source::Source, Decoder, InputDevices, OutputStream};
use std::collections::HashMap;
use std::f64::consts::TAU;
use std::io::Write;
use std::sync::{mpsc, Arc, Mutex};

mod tone;
use tone::*;
mod modulator;
use modulator::*;
mod consts;
use consts::*;

fn main() {
	// A frequency generator. Can play many frequencies at once.
	let freq_gen = Arc::new(Mutex::new(FrequencyGenerator::new(44000.0)));
	
	// The thing that plays sound from FrequencyGenerator.
	let speaker = Speaker::new(freq_gen.clone());
	
	let host = cpal::default_host();
	
	let devices = host.output_devices().unwrap();
	
	for device in devices {
		println!["{}", device.name().unwrap()];
	}
	
	// let mut stream = Stream::new(ctx, name, ss, map)
	
	// Modify freq_gen to play certain stuffs.
	let mut modulator = Modulator::new(FreqRange::new(880.0, 20.0), freq_gen);

	// Write data to our freq gen to be played by the speaker.
	modulator
		.write_all(b"Hello Able!")
		.unwrap();

	std::thread::sleep(std::time::Duration::from_secs(30));
}
