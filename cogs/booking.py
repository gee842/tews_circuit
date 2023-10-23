from database.utils import get_pending_matches

import discord

from discord import Embed, app_commands
from discord.ext import commands


class Booking(commands.Cog):
    def __init__(self, bot):
        self.bot = bot

    @app_commands.command(
        name="pending_matches", description="Check your pending matches."
    )
    async def pending_matches(self, interaction: discord.Interaction):
        caller = interaction.user

        matches = await get_pending_matches(caller.id)
        response = interaction.response
        if len(matches) == 0:
            await response.send_message("You have no pending matches.", ephemeral=True)
            return

        embed = Embed(title="Your pending matches")
        for count, data in enumerate(matches, start=1):
            challenger_id, challenged_id, date = data
            user_id = challenger_id
            if user_id == caller.id:
                user_id = challenged_id

            user = await self.bot.fetch_user(user_id)
            msg = f"You have a challenge on {date} with {user.global_name}."
            embed.add_field(name=f"match #{count}", value=msg)

        await response.send_message(embed=embed)
