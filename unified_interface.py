class HapticDevice:
    """ 
    Base class for all haptic devices. 
    Each specific device driver (like BOS1901) will subclass this and implement the play_pattern method.
    """
    def play_pattern(self, pattern):
        raise NotImplementedError