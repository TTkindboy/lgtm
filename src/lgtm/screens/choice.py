from textual.widgets import OptionList
from textual.screen import Screen

class Choice(OptionList):
    DEFAULT_CSS = """
    Choice {
        & > .option-list--option-highlighted {
            color: $block-cursor-background;
            text-style: bold;
            background: initial;
        }
        &:focus {
            background-tint: $background;
            & > .option-list--option-highlighted {
                color: $block-cursor-background;
                text-style: bold;
                background: initial;
            }
        }

    }
    """

    def __init__(self, *content: str, **kwargs):
        kwargs.setdefault("compact", True)
        items = [f"{i}. {text}" for i, text in enumerate(content, start=1)]
        super().__init__(*items, **kwargs)

class ChoiceScreen(Screen):
    def compose(self):
        yield Choice("Hello", "World", "This is a choice screen")
        # yield OptionList("Option 1", "Option 2", "Option 3", compact=True)
