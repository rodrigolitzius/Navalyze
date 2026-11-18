import { Api } from "./api.js"
import { build_link_list, build_content_stats, build_configurable_graph, build_round_list, artist_link, track_link, GraphDataSource } from "./html.js";
import { format_hours, load_images } from "./util.js";

const api = new Api()

let album_id = new URLSearchParams(window.location.search).get("id");

let album = await api.get_album(album_id)
album = await album.json()

document.title = album.name

let header = document.getElementById("img-header")
load_images(api, [{id: album_id, size: 700, container: header}])

let content = header.querySelector(".content")

content.querySelector(".title").textContent = album.name

for (const artist of album.artists) {
    build_link_list(content.querySelector(".artists"), artist.name, artist_link(artist.id))
}

build_round_list(content.querySelector(".properties"), [
    album.year ? `${album.year}` : null,
])

build_content_stats(content.querySelector(".content-stats"), [
    { label: "Plays", value: `${album.plays}` },
    { label: "Time played", value: format_hours(album.played_hours) },
])

let max = Math.max(...album.tracks.map(item => item.played_hours))

let track_list = document.getElementById("track-list")

for (const track of album.tracks) {
    track_list.insertAdjacentHTML("beforeend",
        `<div class="track">
            <div class="left">
                <a class="track-title" href="${track_link(track.id)}">${track.name}</a>
                <p class="track-time" title="${track.plays} plays">${format_hours(track.played_hours)}</p>
            </div>
            <span class="track-bar"</span>
        </div>`
    )

    track_list.querySelector(".track:last-of-type").style.setProperty("--ratio", track.played_hours/max)
}

let graph_element = document.getElementById("graph")

build_configurable_graph(
    graph_element.querySelector("canvas"),
    graph_element.querySelector(".graph-type-selection"),
    new class extends GraphDataSource {
        async daily()    { return {[album.name]: await api.get_album_graph(album_id, 48, "daily").then((v) => v.json())} }
        async weekly()   { return {[album.name]: await api.get_album_graph(album_id, 7, "weekly").then((v) => v.json())} }
        async all_time() { return {[album.name]: await api.get_album_graph(album_id, 50, "all_time").then((v) => v.json())} }
    }
)
