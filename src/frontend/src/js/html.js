function artist_link(id) {
    return `artist.html?id=${id}`
}

function album_link(id) {
    return `album.html?id=${id}`
}

function track_link(id) {
    return `track.html?id=${id}`
}

function playlist_link(id) {
    return `playlist.html?id=${id}`
}

function build_link_list(element, text, link) {
    element.insertAdjacentHTML("beforeend",
        `<a href="${link}">${text}</a><p>,</p>`
    )
}

function build_round_list(element, entries) {
    for (const entry of entries) {
        if (!entry) { continue; }

        element.insertAdjacentHTML("beforeend",
            `<span class="round-list-item">${entry}</span>`

        )
    }
}

function build_content_stats(element, entries) {
    for (const entry of entries) {
        element.insertAdjacentHTML("beforeend",
            `<div class="content-stats-card">
                <p class="content-stats-label">${entry.label}</p>
                <p class="content-stats-value">${entry.value}</p>
            </div>`
        )
    }
}

export {build_link_list, build_content_stats, build_round_list, artist_link, album_link, track_link, playlist_link}
