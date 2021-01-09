pub mod utils {
    use crate::tape::tape::Tape;
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    use cpal::{Device, Stream, StreamConfig};
    use ringbuf::{Consumer, Producer, RingBuffer};
    use std::sync::mpsc::{channel, Receiver, Sender};

    pub fn create_input_stream(
        input_device: &Device,
        config: &StreamConfig,
        mut producer: Producer<f32>,
    ) -> Stream {
        let input_data = move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let mut output_fell_behind = false;
            for &sample in data {
                if producer.push(sample).is_err() {
                    output_fell_behind = true;
                }
            }

            if output_fell_behind {
                // eprintln!("output stream fell behind: try increasing latency");
            }
        };

        input_device
            .build_input_stream(config, input_data, err_fn)
            .unwrap()
    }

    pub fn create_output_stream_2(
        output_device: &Device,
        config: &StreamConfig,
        receiver: Receiver<Tape<f32>>,
        time_sender: Sender<f32>,
    ) -> Stream {
        const TAPE_LENGTH: usize = 44100 * 4;
        let mut tape = Tape::<f32>::new(0.0, TAPE_LENGTH);
        let mut index = 0;
        let output_data = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for sample in data {
                *sample = tape.audio[index];
                index += 1;

                if index == tape.audio.len() {
                    index = 0;
                }
            }

            time_sender.send(512 as f32 / 44100 as f32).unwrap();

            for r in receiver.try_recv() {
                println!("received: {}", r.audio.len());
                tape = r;
            }
        };

        output_device
            .build_output_stream(config, output_data, err_fn)
            .unwrap()
    }

    pub fn create_output_stream(
        output_device: &Device,
        config: &StreamConfig,
        mut consumer: Consumer<f32>,
        time_sender: Sender<f32>,
    ) -> Stream {
        let output_data = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let mut input_fell_behind = false;
            for sample in data {
                *sample = match consumer.pop() {
                    Some(s) => s,
                    None => {
                        input_fell_behind = true;
                        0.0
                    }
                };
            }

            time_sender.send(512 as f32 / 44100 as f32).unwrap();

            if input_fell_behind {
                // eprintln!("input stream fell behind: try increasing latency");
            }
        };

        output_device
            .build_output_stream(config, output_data, err_fn)
            .unwrap()
    }

    pub fn err_fn(err: cpal::StreamError) {
        eprintln!("an error occured on stream: {}", err);
    }
}
