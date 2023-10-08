import discord

from views import UserSelect

from discord import app_commands
from discord.ext import commands

class Challenge(commands.Cog):
    def __init__(self, bot):
        self.bot = bot

    @app_commands.command(name="challenge", description="Challenge someone.")
    async def challenge(self, interaction: discord.Interaction) -> None:
        user_select = UserSelect()
        await interaction.response.send_message(view=user_select)
