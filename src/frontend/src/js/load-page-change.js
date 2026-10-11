const PAGE_SIZE = 20

function load_nav_buttons(parent_element, page_url) {
    console.log(parent_element)
    let page = get_page()

    parent_element.querySelector("#next-page").addEventListener("click", function() {
        window.location.replace(`${page_url}?page=${page+1}`)
    });

    parent_element.querySelector("#previous-page").addEventListener("click", function () {
        let next = page-1
        if (page <= 0) {
            next = 0
        }
        window.location.replace(`${page_url}?page=${next}`)
    });
}

function get_page() {
    let page = new URLSearchParams(window.location.search).get("page");
    page = page ? parseInt(page, 10) : 0
    if (Number.isNaN(page) || page < 0) page = 0

    return page
  }

export { load_nav_buttons, get_page, PAGE_SIZE}
