from typing import List, Tuple

from discord import Interaction

from discord.ui import View, Select
from discord.ui import select

from database.utils import cancel_match


class CancelMatch(View):
    user_matches: List[Tuple[str, str, str]] = []

    def __init__(self, user_matches):
        super().__init__()
        self.user_matches = user_matches
        for count, (name, _, date) in enumerate(user_matches):
            msg = f"vs. {name} on {date}"
            self.select_match_to_cancel.add_option(label=msg, value=str(count))

    @select(placeholder="Which match to cancel?")  # type: ignore
    async def select_match_to_cancel(
        self, interaction: Interaction, select_item: Select
    ):
        index: int = int(select_item.values[0])  # type: ignore
        (_, user_id, date) = self.user_matches[index]
        # await cancel_match(int(user_id), date)

        response = interaction.response
        await response.send_message("Done. Match cancelled.")
