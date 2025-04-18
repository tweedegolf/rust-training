# Unit 5.2 - Rust in the Cloud

In this section, we will show how you can use Rust to create cloud services. The [first exercise](#exercise-521-lettuce-crop-aws) uses Amazon's AWS, and the [second exercise](#exercise-522-lettuce-crop-scaleway) uses European cloud service provider Scaleway.

## Exercise 5.2.1: Lettuce Crop AWS
In this exercise, we will port our Lettuce Crop website from exercise 5.1.1 to the cloud using [AWS Lambda](https://aws.amazon.com/lambda/). AWS Lambda allows you to run code in the cloud in a serverless configuration. This means that machines in Amazon's data centers will automatically start running your code when needed, which means you do not have to worry about managing servers, and you only pay for the compute time you use.

<div class="warning">

For this exercise, the [AWS free tier](https://aws.amazon.com/free/) should be sufficient. However, please do remember to shut off your Lambdas once you are done testing to avoid any unexpected costs! See the [free tier page](https://console.aws.amazon.com/billing/home#/freetier) in the billing and cost management section of the AWS console to see how much of the free tier quotas you have left this month.

</div>

### 5.2.1.A Setting up Cargo Lambda
To build for AWS Lambda with Rust, we will use [Cargo Lambda](https://www.cargo-lambda.info/). You can install Cargo Lambda with [Cargo Binstall](https://github.com/cargo-bins/cargo-binstall):
```
cargo binstall cargo-lambda
```
You may also need to install [Zig](https://ziglang.org/), which is used for [cross-compilation](https://www.cargo-lambda.info/guide/cross-compiling.html). Cargo Lambda will inform you if Zig is not installed when building your Lambda, in which case it will attempt to help you install it automatically via `pip` or `npm`.


Alternatively, you can use any of the other installation methods for Cargo Lambda found [here](https://www.cargo-lambda.info/guide/installation.html).

### 5.2.1.B Axum router with Lambda HTTP
The [`lambda_runtime`](https://crates.io/crates/lambda_runtime/) crate provides the runtime for AWS Lambdas written in Rust. The [`lambda_http`](https://crates.io/crates/lambda_http) crate provides an abstraction layer on top of the `lambda_runtime` to make it easy to develop HTTP servers on AWS Lambda with Rust, which is ideal for small dynamic websites or REST APIs.

Add `lambda_http` to the Rust project with:
```
cargo add lambda_http
```

Since `lambda_http` is able to run `axum` routers, we only really need to change the main function to convert our Lettuce Crop server to a Lambda. We create our `Router` as usual, but instead of serving it with `axum::serve`, we run the `Router` with the `run` function from `lambda_http`:
```rust
use lambda_http::{run, tracing, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // required to enable CloudWatch error logging by the runtime
    tracing::init_default_subscriber();

    let app = Router::new()
        .route("/crop", post(crop_image))
        .fallback_service(ServeDir::new("assets"));

    run(app).await
}
```
Update the main function as above and then try out the Lambda with:
```
cargo lambda watch
```
This will emulate the Lambda locally on your device, serving it on [http://localhost:9000/](http://localhost:9000/) by default.

### 5.2.1.C Setting up a Lambda function in the AWS console
Now that we've tested our Lambda locally, let's create a Lambda function in the AWS console. Go to the [AWS Lambda page](https://console.aws.amazon.com/lambda/home) in the AWS console, and click "Create a function". Then, configure it as follows:

1. Select "Author from scratch"
2. Give it a name, for example "lettuce-crop"
3. Select the "Amazon Linux 2023" runtime
4. Select "arm64" architecture (which offers lower costs compared to x86_64)
5. In "Additional Configurations" enable "Enable function URL", and select Auth type "NONE" to get a publicly accessible URL for your Lambda function

Finally, click "Create function" and wait a few seconds for your Lambda to be created.

### 5.2.1.D Building & deploying our Lambda function
Before we deploy our Lambda, we first have to build our project with the appropriate architecture:
```
cargo lambda build --release --arm64 --output-format zip
```
This will generate a `bootstrap.zip` in the `target/lambda/{project name}` folder, which we can upload in the AWS console to deploy our Lambda.

However, this zip file does not contain our assets. If we want our Lambda to be able to serve our HTML document and the corresponding CSS file and image, we have to include these assets. Let's create a `CargoLambda.toml` config file to specify how we want to build our Lambda, and include the following:
```toml
[build]
arm64 = true
output_format = "zip"
include = ["assets/index.html", "assets/styles.css", "assets/crop.webp"]
```
If we now build our Lambda with `cargo lambda build --release` we will get a zip that also contains our assets (we no longer need the `--arm64` and `--output-format` command line arguments, as these are now set in our config file).

Alternatively, if you are using [memory-serve](https://docs.rs/memory-serve/latest/memory_serve/) to serve the assets, as described in exercise 5.1.1.G, you will not need to include the assets in the zip, as they already will be included in the binary.

To deploy the Lambda, click the "Upload from" button in the "Code" tab for our Lambda in the AWS console. Then, upload the `bootstrap.zip` file. Now, the Lambda should be live! Open the function URL listed in the function overview at the top of the page to try it out!

You can also use `cargo lambda deploy` to deploy your Lambda via the CLI. However, this does require you to set up [AWS credentials](https://www.cargo-lambda.info/guide/automating-deployments.html) first.

Note that AWS Lambda only accepts files up to 50 MB, for larger projects you can instead upload to an [S3 bucket](https://aws.amazon.com/s3/). S3 does not have a free tier, but it does have a 12-month free trial.

### 5.2.1.E Analyzing Lambda usage via CloudWatch
Now that our Lambda is up and running, let's take a look around the AWS console. If you go to the "Monitor" tab, you can see some metrics about the requests handled by the Lambda function. These basic metrics are automatically gathered by CloudWatch free of charge.

If you scroll down to CloudWatch Logs, you will see recent invocations of the Lambda function. If you click on the log stream of one of these requests, you will see the logs produced while handling the request. The outputs from any `println!`'s or logs from the [`tracing`](https://docs.rs/tracing/latest/tracing/) crate should show up here. The free tier of CloudWatch allows you to store up to 5 GB of log data for free.

You can also see a list of the most expensive invocations on the "Monitor" tab. The cost is measured in gigabyte-seconds, which is the amount of memory used for the duration it took to handle the request. The free tier for AWS Lambda gives you 1,000,000 requests and 400,000 gigabyte-seconds for free per month.

By default, Lambdas are configured with 128 MB of memory, which can be increased in the "Configuration" tab (but it cannot be set lower than 128 MB). In this tab you can also configure the timeout for handling requests. By default, the Lambda will time out after 3 seconds, but this can be changed if needed.

#### Where to go from here?

- The [Rust Runtime for AWS Lambda GitHub repository](https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples) contains a bunch of useful examples, which show for example how to [interact with S3 buckets](https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples/basic-s3-thumbnail) or how to [create Lambda extensions](https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples/extension-custom-service).
- The [AWS SDK for Rust](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/welcome.html) allows you to interact with AWS services via Rust.

Remember to throttle or delete the Lambda function once you are done testing to prevent unexpected costs!


## Exercise 5.2.2: Lettuce Crop Scaleway
In this exercise, we will port our Lettuce Crop website from exercise 5.1.1 to the cloud using [Scaleway Serverless Functions](https://www.scaleway.com/en/serverless-functions/). Scaleway Serverless Functions allows you to run code in the cloud in a serverless configuration. This means that machines in Scaleway's data centers will automatically start running your code when needed, which means you do not have to worry about managing servers, and you only pay for the compute time you use.

<div class="warning">

For this exercise, the free resources offered by Scaleway should be sufficient. However, please do remember to shut off your serverless functions once you are done testing to avoid any unexpected costs! See the [serverless pricing page](https://www.scaleway.com/en/pricing/serverless/) for up-to-date information about the costs associated with running serverless functions.

</div>

### 5.2.2.A General project structure
Serverless functions on Scaleway work by providing a handler function that takes a request and returns a response. In `exercises/5-rust-for-web/2-rust-in-the-cloud/2-lettuce-crop-scaleway` we have laid out the basic project structure for a Scaleway serverless function. In `src/handler.rs` there is a handler function that turns any request into a response with the body "Hello, world!".

To expose this function to Scaleway, we must configure our project as a library, using `src/handler.rs` as the main entry point. To do this, add the following to the `Cargo.toml`:
```toml
[lib]
path = "src/handler.rs"
```


### 5.2.2.B Deploying the serverless function
To deploy our serverless function, go to the [functions page](https://console.scaleway.com/functions/) in the Scaleway console, select your preferred region, and pick a namespace (or create a new one). In this namespace, click the "Create function" button and select Rust as the runtime.

To upload our code, we first create a zip file with the `src` directory and the `Cargo.toml`. Upload this zip to Scaleway, and set the handler to the name of the function in `src/handler.rs` that you want to use to handle requests (`my_handler` in our case).

Now you can set a name and configure to resources as needed, and then you can create the function. Wait for it to deploy (which might take a couple of minutes), and then go to the function endpoint URL to see the serverless function in action.


### 5.2.2.C Request handling with axum routing
To add some more interesting functionality to this handler, we can set up an axum router like we did in exercise 5.1.1. However, with serverless functions, we cannot serve the router using `axum::serve`. Instead, we can call the router as a service with the currently incoming request:

```rs
let mut router = Router::new()
    .route("/", get("Hello, world!"))
    .route("/crop", get("Lettuce crop!"));

router.as_service().call(req).await.unwrap()
```
This `call` method will then provide a response based on the request using the specified routes.

Go ahead and modify the handler to use a router with a couple of different routes, and redeploy it on Scaleway to test it.


### 5.2.2.D Serving files from serverless functions
To be able to host Lettuce Crop on a Scaleway serverless function, we need to be able to serve the static assets for the website. Normally, we could use `ServeDir` for this, but Scaleway's serverless function don't have access to files during runtime. Instead, we can include the files in the binary using memory-serve, as described in [exercise 5.1.1.G](/rust-for-web-servers.html#511g-serving-files-from-memory-bonus).

Copy over the `assets` folder and the `crop_image` function from exercise 5.1.1, and configure the router to crop images sent with POST requests to `/crop`, and serve the assets using memory-serve as a fallback service. If you now redeploy your code (don't forget to also include the `assets` folder in the zip), we should now have a fully working version of Lettuce Crop on Scaleway!

> Note that in real world applications, it is better (and cheaper!) to host the static parts of the webpage separately using e.g. [storage buckets](https://www.scaleway.com/en/docs/object-storage/api-cli/bucket-website-api/), and only handle the interactive part (i.e. the post requests to crop images) in the serverless function.

