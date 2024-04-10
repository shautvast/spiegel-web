**Spiegel** image filter project

- rust/webassembly for image processing
- vanilla javascript for the user interface
- no server (just static pages)

Live demo at https://shautvast.github.io/spiegel-demo/

* Sorry for the poor performance, especially on larger images.
* It uses the median image filter from image.rs. That in itself can be pretty slow.
(Although the implementation uses a _sliding window histogram_, which I think is pretty nifty).
* And on top of that, I created this custom flood fill algorithm, 
that instead of filling it with with a single color, looks up a sample from the 
Spiegel book (that has a corresponding color) and takes the pixels from that. 

sample output
![sample](https://github.com/shautvast/spiegel-web/blob/main/unsplash.png)
