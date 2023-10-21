let card_list = document.getElementById("card-list");

function add_card(id, title, content) {
  card_list.innerHTML+= 
  `<li class="pure-menu-item pure-menu-link card ">
    <div class="card-title">
      <a href='/post/${id}'>
        ${title}
      </a>
    </div>
    <div class="card-content">
      ${content}
    </div>
  </li>`
}

async function refresh_posts() {
  let response = await fetch("/api/getPosts/50")
  
  let json = await response.json();

  for (let post of json) {
    add_card(post.id, post.title, post.content)
  }
}

refresh_posts()
