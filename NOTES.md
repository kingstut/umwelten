## Haptic devices features
#### Piezoelectric Actuators:
- Frequency Response: Can produce very precise vibrations at specific frequencies.
- Voltage to Displacement Ratio: The amplitude of vibration is controlled by the applied voltage.

#### Eccentric Rotating Mass (ERM):
- RPM Range
- Mass of the Rotating Part: Affects the strength of the vibration.

#### Linear Resonant Actuators (LRA):
- Resonance Frequency: The frequency at which the LRA operates most efficiently.
- Acceleration/Force: How hard it can vibrate.

### Feedback Modes:
- Continuous vs. Pulsed Feedback
- Static vs. Dynamic Feedback: Static feedback doesn't change unless programmed to, while dynamic feedback can change based on user interaction or other external factors.

## BOS1901 FIFO 
- Burst Mode: Instead of sending a waveform sample to the BOS1901 every time it's needed (which would require frequent and timely communication), we can fill the FIFO with multiple samples at once in a 'burst'. The BOS1901 will then use these samples in order.
- Asynchronous Operation: Once the FIFO is loaded with waveform data, the BOS1901 can start processing this data and driving the actuator.

### Key FIFO Points from the Datasheet:
- The FIFO can hold 64 samples.
- Each sample is a 12-bit two’s complement format.
- The FIFO automatically outputs its data at a read-out rate defined by the PLAY bits.
- For continuous haptic feedback, we need to ensure that new data is written to the FIFO at roughly the same rate it's read out, so it doesn't run empty.
- If the FIFO does become empty, it maintains the last valid data.
