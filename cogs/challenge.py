from datetime import datetime
from database.utils import get_pending_matches

from forms.challenge_submission import ChallengeSubmission
from forms.finish_match import FinishMatch

from database.challenge import new_challenge

import discord

from discord import Embed, app_commands, ChannelType
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
        response = interaction.response
        advise = (
            "It is recommended for you and your opponent to run /pending_matches "
            + "to prevent setting a challenge that's too close to one another."
        )

        await response.send_message(advise, view=chal_sub, ephemeral=True)
        await chal_sub.wait()

        if not chal_sub.cancelled:
            year = datetime.now().date().year
            date = f"{year}-{chal_sub.month[0]}-{chal_sub.day} {chal_sub.time}:00"  # type: ignore

            await new_challenge(date, interaction.user, chal_sub.user)  # type: ignore

    @app_commands.command(
        name="finish", description="Mark a match as finish and process point totals."
    )
    async def finish(self, interaction: discord.Interaction) -> None:
        response = interaction.response

        match = FinishMatch()
        await response.send_message(view=match, ephemeral=True)
        await match.wait()

    @app_commands.command(
        name="pending_matches", description="Check your pending matches."
    )
    async def pending_matches(self, interaction: discord.Interaction):
        matches = await get_pending_matches(interaction.user.id)
        response = interaction.response
        if len(matches) == 0:
            await response.send_message("You have no pending matches.", ephemeral=True)
            return

        embed = Embed(title="Your pending matches")
        for count, data in enumerate(matches, start=1):
            challenged, date = data
            challenged = await self.bot.fetch_user(challenged)
            msg = f"You have a challenge on {date} with {challenged}."
            embed.add_field(name=f"match #{count}", value=msg)

        await response.send_message(embed=embed)
