const PAGE_SIZE = 20

function get_page_size() {
    let page_size_input = document.getElementById("page-size-selector")
    page_size_input.value = localStorage.getItem("page-size") ?? localStorage.setItem("page-size", PAGE_SIZE)

    return parseInt(page_size_input.value, 10)
}

function load_nav_buttons(parent_element, page_url) {
    get_page_size()

    let [start, end] = get_page()

    parent_element.querySelector("#next-page").addEventListener("click", function () {
        let page_size = get_page_size()

        let url = new URL(window.location.href)

        url.searchParams.set("start", end)
        url.searchParams.set("end", end+page_size)

        window.location.replace(url)
    });

    parent_element.querySelector("#previous-page").addEventListener("click", function () {
        let page_size = get_page_size()

        let new_start = Math.max(0, start - page_size)
        let new_end = Math.max(0, start)
        if (new_end <= new_start) {
            new_end = new_start + page_size
        }
        if ((new_end - new_start) < page_size) {
            new_end += page_size - (new_end - new_start)
        }

        let url = new URL(window.location.href)

        url.searchParams.set("start", new_start)
        url.searchParams.set("end", new_end)

        window.location.replace(url)
    });

    parent_element.querySelector("#page-size-selector").addEventListener("input", function () {
        localStorage.setItem("page-size", this.value)
    })
}

function get_page() {
    let start = new URLSearchParams(window.location.search).get("start");
    let end = new URLSearchParams(window.location.search).get("end");
    start = start ? parseInt(start, 10) : 0
    end = end ? parseInt(end, 10) : get_page_size()

    if (Number.isNaN(start) || start < 0) start = 0
    if (Number.isNaN(end) || end < 0) end = get_page_size()

    return [start, end]
}

export { load_nav_buttons, get_page, PAGE_SIZE }
