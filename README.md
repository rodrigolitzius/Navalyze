> [!WARNING]
This is a WIP tool. Expect missing features, bugs and rough edges.

# Navalyze
Analyse your music listening habits for your Navidrome library.

# Features
This is what Navalyze currently supports

***Frontend:***
- **Most played:** artists/albums/tracks
- **Artist overview:** View your artists's albums

***Backend***
- \*all the above*
- **Range selection**: Filter your scrobbles by starting/ending timestamps
- **Album information:** View information for any album

# Why Navalyze?
There are various services with the goal of storing and analyzing your listening history, so what makes Navalyze different?

One of the things that bothered me with these other apps was the fact that your Navidrome library and your "scrobble library" are independant. 

That means some metadata for your tracks is likely to be wrong. For example, last.fm is terrible for multi artist tracks, and ListenBrainz doesn't handle songs without MusicBrainz tags very well.

It also means any changes in Navidrome (Like changing a file's metadata or manually deleting scrobbles) will not be mirrored on the other side. A lot of manual intervention is required to keep both up to date with each other

Navalyze, however, is entirely integrated with Navidrome. Any information present in Navalyze will be consistent with Navidrome.

# How to use
> [!NOTE]
Navalyze uses the /api/scrobble/ endpoint, meaning only versions after [pr 5761](https://github.com/navidrome/navidrome/pull/5761) are supported.

Deploying Navalyze is a bit cumbersome, as it is still in development, but here's what you have to do:

1. Clone the repo
1. Compile the backend at /src/backend/ using `cargo build --release` and run with `cargo run -- --mbz-token <your-listenbrainz-token> -p <port>`. You can omit the mbz-token parameter if you don't use MusicBrainz or don't wish to.
1. Go to src/frontend/ and run an HTTP server there. I do this with `python -m http.server`
1. Now access your HTTP server (if you used python, it should be accesible at http://localhost:8000)
1. backend-url will be `http://localhost:<port>`
1. Navidrome url is the address that points to Navidrome.
1. The rest of the fields are your credentials.

> [!NOTE]
The web interface is in Brazilian Portuguese

# MusicBrainz integration
> [!NOTE]
MusicBrainz integration is very WIP

If your library has MusicBrainz tags, Navalyze can take advantage of them to gather additional information for analysis, but this is completely optional, so no worries if you don't use MusicBrainz
