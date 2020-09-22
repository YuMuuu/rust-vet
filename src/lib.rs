//ref:  https://vaporsoft.net/creating-an-audio-plugin-with-rust-vst/

#[macro_use]
extern crate vst;
extern crate rand;

use vst::plugin::{Info, Plugin, Category};
use vst::buffer::AudioBuffer;
use vst::event::Event;
use vst::api::Events;
use rand::random;


#[derive(Default)]
struct Whisper {
    notes: u8
}


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
        // 早期リターンしてる。多分良くない
        if self.notes == 0 { return }

        //inputとoutputのbuffer
        let (_, mut output_buffer) = buffer.split();

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

    fn process_events(&mut self, events: &Events) {
        for event in events.events() {
            match event {
                Event::Midi(ev) => {
                    // https://www.midi.org/specifications/item/table-1-summary-of-midi-message

                    match ev.data[0] {
                        // if note on, increment our counter
                        144 => self.notes += 1u8,

                        // if note off, decrement our counter
                        128 => self.notes -= 1u8,
                        _ => (),
                    }
                    // if we cared about the pitch of the note, it's stored in `ev.data[1]`.
                },
                _ => (),
            }
        }
    }
}



plugin_main!(Whisper);