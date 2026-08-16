import {Api, get_image_url} from "./api.js"

const api = new Api()

async function fill_card(entries, card_id, href) {
    for (const entry of entries) {
        let image_url = await get_image_url(api, entry.id, 400)

        let list = document.querySelector(`${card_id}`)

        list.insertAdjacentHTML("beforeend",
            `<div class="card">
                <img class="card-img" src="${image_url}" alt="Artist Cover">
                <a href="${href}?id=${entry.id}">
                    <p class="card-name">${entry.name}</p>
                </a>
                <p class="card-footer" title="${entry.plays} plays">${entry.played_hours.toFixed(2)}h</p>
            </div>`
        )
    }
}

try {
    let response = await api.get_most_played_artists(30)
    let artists = await response.json()
    fill_card(artists, "#top-artists", "artist.html")

    response = await api.get_most_played_albums(30)
    let albums = await response.json()
    fill_card(albums, "#top-albums", "album.html")

    response = await api.get_most_played_tracks(30)
    let tracks = await response.json()
    fill_card(tracks, "#top-tracks")
} catch (error) {
    console.error(error)
}
