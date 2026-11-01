# forecast-cli

## Project Goal

I often will check the weather for the next day and take into account the feels like, actual temperature and the weather (wind, precipitation, etc). This grows tedious and could totally be somewhat automated.

## MVP Scope

- Retrieve next day’s weather forecast
- Analyze weather (feels-like, actual, etc)
- Output/display user recommendations

## Future Plans (V2.0+)

- Allow users to customize clothing preferences (what to wear in precipitation vs mostly cloudy, etc)
- Save/track previous recommendations and feedback on them for future use
- Recommend outfits based on time of day, rather than day as a whole

## Tech Stack

**Language:** Rust

**Libraries:**
- **reqwest:** for making HTTP requests
- **serde/serde_json:** for parsing JSON responses into usable Rust objects
- **clap:** for handling command line arguments

**Data Storage:**

No persistent data storage is really needed, if it is needed in future, a simple JSON file will suffice.

**External Data Source:**

Open-Meteo Weather API: for free weather data retrieval, accessed through the earlier mentioned reqwest package

## Chunked Build Plan

1. Initialize a Rust project using cargo, set up the Github repo, and install required dependencies through cargo.
2. Run some sample tests of the Open-Meteo API to understand data response, structure, etc.
3. Set up a prototype of retrieval through Rust to fetch weather data from API.
4. Parse retrieved JSON data to extract needed info like temp, precipitation, etc.
5. Set up CLI interface to accept user commands, arguments, and display weather info.
6. Test applications with different weather conditions, bad args, etc and implement error handling.
7. Finalize (clean up workspace (remove temp testing files), write README.md, ensure all work is committed and builds).

## Documentation

- [Eisenhower Matrix](docs/eisenhower-matrix.pdf)
- [System Diagram](docs/system-diagram.png)
