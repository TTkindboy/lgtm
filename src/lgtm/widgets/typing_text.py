from textual.reactive import reactive
from textual.widgets import Static


class TypingText(Static): # TODO: FIX EXTERIOR UPDATE BEHAVIOR 
    index = reactive(0)
    
    def __init__(self, content: str, speed: int = 30, **kwargs):
        self._text = content
        self.seconds_per_char = 1 / speed
        super().__init__(content, **kwargs)

    def _advance(self):
        self.index+=1
    
    def on_mount(self):
        self.set_interval(self.seconds_per_char, self._advance, repeat=len(self._text))
    

    def render(self) -> str:
        return self._text[:self.index]
