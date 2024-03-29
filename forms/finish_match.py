from discord import Interaction, User, Member

from discord.ui import View, Button, UserSelect, Select
from discord.ui import select, button

from database.challenge import (
    update_player_info,
    finish_match,
    has_match_with_player,
)


class FinishMatch(View):
    """This view is used by the `/finish` command."""

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
        winner = self.caller.id
        loser = self.selected_user.id

        caller_update = await update_player_info(winner, loser, True)
        selected_user_update = await update_player_info(loser, winner, False)

        followup = interaction.followup
        await interaction.response.send_message(
            f"{self.caller}: {caller_update}\n"
            + f"{self.selected_user}: {selected_user_update}"
        )

        self.disable_everything()
        await followup.edit_message(interaction.message.id, view=self)  # type: ignore

    @button(label="You lost :(")
    async def selected_user_wins(self, interaction: Interaction, button: Button):
        winner = self.selected_user.id
        loser = self.caller.id

        selected_user_update = await update_player_info(winner, loser, True)
        caller_update = await update_player_info(loser, winner, False)

        followup = interaction.followup
        await interaction.response.send_message(
            f"{self.selected_user.global_name}: {selected_user_update}\n"
            + f"{self.caller.global_name}: {caller_update}"
        )

        await followup.edit_message(interaction.message.id, view=self)  # type: ignore

    def disable_everything(self):
        for item in self.children:
            item.disabled = True  # type: ignore

        self.stop()
