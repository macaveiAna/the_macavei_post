### Ana Macavei, CS-410P, Fall 2023

## Project Name: The Macavei Post

## Project Topic Area: A Personal Blog and Website

## Project Vision: 
The Macavei Post is a personal blog and website that aims to be aesthetically designed and user-friendly. The website will have a Home Page featuring a one-line favorite quote for a personalized touch. The website will include an About Me page, a page for my favorite Music, Books, Quotes and Photos. Add a login, and contact form.

## Discussion of Concerns:
During this project, one concern is the process of transforming the Rust code into a website. I plan to extensively research the existing Rust’s web development tools and frameworks. I might be interested in researching any Rust crates for backend and potentially frontend development. Along with any online tutorials or guides for the learning process of turning the Rust code into a functional website. 

## Requirements

- One or more Rust crates. The project can be a binary crate, a library
crate or some combination. A library crate plus a small driver application
is a really nice project style; something to consider.

- Each crate should be something vaguely like 300-500 lines of code (size
doesn’t matter so much as effort and quality) performing a coherent
function.

- The project should include a README.md writeup describing what was built,
how it worked, what didn’t work, and what lessons were learned.

- Code should be written in a reasonable modular style. Code must not have
rustfmt or clippy errors. Code should include internal unit tests where
appropriate. Items in crate public interfaces must have good rustdoc. The
project commit history must accurately reflect project development.

- The project must build using stable Rust, unless otherwise approved. 
unsafe code should be kept to an absolute minimum.

## How to Start The Development Server

run `cargo run`
enter this in url `http://127.0.0.1:8080/`

## Write-Up
I was having a lot of trouble displaying the image. The result I kept getting was the image would flash anytime I would reload the page. The error was that image file 
was in my dist file. My dist file had an index.html. I had another index.html at the root directory. I pretty much just needed to delete that file and cd into the dist 
file. Then I went ahead and re-ran trunk and success.

Now I am running into the error of not being able to change the image... And the dist file is copied with a copied index.html. 

I was having trouble calling my router.rs in main. Turns out, I needed to make sure the yew_router and yew version in the Cargo.toml were the same version. I spent about an hour trying to resolve this (sadly melting).

After a certain point, I realized that yew was probably going to be much harder to use to build a blog website, so I moved on to trying out Actix-Web and Handlebars.

I really enjoy building with Actix-web so far. One thing I am working on right now that I am having trouble connecting is that I want to create
the home page with the ability to navigate to the about me page without using the url. For example, right now the way to access my home page is to enter in this url: 
http://127.0.0.1:8080/
And to access the about me page I have to enter this url:
http://127.0.0.1:8080/about
I want to be able to click on the home page to navigate instead. 

I was able to figure this out fairly quickly. I just needed to modify the posts.html file to have the link to the about me page.

I had an extremely hard time trying to get authentication to work. I was getting a lot of cargo errors. And I ended up just commenting out the 
code because I almost lost my program and decided it wasn't worth it to have it not run. I wanted to show what I was able to get running. 

While I encountered challenges in getting the authentication part of the code to function, my determination remains unwavering. Despite the 
current setback, I am committed to continuing the development of this website even after I submit it. I have scheduled dedicated time during 
the winter break to further refine and troubleshoot the authentication aspect. My goal is to transform this project into a showcase piece that 
I can proudly highlight in future job opportunities.


## Testing

run `Cargo test` to test 


### Resources

https://actix.rs/docs/getting-started/
https://actix.rs/docs/url-dispatch/
https://www.w3schools.com/css/css_intro.asp
https://docs.rs/actix-web/latest/actix_web
https://docs.rs/actix-identity/latest/actix_identity/
https://actix.rs/docs/testing/