const useWangdenticonModule = (module) => {
  const canvas = document.getElementById("canvas");
  const ctx = canvas.getContext("2d");
  const image = new Image();
  image.onload = () => {
    ctx.drawImage(image, 0, 0);
  };
  const nameTextBox = document.getElementById("name-textbox");
  const gridSizeSelector = document.getElementById("grid-size");
  const invertToggle = document.getElementById("invert?");
  const generateButton = document.getElementById("generate");

  const renderWangdenticon = () => {
    const name = nameTextBox.value;
    const imageStr = module.generate(
      name,
      gridSizeSelector.value * 1,
      invertToggle.checked,
      255
    );
    image.src = `data:image/png;base64,${imageStr}`;
  };

  generateButton.addEventListener("click", renderWangdenticon);
};

import("../pkg/index.js").then(useWangdenticonModule).catch(console.error);
