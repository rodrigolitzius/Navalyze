import { Api, get_wrapped_url } from "./api";

let api = new Api()

try {
    let templates = await api.get_templates()

    if (!templates.ok) {
        let error_msg = await templates.json()
        throw new Error(error_msg.error)
    }

    templates = await templates.json()

    document.getElementById("loaded").hidden = false
    document.getElementById("loading").hidden = true

    let templates_select = document.getElementById("templates")

    for (const template of templates) {
        templates_select.insertAdjacentHTML("beforeend",
            `<option class="${template.slug}">${template.name}</option>`
        )
    }

    document.getElementById("generate").addEventListener("click", async () => {
        wrapped = document.getElementById("wrapped")

        wrapped.querySelector(".placeholder-img h1").textContent = "Loading..."

        wrapped.querySelector(".placeholder-img").hidden = false
        wrapped.querySelector("img").hidden = true

        wrapped.querySelector("img").src = await get_wrapped_url(api,
            templates_select.selectedOptions[0].classList
        )

        wrapped.querySelector(".placeholder-img").hidden = true
        wrapped.querySelector("img").hidden = false
    })
} catch (error) {
    alert(`Error when fetching templates. The API might be down.\n\nHere's the full error: ${error.message}`)
    window.location.replace("dashboard.html")
}
