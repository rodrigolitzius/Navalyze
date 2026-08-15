import {Api} from "./api.js"

const api = new Api()

function get_form() {
    const form = document.querySelector("#login-form")
    const btn = document.querySelector("#login-btn")

    return [form, btn]
}

function fill_default_timezone() {
    const timezone_input = document.querySelector("#timezone")
    timezone_input.value = Intl.DateTimeFormat().resolvedOptions().timeZone
}

const [form, btn] = get_form()

fill_default_timezone()

form.addEventListener("submit", async function (event) {
    event.preventDefault()

    const url = document.querySelector("#navidrome-url").value
    const username = document.querySelector("#username").value
    const password = document.querySelector("#password").value
    const timezone = document.querySelector("#timezone").value

    if (!url || !username || !password || !timezone) {
        alert("Preencha todos os campos")
        return
    }

    const original_text = btn.textContent
    btn.textContent = "Entrando..."
    btn.disabled = true

    try {
        let response = await api.login(username, password, url)

        if (!response.ok) {
            let error_data = await response.json().catch(() => ({}))
            throw new Error(error_data.error || `Erro HTTP ${response.status}`)
        }

        let data = await response.json()

        if (data.id) {
            localStorage.setItem("token", data.id)
            localStorage.setItem("navidrome-url", url)
            localStorage.setItem("timezone", timezone)
            window.location.replace("index.html")
        } else {
            alert("Login falhou: token não recebido")
        }
    } catch (error) {
        alert(error.message)
    } finally {
        btn.textContent = original_text
        btn.disabled = false
    }
})