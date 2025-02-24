use clap::Parser;
#[cfg(feature = "tls")]
use clap::ValueHint;
use hyper::header::{HeaderMap, HeaderName, HeaderValue};
use std::net::IpAddr;
#[cfg(feature = "tls")]
use std::path::PathBuf;

#[derive(Debug, Clone, Parser)]
#[clap(name = "dummyhttp", author, about, version)]
pub struct Args {
    /// Be quiet (log nothing)
    #[clap(short, long)]
    pub quiet: bool,

    /// Be verbose (log data of incoming and outgoing requests). If given twice it will also log
    /// the body data.
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: u8,

    /// Port on which to listen
    #[clap(short, long, default_value = "8080")]
    pub port: u16,

    /// Headers to send (format: key:value)
    #[clap(short = 'H', long, parse(try_from_str = parse_header))]
    pub headers: Vec<HeaderMap>,

    /// HTTP status code to send
    #[clap(short, long, default_value = "200")]
    pub code: u16,

    /// HTTP body to send
    ///
    /// Supports Tera-based templating (https://tera.netlify.app/docs/) with a few additional
    /// functions over the default built-ins:
    ///
    /// uuid() - generate a random UUID
    /// lorem(words) - generate `words` lorem ipsum words
    ///
    /// Example: dummyhttp -b "Hello {{ uuid() }}, it's {{ now() | date(format="%Y") }} {{ lorem(words=5)}}"
    #[clap(short, long, default_value = "dummyhttp", verbatim_doc_comment)]
    pub body: String,

    /// Interface to bind to
    #[clap(
        short,
        long,
        parse(try_from_str = parse_interface),
        number_of_values = 1,
        default_value = "0.0.0.0"
    )]
    pub interface: IpAddr,

    /// Generate completion file for a shell
    #[clap(long = "print-completions", value_name = "shell", arg_enum)]
    pub print_completions: Option<clap_complete::Shell>,

    /// Generate man page
    #[clap(long = "print-manpage")]
    pub print_manpage: bool,

    /// TLS certificate to use
    #[cfg(feature = "tls")]
    #[clap(long = "tls-cert", alias = "cert", requires = "tls-key", value_hint = ValueHint::FilePath)]
    pub tls_cert: Option<PathBuf>,

    /// TLS private key to use
    #[cfg(feature = "tls")]
    #[clap(long = "tls-key", alias = "key", requires = "tls-cert", value_hint = ValueHint::FilePath)]
    pub tls_key: Option<PathBuf>,
}

/// Checks wether an interface is valid, i.e. it can be parsed into an IP address
fn parse_interface(src: &str) -> Result<IpAddr, std::net::AddrParseError> {
    src.parse::<IpAddr>()
}

/// Parse a header given in a string format into a `HeaderMap`
///
/// Headers are expected to be in format "key:value".
fn parse_header(header: &str) -> Result<HeaderMap, String> {
    let header: Vec<&str> = header.split(':').collect();
    if header.len() != 2 {
        return Err("Wrong header format (see --help for format)".to_string());
    }

    let (header_name, header_value) = (header[0], header[1]);

    let hn = HeaderName::from_lowercase(header_name.to_lowercase().as_bytes())
        .map_err(|e| e.to_string())?;

    let hv = HeaderValue::from_str(header_value).map_err(|e| e.to_string())?;

    let mut map = HeaderMap::new();
    map.insert(hn, hv);
    Ok(map)
}
