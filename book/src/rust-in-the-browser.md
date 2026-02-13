# Unit 5.3 - Rust in the Browser

## Exercise 5.3.1: Lettuce Crop WebAssembly
In exercise 5.1.1, we build a web server that hosts an image cropping service. But do we really need to do this cropping on our server? Wouldn't it be much more privacy-friendly if we could do the image cropping in the user's browser instead of uploading images to our external server?

In this exercise, we will create a new version of our Lettuce Crop website that crops images with [WebAssembly](https://webassembly.org/). WebAssembly allows you to run compiled code in a safe sandboxed environment in the browser. This means we will not need a dedicated server anymore, as the website will only consist of static files which we can be hosted using any HTTP server. You could even host it for free using [GitHub pages](https://pages.github.com/)!

### 5.3.1.A Building with Wasm Pack
In `exercises/5-rust-for-web/3-rust-in-the-browser/1-lettuce-crop-wasm` we have set up a basic WebAssembly project. As you can see in the `Cargo.toml`, the project has been configured as a dynamic library (`"cdylib"`). We've also added the `wasm-bindgen` crate as a dependency, which is used to generate WebAssembly bindings.

To build the project, we will use `wasm-pack`. First, install `wasm-pack` with:
```
cargo install wasm-pack
```
Then, build the project with `wasm-pack`. Since we want to use it in the browser, we set the [wasm-pack target](https://rustwasm.github.io/docs/wasm-pack/commands/build.html#target) to `web`, and we tell it to put the generate files in the `assets/pkg` folder:
```
wasm-pack build --target web --out-dir assets/pkg
```

Now, a bunch of files should appear in the `assets/pkg` folder:
- A `.wasm` file, which contains the compiled WebAssembly code
- Some `.d.ts` files, which describe the TypeScript types of the generated bindings
- A `.js` file, which contains the JavaScript bindings for our WebAssembly binary

### 5.3.1.B Interacting with JavaScript
So what functionality does the compiled WebAssembly currently include? In `lib.rs` you can see two functions: an extern `alert()` function, and a `hello()` function. Both of these functions have been annotated with `#[wasm_bindgen]` to indicate that we want to bind them with WebAssembly. Extern functions will be bound to existing JavaScript methods, in this case the [window's `alert()` function](https://developer.mozilla.org/en-US/docs/Web/API/Window/alert) which shows a popup dialog.

Let's add the WebAssembly to our website. Add the following JavaScript in the `<body>` of the `index.html` to load the WebAssembly binary and call our `hello()` function when we press the submit button:
```html
<script type="module">
    import init, { hello } from "./pkg/lettuce_crop_wasm.js";
    init().then(() => {
        const submit_button = document.querySelector('input[type="submit"]');
        submit_button.onclick = () => {
            hello("WebAssembly");
        }
    });
</script>
```

To try out the website, you can use any HTTP server that is able to serve local files. You could use `axum` to host the files like we did in exercise 5.1.1, but you can also use for example `npx http-server` if you have `npm` installed.

### 5.3.1.C Cropping images
Let's add a `crop_image(bytes: Vec<u8>, max_size: u32) -> Vec<u8>` function to our Rust library that will crop our images. You can use the same logic as in exercise 5.1.1 (part D and E) to create a `DynamicImage` from the input bytes, crop it, and export it as WebP. Mark the function with `#[wasm_bindgen]` and rebuild the library to generate WebAssembly bindings for it.

If you look at the generated JavaScript bindings, you will see that the `Vec<u8>`s for the `crop_image` function have been turned into `Uint8Array`s. We will need to write some JavaScript to read the user's selected image and give it to our `crop_image` as a `Uint8Array`.

First, let's grab our other two input elements:
```js
const max_size = document.querySelector('input[name="max_size"]');
const image = document.querySelector('input[name="image"]');
```
Then, in the `onclick` of the submit button, you can grab the selected file using `image.files[0]`. To get the contents of the file, we will use a [`FileReader`](https://developer.mozilla.org/en-US/docs/Web/API/FileReader):
```js
const file = image.files[0];
const reader = new FileReader();
reader.onload = (evt) => {
    const bytes = new Uint8Array(evt.target.result);
    const cropped_bytes = crop_image(bytes, max_size.value); // call our function
    // TODO: do something with the cropped_bytes
};
reader.readAsArrayBuffer(file);
```
Finally, to display the resulting cropped image to the user, we will construct a [`Blob`](https://developer.mozilla.org/en-US/docs/Web/API/Blob) from the `Uint8Array`, and turn this `Blob` into a URL to which we will redirect the user:
```js
window.location.href = URL.createObjectURL(new Blob([cropped_bytes]));
```
If you select an invalid file, you will get an error in the browser console. Feel free to add some better error handling by using a [try-catch](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/try...catch), and by validating whether `image.files[0]` exists before reading it. It would also be nice to verify that `max_size` has a sensible value.

### 5.3.1.D Using the web-sys crate (bonus)
Instead of using JavaScript to interact with the HTML document or manually binding extern JavaScript functions using `#[wasm_bindgen]` like we saw with `alert()`, we can also use the [`web-sys`](https://crates.io/crates/web-sys) crate. This crate provides bindings for the JavaScript web APIs available in the browser. However, most of these APIs have to be manually enabled with individual features.

Add the `web-sys` crate to your project with all the needed features enabled:
```
cargo add web-sys --features "Window,Document,HtmlElement,HtmlImageElement,Blob,Url"
```

Now, instead having the `crop_image` function return an array of bytes, let's have it instead append an image to HTML document:
- First, get the HTML body element:
    ```rust
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    ```
- Then, we can create an HTML image element:
    ```rust
    let img = document.create_element("img").unwrap();
    let img: web_sys::HtmlImageElement = img.dyn_into().unwrap();
    ```
- To set the source of the image, we will again need to create a `Blob` to get a temporary data URL. For this, we first create a JavaScript array:
    ```rust
    let bytes = web_sys::js_sys::Array::new();
    bytes.push(&web_sys::js_sys::Uint8Array::from(&buffer[..]));
    ```
- And then we can create a Blob and create a URL:
    ```rust
    let blob = web_sys::Blob::new_with_u8_array_sequence(&bytes).unwrap();
    let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
    ```
- And finally, we can set the image's source and append the image to the document's body:
    ```rust
    img.set_src(&url);
    body.append_child(&img).unwrap();
    ```
- Remember to also update the JavaScript code in the HTML document accordingly.
