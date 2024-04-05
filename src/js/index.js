import MainImage from "./nine_yards.jpg";
import "../css/styles.css";

let canvas, ctx, blur_factor;

import("../../webclient/pkg").then((module) => {
  // Setup images
  const sourceImage = new Image();
  sourceImage.src = MainImage;
  sourceImage.onload = () => {
    setUpCanvas();
  };

  const slider = document.getElementById("slider");
  const progress = document.getElementById("progress");
  slider.onchange = filterImage;
  slider.value = 0;

  function filterImage(event) {
    ctx.drawImage(sourceImage, 0, 0);
    let sliderValue = parseInt(event.target.value);
    blur_factor = sliderValue / 5;
    let rust_image = module.open_image(canvas, ctx);

    // module.gaussian_blur(rust_image, blur_factor);
    // module.median(rust_image, blur_factor, blur_factor);

    module.spiegel(rust_image, blur_factor);
    module.putImageData(canvas, ctx, rust_image);
    const image_container = document.getElementById("image_container");
    let rect = image_container.getBoundingClientRect();
    canvas.setAttribute(
      "style",
      `visibility:visible;position:absolute;top:${rect.top};z-index:100`,
    );
    // image_container.setAttribute("style", "visibility:hidden");
  }

  function setUpCanvas() {
    let element = document.getElementById("image_container");
    element.appendChild(sourceImage);

    canvas = document.getElementById("canvas");
    canvas.width = sourceImage.width;
    canvas.height = sourceImage.height;
    sourceImage.setAttribute("style", "width:50vw");

    ctx = canvas.getContext("2d");
    ctx.drawImage(sourceImage, 0, 0);
  }
});
