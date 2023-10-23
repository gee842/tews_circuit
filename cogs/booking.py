from database.utils import get_pending_matches

import discord

from discord import Embed, Interaction, SelectOption, app_commands
from discord.ext import commands

from forms.cancel_match import CancelMatch


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

    @app_commands.command(name="cancel", description="Cancel a match.")
    async def cancel(self, interaction: Interaction):
        matches = await get_pending_matches(interaction.user.id)
        response = interaction.response
        if len(matches) == 0:
            await response.send_message("You don't have any pending matches.")
            return

        guild = interaction.guild
        if guild is None:
            return

        caller_id = interaction.user.id
        user_matches = []
        for challenger_id, challenged_id, date in matches:
            if caller_id == int(challenger_id):
                user_to_mention = challenged_id
            else:
                user_to_mention = challenger_id

            user_to_mention = await self.bot.fetch_user(user_to_mention)
            if user_to_mention is None:
                print(f"User with id '{challenged_id}' does not exist")
                return

            option = (user_to_mention.global_name, challenged_id, date)
            user_matches.append(option)

        cancel_match = CancelMatch(user_matches)
        msg = (
            "Keep in mind that once you select a match, "
            + "it will be cancelled and it cannot be undone. "
            + "You will need to re-book a match."
        )
        await response.send_message(msg, view=cancel_match, ephemeral=True)
