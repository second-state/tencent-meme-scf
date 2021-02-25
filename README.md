[English](README-en.md) | [Live demo!](https://sls-website-ap-hongkong-hmtn9c-1302315972.cos-website.ap-hongkong.myqcloud.com/)

# 快速开始

Clone 这个 repo。从 repo 的根目录，您可以拉取我们预先配置的 dev Docker image 并登录进去。请见[国内的 Docker hub 镜像](https://yeasy.gitbook.io/docker_practice/install/mirror)。

```
$ git clone https://gitee.com/secondstate/tencent-meme-scf
$ cd tencent-meme-scf

$ docker pull secondstate/tencent-tensorflow-scf
$ docker run --rm -it -v $(pwd):/app secondstate/tencent-tensorflow-scf
(docker) $
```

> 你不一定要使用我们的 Docker image。要在您自己的计算机或容器上构建，请确保您已经安装了[Serverless 框架](https://www.serverless.com/framework/docs/providers/tencent/guide/installation/), [Rust](https://www.rust-lang.org/tools/install), 和[ssvmup](https://www.secondstate.io/articles/ssvmup/).

在 Docker 容器中运行以下命令，以构建和部署整个应用程序。

```
(docker) $ cd /app
(docker) $ sls deploy
... ...
  website:       https://sls-website-ap-hongkong-kfdilz-1302315972.cos-website.ap-hongkong.myqcloud.com
  vendorMessage: null

63s › tencent-meme-scf › "deploy" ran for 3 apps successfully.
```

在浏览器中加载网站 URL，就开始使用函数来产生图片啦。

> 在中国大陆，如果 GitHub 很慢，可以用我们在 Gitee 的镜像。`git clone https://gitee.com/secondstate/tencent-meme-scf.git`

# 创建你自己的云函数

Fork 这个 repo，使用 `Code | Open with Codespaces` 按钮来在浏览器中打开 Github Codespaces IDE 。第一次启动时，需要花费几分钟。 

## 低代码开发

一旦 Codespaces IDE 启动了, 你就可以根据自己的应用程序需求来对源代码进行修改，自定义函数。

* 在 `src/main.rs` 文件中更改图片产生的逻辑。
* 在 `website/content/index.html` 文件中对前端UI进行更改。

## 创建

在 Codespaces 打开 `Terminal` 窗口, 然后运行下面的命令行来创建云函数。

```
$ ssvmup build --enable-aot
```

## 部署

在 `Terminal` 窗口，运行下面的命令行将云函数部署到腾讯云上。

```
$ cp pkg/scf.so scf/

$ sls deploy
... ...
  website:       https://sls-website-ap-hongkong-kfdilz-1302315972.cos-website.ap-hongkong.myqcloud.com
```

在浏览器内加载部署好的 URL。 Have fun!

## 更多

这个 repo 的各个 branches 有不同的 memes
