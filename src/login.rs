use greetd_ipc::codec::SyncCodec;
use greetd_ipc::{AuthMessageType, ErrorType, Request, Response};
use log::{error, info, warn};
use std::env;
use std::os::unix::net::UnixStream;

use crate::config::Runner;

pub enum LoginFailure {
    /// The login attempt failed due to invalid auth credentials
    AuthError,
    /// The login attempt failed due to missing auth credentials
    MissingFields,
    /// There was an error during the login attempt
    Error,
}

pub enum LoginResult {
    /// The login attempt failed
    Failure(LoginFailure),
    /// The login attempt succeeded
    Success,
}

pub fn handle_login(
    username: String,
    password: String,
    runner: &Runner
) -> LoginResult {
    if username.is_empty() || password.is_empty() {
        return LoginResult::Failure(LoginFailure::MissingFields);
    }

    let Ok(path) = env::var("GREETD_SOCK") else {
        error!("unable to find GREETD_SOCK environment variable");
        return LoginResult::Failure(LoginFailure::Error)
    };
    // See: https://github.com/kennylevinsen/greetd/blob/master/agreety/src/main.rs
    let mut stream = match UnixStream::connect(path) {
        Ok(stream) => stream,
        Err(err) => {
            error!("unable to open stream: {err}");
            return LoginResult::Failure(LoginFailure::Error)
        },
    };
    let mut starting = false;
    let mut next_request = Request::CreateSession { username };

    loop {
        if let Err(err) = next_request.write_to(&mut stream) {
            error!("unable to write to greetd socket: {err}");
            return LoginResult::Failure(LoginFailure::Error)
        };

        let response = match Response::read_from(&mut stream) {
            Ok(response) => response,
            Err(err) => {
                error!("unable to read response from greetd socket: {err}");
                return LoginResult::Failure(LoginFailure::Error)
            },
        };

        match response {
            Response::AuthMessage {
                auth_message,
                auth_message_type,
            } => {
                let response = match auth_message_type {
                    AuthMessageType::Visible | AuthMessageType::Secret => Some(password.clone()),
                    AuthMessageType::Info => {
                        info!("auth message info: {auth_message}");
                        None
                    }
                    AuthMessageType::Error => {
                        warn!("auth message error: {auth_message}");
                        None
                    }
                };

                next_request = Request::PostAuthMessageResponse { response };
            }
            Response::Success => {
                if starting {
                    return LoginResult::Success;
                } else {
                    starting = true;
                    next_request = Request::StartSession {
                        env: runner.env.clone(),
                        cmd: vec![runner.run.clone()],
                    }
                }
            }
            Response::Error { error_type, description } => {
                if let Err(err) = Request::CancelSession.write_to(&mut stream) {
                    error!("unable to close greetd session: {err}");
                    return LoginResult::Failure(LoginFailure::Error)
                }
                return match error_type {
                    ErrorType::AuthError => LoginResult::Failure(LoginFailure::AuthError),
                    ErrorType::Error => {
                        error!("error during login attempt: {description}");
                        LoginResult::Failure(LoginFailure::Error)
                    }
                };
            }
        }
    }
}
