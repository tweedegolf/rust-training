# Unit 5.1 - Rust for Web Servers

<a href="/slides/5_1-rust-for-web/" target="_blank">Slides</a>

## Exercise 5.1.1: Lettuce Crop
In this exercise, we will build a simple web server with [`axum`](https://lib.rs/crates/axum) which allows users to upload images to crop them. You will learn how to serve static HTML pages along with their associated style sheets and images, and you will learn how to handle POST requests with multipart form data to receive the uploaded images.

### 3.1.1.A Hello axum
In `exercises/5-rust-for-web/1-rust-for-web/1-lettuce-crop` we have set up the start of our web server. It currently only serves "Hello, world!" for GET requests on the main page. Run the program and go to [http://[::]:7000/](http://[::]:7000/) in your browser to see if it works.

Note that [http://[::]:7000/](http://[::]:7000/) is the default address for [IPv6](https://en.wikipedia.org/wiki/IPv6). If you do not want to use IPv6, you can use [http://0.0.0.0:7000/](http://0.0.0.0:7000/) instead. The website will also be available on local host, see for example [http://localhost:7000/](http://localhost:7000/), [http://127.0.0.1:7000/](http://127.0.0.1:7000/), or [http://[::1]:7000/](http://[::1]:7000/) (IPv6).

In `main.rs` you can see the `Router` that is used to serve "Hello, world!". We can chain multiple routes to serve multiple end-points. Try adding a second route which serves GET requests on another page (e.g. `/hello`).

### 3.1.1.B Serving static files
Currently, our web server only serves static strings. To serve static HTML documents, CSS style sheets, images and other files, we will use the [`ServeDir`](https://docs.rs/tower-http/latest/tower_http/services/struct.ServeDir.html) file server from `tower_http`. We can add this file server to our router as a fallback service to resolve any request which does not match any other defined route with our file server.

Add a [`fallback_service`](https://docs.rs/axum/latest/axum/routing/struct.Router.html#method.fallback_service) to the router with a `ServeDir` that serves files from the `assets` folder.

If you now go to [http://0.0.0.0:7000/index.html](http://0.0.0.0:7000/index.html) you should see the Lettuce Crop web page with appropriate styling and an image of a lettuce.

By default, `ServeDir` will automatically append `index.html` when requesting a path that leads to a directory. This means that if you remove the "Hello, world!" route for `/` from the router, you will also see the Lettuce Crop page on the main page of the website!

### 3.1.1.C POST requests and dynamic responses
On the Lettuce Crop page we have set up an HTML form, which when submitted sends a POST request to `/crop`:
```html
<form action="/crop" method="post" enctype="multipart/form-data">
```
POST requests are requests that can contain additional data to send to the server. In this case, the form data, consisting of an image and the max size value, will be sent along with the request.

If you select an image and press the crop button, you will be redirected to `/crop`, which currently does not exist. If you open the browser's developer tools (right click > Inspect, or ctrl+shift+i) and go to the network tab, you should see the POST request which currently returns status 405 (Method Not Allowed). The `/crop` route is currently handled by our fallback service, which does not accept POST requests. If you go to [http://0.0.0.0:7000/crop](http://0.0.0.0:7000/crop) directly without using the form, the browser will instead send a regular GET request, which will return status 404 (Not Found).

Let's add a route for `/crop` to our router which will handle the POST requests from the form. You can specify the route in the same way as we did for GET requests, but using `post` instead of `get`.

Instead of returning a static string, we can also use a function to respond to requests. Define the following function and pass it to the `post` method for `/crop`:
```rust
async fn crop_image() -> String {
    format!("Hi! This is still a work in progress. {}", 42)
}
```

### 3.1.1.D Handling uploaded files (multipart form data)
So how do we get the form data from the POST request? With `axum`, we use [extractors](https://docs.rs/axum/latest/axum/extract/) to get information about the request, such as headers, path names or query parameters. Normally, we would use the [`Form` extractor](https://docs.rs/axum/latest/axum/struct.Form.html) to get the submitted form data. However, because we want the user to be able to upload an image, we use the [multipart form data](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/POST#multipart_form_submission) encoding, as specified by the `enctype` in the HTML `form` tag.

To extract multipart form data in `axum`, we use the [`Multipart` extractor](https://docs.rs/axum/latest/axum/extract/struct.Multipart.html). Unlike the `Form` extractor, the `Multipart` extractor does not automatically deserialize the data into a convenient struct. Instead, we will have to manually loop through the fields and deserialize the data we need.

Add `mut multipart: Multipart` as a parameter to our `crop_image` function to extract the multipart form data. Then, use the following loop to print all available fields that were included in the POST request:
```rust
while let Some(field) = multipart.next_field().await.unwrap() {
    let name = field.name().unwrap().to_string();
    let bytes = field.bytes().await.unwrap();
    println!("{name}: {} bytes long", bytes.len());
}
```
Once you submit the form, it should show an `image` field containing the image data and a `max_size` field corresponding to the max size number input field in the form.  

Let's deserialize the two form fields:
- The `image` field consists of the bytes that make up the image. We will use an `ImageReader` from the `image` crate to read the image data:
    ```rust
    ImageReader::new(Cursor::new(bytes)).with_guessed_format().unwrap().decode()
    ```
  This will return a `DynamicImage`, which can be a variety of different image formats. With the `image` crate we will be able to crop and resize this image.

- The `max_size` field contains a number encoded a plain text. You can retrieve the text using `field.text()` instead of `field.bytes()`, and you can parse it into a number using [`.parse()`](https://doc.rust-lang.org/std/string/struct.String.html#method.parse). Let's make it a `u32`.

We will leave it up to you to implement the logic to deserialize these two fields and turn them into a `DynamicImage` and a `u32` that can be used after we're done looping through all the fields.

Change the string returned by `crop_image` to the following to verify that it works:
```rust
format!("Image size: {}x{}\nMax size: {}", image.width(), image.height(), max_size)
```

### 3.1.1.E Sending the cropped image as response
Let's crop the `DynamicImage` into a square using the following code:
```rust
let size = min(min(image.width(), image.height()), max_size);
let image = image.resize_to_fill(size, size, imageops::FilterType::Triangle);
```
The size of the cropped square image is the minimum of the image's width, height and the configured maximum size. The `resize_to_fill` method will crop and resize the image to our size and center it appropriately.

Now that we have cropped the image, we need to send it back to the client. We encode the image back into an image format with `write_to`; we've chosen to return the cropped images as WebP's:
```rust
let mut image_buffer = Vec::new();
image
    .write_to(&mut BufWriter::new(Cursor::new(&mut image_buffer)), ImageFormat::WebP)
    .unwrap();
```

To send these bytes as an image to the client, we will have to create a response with a proper content type header and our image buffer as a body. Update the `crop_image` to return a [`Response`](https://docs.rs/axum/latest/axum/response/type.Response.html) instead of a `String`, and construct a response with [`Response::builder()`](https://docs.rs/http/latest/http/response/struct.Builder.html). Set the ["content-type" header](https://docs.rs/http/latest/http/header/constant.CONTENT_TYPE.html) to match your chosen image format (for example `image/webp` for WebP images), and construct a body from the image buffer using [`Body::from`](https://docs.rs/axum/latest/axum/body/struct.Body.html#impl-From%3CVec%3Cu8%3E%3E-for-Body).

If you now submit an image on the site, it should be returned to you cropped into a square!

### 3.1.1.F Error handling & input validation
Currently, the handler likely contains many `.unwrap()`s, which may panic. Luckily, `axum` catches these panics from our handler and will keep running after printing the panic. However, the user will not get any proper response from `axum` when these panics happen. To give the client some feedback about what went wrong, we can implement some better error handling.

Let's change our `crop_image` function to return a `Result<Response, (StatusCode, &'static str)>`. This gives us the ability to return errors consisting of an HTTP status code and a static string.

For example, let's say the user uploads a corrupted image. Then, the `.decode()` method of our `ImageReader` will return an error, causing the `.unwrap()` to panic. Let's replace the `.unwrap()` with a `.map_err` that notifies the user that they did a bad request:
```rs
.map_err(|_| (StatusCode::BAD_REQUEST, "Error: Could not decode image"))?
```

Similarly, you can also add appropriate error handling in other places, returning appropriate [HTTP status codes](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status).

Currently, our the size of our cropped image is defined as the minimum of the original image's width and height, and the set `max_size` value. The `max_size` value has a maximum of 2048 set in the HTML form. However, you should never trust the data coming from the client-side as HTML and JavaScript code running on the client's device can easily be modified, and the client can send modified HTTP requests. So let's also return a `StatusCode::BAD_REQUEST` if `max_size` is larger than 2048.

By default, there is a 2 MB limit for request bodies. If a user submits an image larger than this limit, the `.bytes()` call on the multipart field will return an error. In this case, we could return a `StatusCode::PAYLOAD_TOO_LARGE`. If you want to accept larger images, you can configure a larger limit by setting a custom [`DefaultBodyLimit`](https://docs.rs/axum/latest/axum/extract/struct.DefaultBodyLimit.html).

### 3.1.1.G Serving files from memory (bonus)
Currently, the static files are served from the `assets` folder. Instead, we can also bundle these files into the binary with [memory-serve](https://docs.rs/memory-serve/latest/memory_serve/). Not only is it convenient to bundle all files into a single binary, but it can also improve performance!

After adding memory-serve to your project with `cargo add memory-serve`, we can define a memory router as follows:
```rust
let memory_router = MemoryServe::new(load_assets!("assets"))
    .index_file(Some("/index.html"))
    .into_router();
```
Now we can use this memory router as fallback service instead of the `ServeDir`.

If you build the project in release mode (`cargo build --release`), you will see the files in the `assets` folder being included in the binary!


## Exercise 5.1.2: Pastebin
This exercise is about writing a simple [pastebin](https://en.wikipedia.org/wiki/Pastebin) web server. The web server will again be powered by [`axum`](https://docs.rs/axum). For this exercise, you will need to set up the project yourself.

- Data is kept in memory. Bonus if you use a database or `sqlite`, but first make the app function properly without.
- Expose a route to which a POST request can be sent, that accepts some plain text, and stores it along with a freshly generated UUID. The UUID is sent in the response. You can use the [`uuid` crate](https://docs.rs/uuid/latest/uuid/) to generate UUIDs.
- Expose a route to which a GET request can be sent, that accepts a UUID and returns the plain text corresponding to the UUID, or a 404 error if it doesn't exist.
- Expose a route to which a DELETE request can be sent, that accepts a UUID and deletes the plain text corresponding to that UUID.
