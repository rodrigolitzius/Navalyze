import { Api } from "./api.js"
import { setup_timezone_select } from "./util.js"

const api = new Api()

let server_settings = await api.get_server_settings()
server_settings = await server_settings.json()

let navidrome_url_field = document.getElementById("login-form").querySelector(".navidrome-url-field")

navidrome_url_field.hidden = server_settings.static_navidrome_url
navidrome_url_field.querySelector("input").required = !server_settings.static_navidrome_url

document.querySelector(".login-card").hidden = false

function get_form() {
    const form = document.querySelector("#login-form")
    const btn = document.querySelector("#login-btn")

    return [form, btn]
}

function fill_default_timezone(timezone_input) {
    timezone_input.value = Intl.DateTimeFormat().resolvedOptions().timeZone
}

const [form, btn] = get_form()

let timezone_input = document.querySelector("#timezone")
setup_timezone_select(timezone_input)

form.addEventListener("submit", async function (event) {
    event.preventDefault()

    const url = document.querySelector("#navidrome-url").value
    const username = document.querySelector("#username").value
    const password = document.querySelector("#password").value
    const timezone = document.querySelector("#timezone").value

    if (!(url || server_settings.static_navidrome_url) || !username || !password || !timezone) {
        alert("Fill all fields")
        return
    }

    const original_text = btn.textContent
    btn.textContent = "Working..."
    btn.disabled = true

    try {
        let response = await api.login(username, password, url)

        if (!response.ok) {
            let error_data = await response.json().catch(() => ({}))
            throw new Error(error_data.error || `HTTP Error: ${response.status}`)
        }

        let data = await response.json()

        if (data.id) {
            localStorage.setItem("token", data.id)
            localStorage.setItem("timezone", timezone)
            window.location.replace("index.html")
        } else {
            alert("Login failed: token not received")
        }
    } catch (error) {
        alert(error.message)
    } finally {
        btn.textContent = original_text
        btn.disabled = false
    }
})
