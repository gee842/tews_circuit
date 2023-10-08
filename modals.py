import discord
from discord import ui 

class ChallengeSubmission(ui.Modal, title="Test"):
    users = ui.UserSelect(placeholder="wumpus#1234", disabled=False)
    answer = ui.TextInput(label='Answer', style=discord.TextStyle.paragraph)

    async def on_submit(self, interaction: discord.Interaction):
        await interaction.response.send_message(f"Thanks for your response.")
