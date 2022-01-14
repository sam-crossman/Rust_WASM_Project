
function draw(state){
    const canvas = document.getElementById("main_canvas");
    const context = canvas.getContext("2d");
    // context.fillStyle = "blue";
    // context.fillRect(0, 0, 50, 50);

    context.strokeStyle = "grey"
    context.lineWidth  = 1;
    const image = state.image;
    const width = image.width();
    const height = image.height();
    const cell_size = image.cell_size();
    const cells = image.cells();
    for (let x = 0; x < width; x++){
        for (let y = 0; y < height; y++){
            const index = (y * width + x) * 3;
            const color = `rgb(${cells[index + 0]}, ${cells[index + 1]}, ${cells[index + 2]})`;
            context.fillStyle = color;
            context.fillRect(x * cell_size, y * cell_size, cell_size, cell_size);
        }
    }
    for (let x = 0; x <= width; x++) {
        context.beginPath();
        context.moveTo(x * cell_size + .5, 0);
        context.lineTo(x * cell_size, height * cell_size);
        context.stroke();

    }
    for (let y = 0; y <= height; y++) {
        context.beginPath();
        context.moveTo(0, y * cell_size + .5);
        context.lineTo(width * cell_size, y * cell_size);
        context.stroke();
    }
}
function setupCanvas(state) {
    const image = state.image;
    const canvas = document.getElementById("main_canvas");
    const cell_size = image.cell_size();
    canvas.addEventListener("click", (event) => {
        const rect = canvas.getBoundingClientRect();
        let x = event.clientX - rect.left;
        let y = event.clientY - rect.top;
        x = Math.floor(x/cell_size);
        y = Math.floor(y/cell_size);
        image.add_particle(x,y,state.type);
        image.update_color();
        draw(state)
    });
    canvas.addEventListener("mousemove", (event) => {
        if (!state.drag) return;
        const rect = canvas.getBoundingClientRect();
        let x = event.clientX - rect.left;
        let y = event.clientY - rect.top;
        x = Math.floor(x/cell_size);
        y = Math.floor(y/cell_size);
        image.add_particle(x,y,state.type);
        image.update_color();
        draw(state)
    });
    canvas.addEventListener("mousedown", event => {
        state.drag = true;
    });
    canvas.addEventListener("mouseup", event => {
        state.drag = false;
    });
    document.getElementById("sand_button").addEventListener("click",  (event) => {
        state.type = "sand";
    });
    document.getElementById("water_button").addEventListener("click",  (event) => {
        state.type = "water";
    });
    var i = 0;
    //const counter = document.getElementById("counter");
    var intervalId = window.setInterval(function(){
        // counter.textContent = i.toString();
        // i++;
        image.update_particle();
        image.update_color();
        draw(state);
    }, 100);
}
async function main(){
    const lib = await import("../pkg/index.js").catch(console.error);
    const image = new lib.Image(50,50,10);
    const state = {
        image, 
        type: "sand",
        drag: false
    }
    draw(state);
    setupCanvas(state);
}
main();
