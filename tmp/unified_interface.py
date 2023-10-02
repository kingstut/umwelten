class HapticDevice:
    """ 
    Base class for all haptic devices. 
    Each specific device driver (like BOS1901) will subclass this and implement the play_pattern method.
    """
    def init_sensor(self):
        raise NotImplementedError

    def set_frequency(self, freq):
        raise NotImplementedError

    def get_feedback_mode(self):
        raise NotImplementedError
    
    def play_pattern(self, pattern):
        raise NotImplementedError