from textual.app import App
from .screens.title import TitleScreen
from .screens.choice import ChoiceScreen

class LGTMApp(App):
    SCREENS = {"title": TitleScreen, "choice": ChoiceScreen}

    def on_mount(self) -> None:
        self.push_screen("title")

def main():
    LGTMApp().run()
