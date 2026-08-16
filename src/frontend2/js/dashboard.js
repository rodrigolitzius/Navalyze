import {Api} from "./api.js"

const api = new Api()

async function get_image_url(id, size) {
    const image_response = await api.get_cover_art(id, size)

    if (!image_response.ok) {
        throw new Error(`Fetch failed: Status ${image_response.status}`)
    }

    let image_blob = await image_response.blob();
    return URL.createObjectURL(image_blob);
}

async function fill_card(entries, card_id) {
    entries.forEach(async (entry) => {
        let image_url = await get_image_url(entry.id, 400)

        let list = document.querySelector(`${card_id}`)

        list.insertAdjacentHTML("beforeend",
            `<div class="label-img">
                <img src="${image_url}" alt="Artist Cover">
                <p class="label-title">${entry.name}</p>
                <p class="label-footer" title="${entry.plays} plays">${entry.played_hours.toFixed(2)}h</p>
            </div>`
        )
    })
}

try {
    let response = await api.get_most_played_artists(30)
    let artists = await response.json()
    fill_card(artists, "#top-artists")

    response = await api.get_most_played_albums(30)
    let albums = await response.json()
    fill_card(albums, "#top-albums")

    response = await api.get_most_played_tracks(30)
    let tracks = await response.json()
    fill_card(tracks, "#top-tracks")
} catch (error) {
    console.error(error)
}
