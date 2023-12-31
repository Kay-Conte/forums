let card_list = document.getElementById("card-list");
let new_post_button = document.getElementById("new-post");

let id = window.location.href.split("/").pop();

function add_card(id, title, content) {
  let c = "";

  for (let row of content.split("\n")) {
    c += `<p>${row}</p>`
  }

  card_list.innerHTML+= 
  `<li class="pure-menu-item pure-menu-link card ">
    <div class="card-title">
      <a href='/post/${id}'>
        ${title}
      </a>
    </div>
    <div class="card-content">
      ${c}
    </div>
  </li>`;
}

async function refresh_posts() {
  let response = await fetch(`/api/getPosts/${id}`)
  
  let json = await response.json();

  for (let post of json) {
    add_card(post.id, post.title, post.content)
  }
}

refresh_posts()

new_post_button.addEventListener("click", () => {
  window.location.href = `/createPost/${id}`;
})
