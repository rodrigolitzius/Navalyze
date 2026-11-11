import { Api } from "./api.js"
import { build_content_stats, album_link, build_round_list} from "./html.js";
import { format_hours, build_time_graph, load_images } from "./util.js";

const api = new Api()

let artist_id = new URLSearchParams(window.location.search).get("id");

let artist = await api.get_artist(artist_id)
artist = await artist.json()

let header = document.getElementById("img-header")
load_images(api, [{ id: artist_id, size: 700, container: header }])

let content = header.querySelector(".content")

content.querySelector(".title").textContent = artist.name

build_round_list(content.querySelector(".properties"), [
    `${artist.album_count} albums`,
    artist.artist_type ? `${artist.artist_type}` : null,
    artist.gender ? `${artist.gender}` : null
])

build_content_stats(content.querySelector(".content-stats"), [
    { label: "Plays", value: `${artist.plays}` },
    { label: "Time played", value: format_hours(artist.played_hours) },
])

let albums = document.getElementById("albums-list")
let images = new Array()

for (const album of artist.albums) {
    albums.insertAdjacentHTML("beforeend",
        `<a href="${album_link(album.id)}" class="album-entry">
            <div class="card album-entry">
                <div class="img-container">
                    <img class="card-img">
                </div>
                <p class="card-name">${album.name}</p>
                <p class="card-footer" title="${album.plays} plays">${format_hours(album.played_hours)}</p>
            </div>
        </a>`
    )

    images.push({id: album.id, size: 400, container: albums.lastElementChild})
}

load_images(api, images)

let artist_activity = await api.get_artist_graph(artist_id, 50)
artist_activity = await artist_activity.json()

build_time_graph(document.getElementById("graph-artist"), {[artist.name]: artist_activity})
