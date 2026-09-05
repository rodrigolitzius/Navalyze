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

No releases are currently available, so you'll have to build it yourself
## Dependencies
Firstly, you'll need to install these for the frontend and backend
- frontend
    - npm (used to install Vite and Chart.JS)
- backend
    - Rust (nightly)

Clone the repo

## Building/running
Clone the repo: `git clone github.com/rodrigolitzius/Navalyze`

Go into the project folder: `cd Navalyze`

To build, simply run `./build.sh`. 

to run, do `./run.sh -m <your-listenbrainz-token> -p <port>`

The backend may not work properly if listenbrainz isn't available, so only use the `-m` if you know listenbrainz is running and you want to use it.

If your Navidrome's URL has an invalid SSL certificate, you can add `-c` to ignore it.

Now you should be able to access the website at `http://localhost:<port>`

# MusicBrainz integration
> [!NOTE]
MusicBrainz integration is very WIP

If your library has MusicBrainz tags, Navalyze can take advantage of them to gather additional information for analysis, but this is completely optional, so no worries if you don't use MusicBrainz
