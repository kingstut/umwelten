# Interface with each device using its specific communication protocol.

from tmp.unified_interface import HapticDevice
import spidev

class BOS1901Driver(HapticDevice):
    def __init__(self, FIFO_WRITE_ADDRESS=0x00, STATUS_ADDRESS=0x0C, FIFO_FULL_BIT=7, 
                 CONFIG_ADDRESS = 0x05,
                 bus=0, device=0, max_speed_hz=35000000,):
        
        self.spi = spidev.SpiDev()
        self.spi.open(bus, device)  # Bus and device values; adjust accordingly
        self.spi.mode = 0b01  # Confirm from BOS1901 datasheet
        self.spi.max_speed_hz = max_speed_hz
        self.spi.bits_per_word = 12

        self.FIFO_WRITE_ADDRESS = FIFO_WRITE_ADDRESS
        self.STATUS_ADDRESS = STATUS_ADDRESS
        self.FIFO_FULL_BIT  = FIFO_FULL_BIT 
        self.CONFIG_ADDRESS = CONFIG_ADDRESS 
    
    def write_to_fifo(self, data):
        """Writes waveform data to the FIFO.""" 
        for value in data:
            # Ensure it's 12-bits
            assert 0 <= value < 4096, "Data value out of range for 12 bits"

            # Combine the address and the data for SPI transmission
            message = (self.FIFO_WRITE_ADDRESS << 12) | value
            self.spi.xfer2([message >> 8, message & 0xFF])

    def is_fifo_full(self):
        """Checks if the FIFO is full."""
        response = self.spi.xfer2([self.STATUS_ADDRESS, 0x00])  # Read command
        return bool(response[1] & self.FIFO_FULL_BIT)

    def is_fifo_empty(self):
        return not self.is_fifo_full()

    def start_playback(self, playback_rate=0x0):
        """
        Start waveform playback.       
        Parameters:
            playback_rate (byte): Playback rate configuration (default is 0x0 for 1024 ksps)
        """
        
        # Read current CONFIG register value
        current_config = self.spi.xfer2([self.CONFIG_ADDRESS | 0x80, 0x00])[1] 

        # Set the OE bit to 1 (enable playback)
        current_config |= (1 << 4)
        
        # Set the PLAY bits with the desired playback_rate
        # First, clear the PLAY bits:
        current_config &= ~(0x7)  
        # Now set them:
        current_config |= (playback_rate & 0x7)

        # Write updated value back to the CONFIG register
        self.spi.xfer2([self.CONFIG_ADDRESS, current_config])

    def stop_playback():
        # Write to the appropriate register to stop waveform playback
        pass

    def reset_fifo():
        # Write to the appropriate register to reset the FIFO
        pass
