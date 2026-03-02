from textual.app import App, ComposeResult
from textual.containers import Container
from textual.widgets import Static
from pyfiglet import figlet_format


class TitleScreen(App):
    CSS = """
    Screen {
        align: center middle;
    }

    #root {
        width: 100%;
        height: 100%;
        align: center middle;
    }

    #title {
        text-style: bold;
        content-align: center middle;
    }

    #tagline {
        margin-top: 3;
        opacity: 0.75;
        content-align: center middle;
    }


    #hint {
        margin-top: 2;
        opacity: 0.6;
        content-align: center middle;
    }
    """

    def compose(self) -> ComposeResult:
        yield Container(
            Static(figlet_format("LGTM", font="ansi_regular").rstrip(), id="title"),
            Static("A CYOA game about code review purgatory", id="tagline"),
            Static("press enter to start", id="hint"),
            id="root",
        )

    def on_key(self, event) -> None:
        if event.key == "enter":
            self.exit()

def main():
    TitleScreen().run()
