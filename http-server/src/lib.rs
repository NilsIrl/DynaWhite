#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

#[macro_use]
extern crate lazy_static;

use std::env;

lazy_static! {
    static ref SMTP_USERNAME: String = env::var("SMTP_USERNAME").unwrap();
    static ref SUPPORT_EMAIL_ADDRESS: String = env::var("SMTP_USERNAME").unwrap();
    static ref WEBSITE_URL: String = env::var("WEBSITE_URL").unwrap();
    static ref MC_SERVER_ADDR: String = env::var("MC_SERVER_ADDR").unwrap();
    static ref SMTP_ADDR: String = env::var("SMTP_ADDR").unwrap();
    static ref SMTP_PORT: u16 = env::var("SMTP_PORT").unwrap().parse().unwrap();
    static ref SMTP_PASSWORD: String = env::var("SMTP_PASSWORD").unwrap();
    static ref VALID_DOMAIN: String = env::var("VALID_DOMAIN").unwrap();
}

use askama::Template;

use jni::{
    objects::{GlobalRef, JClass},
    JNIEnv, JavaVM,
};

use rocket::{request::Form, State};
use rocket_contrib::uuid::{uuid_crate, Uuid};

use serde::Deserialize;

use std::{
    convert::TryInto,
    str::FromStr,
    sync::{Arc, Mutex},
    time::SystemTime,
};

#[derive(Deserialize)]
struct MojangUUIDResponse {
    id: String,
}
struct SentEmails {
    token: uuid_crate::Uuid,
    mojang_uuid: Uuid,
    at: SystemTime,
}
#[derive(Template)]
#[template(path = "register.html")]
struct RegisterTemplate {
    valid: bool,
    messages: Vec<String>,
}

#[derive(Template)]
#[template(path = "verify.html")]
struct VerifyTemplate {
    valid: bool,
    message: String,
}

#[derive(FromForm)]
struct UserForm {
    email: String,
    username: String,
}
#[get("/")]
fn register_get() -> RegisterTemplate {
    RegisterTemplate {
        valid: false,
        messages: Vec::new(),
    }
}

fn whitelist(uuid: Uuid, jvm: &JavaVM, plugin: &GlobalRef) {
    let env = jvm.attach_current_thread().unwrap();
    env.call_method(
        env.call_method(
            env.call_method(plugin.as_obj(), "getServer", "()Lorg/bukkit/Server;", &[])
                .unwrap()
                .l()
                .unwrap(),
            "getOfflinePlayer",
            "(Ljava/util/UUID;)Lorg/bukkit/OfflinePlayer;",
            &[env
                .new_object(
                    "java/util/UUID",
                    "(JJ)V",
                    &[
                        i64::from_be_bytes(uuid.as_bytes()[0..8].try_into().unwrap()).into(),
                        i64::from_be_bytes(uuid.as_bytes()[8..16].try_into().unwrap()).into(),
                    ],
                )
                .unwrap()
                .into()],
        )
        .unwrap()
        .l()
        .unwrap(),
        "setWhitelisted",
        "(Z)V",
        &[true.into()],
    )
    .unwrap();
}

#[get("/verify?<token>")]
fn verify<'a>(
    token: Uuid,
    pending_users: State<Mutex<Vec<SentEmails>>>,
    jvm: State<JavaVM>,
    plugin: State<GlobalRef>,
) -> VerifyTemplate {
    let mut pending_users = pending_users.lock().unwrap();
    match pending_users
        .iter()
        .position(|pending_user| pending_user.token == *token)
    {
        Some(i) => {
            if pending_users[i].at.elapsed().unwrap().as_secs() < 60 * 60 * 24 {
                whitelist(pending_users[i].mojang_uuid, &*jvm, &*plugin);
                pending_users.remove(i);
                VerifyTemplate {
                    valid: true,
                    message: format!("You have been successfully registered. You can now join the server at ip/domain '{}'", MC_SERVER_ADDR.as_str()),
                }
            } else {
                pending_users.remove(i);
                VerifyTemplate {
                    valid: false,
                    message: "The link has expired, re-register.".to_string(),
                }
            }
        }
        None => VerifyTemplate {
            valid: false,
            message: "The token is invalid".to_string(),
        },
    }
}

