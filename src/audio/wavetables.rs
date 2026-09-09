// This file will contain all of the patterns for our generated waves.
// -> for ease, this will start out as just a couple of different waves, but should be able to do
// multiple. See the enum for exact styles.
//
// Each generation will take in frequence, phase, offset. Duration is not needed, as that is
// determined by how many times you loop.

pub enum WaveTableType {
    Sin,
    Triangle,
    Square,
    Sawtooth,
    Pulse,
}

pub struct WaveTableParams {
    pub freq: f32,
    pub phase: f32,
    pub offset: f32,
    pub wave_type: WaveTableType,
}

pub struct WaveTableContainer {
    pub wave: Vec<f64>,
    pub params: WaveTableParams,
}

// For now, we will simply return an option. We will probably want to pass back if an error should
// occur, however.
pub fn generate_wavetable(wave_params: WaveTableParams) -> Option<WaveTableContainer> {
    match wave_params.wave_type {
        WaveTableType::Sin => {
            return generate_sin(wave_params);
        }
        WaveTableType::Triangle => {
            return generate_triangle(wave_params);
        }
        WaveTableType::Square => {
            return generate_square(wave_params);
        }
        WaveTableType::Sawtooth => {
            return generate_sawtooth(wave_params);
        }
        WaveTableType::Pulse => {
            return generate_pulse(wave_params);
        }
    }
}

fn generate_sin(wave_params: WaveTableParams) -> Option<WaveTableContainer> {
    let wave = new(WaveTableContainer);

    return wave;
}
fn generate_triangle(wave_params: WaveTableParams) -> Option<WaveTableContainer> {
    let wave = new(WaveTableContainer);

    return wave;
}
fn generate_square(wave_params: WaveTableParams) -> Option<WaveTableContainer> {
    let wave = new(WaveTableContainer);

    return wave;
}
fn generate_sawtooth(wave_params: WaveTableParams) -> Option<WaveTableContainer> {
    let wave = new(WaveTableContainer);

    return wave;
}
fn generate_pulse(wave_params: WaveTableParams) -> Option<WaveTableContainer> {
    let wave = new(WaveTableContainer);

    return wave;
}
