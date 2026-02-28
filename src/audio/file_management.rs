// This file contains the nessecary reading and writing of audio files.

use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::conv::IntoSample;
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;






// The MWE below is a modified form of the getting_started, located at
// https://github.com/pdeljanov/Symphonia/blob/master/symphonia/examples/getting-started.rs
// with tweaks to incorporate it into the GUI based application.


pub async fn open_audio(file_name: String, file_extension: &str ) -> std::result::Result< Option<Vec<f32>>, Box<dyn std::error::Error> > {  
    let source = std::fs::File::open(file_name)?; // If we have any issues opening the file path, we want to pass it back to the GUI via the called function (yay match).

    let mss = MediaSourceStream::new(Box::new(source), Default::default());

    let mut hint = Hint::new();
    hint.with_extension(file_extension); // We could also make this automatic instead of passing in the file extension, but for now I will leave like this 

    // define and use default opts for metadata / fmt readers

    let meta_opts: MetadataOptions = Default::default();
    let fmt_opts: FormatOptions = Default::default();

    // we can then probe:
    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &fmt_opts, &meta_opts)?;

    // Get the format reader
    let mut format = probed.format;


    // Find the first audio track
    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .ok_or(std::fmt::Error)?;


    // Get default options for the decoder
    let dec_opts: DecoderOptions = Default::default();


    // Create the decoder for the track
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &dec_opts)?;


    
    // Store the track identifier, it will be used to filter packets.
    let track_id = track.id;


    // The below is copied from the other example, so we can have a buffer that we can return in the format that we are expecting!

    let mut _sample_count = 0;
    let mut sample_buf = None;
    let mut all_samples: Vec<f32> = Vec::new();

    loop {
        // Get the next packet from the format reader.
        let packet = match format.next_packet() {
            Ok(p) => p,
            Err(Error::IoError(ref e))
                if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                    break Ok(Some(all_samples))
                    // If you ever need to do some verification, it is here where this is nessecary! 
                }
            Err(e) => break Err(Box::new(e))?

            
        };

        // If the packet does not belong to the selected track, skip it.
        if packet.track_id() != track_id {
            continue;
        }

        // Decode the packet into audio samples, ignoring any decode errors.
        match decoder.decode(&packet) {
            Ok(audio_buf) => {
                // The decoded audio samples may now be accessed via the audio buffer if per-channel
                // slices of samples in their native decoded format is desired. Use-cases where
                // the samples need to be accessed in an interleaved order or converted into
                // another sample format, or a byte buffer is required, are covered by copying the
                // audio buffer into a sample buffer or raw sample buffer, respectively. In the
                // example below, we will copy the audio buffer into a sample buffer in an
                // interleaved order while also converting to a f32 sample format.

                // If this is the *first* decoded packet, create a sample buffer matching the
                // decoded audio buffer format.
                if sample_buf.is_none() {
                    // Get the audio buffer specification.
                    let spec = *audio_buf.spec();

                    // Get the capacity of the decoded buffer. Note: This is capacity, not length!
                    let duration = audio_buf.capacity() as u64;

                    // Create the f32 sample buffer.
                    sample_buf = Some(symphonia::core::audio::SampleBuffer::<f32>::new(duration, spec));
                }

                // Copy the decoded audio buffer into the sample buffer in an interleaved format.
                if let Some(buf) = &mut sample_buf {
                    buf.copy_interleaved_ref(audio_buf);  
                    all_samples.extend_from_slice(buf.samples());

                    // // The samples may now be access via the `samples()` function.
                    // sample_count += buf.samples().len();
                    // print!("\rDecoded {} samples", sample_count);
                }
            }
            Err(Error::DecodeError(_)) => {
                // I believe this means we are end of file based on what I can tell from reading the source code.
                // therefore, we want to return the sample buffer!

                // NOTE: This is probably some form of improper, but until it explodes in my face, I'll keep using it!

                continue

            },
            Err(e) => {
                return Err(Box::new(e))?
            },
        }
    }



}