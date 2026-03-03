from textual.app import App
from .screens.title import TitleScreen
from .screens.choice import ChoiceScreen

class LGTMApp(App):
    SCREENS = {"title": TitleScreen}

    def on_mount(self) -> None:
        self.push_screen(ChoiceScreen())

def main():
    LGTMApp().run()
