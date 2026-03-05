from textual.app import ComposeResult
from textual.screen import Screen
from textual.widgets import OptionList, Static
from ..widgets.typing_text import TypingText # maybe find a way to do relative imports

class Choice(OptionList, inherit_css=False):
    COMPONENT_CLASSES = OptionList.COMPONENT_CLASSES.copy()
    DEFAULT_CSS = """
    Choice {
        height: auto;
        & > .option-list--option-highlighted {
            color: $block-cursor-background;
            text-style: bold;
        }
        & > .option-list--option-hover {
            background: $block-hover-background;
        }
    }
    """

    # TODO: Allow passing in Option instances
    def __init__(self, *content: str, **kwargs):
        kwargs.setdefault("compact", True) # no-op i think
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

    def compose(self) -> ComposeResult:
        yield TypingText("This is the choice screen. You can choose from the options below.", 30)
        self.result = Static("", id="result")
        yield self.result
        yield Choice("Hello", "World", "This is a choice screen", "Quit")
        

    def on_option_list_option_selected(self, event: OptionList.OptionSelected):
        if event.option_index == 3: # SO FRAGILE 💀
            self.app.exit()
        self.result.update(event.option.prompt)
