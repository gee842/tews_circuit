from discord import SelectOption
from discord import Interaction

from discord.ui import Select, View, select, UserSelect

class Months(Select):
    def __init__(self):
        month_names = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"]
        options = [SelectOption(label=month, value=month[:3]) for month in month_names]
        super().__init__(options=options, placeholder="Select the month of the challenge.")

    async def callback(self, interaction: Interaction):
        view = self.view
        if view is None:
            return

        await view.save_month(interaction, self.values)

class DaysFirstHalf(Select):
    def __init__(self):
        options = [SelectOption(label=str(x), value=str(x)) for x in range (1, 16)]
        super().__init__(options=options, placeholder="Select the days of the challenge (1 - 15).")

    async def callback(self, interaction: Interaction):
        view = self.view
        if view is None:
            return

        await view.save_day(interaction, self.values)

class DaysSecondHalf(Select):
    def __init__(self):
        options = [SelectOption(label=str(x), value=str(x)) for x in range (17, 31)]
        super().__init__(options=options, placeholder="Select the days of the challenge (16 - 31).")

    async def callback(self, interaction: Interaction):
        view = self.view
        if view is None:
            return

        await view.save_day(interaction, self.values)

class ChallengeSubmission(View):
    user = None
    month = None
    day = None

    @select(cls=UserSelect, placeholder="Select the user.")
    async def select_user(self, interaction: Interaction, select_item: Select):
        self.user = select_item.values

        month_select = Months()
        days_select = DaysFirstHalf()
        days_second_select = DaysSecondHalf()

        self.add_item(month_select)
        self.add_item(days_select)
        self.add_item(days_second_select)

        await interaction.response.edit_message(view=self)

    async def save_month(self, interaction: Interaction, month: str):
        self.month = month
        await interaction.response.send_message("Month saved.")

    async def save_day(self, interaction: Interaction, day: str):
        self.day = day
        await interaction.response.send_message("Day saved.")

    async def interaction_check(self, interaction: Interaction, /) -> bool:
        message = interaction.message
        if message is None:
            return False

        if self.user is not None:
            user = self.user[0]
            # Can't challenge yourself or a bot
            if interaction.user == user or message.author.bot:
                return False

        return True

    async def on_timeout(self) -> None:
        # Step 2
        for item in self.children:
            item.disabled = True

        # Step 3
        # error here
        await self.message.edit(view=self)
