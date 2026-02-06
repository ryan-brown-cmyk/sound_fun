# Sound Fun

This project is just some messing around with auditory processing and production! 

It is built of 3 main components: 
- GUI 
- Playback & Analysis
- Production

The GUI interfaces with the other two, and the production uses the second, but they are not inherently linked! 

Notably, the list is also in order of their implementation, as each builds on the previous (with the exception of the GUI, as that may evolve to be modified by the latter, but still needs to be built first).

## Current Progress

Current state: GUI Development and Testing. 

GUI: 10%

P&A: 0%

Prod.: 0%

## Roadmap

### GUI

Build up GUI. Aiming to have an entirely containerized application, so if a different layout is desired, it is easy to swap in code, or eventually have layout be configurable. Will not be super fancy, mostly the bare minimum to get functionality (so file, device select, graph output, graph input, etc).

### P&A 

Have feedback loop enabled with filtering, to go directly from microphone to speaker. 

Have live spectrum.

Filter customization should be specifiable via a number, phasor, or similar, based on normalized frequency. (and by extension, sampling rate must be specified, or inferred from file metadata, when available).

Should be able to output into stream buffer (playback) or write into a new file, filtered. 

Potentially allow for an "easy" EQ mode, that is more similar to what is found in cars or simpler applications.

### Production

Aiming for a "tracker" esque UI / interaction, although the samples should be able to be created on thier own.

## Notes

This project was inspired by one of the Coding Adventures video, but makes use of no code stated in that video, only some of the ideas. 

