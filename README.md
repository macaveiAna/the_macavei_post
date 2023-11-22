### Ana Macavei, CS-410P, Fall 2023

## Project Name: The Macavei Post

## Project Topic Area: A Personal Blog and Website

## Project Vision: 
The Macavei Post is a personal blog and website that aims to be aesthetically designed and user-friendly. The website will have a Home Page featuring a one-line favorite quote for a personalized touch. The website will include an About Me page, a Photos page and pages for my favorite Music, Books and Quotes. The website will also provide authorized personnel only for adding content, removing content, and editing content. 

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

run `trunk serve --open`

## Write-Up
I was having a lot of trouble displaying the image. The result I kept getting was the image would flash anytime I would reload the page. The error was that image file 
was in my dist file. My dist file had an index.html. I had another index.html at the root directory. I pretty much just needed to delete that file and cd into the dist 
file. Then I went ahead and re-ran trunk and success.

Now I am running into the error of not being able to change the image... And the dist file is copied with a copied index.html. 

## Testing

### Resources

https://yew.rs/docs/tutorial 
https://trunkrs.dev/assets/#images-other-resources
