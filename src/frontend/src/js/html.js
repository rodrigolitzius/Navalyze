function artist_link(id) {
    return `artist.html?id=${id}`
}

function album_link(id) {
    return `album.html?id=${id}`
}

function track_link(id) {
    return `track.html?id=${id}`
}

function build_link_list(element, text, link) {
    element.insertAdjacentHTML("beforeend",
        `<a href="${link}">${text}</a><p>,</p>`
    )
}

export {build_link_list, artist_link, album_link, track_link}
