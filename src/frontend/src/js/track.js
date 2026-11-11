import { Api } from "./api.js"
import { build_link_list, build_content_stats, artist_link, album_link } from "./html.js"
import { format_hours, build_time_graph, load_images } from "./util.js";

const api = new Api()

let track_id = new URLSearchParams(window.location.search).get("id");

let track_response = await api.get_track(track_id)
let track = await track_response.json()

let header = document.getElementById("img-header")
load_images(api, [{ id: track_id, size: 700, container: header }])

if (Array.isArray(track)) {
    header.insertAdjacentHTML("beforeend",
        `<div class="content">
            <h1>Nothing found</h1>
        </div>`
    )
} else {
    let content = header.querySelector(".content")

    content.querySelector(".title").textContent = track.name

    build_link_list(
        content.querySelector(".album"),
        track.album,
        album_link(track.album_id),
    )

    for (const artist of track.artists) {
        build_link_list(
            content.querySelector(".artists"),
            artist.name,
            artist_link(artist.id),
        )
    }

    build_content_stats(content.querySelector(".content-stats"), [
        { label: "Plays", value: `${track.plays}` },
        { label: "Time played", value: format_hours(track.played_hours) },
    ])
}

let track_activity = await api.get_track_graph(track_id, 50)
track_activity = await track_activity.json()

build_time_graph(document.getElementById("graph-track"), {[track.name]: track_activity})
