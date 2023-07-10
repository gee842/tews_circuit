# Features
- Run `/challenge <username>` to challenge the user
- View your pending matches with `pending_matches`

# Notes
- Project uses nightly features like `collect_into` which means the bot might break in the future if rust is updated.
- Custom error codes are used
    - 999 - You passed in an invalid format. Please refer to the examples provided. Occurs when a user challengers a user but provides the wrong date-time format.
    - 998 - Chrono related error.
