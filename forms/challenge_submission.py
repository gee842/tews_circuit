import time

from discord import SelectOption, Interaction, ButtonStyle

from discord.ui import View, Modal
from discord.ui import UserSelect, Select, select
from discord.ui import Button, button

from discord.ui import TextInput


class ChallengeSubmission(View):
    user = None
    month = None
    cancelled = False

    # These two are handled by the modal DateTime
    day = None
    time = None

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

    @button(label="Submit", style=ButtonStyle.grey)
    async def submit(self, interaction: Interaction, button: Button):
        response = interaction.response
        fields_not_filled = self.month is None or self.user is None

        if fields_not_filled:
            msg = "The user is not filled out or you have chosen a bot/yourself."
            if self.user is None:
                await response.send_message(msg, ephemeral=True)

                return

            if self.month is None:
                msg = "The month is not filled out."
                await response.send_message(msg, ephemeral=True)

                return

        date_time = DateTime()
        await response.send_modal(date_time)
        await date_time.wait()

        self.time = date_time.date_time
        self.day = date_time.day

        await self.disable_everything()

    @button(label="Cancel", style=ButtonStyle.grey)
    async def cancel(self, interaction: Interaction, button: Button):
        await self.disable_everything()
        await interaction.response.edit_message(
            content="Challenge cancelled.", view=self
        )

        self.cancelled = True

    async def disable_everything(self):
        for item in self.children:
            item.disabled = True  # type: ignore

        self.stop()


class DateTime(Modal, title="Day and time of match"):
    date_time = TextInput(label="Time of match (24 hour)", placeholder="18:30, 23:30")
    day = TextInput(label="Day of the month", placeholder="20, 31")

    async def on_submit(self, interaction: Interaction):
        response = interaction.response
        try:
            time.strptime(f"{self.day.value} {self.date_time.value}", "%d %H:%M")
        except ValueError:
            await response.send_message(
                "Invalid day or time. Try again.", ephemeral=True
            )
            return

        await interaction.response.send_message("Time saved.", ephemeral=True)
        self.stop()
