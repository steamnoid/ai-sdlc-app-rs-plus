//! The model's own words, read out of the envelope an endpoint wraps every answer in.
//!
//! **This function had no test of its own.** Two use cases call it, and both were proved on a
//! double returning a fixed string — so a real run walked seven gates and six models and the
//! answer was read out of an envelope nobody in this repository had ever looked at. The module
//! says so itself: "one function and not one per caller, because this is where #102 and #109 were
//! burned" — **and a function declared as the only door is the one function whose own reading
//! nobody checks.**
//!
//! **A socket and not a fixture, because this reader is only ever handed what an endpoint wrote.**
//! A loopback endpoint answering with one canned response is the shape of every answer this
//! function has been handed, and nothing leaves this machine: no network, no credential, no
//! model, and no program stubbed — the bytes are read off a socket the test itself served.

use aisdlc_run::the_words_inside_the_endpoints_envelope::{
    AnAnswerNobodyCanRead, the_words_inside_the_endpoints_envelope,
};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

/// One endpoint on loopback, answering every connection with the same envelope.
///
/// **The address is what a caller gets back and not the endpoint**, because the thread that
/// serves it owns it — a value handed to `thread::spawn` has moved, and a test returning it
/// afterwards would be returning something it no longer has.
struct AnEndpointAnsweringWith {
    what_it_would_answer_with: String,
}

impl AnEndpointAnsweringWith {
    /// An envelope in the shape an endpoint wraps a model's words in.
    ///
    /// **Built with `serde_json` and not written by hand**, because this repository has a gate
    /// called `no_body_is_built_by_a_hand` and a fixture is not an exception to it.
    fn wrapping(what_the_model_said: &str) -> String {
        let the_envelope = serde_json::json!({
            "choices": [{"message": {"content": what_the_model_said}}]
        });
        serde_json::to_string(&the_envelope).expect("a json value is written")
    }

    /// A loopback endpoint answering with these words, and the address it answers on.
    ///
    /// **Bound to port zero and not to a port it names**, because a test naming a port fails on a
    /// machine where that port is taken — and a test that fails for a reason the work did not
    /// cause is a test nobody trusts.
    fn answering_with(what_it_would_answer_with: String) -> String {
        let the_listener = TcpListener::bind("127.0.0.1:0").expect("a loopback port is free");
        let the_address = the_listener
            .local_addr()
            .expect("a bound address")
            .to_string();
        let the_endpoint = Self {
            what_it_would_answer_with,
        };
        std::thread::spawn(move || {
            while let Ok((a_connection, _)) = the_listener.accept() {
                the_endpoint.answer(a_connection);
            }
        });
        the_address
    }

    /// Read what was asked and write what was to be answered.
    ///
    /// **A read before a write, because an endpoint answers after a request** and a client that
    /// writes a large body before reading can deadlock on a full socket buffer.
    fn answer(&self, a_connection: TcpStream) {
        let mut the_client = a_connection;
        let mut the_request = [0u8; 4096];
        let _ = the_client.read(&mut the_request);
        let _ = the_client.write_all(self.what_it_would_answer_with.as_bytes());
    }
}

/// What an endpoint really answered with, read off a socket rather than handed over by a fixture.
fn what_the_endpoint_answered_with(the_answer: &str) -> String {
    let the_address = AnEndpointAnsweringWith::answering_with(the_answer.to_string());
    let mut the_connection = TcpStream::connect(&the_address).expect("the endpoint is listening");
    let _ = the_connection.write_all(b"POST /v1/chat/completions HTTP/1.1\r\n\r\n");
    let mut what_came_back = String::new();
    let _ = the_connection.read_to_string(&mut what_came_back);
    what_came_back
}

#[test]
fn the_models_own_words_are_read_out_of_the_envelope_and_not_the_envelope() {
    let what_the_endpoint_sent = what_the_endpoint_answered_with(
        &AnEndpointAnsweringWith::wrapping("APPROVED, decided_by: Alice"),
    );

    let the_words = the_words_inside_the_endpoints_envelope(&what_the_endpoint_sent);

    assert_eq!(
        the_words,
        Ok("APPROVED, decided_by: Alice".to_string()),
        "**because the model was asked to say one word and answered with JSON around that word** \
         — a reader that took the whole answer to be the word decides no model can ever rule, and \
         **the two failures this function was made for are in this one string**: the first word \
         is the yes and the rest is the forgery. What the socket delivered: {what_the_endpoint_sent:?}"
    );
}

#[test]
fn an_answer_that_is_no_envelope_quotes_back_what_came() {
    let what_the_endpoint_said = "Failed to connect to localhost port 11434";

    let the_refusal = the_words_inside_the_endpoints_envelope(what_the_endpoint_said);

    assert_eq!(
        the_refusal,
        Err(AnAnswerNobodyCanRead::because(what_the_endpoint_said)),
        "**and the refusal quotes the whole answer back**, because *Failed to connect to localhost \
         port 11434* is what the first real run got, and **a reader that paraphrased it would \
         have thrown away the only clue there was**"
    );
}

#[test]
fn an_envelope_holding_no_content_is_refused_and_quotes_back_what_came() {
    let what_the_endpoint_said = "{\"choices\":[]}";

    let the_refusal = the_words_inside_the_endpoints_envelope(what_the_endpoint_said);

    assert_eq!(
        the_refusal,
        Err(AnAnswerNobodyCanRead::because(what_the_endpoint_said)),
        "**because an envelope with nothing in it is not an answer**, and the words a caller needs \
         in order to decide whether to retry are the words that came back"
    );
}

#[test]
fn a_content_that_is_not_a_string_is_refused_and_quotes_back_what_came() {
    let what_the_endpoint_said = "{\"choices\":[{\"message\":{\"content\":{\"ruled\":false}}}]}";

    let the_refusal = the_words_inside_the_endpoints_envelope(what_the_endpoint_said);

    assert_eq!(
        the_refusal,
        Err(AnAnswerNobodyCanRead::because(what_the_endpoint_said)),
        "**because an endpoint that answered with a JSON object where the words go is not an \
         endpoint that answered with words**, and a reader that reached past it into the object \
         would hand a caller a `Value` where a `String` was promised"
    );
}
