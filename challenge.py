from typing import Tuple
import discord

from views import UserSelect, Calendar

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

        username = await self.user_selection(interaction)
        if username is None:
            return

        await channel.send(f"You wish to challenge {username.global_name}? Good luck!")

        # TODO: FIX: everything below here isn't working.
        date = await self.date_selection(interaction)
        if date is None:
            return

        (day, month) = date
        await channel.send(f"The challenge is on {day} {month}!")

    async def user_selection(self, interaction: discord.Interaction) -> discord.User | None:
        user_select = UserSelect()
        await interaction.response.send_message(view=user_select)

        await user_select.wait()

        username = user_select.user

        if username is None:
            return

        username = username[0]
        return username

    async def date_selection(self, interaction: discord.Interaction) -> Tuple[str, dict[str, str]] | None:
        calendar = Calendar()
        await interaction.response.send_message(view=calendar)
        await calendar.wait()

        day = calendar.day
        month = calendar.month

        if day is None or month is None:
            return

        return (day, month)
