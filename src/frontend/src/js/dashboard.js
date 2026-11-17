import { Api } from "./api.js"
import { build_configurable_graph, GraphDataSource, track_link } from "./html.js"
import { relative_time, format_hours, load_images } from "./util.js"

const api = new Api()

function thumb_html(image_url) {
    return image_url
        ? `<img class="ranked-thumb" src="${image_url}" alt="">`
        : `<div class="ranked-thumb"></div>`
}

async function fill_featured_item(entry, featured_id, href) {
    const featured = document.querySelector(featured_id)
    featured.innerHTML = ""

    if (!entry) return

    const inner = `
        <div class="img-container">
            <img class="card-img">
        </div>
        <div class="list-featured-info">
            <div class="list-featured-text">
                <p class="list-featured-name">${entry.name}</p>
            </div>
            <span class="list-featured-stat" title="${entry.plays} plays">${format_hours(entry.played_hours)}</span>
        </div>
    `

    featured.insertAdjacentHTML("beforeend",
        href
            ? `<a class="list-featured-link" href="${href}?id=${entry.id}">${inner}</a>`
            : `<div class="list-featured-link">${inner}</div>`
    )

    load_images(api, [{id: entry.id, size: 800, container: featured}])
}

async function fill_ranked_list(entries, list_id, featured_id, href) {
    const list = document.querySelector(list_id)
    list.innerHTML = ""

    if (!entries || entries.length === 0) {
        list.insertAdjacentHTML("beforeend", `<li class="list-empty">No data</li>`)
        await fill_featured_item(null, featured_id, href)
        return
    }

    await fill_featured_item(entries.shift(), featured_id, href)

    let images = new Array()

    entries.forEach((entry, i) => {
        const inner = `
            <span class="ranked-rank">${i + 2}</span>
            <img class="ranked-thumb">
            <div class="ranked-info">
                <p class="ranked-name">${entry.name}</p>
            </div>
            <span class="ranked-stat" title="${entry.plays} plays">${format_hours(entry.played_hours)}</span>
        `

        list.insertAdjacentHTML("beforeend",
            href
                ? `<li><a class="ranked-item" href="${href}?id=${entry.id}">${inner}</a></li>`
                : `<li><div class="ranked-item">${inner}</div></li>`
        )

        images.push({id: entry.id, size: 200, container: list.lastElementChild})
    })

    load_images(api, images)
}

async function fill_recent_strip(songs) {
    const strip = document.getElementById("recent-strip")
    strip.innerHTML = ""

    if (!songs || songs.length === 0) {
        strip.insertAdjacentHTML("beforeend", `<p class="list-empty">Nothing to see here.</p>`)
        return
    }

    let images = new Array()

    for (const song of songs) {
        const artist_names = (song.artists || []).map(a => a.name).join(", ")

        strip.insertAdjacentHTML("beforeend",
            `<a class="recent-card" href="${track_link(song.id)}">
                <div class="img-container">
                    <img class="card-img">
                </div>
                <p class="recent-title">${song.title}</p>
                <p class="recent-sub">${artist_names}</p>
                <p class="recent-time">${relative_time(song.timestamp)}</p>
            </a>`
        )

        images.push({id: song.id, size: 400, container: strip.lastElementChild})
    }

    load_images(api, images)
}

async function load_stats() {
    try {
        const response = await api.get_stats()
        const stats = await response.json()

        document.getElementById("stat-plays").textContent = stats.plays
        document.getElementById("stat-hours").textContent = format_hours(stats.played_hours)
        document.getElementById("stat-artists").textContent = stats.artists
        document.getElementById("stat-albums").textContent = stats.albums
        document.getElementById("stat-tracks").textContent = stats.tracks
    } catch (error) {
        console.error("Failure loading /api/stats", error)
    }
}

async function load_top_lists() {
    try {
        const response = await api.get_most_played_artists(5)
        const artists = await response.json()
        await fill_ranked_list(artists, "#top-artists", "#top-artists-featured", "artist.html")
    } catch (error) {
        console.error("Failure to load artists", error)
    }

    try {
        const response = await api.get_most_played_albums(5)
        const albums = await response.json()
        await fill_ranked_list(albums, "#top-albums", "#top-albums-featured", "album.html")
    } catch (error) {
        console.error("Failure to load albums", error)
    }

    try {
        const response = await api.get_most_played_tracks(5)
        const tracks = await response.json()
        await fill_ranked_list(tracks, "#top-tracks", "#top-tracks-featured", "track.html")
    } catch (error) {
        console.error("Failure to load tracks", error)
    }
}

async function load_recent() {
    try {
        const response = await api.get_recently_played(20, 0)
        const songs = await response.json()
        await fill_recent_strip(songs)
    } catch (error) {
        console.error("Failure to load recently played", error)
    }
}

async function load_graphs() {
    try {
        let graph_element = document.getElementById("graph-hour")

        build_configurable_graph(
            graph_element.querySelector("canvas"),
            graph_element.querySelector(".graph-type-selection"),
            new class extends GraphDataSource {
                async daily()    { return {"Every scrobble": await api.get_all_graph(48, "daily").then((v) => v.json())} }
                async weekly()   { return {"Every scrobble": await api.get_all_graph(7, "weekly").then((v) => v.json())} }
                async all_time() { return {"Every scrobble": await api.get_all_graph(50, "all_time").then((v) => v.json())} }
            }
        )

    } catch (error) {
        console.error("Failed to load graph", error)
    }
}

load_stats()
load_top_lists()
load_recent()
load_graphs()
