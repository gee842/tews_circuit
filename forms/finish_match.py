from discord import Interaction, User, Member

from discord.ui import View, Button, UserSelect, Select
from discord.ui import select, button

from database.challenge import (
    update_player_info,
    finish_match,
    has_match_with_player,
)


class FinishMatch(View):
    caller: User | Member = None  # type: ignore
    selected_user: User | Member = None  # type: ignore

    def __init__(self):
        super().__init__(timeout=180)
        self.remove_item(self.caller_wins)
        self.remove_item(self.selected_user_wins)

    @select(cls=UserSelect, placeholder="Who do you have a challenge with?")  # type: ignore
    async def select_user(self, interaction: Interaction, select_item: Select):
        values = select_item.values
        if values is None:
            return

        self.selected_user: User = values[0]  # type: ignore
        self.caller = interaction.user

        response = interaction.response
        result = await has_match_with_player(self.caller.id, self.selected_user.id)  # type: ignore
        if result is None:
            await response.send_message("You do not have a match with that player.")
            return

        (challenger, challenged, date) = result
        await finish_match(challenger, challenged, date)
        self.add_item(self.caller_wins)
        self.add_item(self.selected_user_wins)

        await response.edit_message(view=self)

    @button(label="You win :D")
    async def caller_wins(self, interaction: Interaction, button: Button):
        caller_update = await update_player_info(self.caller.id, True)
        selected_user_update = await update_player_info(self.selected_user.id, False)

        await interaction.response.send_message(
            f"{self.caller}: {caller_update}\n"
            + f"{self.selected_user}: {selected_user_update}"
        )

    @button(label="You lost :(")
    async def selected_user_wins(self, interaction: Interaction, button: Button):
        caller_update = await update_player_info(self.caller.id, False)
        selected_user_update = await update_player_info(self.selected_user.id, True)

        await interaction.response.send_message(
            f"{self.selected_user}: {selected_user_update}\n"
            + f"{self.caller}: {caller_update}"
        )
