import { Api, get_image_url } from "./api.js"
import { build_link_list, artist_link, album_link } from "./html.js"

const api = new Api()

let track_id = new URLSearchParams(window.location.search).get("id");

let track_response = await api.get_track(track_id)
let track = await track_response.json()

let header = document.getElementById("image-header")

if (Array.isArray(track)) {
    header.insertAdjacentHTML("beforeend",
        `<div class="content">
            <h1>Nenhum dado encontrado</h1>
        </div>`
    )
} else {
    let image_url = null
    try {
        image_url = await get_image_url(api, track_id, 700)
    } catch {
        image_url = null
    }

    header.insertAdjacentHTML("beforeend",
        `<img src="${image_url}">
        <div class="content">
            <h1 class="track-title">${track.name}</h1>
            <div id="artists" class="link-list"></div>
            <div id="album" class="link-list"></div>
        </div>`
    )

    build_link_list(
        header.querySelector("#album"),
        track.album,
        album_link(track.album_id),
    )

    for (const artist of track.artists) {
        build_link_list(
            header.querySelector("#artists"),
            artist.name,
            artist_link(artist.id),
        )
    }

    let last_played = track.timestamps && track.timestamps.length
        ? new Date(Math.max(...track.timestamps) * 1000).toLocaleDateString("pt-BR")
        : null

    let stats = document.getElementById("track-stats")

    let stat_items = [
        { label: "Reproduções", value: `${track.plays}` },
        { label: "Tempo ouvido", value: format_duration(track.played_hours) },
        { label: "Última vez", value: last_played ?? "—" }
    ]

    for (const stat of stat_items) {
        stats.insertAdjacentHTML("beforeend",
            `<div class="track-stat-card">
                <p class="track-stat-label">${stat.label}</p>
                <p class="track-stat-value">${stat.value}</p>
            </div>`
        )
    }
}

function format_duration(hours) {
    if (hours < 1) {
        return `${Math.round(hours * 60)} min`
    }

    return `${hours.toFixed(1)}h`
}
