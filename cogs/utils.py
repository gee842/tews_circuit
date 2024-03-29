"""
Utility functions to make sure the bot is working properly
and to sync whatever slash commands the bot has.
"""

from typing import Optional, Literal

import discord

from discord.ext import commands
from discord.ext.commands import Greedy, Context


class Utils(commands.Cog):
    def __init__(self, bot):
        self.bot = bot
        self._last_member = None

    @commands.command(name="ping")
    async def ping(self, ctx):
        await ctx.send("Dong!")

    @commands.command(name="sync")
    async def sync(
        self,
        ctx: Context,
        guilds: Greedy[discord.Object],
        spec: Optional[Literal["sync", "copy", "reset"]] = None,
    ) -> None:
        """
        Used to manage if a guild has access to the bot's slash commands.

        When called without a guild id specified, the command assumes you want to make
        changes to the guild the command is called from.

        Options:
            - sync: Syncs all comands to the current guild.
            - copy: Copy all commands to every guild the bot is in.
            - reset: Removes all commands from every guild.
        """
        if ctx.author.id != 275797064674312193:
            print("Lacking permissions")
            return

        tree = ctx.bot.tree

        if not guilds:
            if spec == "sync":
                synced = await tree.sync(guild=ctx.guild)
            elif spec == "copy":
                tree.copy_global_to(guild=ctx.guild)
                synced = await tree.sync(guild=ctx.guild)
            elif spec == "reset":
                tree.clear_commands(guild=ctx.guild)
                await tree.sync(guild=ctx.guild)
                synced = []
            else:
                synced = await tree.sync()

            msg = "globally" if spec is None else "to the current guild."
            await ctx.send(f"Synced {len(synced)} commands {msg}")
            return

        ret = 0
        for guild in guilds:
            try:
                await tree.sync(guild=guild)
            except discord.HTTPException:
                pass
            else:
                ret += 1

        await ctx.send(f"Synced the tree to {ret}/{len(guilds)}.")
