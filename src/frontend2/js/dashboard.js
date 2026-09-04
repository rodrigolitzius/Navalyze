import { Api, get_image_url } from "./api.js"
import { artist_link, album_link } from "./html.js"

const api = new Api()

function format_number(n) {
    return n.toLocaleString("pt-BR")
}

function format_hours(hours) {
    return `${hours.toLocaleString("pt-BR", { maximumFractionDigits: 1 })} h`
}

function relative_time(timestamp_seconds) {
    const diff_seconds = Math.max(0, (Date.now() / 1000) - timestamp_seconds)

    const minutes = Math.floor(diff_seconds / 60)
    if (minutes < 1) return "agora"
    if (minutes < 60) return `há ${minutes} min`

    const hours = Math.floor(minutes / 60)
    if (hours < 24) return `há ${hours} h`

    const days = Math.floor(hours / 24)
    return `há ${days} d`
}

async function safe_image_url(entry_id) {
    try {
        return await get_image_url(api, entry_id, 200)
    } catch {
        return null
    }
}

function thumb_html(image_url) {
    return image_url
        ? `<img class="ranked-thumb" src="${image_url}" alt="">`
        : `<div class="ranked-thumb"></div>`
}

async function fill_featured_item(entry, featured_id, href, badge, sub_field) {
    const featured = document.querySelector(featured_id)
    featured.innerHTML = ""

    if (!entry) return

    const image_url = await safe_image_url(entry.id)
    const sub_text = sub_field && entry[sub_field] ? `<p class="list-featured-sub">${entry[sub_field]}</p>` : ""

    const inner = `
        ${image_url ? `<img src="${image_url}" alt="">` : `<div class="ranked-thumb-empty"></div>`}
        <span class="list-featured-badge">${badge}</span>
        <div class="list-featured-info">
            <div class="list-featured-text">
                <p class="list-featured-name">${entry.name}</p>
                ${sub_text}
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

async function fill_ranked_list(entries, list_id, featured_id, href, badge, sub_field) {
    const list = document.querySelector(list_id)
    list.innerHTML = ""

    if (!entries || entries.length === 0) {
        list.insertAdjacentHTML("beforeend", `<li class="list-empty">Sem dados ainda.</li>`)
        await fill_featured_item(null, featured_id, href, badge, sub_field)
        return
    }

    await fill_featured_item(entries[0], featured_id, href, badge, sub_field)

    const rest = entries.slice(1, 5)

    for (let i = 0; i < rest.length; i++) {
        const entry = rest[i]
        const image_url = await safe_image_url(entry.id)
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
        strip.insertAdjacentHTML("beforeend", `<p class="list-empty">Nada tocado recentemente.</p>`)
        return
    }

    for (const song of songs.slice(0, 10)) {
        const image_url = await safe_image_url(song.id)
        const artist_names = (song.artists || []).map(a => a.name).join(", ")

        strip.insertAdjacentHTML("beforeend",
            `<a class="recent-card" href="${album_link(song.album_id)}">
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

        document.getElementById("stat-plays").textContent = format_number(stats.plays)
        document.getElementById("stat-hours").textContent = format_hours(stats.played_hours)
        document.getElementById("stat-days").textContent = `≈ ${Math.round(stats.played_hours / 24)} dias ouvindo música`
        document.getElementById("stat-artists").textContent = format_number(stats.artists)
        document.getElementById("stat-albums").textContent = format_number(stats.albums)
        document.getElementById("stat-tracks").textContent = format_number(stats.tracks)
    } catch (error) {
        console.error("Falha ao carregar /api/stats", error)
    }
}

async function load_top_lists() {
    try {
        const response = await api.get_most_played_artists(30)
        const artists = await response.json()
        await fill_ranked_list(artists, "#top-artists", "#top-artists-featured", "artist.html", "Top artista", null)
    } catch (error) {
        console.error("Falha ao carregar artistas mais ouvidos", error)
    }

    try {
        const response = await api.get_most_played_albums(30)
        const albums = await response.json()
        await fill_ranked_list(albums, "#top-albums", "#top-albums-featured", "album.html", "Top álbum", "artist")
    } catch (error) {
        console.error("Falha ao carregar álbuns mais ouvidos", error)
    }

    try {
        const response = await api.get_most_played_tracks(30)
        const tracks = await response.json()
        await fill_ranked_list(tracks, "#top-tracks", "#top-tracks-featured", null, "Top faixa", "artist")
    } catch (error) {
        console.error("Falha ao carregar faixas mais ouvidas", error)
    }

    try {
        const response = await api.get_most_played_playlists(30)
        const playlists = await response.json()
        await fill_ranked_list(playlists, "#top-playlists", "#top-playlists-featured", null, "Top playlist", null)
    } catch (error) {
        console.error("Falha ao carregar playlists mais ouvidas", error)
    }
}

async function load_recent() {
    try {
        const response = await api.get_recently_played(10, 0)
        const songs = await response.json()
        await fill_recent_strip(songs)
    } catch (error) {
        console.error("Falha ao carregar recentemente tocadas", error)
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