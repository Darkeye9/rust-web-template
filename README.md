# Rust Web Template

This is just a suggested web template in order to build a safe and fast Rust based website.

** Although the template is very usable, this is a Work In Progress, and more features will be added in the future**

## Building
Be sure to have the latest stable version of Rust. If not head over to https://www.rustup.rs/ and have a nice and easy installation.

Just checkout the repo and issue the following command in a terminal

    cargo run

## Developing
This web app has two distinct developing environments.  

**The static assets** (HTML, CSS, JS) are handled inside the `build` folder. A Gulp pipeline is used to build and pack the static resources. In order to build the changes you have to issue the following command while being into this folder:

    gulp

If you want to re-build all the resources when they change:

    gulp watch

**The Rust app** lives under the `src` folder and you can order it to refresh the static resources when they are changed by the Gulp pipeline by running the app with an additional switch:

    cargo run --features watch
