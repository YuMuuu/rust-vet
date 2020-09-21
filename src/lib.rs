// https://vaporsoft.net/creating-an-audio-plugin-with-rust-vst/

#[macro_use]
extern crate vst;
extern crate rand;

use vst::plugin::{Info, Plugin, Category};
use vst::buffer::AudioBuffer;
use rand::random;


#[derive(Default)]
struct Whisper;

impl Plugin for Whisper {
    fn get_info(&self) -> Info {
        Info {
            name: "Whisper".to_string(),
            inputs: 0,
            outputs: 2,
            category: Category::Synth,
            unique_id: 1337,

            ..Default::default()
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {

        //inputとoutputのbuffer
        let (_, output_buffer) = buffer.split();

        //outputチャンネル、ステレオになっている（？
        for output_channel in output_buffer.into_iter() {
            for output_sample in output_channel {
                // For every sample, we want to add a random value from
                // -1.0 to 1.0.
                // ホワイトノイズを生成する
                *output_sample = (random::<f32>() - 0.5f32) * 2f32;
            }
        }
    }
}



// Make sure you call this, or nothing will happen.
plugin_main!(Whisper);