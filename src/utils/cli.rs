use clap::{App, Arg, ArgMatches};

pub fn get_cli<'a>() -> ArgMatches<'a> {
    let matches = App::new("RustyCMS")
        .version("0.0.1")
        .author("Dark_eye <dark.eye9@gmail.com>")
        .about("Serve your site with speed and safety!")
        .arg(Arg::with_name("new_site")
            .long("new")
            .help("Creates a new site"))
        .get_matches();
    return matches;
}
