# Interface with each device using its specific communication protocol.

from unified_interface import HapticDevice
import spidev

class BOS1901Driver(HapticDevice):
    def __init__(self, bus=0, device=0, max_speed_hz=35000000):
        self.spi = spidev.SpiDev()
        self.spi.open(bus, device)  # Bus and device values; adjust accordingly
        self.spi.mode = 0  # Confirm from BOS1901 datasheet
        self.spi.max_speed_hz = max_speed_hz
        self.spi.bits_per_word = 16
    
    def send_command(self, command):
        # SPI communication details here
        pass

    def play_pattern(self, pattern):
        # Translate the pattern into commands for BOS1901
        pass