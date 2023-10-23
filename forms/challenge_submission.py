from datetime import datetime

from discord import SelectOption, Interaction, ButtonStyle, User

from discord.ui import View, Modal
from discord.ui import UserSelect, Select, select
from discord.ui import Button, button

from discord.ui import TextInput

from database.utils import player_has_match_at_time


class ChallengeSubmission(View):
    user: User | None = None
    month = None
    cancelled = False

    # These two are handled by the modal DateTime
    day = None
    time = None

    month_mapping = {
        "January": "01",
        "February": "02",
        "March": "03",
        "April": "04",
        "May": "05",
        "June": "06",
        "July": "07",
        "August": "08",
        "September": "09",
        "October": "10",
        "November": "11",
        "December": "12",
    }

    months = [
        SelectOption(label=month, value=value) for month, value in month_mapping.items()
    ]

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
            self.user = values[0]  # type: ignore

    @select(cls=Select, placeholder="Month of the challenge.", options=months)
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

        current_month = datetime.now().date().month
        selected_month = self.month[0]  # type: ignore
        selected_month = datetime.strptime(selected_month, "%m").month

        if selected_month < current_month:
            await response.send_message(
                "Choose a future/present month.", ephemeral=True
            )
            return

        date_time = DateTime()
        followup = interaction.followup
        await response.send_modal(date_time)
        await date_time.wait()

        self.time = date_time.time
        self.day = date_time.day

        current_time = datetime.now()

        year = current_time.date().year
        selected_time = f"{year}-{selected_month}-{self.day} {self.time}:00"
        selected_time = datetime.strptime(selected_time, "%Y-%m-%d %H:%M:%S")

        if selected_time < current_time:
            await followup.send("Please choose a future/present time.", ephemeral=True)
            return

        selected_time_str = selected_time.__str__()

        # If caller has match
        if await player_has_match_at_time(interaction.user.id, selected_time_str):
            await followup.send(
                f"You have a match on {selected_time_str}. Choose another time."
            )
            return

        # If challenged has match
        if await player_has_match_at_time(self.user.id, selected_time_str):  # type: ignore
            await followup.send(
                f"{self.user} has a match on {selected_time_str}. Choose another time."
            )
            return

        await followup.send("A challenge has been booked.")
        await self.disable_everything()

        message = interaction.message
        if message is None:
            return 

        await followup.edit_message(message.id, view=self)

    @button(label="Cancel", style=ButtonStyle.grey)
    async def cancel(self, interaction: Interaction, button: Button):
        self.cancelled = True

        await self.disable_everything()
        await interaction.response.edit_message(
            content="Challenge cancelled.", view=self
        )

    async def disable_everything(self):
        for item in self.children:
            item.disabled = True  # type: ignore

        self.stop()


class DateTime(Modal, title="Day and time of match"):
    time = TextInput(label="Time of match (24 hour)", placeholder="18:30, 23:30")
    day = TextInput(label="Day of the month", placeholder="20, 31")

    async def on_submit(self, interaction: Interaction):
        response = interaction.response
        try:
            datetime.strptime(f"{self.day.value} {self.time.value}", "%d %H:%M")
        except ValueError:
            await response.send_message(
                "Invalid day or time. Try again.", ephemeral=True
            )
            return

        await interaction.response.send_message("Time saved.", ephemeral=True)
        self.stop()
