from datetime import datetime

import discord

from forms.challenge_submission import ChallengeSubmission
from database.challenge import new_challenge

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

        chal_sub = ChallengeSubmission()
        await interaction.response.send_message(view=chal_sub, ephemeral=True)
        await chal_sub.wait()

        year = datetime.now().date().year
        date = f"{chal_sub.day} {chal_sub.month[0]} {year} {chal_sub.time}"  # type: ignore

        await new_challenge(date, interaction.user, chal_sub.user)  # type: ignore
