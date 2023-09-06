# Define and store various haptic patterns

from waveform_utils import generate_waveform

class HapticPattern:
    def __init__(self, duration, intensity, frequency, sample_rate):
        self.duration = duration
        self.intensity = intensity
        self.frequency = frequency
        self.sample_rate = sample_rate
        self.waveform = generate_waveform(frequency, intensity, sample_rate, duration)