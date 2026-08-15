import {Api} from "./api.js"

const api = new Api()

function get_banner() {
    const name = document.querySelector("#artist-banner > p")
    const img = document.querySelector("#artist-banner > img")

    return [name, img]
}

async function get_image_url(id, size) {
    const image_response = await api.get_cover_art(id, size)

    if (!image_response.ok) {
        throw new Error(`Fetch failed: Status ${image_response.status}`)
    }

    let image_blob = await image_response.blob();
    return URL.createObjectURL(image_blob);
}

async function fill_card(entries, card_id) {
    for (const [index, entry] of entries.entries()) {
        let image_url = await get_image_url(entry.id, 400)

        let top_artists = document.querySelector(`${card_id} > .big-list`)

        top_artists.insertAdjacentHTML("beforeend",
            `<div class="${index === 0 ? 'big-list-first' : 'big-list-item'}">
                <img src="${image_url}" alt="Artist Cover">
                <p>${entry.name}</p>
            </div>`
        )
    }
}

try {
    let response = await api.get_most_played_artists(6)
    let artists = await response.json()

    let [artist_banner_name, artist_banner_img] = get_banner()
    artist_banner_name.textContent = artists[0].name
    artist_banner_img.src = await get_image_url(artists[0].id, 400)
    fill_card(artists, "#top-artists")

    response = await api.get_most_played_albums(6)
    let albums = await response.json()
    fill_card(albums, "#top-albums")

    response = await api.get_most_played_tracks(6)
    let tracks = await response.json()
    fill_card(tracks, "#top-tracks")
} catch (error) {
    console.error(error)
}
