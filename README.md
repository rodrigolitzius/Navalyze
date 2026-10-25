> [!WARNING]
This is a WIP tool. Expect bugs and rough edges.

# Navalyze
This project aims to provide Navidrome users detailed and configurable analysis of their music listening history and habits, much like Last.fm and ListenBrainz.

## Features
This is what Navalyze currently supports:
- How many artists/albums/tracks/scrobbles you have, including the total amount of time played
- A list of your most played playlists/artists/albums/tracks and genres
- Statistics for any playlist/artist/album/track (For example, albums and playlists show all tracks and how much you played each one)
- A graph of activity over time for your playlists/artists/albums/tracks and genres/languages/release-types.
- A graph for how frequently you play songs during your day
- A list of your recently played songs

Additionally:
- You can filter your scrobbles by starting/ending date and time, so you can analyze your history from/to any point in time.
- Your scrobbles are automatically updated.
- Everything uses the duration of the songs for the analysis rather than the play count. This is often times more accurate since, for example, 10 plays on a 5m song is much more than 10 plays on a 2m song

If you're developing your own project and need any of the above data, you can use Navalyze's API to fetch it. (As long as you follow the license). The API still needs a few touches for ease-of-use though.

## Known issues
(Clearly not an exhaustive list. These are just important things to keep in mind)
- Your Navalyze session can expire and the website doesn't realize it, so it just breaks.
- No logout button. You have to change the url to /login.html manually

# Why Navalyze?
There are various services with the goal of storing and analyzing your listening history, so what makes Navalyze different?

One of the things that bothered me with these other apps was the fact that your Navidrome library and your "scrobble library" are independant. 

That means some metadata for your tracks is likely to be wrong. For example, last.fm is terrible for multi artist tracks, and ListenBrainz doesn't handle songs without MusicBrainz tags very well.

It also means any changes in Navidrome (Like changing a file's metadata or manually deleting scrobbles) will not be mirrored on the other side. A lot of manual intervention is required to keep both up to date with each other.

Navalyze, however, is entirely integrated with Navidrome. Any information present in Navalyze will be consistent with Navidrome.

Finally, a lot of services don't offer many informations about your history. Navalyze tries to give you as much as possible.

# How to use
> [!NOTE]
Only Navidrome versions 0.64.0 and above are supported.

No binary releases are currently available, so you'll have to build it yourself:

1. Clone the repo: `git clone https://github.com/rodrigolitzius/Navalyze`

1. Go into the project folder: `cd Navalyze`

1. Choose how to install Navalyze: [Docker](#docker) (Recommended) and [manual compilation](#compiling) (Useful for developers)

## Docker
1. Run this to build the docker image yourself: `docker build -t rodrigolitzius/navalyze:dev .`

1. Create a folder for Navalyze. Inside that create another one called `data` _before_ running the container. If you run the container before creating the directory, Docker will create one automatically with the wrong permissions, so you'll get permission errors while trying to move or edit files inside it.

1. Move the [default settings file](./settings.toml) to the `data` folder and edit what you need. Every setting has a comment above explaining it.

When running Navalyze inside a docker container, you have to make sure your Navidrome instance(s) are acessible from inside the container. There are a few ways to do that depending on your setup and preferences:

### Navidrome running locally
If you're running Navidrome on the same machine as Navalyze, Here's some options:
Note that this is not an exhaustive list

1. The easiest way is to set your network mode inside the container to host. All you have to do is uncomment `network_mode: host` in `docker-compose.yml`. file Refer to [this](https://docs.docker.com/compose/how-tos/networking/#change-the-network-mode) for more details on this.

1. You can also add the Navalyze service to your Navidrome's docker-compose.yml file. Refer [here](https://docs.docker.com/get-started/docker-concepts/the-basics/what-is-docker-compose/)

1. Lastly, you can create two networks that both Navalyze and Navidrome share: [compose networking](https://docs.docker.com/compose/how-tos/networking/)

### Navidrome running externally
If Navidrome is running outside of your machine, you don't have to set up anything, because every docker container can access the outside internet by default. However, if Navidrome is only reachable by IPv6, you will also have to set up [IPv6 networking](https://docs.docker.com/engine/daemon/ipv6/). You will get a "Network unreachable" error otherwise. 

If you don't know whether it is IPv6 only, it probably isn't.

## Compiling
1. Firstly, you'll need to install these:
    - frontend: [npm](https://www.npmjs.com/) (used to install Vite and the frontend dependencies)
    - backend: [Rust](https://rust-lang.org/) (nightly version, because of [Polonius](https://blog.rust-lang.org/2026/08/04/enabling-polonius-alpha-on-nightly/))
    - building: [Just](https://just.systems/) (A simple way to combine backend and frontend builds)

1. After cloning and installing dependencies, run `just build` to build.

1. Move the [`settings.toml`](./settings.toml) to the `build` folder created by the previous step and edit it to your needs. If you prefer the command line, run `just run "--help"` to see the available options. Options set in the CLI override those in `settings.toml`.

1. Finally, to run, do `just run ""`, or `just run "<options>"`

# MusicBrainz integration
> [!NOTE]
MusicBrainz integration is very WIP, and may break the user experience

If your library has MusicBrainz tags, Navalyze can take advantage of them to gather additional information for analysis. See the settings for how to set it up.

It is completely optional, however, so no worries if you don't use MusicBrainz.
