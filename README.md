# Rustbridge-CLI

A little administrative tool to add organizers to the [Rustbridge](https://github.com/rustbridge/rustbridge.io) database.

# Requirements 
Follow the [installation instructions](https://github.com/rustbridge/rustbridge.io/blob/master/README.md) for the Rustbridge prodject.
Once the website/database is set up, and all of your tables are created, compile the Rustbridge-CLI using the command `cargo build --release`.

## Salt generation
After building, navigate to the build directory, `cd target/release/` and run the command: `./rustbridge-cli salt` to generate a random salt for the database. 

## Adding organizers
Once the database has been salted, use the command: `./rustbridge-cli add -u <username> -p <password>` to add a new organizer to the database.

