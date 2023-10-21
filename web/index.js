let card_list = document.getElementById("card-list");

function add_card(title, content) {
  card_list.innerHTML+= 
  `<li class="pure-menu-item pure-menu-link card ">
    <div class="card-title">
      ${title}
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
    add_card(post.title, post.content)
  }
}

refresh_posts()
