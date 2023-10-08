from typing import Dict
import discord
from discord import ui
from discord.ui import Select, View, select

class UserSelect(View):
    user = None

    @select(cls=ui.UserSelect, placeholder="Select a user")
    async def challenge_user(self, interaction: discord.Interaction, select_item: Select):
        self.user = select_item.values

        await interaction.response.defer()
        self.stop()

class Calendar(ui.View):
    day: str | None = None
    days = [discord.SelectOption(label=str(i), value=str(i)) for i in range(1, 32)]

    month: Dict[str, str] | None = None

    month_names = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"]
    months = [discord.SelectOption(label=month, value=month[:3]) for month in month_names]

    @ui.select(options = months, placeholder="Select the month of the challenge.")
    async def select_month(self, interaction, select_item):
        month = select_item.values
        await interaction.response.defer()

        self.stop()

    @ui.select(options = days, placeholder="Select the day of the challenge.")
    async def select_day(self, interaction, select_item):
        day = select_item.values
        await interaction.response.defer()

        self.stop()
