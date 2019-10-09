# vim - IDE

https://about.okhin.fr/2018/08/03/my-vim-setup-with-some-rust-specifities/
https://kadekillary.work/post/rust-ide/
http://seenaburns.com/vim-setup-for-rust/

    0. Add plugin 'neoclide/coc-rls' to vim, so the .vimrc like this:

        Plug 'neoclide/coc.nvim', {'do': 'yarn install --frozen-lockfile'}  | " sometimes find references fail
        Plug 'neoclide/coc-rls'   | " which base on 'rls': https://github.com/rust-lang/rls

    0.1. coc-rls depend on [rls](https://github.com/rust-lang/rls) which depend on [racer](https://github.com/racer-rust/racer)
       - install racer:
          $ rustup update nightly
          $ rustup toolchain add nightly
          $ rustup component add rls rust-analysis rust-src
          $ cargo +nightly install racer        <=== current have error when install

             Compiling rustc-ap-rustc_errors v583.0.0
              error[E0046]: not all trait items implemented, missing: `mixed_site`
                 --> /home/wilson/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-syntax-583.0.0/ext/proc_macro_server.rs:657:1
                  |
              657 | impl server::Span for Rustc<'_> {
                  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `mixed_site` in implementation

          $ racer complete std::io::B     <=== test it: should show some completions

    1. Install rustup (Rust toolchain manager).
    2. Install this extension in your vim by:
      :CocInstall coc-rls
    3. Open a Rust project. Open the folder for the whole project (i.e., the folder containing 'Cargo.toml'), not the 'src' folder.
        You'll be prompted to install the RLS. Once installed, the RLS should start building your project.

    4. The RLS gets its source data from the compiler and from Racer
        https://github.com/racer-rust/vim-racer

    5. tags: https://github.com/dan-t/rusty-tags
      $ cargo install rusty-tags
      add to zshrc:
      $ export RUST_SRC_PATH=$(rustc --print sysroot)/lib/rustlib/src/rust/src/
      add to vimrc:
      autocmd BufRead *.rs :setlocal tags=./rusty-tags.vi;/


# Hands-On Data Structures and Algorithms with Rust

<a href="https://www.packtpub.com/application-development/hands-data-structures-and-algorithms-rust?utm_source=github&utm_medium=repository&utm_campaign=9781788995528 "><img src="https://d1ldz4te4covpm.cloudfront.net/sites/default/files/imagecache/ppv4_main_book_cover/B10269_cover.png" alt="Hands-On Data Structures and Algorithms with Rust" height="256px" align="right"></a>

This is the code repository for [Hands-On Data Structures and Algorithms with Rust](https://www.packtpub.com/application-development/hands-data-structures-and-algorithms-rust?utm_source=github&utm_medium=repository&utm_campaign=9781788995528 ), published by Packt.

**Learn programming techniques to build effective, maintainable, and readable code in Rust 2018**

## What is this book about?
Rust has come a long way and is now utilized in several contexts. Its key strengths are its software infrastructure and resource-constrained applications, including desktop applications, servers, and performance-critical applications, not forgetting its importance in systems' programming. This book will be your guide as it takes you through implementing classic data structures and algorithms in Rust, helping you to get up and running as a confident Rust programmer.

This book covers the following exciting features:
Design and implement complex data structures in Rust
Analyze, implement, and improve searching and sorting algorithms in Rust
Create and use well-tested and reusable components with Rust
Understand the basics of multithreaded programming and advanced algorithm design
Become familiar with application profiling based on benchmarking and testing
Explore the borrowing complexity of implementing algorithms

If you feel this book is for you, get your [copy](https://www.amazon.com/dp/178899552X) today!

<a href="https://www.packtpub.com/?utm_source=github&utm_medium=banner&utm_campaign=GitHubBanner"><img src="https://raw.githubusercontent.com/PacktPublishing/GitHub/master/GitHub.png"
alt="https://www.packtpub.com/" border="5" /></a>

## Instructions and Navigations
All of the code is organized into folders. For example, Chapter02.

The code will look like the following:

```
fn my_function() {
    let x = 10;
    do_something(x); // ownership is moved here
    let y = x; // x is now invalid!
}
```

**Following is what you need for this book:**
This book is for developers seeking to use Rust solutions in a practical/professional setting; who wants to learn essential Data Structures and Algorithms in Rust. It is for developers with basic Rust language knowledge, some experience in other programming languages is required.

With the following software and hardware list you can run all code files present in the book (Chapter 1-12).
### Software and Hardware List
| Chapter | Software required | OS required |
| -------- | ------------------------------------ | ----------------------------------- |
| 1-12 | rustc, cargo | Windows, Linux, MacOS |
| 1-12 | Visual Studio Code | Windows 7 or later, Linux, or MacOS 10.9+  |
| 1-12 | Rust for Visual Studio Code | Windows 7 or later, Linux, or MacOS 10.9+  |

We also provide a PDF file that has color images of the screenshots/diagrams used in this book. [Click here to download it](https://www.packtpub.com/sites/default/files/downloads/9781788995528_ColorImages.pdf).

### Related products
* Hands-On Concurrency with Rust [[Packt]](https://www.packtpub.com/application-development/hands-concurrency-rust?utm_source=github&utm_medium=repository&utm_campaign=9781788399975 ) [[Amazon]](https://www.amazon.com/dp/1788399978)

* Rust Quick Start Guide [[Packt]](https://www.packtpub.com/application-development/rust-quick-start-guide?utm_source=github&utm_medium=repository&utm_campaign=9781789616705 ) [[Amazon]](https://www.amazon.com/dp/1789616700)

## Get to Know the Author
**Claus Matzinger**
is a software engineer with a very diverse background. After working in a small company maintaining code for embedded devices, he joined a large corporation to work on legacy Smalltalk applications. This led to a great interest in programming languages early on, and Claus became the CTO for a health games start-up based on Scala technology.

Since then, Claus' roles have shifted toward customer-facing roles in the IoT database-technology start-up crate.io and, most recently, Microsoft. There, he hosts a podcast, writes code together with customers, and blogs about the solutions arising from these engagements. For more than 5 years, Claus has implemented software to help customers innovate, achieve, and maintain success.

Check out his blog at [blog.x5ff.xyz](https://blog.x5ff.xyz)!

### Suggestions and Feedback
[Click here](https://docs.google.com/forms/d/e/1FAIpQLSdy7dATC6QmEL81FIUuymZ0Wy9vH1jHkvpY57OiMeKGqib_Ow/viewform) if you have any feedback or suggestions.
