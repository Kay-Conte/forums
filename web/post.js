let post_container = document.getElementById("post-container");

async function load_post() {
  let id = window.location.href.split("/").pop();

  let response = await fetch(`/api/getPost/${id}`);

  let post = await response.json();

  post_container.innerHTML += 
   `<div class="card-title">${post.title}</div>
    <br></br>
    <div class="card-content"></div>`;

  let div = post_container.querySelector(".card-content");

  for (let row of post.content.split("\n")) {
    div.innerHTML += `<p>${row}</p>`
  }
}

load_post()
