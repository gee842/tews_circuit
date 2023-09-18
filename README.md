# Features
- Run `/challenge <username>` to challenge the user
- View your pending matches with `pending_matches`

# Convention
- `unwraps` are to be used when you are 100% sure that a command absolutely *cannot* fail in its execution. 

# Notes
- When creating slash commands the slash command must be responded to in some way.
    - `ctx.say()`
    - `ctx.send()`
- `SqlxError::Protocol` is used when returning an Sqlx related error that doesn't fit into a particular error enum.
- Run the bot with  `RUST_LOG=none,tews_circuit=info cargo r` to view logging output. Running with just `cargo r` will result in *no* output aside from errors and `println`.

# TODO
- [ ] Groups/Ranks. A maximum of 8 people can hold a rank. So 8 Golds, 8 Emeralds, etc.
