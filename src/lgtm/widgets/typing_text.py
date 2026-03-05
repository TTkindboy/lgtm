from textual.visual import VisualType
from time import monotonic
from textual.reactive import reactive
from textual.widgets import Static


class TypingText(Static): # TODO: FIX EXTERIOR UPDATE BEHAVIOR WHEN 
    start_time: reactive[float] = reactive(monotonic)  # ty:ignore[invalid-assignment]
    time = reactive(0.0)

    def on_mount(self):
        self.set_interval(1 / 60, self.update_time)
    
    def update_time(self):
        self.time = monotonic() - self.start_time
    
    def watch_time(self, time: float):
        num_chars = int(self.time // self.seconds_per_char)
        # idk if this checks if it changed? maybe some fancy reactive magic with render() should be used here instead.
        self.update(self.text[:min(num_chars, len(self.text))])
    
    def __init__(self, content: str, speed: int = 30, **kwargs):
        self.text = content
        self.seconds_per_char = 1 / speed
        super().__init__(content, **kwargs)
