use cpal::traits::{DeviceTrait, StreamTrait, HostTrait};
use std::fs::File;
use std::io::BufWriter;
use std::sync::mpsc::channel;
use hound::SampleFormat;
use anyhow;
use hound;

fn main() -> Result<(), anyhow::Error> 
{
    let host = cpal::default_host();
    let device = host.default_input_device().ok_or_else(|| anyhow::anyhow!("No input device found"))?;
    println!("Input device: {}", device.name()?);

    let config = device.default_input_config()?;
    println!("Default input config: {:?}", config);

    // Channel for transferring samples from callback to main thread
    let (sender, receiver) = channel();

    // Error handling for the audio stream
    let err_fn = |err| eprintln!("An error occurred on the audio input stream: {}", err);

    // Building the stream
    let stream_config = config.clone().into();
    let stream = device.build_input_stream(
        &stream_config,
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let _ = sender.send(data.to_owned());  // Use '_' to ignore 'send' Result (avoid panic in callback)
        },
        err_fn,
        None,
    )?;
    stream.play()?;

    let mut recorded_samples: Vec<f32> = Vec::new();
    let record_time = std::time::Duration::from_secs(5);  // Record for 5 seconds
    let start_time = std::time::Instant::now();

    // Recording loop
    while start_time.elapsed() < record_time {
        if let Ok(data) = receiver.try_recv() {
            recorded_samples.extend_from_slice(&data);
        }
        std::thread::sleep(std::time::Duration::from_millis(100));  // Sleep to reduce busy waiting
    }

    println!("Recorded samples count: {}", recorded_samples.len());

    // Stop the stream
    stream.pause()?;

    // Saving the recorded data
    let spec = hound::WavSpec {
        channels: config.channels(),
        sample_rate: 16000,
        bits_per_sample: 32,
        sample_format: SampleFormat::Float,
    };

    let writer = BufWriter::new(File::create("output.wav")?);
    let mut wav_writer = hound::WavWriter::new(writer, spec)?;
    for sample in recorded_samples {
        wav_writer.write_sample(sample)?;  // Convert f32 audio sample to i16 and write it
    }
    wav_writer.finalize()?;  // Ensure header and data are flushed and file is closed

    println!("Recording is saved to 'output.wav'");
    Ok(())
}
