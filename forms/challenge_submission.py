from discord import SelectOption
from discord import Interaction
from discord import ButtonStyle

from discord.ui import View
from discord.ui import UserSelect, select, Select
from discord.ui import button, Button


class ChallengeSubmission(View):
    user = None
    month = None
    day = None

    days_first_half = [SelectOption(label=str(x), value=str(x)) for x in range(1, 16)]
    days_second_half = [SelectOption(label=str(x), value=str(x)) for x in range(16, 31)]

    month_names = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ]

    months = [SelectOption(label=month, value=month[:3]) for month in month_names]

    @select(cls=UserSelect, placeholder="Select the user.")  # type: ignore
    async def select_user(self, interaction: Interaction, select_item: Select):
        values = select_item.values
        if values is None:
            return

        user = values[0]

        if interaction.user == user or user.bot:  # type: ignore
            await interaction.response.send_message(
                "You cannot challenge yourself or a bot.", ephemeral=True
            )
        else:
            await interaction.response.send_message("User saved!", ephemeral=True)
            self.user = values[0]

    @select(
        cls=Select, placeholder="Select the month of the challenge.", options=months
    )
    async def select_month(self, interaction: Interaction, select_item: Select):
        self.month = select_item.values
        await interaction.response.send_message("Month saved!", ephemeral=True)

    @select(
        cls=Select,
        placeholder="Select the days of the challenge (1 - 15)",
        options=days_first_half,
    )
    async def select_days_first_half(
        self, interaction: Interaction, select_item: Select
    ):
        self.day = select_item.values
        await interaction.response.send_message("Day saved!", ephemeral=True)

    @select(
        cls=Select,
        placeholder="Select the days of the challenge (16 - 31)",
        options=days_second_half,
    )
    async def select_days_second_half(
        self, interaction: Interaction, select_item: Select
    ):
        self.day = select_item.values
        await interaction.response.send_message("Day saved!", ephemeral=True)

    @button(label="Submit", style=ButtonStyle.grey)
    async def submit(self, interaction: Interaction, button: Button):
        response = interaction.response
        fields_not_filled = (
            self.day is None and self.month is None and self.user is None
        )

        if fields_not_filled:
            if self.day is None:
                await response.send_message(
                    "The day is not filled out.", ephemeral=True
                )

            if self.month is None:
                await response.send_message(
                    "The month is not filled out.", ephemeral=True
                )

            if self.user is None:
                await response.send_message(
                    "The user is not filled out or you have chosen a bot/yourself.",
                    ephemeral=True,
                )

            return

        await self.disable_everything()
        await response.edit_message(content="Challenge registered.", view=self)

        self.stop()

    @button(label="Cancel", style=ButtonStyle.grey)
    async def cancel(self, interaction: Interaction, button: Button):
        await self.disable_everything()
        await interaction.response.edit_message(
            content="Challenge cancelled.", view=self
        )

    async def disable_everything(self):
        for item in self.children:
            item.disabled = True  # type: ignore
