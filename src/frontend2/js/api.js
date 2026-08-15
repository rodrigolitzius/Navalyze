class Api {
    new_request(endpoint) {
        const token = localStorage.getItem("token")

        var request = new Request("/api/" + endpoint, {
            headers: { "Authorization": token }
        })

        return request
    }

    get_cover_art(id, size) {
        var request = this.new_request(`relay/getCoverArt?id=${id}&size=${size}`)

        return fetch(request)
    }

    get_most_played_artists(limit) {
        var request = this.new_request(`most-played/artists?limit=${limit}`)

        return fetch(request)
    }

    get_most_played_albums(limit) {
        var request = this.new_request(`most-played/albums?limit=${limit}`)

        return fetch(request)
    }

    get_most_played_tracks(limit) {
        var request = this.new_request(`most-played/tracks?limit=${limit}`)

        return fetch(request)
    }

    login(username, password, url) {
        var request = new Request("/api/login", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ username: username, password: password, url: url })
        })

        return fetch(request)
    }
}

export {Api}