import { Api, get_image_url } from "./api.js"
import { album_link, artist_link } from "./html.js"

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
        image_url = await get_image_url(api, track_id, 600)
    } catch {
        image_url = null
    }

    let artist_id = null
    try {
        let album_response = await api.get_album(track.album_id)
        let album = await album_response.json()
        artist_id = album.artists?.[0]?.id ?? null
    } catch {
        artist_id = null
    }

    header.insertAdjacentHTML("beforeend",
        `${image_url ? `<div class="track-header-bg" style="background-image:url('${image_url}')"></div>` : ""}
        <div class="track-header-scrim"></div>
        ${image_url ? `<img src="${image_url}">` : ""}
        <div class="content">
            <a class="track-title" href="${album_link(track.album_id)}"><h1>${track.name}</h1></a>
            ${artist_id
                ? `<a class="track-artist" href="${artist_link(artist_id)}">${track.artist}</a>`
                : `<p class="track-artist">${track.artist}</p>`}
        </div>`
    )

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