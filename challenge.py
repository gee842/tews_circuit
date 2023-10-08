from typing import Tuple
import discord
from discord.webhook.async_ import interaction_response_params

from views import ChallengeSubmission 

from discord import app_commands, ChannelType
from discord.ext import commands

class Challenge(commands.Cog):
    def __init__(self, bot):
        self.bot = bot

    @app_commands.command(name="challenge", description="Challenge someone.")
    async def challenge(self, interaction: discord.Interaction) -> None:
        channel = interaction.channel
        if channel is None:
            return

        if channel.type == ChannelType.category or channel.type == ChannelType.forum:
            return

        view = ChallengeSubmission()
        await interaction.response.send_message(view=view)

        # i'd pass view.user and view.month and stuff
        # into another function that will put that data
        # into a database, I didn't write that part yet.
