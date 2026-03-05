from textual.widgets import OptionList, Static
from textual.screen import Screen

class Choice(OptionList, inherit_css=False):
    COMPONENT_CLASSES = OptionList.COMPONENT_CLASSES.copy()
    DEFAULT_CSS = """
    Choice {
        & > .option-list--option-highlighted {
            color: $block-cursor-background;
            text-style: bold;
        }
        height: auto;
    }
    """

    # TODO: Allow passing in Option instances
    def __init__(self, *content: str, **kwargs):
        kwargs.setdefault("compact", True)
        items = [f"{i}. {text}" for i, text in enumerate(content, start=1)]
        super().__init__(*items, **kwargs)

class ChoiceScreen(Screen):
    CSS = """
    Choice {
        dock: bottom;
        margin-bottom: 1;
    }
    #result {
        margin-top: 1;
    }
    """

    def compose(self):
        yield Choice("Hello", "World", "This is a choice screen", "Quit")
        self.result = Static("", id="result")
        yield self.result

    def on_option_list_option_selected(self, event: OptionList.OptionSelected):
        if event.option_index == 3: # SO FRAGILE 💀
            self.app.exit()
        self.result.update(str(event))
