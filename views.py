from typing import Dict

from discord import SelectOption
from discord import Interaction

from discord.ui import Select, View, select, UserSelect

class Months(Select):
    def __init__(self):
        month_names = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"]
        options = [SelectOption(label=month, value=month[:3]) for month in month_names]
        super().__init__(options=options, placeholder="Select the month of the challenge.")

    # do stuff in here
    async def callback(self, interaction: Interaction):
        view = self.view
        if view is None:
            return

        await view.save_month(interaction, self.values)

class Days(Select):
    def __init__(self):
        options = [SelectOption(label=str(x), value=str(x)) for x in range (1, 32)]
        super().__init__(options=options, placeholder="Select the days of the challenge.")

    async def callback(self, interaction: Interaction):
        view = self.view
        if view is None:
            return

        await view.save_day(interaction, self.values)

class ChallengeSubmission(View):
    user = None
    month = None

    @select(cls=UserSelect, placeholder="Select the user.")
    async def select_user(self, interaction: Interaction, select_item: Select):
        self.user = select_item.values

        month_select = Months()
        days_select = Days()

        self.add_item(month_select)
        self.add_item(days_select)

        await interaction.response.edit_message(view=self)

    async def save_month(self, interaction: Interaction, month: str):
        self.month = month
        await interaction.response.send_message("Month saved.")

    async def save_day(self, interaction: Interaction, day: str):
        self.day = day
        await interaction.response.send_message("Day saved.")
