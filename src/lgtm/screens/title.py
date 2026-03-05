from textual.app import Screen, ComposeResult
from textual.containers import Container
from textual.widgets import Static
from pyfiglet import figlet_format


class TitleScreen(Screen):
    CSS_PATH = "title.tcss"
    BINDINGS = [("enter", "start", "Start game")]
    
    def compose(self) -> ComposeResult:
        yield Container(
            Static(figlet_format("LGTM", font="ansi_regular").rstrip(), id="title"),
            Static("A CYOA game about code review purgatory", id="tagline"),
            Static("press enter to start", id="hint"),
            id="root",
        )

    def action_start(self) -> None:
        self.app.switch_screen("choice")
