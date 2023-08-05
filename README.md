# Features
- Run `/challenge <username>` to challenge the user
- View your pending matches with `pending_matches`

# Convention
- `unwraps` are to be used when you are 100% sure that a command absolutely *cannot* fail in its execution. 

# Notes
- `SqlxError::Protocol` is used when returning an Sqlx related error that doesn't fit into a particular error enum.
- Run the bot with  `yjh@fedora ~> RUST_LOG=none,tews_circuit=info cargo r` to view logging output. Running with just `cargo r` will result in *no* output aside from errors.
