export const filters = (module) => {
  let filter_dict = {
    grayscale: function () {
      return module.grayscale(rust_image);
    },
    offset_red: function () {
      return module.offset(rust_image, 0, 15);
    },
    offset_blue: function () {
      return module.offset(rust_image, 1, 15);
    },
    offset_green: function () {
      return module.offset(rust_image, 2, 15);
    },
    primary: function () {
      return module.primary(rust_image);
    },
    solarize: function () {
      return module.solarize(rust_image);
    },
    threshold: function () {
      return module.threshold(rust_image, 200);
    },
    sepia: function () {
      return module.sepia(rust_image);
    },
    decompose_min: function () {
      return module.decompose_min(rust_image);
    },
    decompose_max: function () {
      return module.decompose_max(rust_image);
    },
    grayscale_shades: function () {
      return module.grayscale_shades(rust_image);
    },
    red_channel_grayscale: function () {
      module.single_channel_grayscale(rust_image, 0);
    },
    green_channel_grayscale: function () {
      module.single_channel_grayscale(rust_image, 1);
    },
    blue_channel_grayscale: function () {
      module.single_channel_grayscale(rust_image, 2);
    },
    hue_rotate_hsl: function () {
      module.hue_rotate_hsl(rust_image, 0.3);
    },
    hue_rotate_hsv: function () {
      module.hue_rotate_hsv(rust_image, 0.3);
    },
    hue_rotate_lch: function () {
      module.hue_rotate_lch(rust_image, 0.3);
    },
    lighten_hsl: function () {
      module.lighten_hsl(rust_image, 0.3);
    },
    lighten_hsv: function () {
      module.lighten_hsv(rust_image, 0.3);
    },
    lighten_lch: function () {
      module.lighten_lch(rust_image, 0.3);
    },
    darken_hsl: function () {
      module.darken_hsl(rust_image, 0.3);
    },
    darken_hsv: function () {
      module.darken_hsv(rust_image, 0.3);
    },
    darken_lch: function () {
      module.darken_lch(rust_image, 0.3);
    },
    desaturate_hsl: function () {
      module.desaturate_hsl(rust_image, 0.3);
    },
    desaturate_hsv: function () {
      module.desaturate_hsv(rust_image, 0.3);
    },
    desaturate_lch: function () {
      module.desaturate_lch(rust_image, 0.3);
    },
    saturate_hsl: function () {
      module.saturate_hsl(rust_image, 0.3);
    },
    saturate_hsv: function () {
      module.saturate_hsv(rust_image, 0.3);
    },
    saturate_lch: function () {
      module.saturate_lch(rust_image, 0.3);
    },
    inc_red_channel: function () {
      return module.alter_red_channel(rust_image, 120);
    },
    inc_blue_channel: function () {
      return module.alter_channel(rust_image, 2, 100);
    },
    inc_green_channel: function () {
      return module.alter_channel(rust_image, 1, 100);
    },
    inc_two_channels: function () {
      return module.alter_channel(rust_image, 1, 30);
    },
    dec_red_channel: function () {
      return module.alter_channel(rust_image, 0, -30);
    },
    dec_blue_channel: function () {
      return module.alter_channel(rust_image, 2, -30);
    },
    dec_green_channel: function () {
      return module.alter_channel(rust_image, 1, -30);
    },
    swap_rg_channels: function () {
      return module.swap_channels(rust_image, 0, 1);
    },
    swap_rb_channels: function () {
      return module.swap_channels(rust_image, 0, 2);
    },
    swap_gb_channels: function () {
      return module.swap_channels(rust_image, 1, 2);
    },
    remove_red_channel: function () {
      return module.remove_red_channel(rust_image, 250);
    },
    remove_green_channel: function () {
      return module.remove_green_channel(rust_image, 250);
    },
    remove_blue_channel: function () {
      return module.remove_blue_channel(rust_image, 250);
    },
    emboss: function () {
      return module.emboss(rust_image);
    },
    box_blur: function () {
      return module.box_blur(rust_image);
    },
    sharpen: function () {
      return module.sharpen(rust_image);
    },
    lix: function () {
      return module.lix(rust_image);
    },
    neue: function () {
      return module.neue(rust_image);
    },
    ryo: function () {
      return module.ryo(rust_image);
    },
    gaussian_blur: function () {
      return module.gaussian_blur(rust_image);
    },
    inc_brightness: function () {
      return module.inc_brightness(rust_image, 20);
    },
    inc_lum: function () {
      return module.inc_luminosity(rust_image);
    },
    grayscale_human_corrected: function () {
      return module.grayscale_human_corrected(rust_image);
    },
    blend: function () {
      return module.blend(rust_image, rust_image2, "over");
    },
    overlay: function () {
      return module.blend(rust_image, rust_image2, "overlay");
    },
    atop: function () {
      return module.blend(rust_image, rust_image2, "atop");
    },
    xor: function () {
      return module.blend(rust_image, rust_image2, "xor");
    },
    plus: function () {
      return module.blend(rust_image, rust_image2, "plus");
    },
    multiply: function () {
      return module.blend(rust_image, rust_image2, "multiply");
    },
    burn: function () {
      return module.blend(rust_image, rust_image2, "burn");
    },
    difference: function () {
      return module.blend(rust_image, rust_image2, "difference");
    },
    soft_light: function () {
      return module.blend(rust_image, rust_image2, "soft_light");
    },
    hard_light: function () {
      return module.blend(rust_image, rust_image2, "hard_light");
    },
    dodge: function () {
      return module.blend(rust_image, rust_image2, "dodge");
    },
    exclusion: function () {
      return module.blend(rust_image, rust_image2, "exclusion");
    },
    lighten: function () {
      return module.blend(rust_image, rust_image2, "lighten");
    },
    darken: function () {
      return module.blend(rust_image, rust_image2, "darken");
    },
    watermark: function () {
      return module.watermark(rust_image, watermark_img, 10, 30);
    },
    text: function () {
      return module.draw_text(rust_image, "welcome to WebAssembly", 10, 20);
    },
    text_border: function () {
      return module.draw_text_with_border(
        rust_image,
        "welcome to the edge",
        10,
        20,
      );
    },
    test: function () {
      return module.filter(rust_image, "rosetint");
    },
    pink_noise: function () {
      return module.pink_noise(rust_image);
    },
    add_noise_rand: function () {
      return module.add_noise_rand(rust_image);
    },
    blend: function () {
      return module.blend(rust_image, rust_image2, "over");
    },
    overlay: function () {
      return module.blend(rust_image, rust_image2, "overlay");
    },
    atop: function () {
      return module.blend(rust_image, rust_image2, "atop");
    },
    plus: function () {
      return module.blend(rust_image, rust_image2, "plus");
    },
    multiply: function () {
      return module.blend(rust_image, rust_image2, "multiply");
    },
    burn: function () {
      return module.blend(rust_image, rust_image2, "burn");
    },
    difference: function () {
      return module.blend(rust_image, rust_image2, "difference");
    },
    soft_light: function () {
      return module.blend(rust_image, rust_image2, "soft_light");
    },
    hard_light: function () {
      return module.blend(rust_image, rust_image2, "hard_light");
    },
    dodge: function () {
      return module.blend(rust_image, rust_image2, "dodge");
    },
    exclusion: function () {
      return module.blend(rust_image, rust_image2, "exclusion");
    },
    lighten: function () {
      return module.blend(rust_image, rust_image2, "lighten");
    },
    darken: function () {
      return module.blend(rust_image, rust_image2, "darken");
    },
    watermark: function () {
      return module.watermark(rust_image, watermark_img, 10, 30);
    },
    text: function () {
      return module.draw_text(rust_image, "welcome to WebAssembly", 10, 20);
    },
    text_border: function () {
      return module.draw_text_with_border(
        rust_image,
        "welcome to the edge",
        10,
        20,
      );
    },
    blend: function () {
      return module.blend(rust_image, rust_image2, "over");
    },
    overlay: function () {
      return module.blend(rust_image, rust_image2, "overlay");
    },
    atop: function () {
      return module.blend(rust_image, rust_image2, "atop");
    },
    plus: function () {
      return module.blend(rust_image, rust_image2, "plus");
    },
    multiply: function () {
      return module.blend(rust_image, rust_image2, "multiply");
    },
    burn: function () {
      return module.blend(rust_image, rust_image2, "burn");
    },
    difference: function () {
      return module.blend(rust_image, rust_image2, "difference");
    },
    soft_light: function () {
      return module.blend(rust_image, rust_image2, "soft_light");
    },
    hard_light: function () {
      return module.blend(rust_image, rust_image2, "hard_light");
    },
    dodge: function () {
      return module.blend(rust_image, rust_image2, "dodge");
    },
    exclusion: function () {
      return module.blend(rust_image, rust_image2, "exclusion");
    },
    lighten: function () {
      return module.blend(rust_image, rust_image2, "lighten");
    },
    darken: function () {
      return module.blend(rust_image, rust_image2, "darken");
    },
    watermark: function () {
      return module.watermark(rust_image, watermark_img, 10, 30);
    },
    text: function () {
      return module.draw_text(rust_image, "welcome to WebAssembly", 10, 20);
    },
    text_border: function () {
      return module.draw_text_with_border(
        rust_image,
        "welcome to the edge",
        10,
        20,
      );
    },
  };
  return filter_dict;
};
