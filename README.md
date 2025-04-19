# CrickCLI

A command-line interface application to get live cricket scores.

## Prerequisites

- Rust and Cargo installed on your system
- A RapidAPI key for the Cricbuzz API

## Installation

1. Clone the repository
2. Navigate to the project directory
3. Set up your API key:
   ```bash
   # On Unix-like systems (Linux/macOS)
   export CRICKET_API_KEY="your-api-key-here"
   
   # On Windows (Command Prompt)
   set CRICKET_API_KEY=your-api-key-here
   
   # On Windows (PowerShell)
   $env:CRICKET_API_KEY="your-api-key-here"
   ```
4. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

### Live Matches

To get live cricket scores, run:

```bash
crickcli live
```

To see only IPL matches:

```bash
crickcli live ipl
```

### Recent Matches

To get recent match results, run:

```bash
crickcli recent
```

To see only IPL recent matches:

```bash
crickcli recent ipl
```

### Upcoming Matches

To get upcoming match schedules, run:

```bash
crickcli upcoming
```

To see only IPL upcoming matches:

```bash
crickcli upcoming ipl
```

## Configuration

Before using the application, you need to:

1. Sign up for a RapidAPI account at https://rapidapi.com/
2. Subscribe to the Cricbuzz API
3. Set the `CRICKET_API_KEY` environment variable with your RapidAPI key

## Features

- Get live cricket scores
- Get recent match results
- Get upcoming match schedules
- Filter matches by type (e.g., IPL)
- Display match details including:
  - Teams playing
  - Venue
  - Match status
  - Current scores (for live and recent matches)
  - Match status (for upcoming matches)
  - Wickets and overs (for live and recent matches)

## License

MIT 