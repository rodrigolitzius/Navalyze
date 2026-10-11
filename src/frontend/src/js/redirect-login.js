import {Api} from "./api.js"

let api = new Api

let is_logged_in = await api.auth_check()

if (!is_logged_in) {
    window.location.replace("login.html")
}
