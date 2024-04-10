import "../css/styles.css";
import spiegel from '../img/spieghel.jpg';

let canvas, ctx, originalWidth, originalHeight, canvasTop, strokeSize=1;

import("../../webclient/pkg").then((module) => {
    const slider = document.querySelector("#slider");
    slider.onchange = (event) => {
        strokeSize = parseInt(event.target.value) / 5;
        filterImage(event, true);
    };
    slider.value = 0;
    const applyButton = document.querySelector("#apply");

    applyButton.onclick = (event) => {
        filterImage(event, false);
    };

    function filterImage(event, preview) {
        ctx.drawImage(sourceImage, 0, 0);

        let rust_image = module.open_image(canvas, ctx);
        const out= module.spiegel(rust_image, strokeSize, preview);

        module.putImageData(canvas, ctx, out);
        canvas.setAttribute(
            "style",
            `visibility:visible;position:absolute;top:${canvasTop}px`,
        );
    }


});
document.querySelector("#spieghel").src = spiegel;
const sourceImage = new Image();
sourceImage.id = "source-image";
let element = document.querySelector("#image-container");
element.appendChild(sourceImage);
sourceImage.onload = () => {
    setUpCanvas()
};

function setUpCanvas() {
    canvas = document.querySelector("#canvas");
    originalWidth = sourceImage.width;
    originalHeight = sourceImage.height;

    canvas.width = originalWidth;
    canvas.height = originalHeight;
    sourceImage.setAttribute("style", "width:50vw");
    const imageContainer = document.querySelector("#image-container");
    const rect = imageContainer.getBoundingClientRect();
    canvasTop = rect.top;
    ctx = canvas.getContext("2d");
}

document.querySelector('#upload').addEventListener('change', function (e) {
    let file = e.target.files[0];
    if (file.type.match('image.*')) {
        let reader = new FileReader();
        reader.readAsDataURL(file);
        reader.onload = function (e) {
            const image = document.querySelector('#source-image');
            image.src = reader.result;
            document.querySelector("#image-container").setAttribute("class", "no-border");
            document.querySelector("#upload").setAttribute("class", "hide");
            document.querySelector("#slidecontainer").setAttribute("class", "slidecontainer");
        }
    } else {
        alert("Uploaded file is not an image. Please upload an image file.");
    }
});

function allowDrop(ev) {
    ev.preventDefault();
}

function drag(ev) {
    ev.dataTransfer.setData("text", ev.target.id);
}

function drop(event) {
    event.preventDefault();
    let dt = event.dataTransfer;

    console.log(dt.files);

    document.querySelector("#source-image").src = URL.createObjectURL(dt.files[0]);
    document.querySelector('#image-container').setAttribute("class", "no-border");
    document.querySelector("#upload").setAttribute("class", "hide");
    document.querySelector("#slidecontainer").setAttribute("class", "show");
}