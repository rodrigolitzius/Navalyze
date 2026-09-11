import { Api, get_image_url } from "./api.js"
import { track_link } from "./html.js"
import { relative_time, format_hours } from "./util.js"
import { Chart } from "chart.js/auto"

const api = new Api()

async function safe_image_url(entry_id, size) {
    try {
        return await get_image_url(api, entry_id, size)
    } catch {
        return null
    }
}

function thumb_html(image_url) {
    return image_url
        ? `<img class="ranked-thumb" src="${image_url}" alt="">`
        : `<div class="ranked-thumb"></div>`
}

async function fill_featured_item(entry, featured_id, href) {
    const featured = document.querySelector(featured_id)
    featured.innerHTML = ""

    if (!entry) return

    const image_url = await safe_image_url(entry.id, 800)

    const inner = `
        ${image_url ? `<img src="${image_url}" alt="">` : `<div class="ranked-thumb-empty"></div>`}
        <div class="list-featured-info">
            <div class="list-featured-text">
                <p class="list-featured-name">${entry.name}</p>
            </div>
            <span class="list-featured-stat">${format_hours(entry.played_hours)}</span>
        </div>
    `

    featured.insertAdjacentHTML("beforeend",
        href
            ? `<a class="list-featured-link" href="${href}?id=${entry.id}">${inner}</a>`
            : `<div class="list-featured-link">${inner}</div>`
    )
}

async function fill_ranked_list(entries, list_id, featured_id, href) {
    const list = document.querySelector(list_id)
    list.innerHTML = ""

    if (!entries || entries.length === 0) {
        list.insertAdjacentHTML("beforeend", `<li class="list-empty">No data</li>`)
        await fill_featured_item(null, featured_id, href)
        return
    }

    await fill_featured_item(entries[0], featured_id, href)

    const rest = entries.slice(1, 5)

    for (let i = 0; i < rest.length; i++) {
        const entry = rest[i]
        const image_url = await safe_image_url(entry.id, 200)
        const stat_text = format_hours(entry.played_hours)

        const inner = `
            <span class="ranked-rank">${i + 2}</span>
            ${thumb_html(image_url)}
            <div class="ranked-info">
                <p class="ranked-name">${entry.name}</p>
            </div>
            <span class="ranked-stat">${stat_text}</span>
        `

        list.insertAdjacentHTML("beforeend",
            href
                ? `<li><a class="ranked-item" href="${href}?id=${entry.id}">${inner}</a></li>`
                : `<li><div class="ranked-item">${inner}</div></li>`
        )
    }
}

async function fill_recent_strip(songs) {
    const strip = document.getElementById("recent-strip")
    strip.innerHTML = ""

    if (!songs || songs.length === 0) {
        strip.insertAdjacentHTML("beforeend", `<p class="list-empty">Nothing to see here.</p>`)
        return
    }

    for (const song of songs) {
        const image_url = await safe_image_url(song.id, 400)
        const artist_names = (song.artists || []).map(a => a.name).join(", ")

        strip.insertAdjacentHTML("beforeend",
            `<a class="recent-card" href="${track_link(song.id)}">
                ${image_url ? `<img src="${image_url}" alt="">` : `<img alt="">`}
                <p class="recent-title">${song.title}</p>
                <p class="recent-sub">${artist_names}</p>
                <p class="recent-time">${relative_time(song.timestamp)}</p>
            </a>`
        )
    }
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
        const response = await api.get_most_played_artists(30)
        const artists = await response.json()
        await fill_ranked_list(artists, "#top-artists", "#top-artists-featured", "artist.html")
    } catch (error) {
        console.error("Failure to load artists", error)
    }

    try {
        const response = await api.get_most_played_albums(30)
        const albums = await response.json()
        await fill_ranked_list(albums, "#top-albums", "#top-albums-featured", "album.html")
    } catch (error) {
        console.error("Failure to load albums", error)
    }

    try {
        const response = await api.get_most_played_tracks(30)
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
        let frequency_graph = document.getElementById("graph-hour")

        let graph_data = await api.frequency(48)
        graph_data = await graph_data.json()

        let data = new Array
        for (const [key, value] of Object.entries(graph_data)) {
            data.push({
                x: ((key / (24 * 60 * 60)) * 24).toString(),
                y: value
            })
        }

        new Chart(frequency_graph, {
            type: 'line',
            data: {
                datasets: [{
                    label: "Hours",
                    data: data
                }]
            },
            options: {
                maintainAspectRatio: false
            }
        });
    } catch (error) {
        console.error("Failed to load graph", error)
    }
}

function setup_timezone_select() {
    const select = document.getElementById("timezone-select")

    const system_timezone = Intl.DateTimeFormat().resolvedOptions().timeZone
    const current_timezone = localStorage.getItem("timezone") || system_timezone

    const timezones = typeof Intl.supportedValuesOf === "function"
        ? Intl.supportedValuesOf("timeZone")
        : [system_timezone]

    if (!timezones.includes(current_timezone)) {
        timezones.unshift(current_timezone)
    }

    for (const timezone of timezones) {
        const option = document.createElement("option")
        option.value = timezone
        option.textContent = timezone.replaceAll("_", " ")
        if (timezone === current_timezone) option.selected = true
        select.appendChild(option)
    }

    if (!localStorage.getItem("timezone")) {
        localStorage.setItem("timezone", current_timezone)
    }

    select.addEventListener("change", () => {
        localStorage.setItem("timezone", select.value)
    })
}

setup_timezone_select()
load_stats()
load_top_lists()
load_recent()
load_graphs()
