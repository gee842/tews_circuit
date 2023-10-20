# Challenges
- `challenge` 
    - [x]  Make sure to handle cancellation properly, especially when "Cancel" is chosen in the view and when the user exits out of the modal with escape. First will result in error (no detriment to user), second will insert an entry in the database with None in one of the date-time components.
    - [ ] Ensure there are checks in place to prevent double booking; a player shouldn't be able to have two matches scheduled for the same time.
- `finish_match`
    - [x] Figure out how to add buttons AFTER the user to challenge has been selected.
    - [ ] If the match is with yourself or a bot, the match with yourself in it will be marked as finished.
    - [ ] Add checks for when the incorrect user is selected.
