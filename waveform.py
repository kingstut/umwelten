import numpy as np
import matplotlib.pyplot as plt

def generate_waveform(frequency, amplitude, sample_rate, duration):
    """
    Generates a waveform pattern.
    
    :param frequency: Frequency of the waveform in Hz.
    :param amplitude: Maximum amplitude of the waveform (should be <=1 for normalized waveforms).
    :param sample_rate: Samples per second.
    :param duration: Duration of the waveform in seconds.
    :return: A numpy array containing the waveform values.
    """
    t = np.linspace(0, duration, int(sample_rate * duration), endpoint=False)
    waveform = amplitude * np.sin(2 * np.pi * frequency * t)
    return waveform

def visualize_waveform(waveform, sample_rate):
    """
    Visualizes the generated waveform pattern using matplotlib.
    
    :param waveform: Numpy array containing the waveform values.
    :param sample_rate: Samples per second.
    """
    t = np.linspace(0, len(waveform) / sample_rate, len(waveform), endpoint=False)
    plt.plot(t, waveform)
    plt.xlabel('Time [s]')
    plt.ylabel('Amplitude')
    plt.title('Generated Waveform')
    plt.grid()
    plt.show()

if __name__ == "__main__":
    frequency = 1.0  # 1 Hz for example purposes
    amplitude = 0.9  # 0.9 amplitude for example purposes
    sample_rate = 1000  # 1000 samples/second
    duration = 5  # 5 seconds

    waveform = generate_waveform(frequency, amplitude, sample_rate, duration)
    visualize_waveform(waveform, sample_rate)
