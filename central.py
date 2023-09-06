# Coordinate between different haptic devices

from unified_interface import HapticDevice

class HapticController:
    """
    This central controller manages multiple devices and perform synchronized operations on them
    """
    def __init__(self):
        self.devices = []

    def add_device(self, device):
        if isinstance(device, HapticDevice):
            self.devices.append(device)

    def play_pattern_on_all(self, pattern):
        for device in self.devices:
            device.play_pattern(pattern)