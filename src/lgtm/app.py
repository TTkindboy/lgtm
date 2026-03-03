from textual.app import App
from .title import TitleScreen

class LGTMApp(App):
    SCREENS = {"title": TitleScreen}

    def on_mount(self) -> None:
        self.push_screen("title")

def main():
    LGTMApp().run()
