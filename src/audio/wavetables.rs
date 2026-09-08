// This file will contain all of the patterns for our generated waves.
// -> for ease, this will start out as just a couple of different waves, but should be able to do
// multiple. See the enum for exact styles.
//
// Each generation will take in frequence, phase, offset. Duration is not needed, as that is
// determined by how many times you loop.

pub enum WaveTableType {
    SinWave,
    TriangleWave,
    SquareWave,
    SawtoothWave,
    PulseWave,
}

pub struct WaveTableContainer {
    pub wave: Vec<f64>,
    freq: f32,
    phase: f32,
    offset: f32,
    wave_type: WaveTableType,
}

pub fn generate_wavetable(wave_type: WaveTableType) {
    todo!();
}