fn get_uuid(username: &str) -> Result<Uuid, ()> {
    reqwest::get(&format!(
        "https://api.mojang.com/users/profiles/minecraft/{}",
        username
    ))
    .or(Err(()))
    .and_then(|mut response| response.json::<MojangUUIDResponse>().or(Err(())))
    .and_then(|parsed_response| Uuid::from_str(&parsed_response.id).or(Err(())))
}

use lettre::smtp::SmtpTransport;

#[post("/", data = "<user>")]
fn register_post(
    user: Form<UserForm>,
    smtp_transport: State<Arc<Mutex<SmtpTransport>>>,
    pending_users: State<Mutex<Vec<SentEmails>>>,
) -> RegisterTemplate {
    let mut user = user.into_inner();
    user.email = user.email.trim().to_string();
    let mut template = RegisterTemplate {
        valid: true,
        messages: Vec::new(),
    };
    match user.email.find('@') {
        Some(at_index) => {
            if user.email.split_at(at_index + 1).1 != VALID_DOMAIN.as_str() {
                template.messages.push(format!(
                    "Invalid email address. Only addresses with the domain {} are accepted",
                    VALID_DOMAIN.as_str()
                ));
                template.valid = false;
            }
            if !user.email.chars().next().unwrap().is_digit(10) {
                template.messages.push(format!("The entered email address seems to be a staff email address. To join the server w/o a student address please email the admin at {}.", SUPPORT_EMAIL_ADDRESS.as_str()));
                template.valid = false;
            }
        }
        None => {
            template.valid = false;
            template
                .messages
                .push(String::from("Invalid email address"));
        }
    }
    if template.valid {
        match get_uuid(&user.username) {
            Ok(uuid) => {
                use lettre::Transport;
                let token = uuid_crate::Uuid::new_v4();
                pending_users.lock().unwrap().push(SentEmails {
                    at: SystemTime::now(),
                    mojang_uuid: uuid,
                    token: token,
                });
                let smtp_transport = Arc::clone(&smtp_transport);
                template.messages.push(format!(
                    "An email has been sent to {} in order to confirm your identity",
                    user.email
                ));
                std::thread::spawn(move || {
                    smtp_transport
                        .lock()
                        .unwrap()
                        .send(
                            lettre_email::Email::builder()
                                .to(&*user.email)
                                .from(SMTP_USERNAME.as_str())
                                .subject("Identity verification - OPGSMC")
                                .text(format!(
                                "Click on the link to confirm your identity: {}/verify?token={}",
                                WEBSITE_URL.as_str(), token
                            ))
                                .build()
                                .unwrap()
                                .into(),
                        )
                        .unwrap();
                });
            }
            Err(_) => {
                template.valid = false;
                template
                    .messages
                    .push(format!("{} is an invalid username.", &user.username));
            }
        }
    }
    template
}

#[no_mangle]
pub extern "system" fn Java_re_nilsand_opgsmc_whitelist_App_onEnable(env: JNIEnv, class: JClass) {
    use lettre::smtp::authentication::{Credentials, Mechanism};
    use lettre::smtp::ConnectionReuseParameters;
    let jvm = env.get_java_vm().unwrap();
    let class = env.new_global_ref(*class).unwrap();

    std::thread::spawn(move || {
        rocket::ignite()
            .manage(Arc::new(Mutex::new(
                lettre::SmtpClient::new(
                    (SMTP_ADDR.as_str(), *SMTP_PORT),
                    lettre::ClientSecurity::Required(lettre::ClientTlsParameters::new(
                        SMTP_ADDR.to_string(),
                        native_tls::TlsConnector::builder().build().unwrap(),
                    )),
                )
                .unwrap()
                .credentials(Credentials::new(
                    SMTP_USERNAME.to_string(),
                    SMTP_PASSWORD.to_string(),
                ))
                .authentication_mechanism(Mechanism::Login)
                .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
                .transport(),
            )))
            .manage(Mutex::new(Vec::<SentEmails>::new()))
            .manage(jvm)
            .manage(class)
            .mount("/", routes![register_get, register_post, verify])
            .launch();
    });
}
