[中文](README.md) | [Live demo!](https://sls-website-ap-hongkong-hmtn9c-1302315972.cos-website.ap-hongkong.myqcloud.com/)

# Quick start

Clone this repo. From the repo's root directory, you can pull our pre-configured dev Docker image and log into it.

```
$ git clone https://github.com/second-state/tencent-meme-scf
$ cd tencent-meme-scf

$ docker pull secondstate/tencent-tensorflow-scf
$ docker run --rm -it -v $(pwd):/app secondstate/tencent-tensorflow-scf
(docker) $
```

> You do not have to use our Docker image. To build on your own computer or container, make sure that you have installed the [Serverless Framework](https://www.serverless.com/framework/docs/providers/tencent/guide/installation/), [Rust](https://www.rust-lang.org/tools/install), and [ssvmup](https://www.secondstate.io/articles/ssvmup/).

Run the following command inside the Docker container to build and deploy the entire application.

```
(docker) $ cd /app
(docker) $ sls deploy
... ...
  website:       https://sls-website-ap-hongkong-kfdilz-1302315972.cos-website.ap-hongkong.myqcloud.com
  vendorMessage: null

63s › tencent-meme-scf › "deploy" ran for 3 apps successfully.
```

Load the website URL in any web browser and start to use this function to create a meme.

> GitHub could be very slow behind the Great Chinese Firewall. Clone from Gitee if you are in mainland China. `git clone https://gitee.com/secondstate/tencent-meme-scf.git`

# Build your own meme cloud function

Fork this repo and use the `Code | Open with Codespaces` button to launch Github Codespaces IDE in your browser. It may take a few minutes to start the first time. 

## Low code development

Once the Codespaaces IDE starts, you can make simple changes to the source code to customize it for your own applications.

* Make changes to the image processing logic in `src/main.rs` file. 
* Make changes to the front end UI in the `website/content/index.html` file.

## Build

Open a `Terminal` windon in the Codespaces IDE, and run the following command to build your cloud function.

```
$ ssvmup build --enable-aot
```

## Deploy

In the `Terminal` window, run the following commands to deploy the cloud function to the Tencent Cloud.

```
$ cp pkg/scf.so scf/

$ sls deploy
... ...
  website:       https://sls-website-ap-hongkong-kfdilz-1302315972.cos-website.ap-hongkong.myqcloud.com
```

Load the deployed URL in any web browser and have fun!

## More memes

Check out different branches from this repo to see more memes. If you have a good meme photo, please send us a pull request. :)


