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
    // let config = cpal::SupportedStreamConfig {
    //     channels: 2,
    //     sample_rate: cpal::SampleRate(16000),
    //     buffer_size: cpal::BufferSize::Fixed(512),
    //     sample_format: cpal::SampleFormat::F32,
    // };
    println!("Default input config: {:?}", config);

    let (sender, receiver) = channel();

    let err_fn = |err| eprintln!("An error occurred on the audio input stream: {}", err);

    let stream_config = config.clone().into();
    let stream = device.build_input_stream(
        &stream_config,
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let _ = sender.send(data.to_owned());
        },
        err_fn,
        None,
    )?;
    stream.play()?;

    let mut recorded_samples: Vec<f32> = Vec::new();
    let record_time = std::time::Duration::from_secs(5);
    let start_time = std::time::Instant::now();

    while start_time.elapsed() < record_time {
        if let Ok(data) = receiver.try_recv() {
            recorded_samples.extend_from_slice(&data);
        }
    }

    println!("Recorded samples count: {}", recorded_samples.len());

    stream.pause()?;

    let spec = hound::WavSpec {
        channels: config.channels(),
        sample_rate: config.sample_rate().0,
        bits_per_sample: 32,
        sample_format: SampleFormat::Float,
    };

    let writer = BufWriter::new(File::create("output.wav")?);
    let mut wav_writer = hound::WavWriter::new(writer, spec)?;
    for sample in recorded_samples {
        wav_writer.write_sample(sample)?;
    }
    wav_writer.finalize()?;

    println!("Recording is saved to 'output.wav'");
    Ok(())
}
