let title = document.getElementById("post-title");
let content = document.getElementById("post-content");

let form = document.getElementById("post-form");

form.addEventListener("submit", async (e) => {
  e.preventDefault();

  console.log("submitted");

  await fetch(`/api/createPost`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify({title: title.value, content: content.value})
  })

  window.location.href = "/"
})
