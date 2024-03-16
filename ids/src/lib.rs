pub static REGISTER_BUTTON_ID:&'static str = "register_button_id";
pub static REGISTRATION_FORM_ID:&'static str = "registration_form_id";

pub static EMAIL_INPUT_ID:&'static str = "email_input_id";
pub static PASSWORD_INPUT_ID:&'static str = "password_input_id";

pub static VERIFY_EMAIL_DIV_ID:&'static str = "verify_email_div_id";

pub static REGISTER_ROUTE:&'static str = "/register";
pub static VERIFY_EMAIL_ROUTE:&'static str = "/check_email_for_verification";


pub static ERROR_ERROR_ID:&'static str = "error_template_id";
pub static ERROR_COOKIES_ID:&'static str = "error_cookies_id";

pub static KRATOS_FORM_SUBMIT_ID:&'static str = "kratos_form_submit_id";
/// This function is for use in kratos_html, it takes the name of the input node and it 
/// matches it according to what we've specified in the kratos schema file. If we change the schema.
/// I.e use a phone instead of an email, the identifer id will change and break tests that expect an email.
/// i.e use oidc instead of password, as auth method... that will break tests too.
/// Which is good.
pub fn match_name_to_id(name:String) -> &'static str {
    match name.as_str() {
        "traits.email" => EMAIL_INPUT_ID,
        "password" => PASSWORD_INPUT_ID,
        _ => ""
    }
}