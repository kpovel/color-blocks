/** @type {HTMLDivElement} */
const canvas = document.getElementById("canvas");
/** @type {HTMLDivElement} */
const colors = document.getElementById("colors");
/** @type {null | string} */
let selectedColorId = null;

colors.addEventListener("click", (e) => {
  selectedColorId = e.target.id;
});

const ws = new WebSocket(`ws:${window.location.host}/ws`);

ws.addEventListener("open", () => {
  ws.addEventListener("message", (m) => {
    /** @type {string} */
    const data = m.data;
    const [y, x, color] = data.split(":");

    /** @type {HTMLDivElement} */
    const target = document.getElementById(`${y}:${x}`);
    target.style.backgroundColor = color;

  });

  canvas.addEventListener("click", (e) => {
    const positionId = e.target.id;

    if (selectedColorId) {
      ws.send(`${positionId}:${selectedColorId}`);
    }
  });
});
