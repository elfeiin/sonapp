use rodio::cpal;
use rodio::Device;
use rodio::cpal::traits::HostTrait;
use rodio::{source::Source, Decoder, OutputStream};
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
	// let host = cpal::default_host();
	
	// let mut the_device = None;
	
	// let select = 2;
	
	// for (i, device) in host.output_devices().unwrap().enumerate() {
	// 	the_device = Some(device);
	// 	if i == select {
	// 		break;
	// 	}
	// }
	
	// let device = if let Some(device) = the_device {
	// 	device
	// } else {
	// 	return;
	// };
	
	// Get a output stream handle to the default physical sound device
	let (stream, stream_handle) = OutputStream::try_default().unwrap();
	
	
	let freq_gen = Arc::new(Mutex::new(FrequencyGenerator::new(44000.0)));

	let speaker = Speaker::new(freq_gen.clone());

	// Play the sound directly on the device
	stream_handle.play_raw(speaker.convert_samples()).unwrap();

	let mut modulator = Modulator::new(FreqRange::new(440.0, 20.0), freq_gen.clone());
	
	modulator.write_all(b"Some really long text that I just thought of just now.").unwrap();

	// The sound plays in a separate audio thread,
	// so we need to keep the main thread alive while it's playing.
	// std::thread::sleep(std::time::Duration::from_secs(30));
}
