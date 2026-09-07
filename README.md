> [!WARNING]
This is a WIP tool. Expect missing features, bugs and rough edges.

# Navalyze
This project aims to provide Navidrome users detailed and configurable analysis of their music listening history and habits, much like Last.fm and ListenBrainz.

# Features
This is what Navalyze currently supports

## API
It exposes:
- A list of your most played playlists/artists/albums/tracks
- Statistics for any playlist/artist/album/track (For example, albums and playlists show all tracks and how much you played each one)
- A graph for your playlists/artists/albums/tracks activity over time
- A graph for how frequently you play songs during your day
- How many artists/albums/tracks/scrobbles you have, including the total amount of time played

Additionally:
- All endpoints let you filter your scrobbles by a starting/ending timestamp, so you can analyze your history from/to any point in time.
- All graphs have a configurable timezone and number of data points
- All endpoints use the duration of the songs for the analysis rather than the play count. This is often times more accurate since, for example, 10 plays on a 5m song is much more than 10 plays on a 2m song
- Scrobbles are automatically updated on every request

## Frontend
The frontend currently only supports small subset of the above features, but it we're working on it.

# Why Navalyze?
There are various services with the goal of storing and analyzing your listening history, so what makes Navalyze different?

One of the things that bothered me with these other apps was the fact that your Navidrome library and your "scrobble library" are independant. 

That means some metadata for your tracks is likely to be wrong. For example, last.fm is terrible for multi artist tracks, and ListenBrainz doesn't handle songs without MusicBrainz tags very well.

It also means any changes in Navidrome (Like changing a file's metadata or manually deleting scrobbles) will not be mirrored on the other side. A lot of manual intervention is required to keep both up to date with each other.

Navalyze, however, is entirely integrated with Navidrome. Any information present in Navalyze will be consistent with Navidrome.

# How to use
> [!NOTE]
Navalyze uses the /api/scrobble/ endpoint, meaning only versions after [pr 5761](https://github.com/navidrome/navidrome/pull/5761) are supported.

No binary releases are currently available, so you'll have to build it yourself
## Building
1. Firstly, you'll need to install these:
    - frontend: [npm](https://www.npmjs.com/) (used to install Vite and Chart.JS)
    - backend: [Rust](https://rust-lang.org/) (nightly version, because of [Polonius](https://blog.rust-lang.org/2026/08/04/enabling-polonius-alpha-on-nightly/))
    - building: [Just](https://just.systems/) (A simple way to combine backend and frontend builds)

1. Clone the repo: `git clone https://github.com/rodrigolitzius/Navalyze`

1. Go into the project folder: `cd Navalyze`

1. Run `just build` to build.

## Running
Before running, you first need to set up your configs for Navalyze. This can be done with a `settings.toml` file or the command line.

The recommended way is to use `settings.toml`. move the [default settings file](./settings.toml) to the `build` folder, then edit the settings to your needs.

If you prefer the command line, run `just run "--help"` to see the available options.

Finally, to run, just do `just run ""`, or `just run "<options>"` to override your `settings.toml` options.

# MusicBrainz integration
> [!NOTE]
MusicBrainz integration is very WIP

If your library has MusicBrainz tags, Navalyze can take advantage of them to gather additional information for analysis, but this is completely optional, so no worries if you don't use MusicBrainz
