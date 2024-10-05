# untitled
Get titled images with [naked](https://kindgirls.com/old) girls in them.

A Rust CLI app to download *Girls* from the [kindgirls](https://kindgirls.com/old) website.

### What's `untitled`?

An elevator pitch for `untitled` would be:

"A relatively useless CLI app to interact with the 
[kindgirls](https://kindgirls.com/old) website
built solely for the purpose of learning a thing or two about Rust.
It lets you grab all the images and videos from a given `Girl` URL."

On a side note, I'm probably going to get a cease and desist letter from
the website owner, if they ever find out about this.

### What can `untitled` do?

For now, not much. It downloads the entire *gallery* of images and videos 
(if there are any) from a given kindgirls URL. It has to be a *Girl* URL,
like [this one here.](https://www.kindgirls.com/old/girls.php?id=1633)

### How does `untitled` work?

`untitled` is a CLI app. You run it with a URL as an argument, and it will
first scrape a `Girl's` profiles off of the website, then download all the
`visuals` - images and videos. 

It saves the content to a `kindgirls` folder in the user's home directory.

### How do I use `untitled`?

First, you need to have Rust installed. If you don't, you can get it from
[here.](https://www.rust-lang.org/tools/install)

Then, you need to build `untitled` from source. You can do that by running the
following command in the root directory of the project:

```bash
cargo build --release
```

After that, you can run the binary with a URL as an argument. For example:

```bash
./untitled scrape --url https://www.kindgirls.com/old/girls.php?id=1633
```

This will download all the images and videos from the given URL.

By default, the tool grabs the content *as is*, meaning the images are in the
same resolution as they are on the website. If you want to download the images
in a *full-size* resolution, you can use the `--full-size-image` flag:

```bash
./untitled scrape --full-size-image --url https://www.kindgirls.com/old/girls.php?id=1633
```

### What's next for `untitled`?

Since I'm probably the only person who's ever going to use this silly tool,
I don't really have a roadmap for it. I might add some features to it if I
need them and polish things up a bit, like bugs or performance issues. But other
than that, I don't have any plans for it.

I guess I could add some tests, just for the sake of learning how to
write tests in Rust.

And maybe some CI/CD to create a release build and upload it to GitHub.

### License

`untitled` is licensed under the MIT license. In short, you can do whatever
you want with the code, as long as you don't hold me liable for anything that
might happen as a result of using it.