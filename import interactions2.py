import discord
from circuit import Player, Circuit
bot = discord.Bot()
circuit = Circuit()
# we need to limit the guilds for testing purposes
# so other users wouldn't see the command that we're testing

@bot.command(description="Sends the bot's latency.") # this decorator makes a slash command
async def ping(ctx): # a slash command will be created with the name "ping"
    await ctx.respond(f"Pong! Latency is {bot.latency}")



@bot.command(name='add_player', help='Adds a new player to the circuit')
async def add_player(ctx, name:str):
    circuit.add_player(name)
    await ctx.respond(f'Player {name} added to the circuit.')

@bot.command(name='fight', help='Report fight results')
async def fight(ctx, winner, loser, stocks_lost_match1: int, stocks_lost_match2: int, stocks_lost_match3: int = None):
    circuit.fight(winner, loser, stocks_lost_match1, stocks_lost_match2, stocks_lost_match3)
    await ctx.respond(f'Fight results recorded: {winner} vs {loser}. Winner: {winner}')

@bot.command(name='scoreboard', help='Show the current scoreboard')
async def scoreboard(ctx):
    message = 'Scoreboard:\n\n'
    print(circuit.groups.items())
    for group, players in circuit.groups.items():
        message += f'{group.capitalize()} Group:\n'
        for player_name in players:
            player = circuit.players[player_name]
            message += f'{player.name}: {player.points} points\n'
        message += '\n'
    await ctx.respond(message)

@bot.command(name='player_info', help='Show info about a specific player')
async def player_info(ctx, name):
    player = circuit.players.get(name)
    if player:
        await ctx.respond(f'{player.name}: {player.points} points, {player.group.capitalize()} Group')
    else:
        await ctx.respond(f'Player {name} not found in the circuit.')

bot.run('MTEwMTk1NzYwODA4MTk4NTU0Nw.GceCwl.FBcMQ8_pHXbhdJ8Y8-yMkEAZrZmYd07_wNNfqU')
