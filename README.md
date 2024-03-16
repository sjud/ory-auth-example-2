# Leptos Auth Ory Integration (With Axum)
This repo used [start-axum-workspace](https://github.com/leptos-rs/start-axum-workspace/) as a base.

## How to read this example.

In the e2e crate in the project's root, each feature is labeled in order of implementation and describes the high level over view of the flow.
<br>
Each feature file corresponds to a feature, i.e login.feature -> login.rs which implements the integration with Ory Kratos.
<br>
Comments are written to explain the overall flow of kratos and how the integration works with leptos.

## High Level Overview

Our project is uses a service based paridgm which runs a leptos server alongiside various Ory Services, currently Ketos and Kratos. Kratos provides identification, and we use it when registering users, and credentialing them.
<br>
A normal flow would look something like:<br>
<ul>
<li>
I go to the homepage,I click register
</li>
</li>
I am redirected to the register page, the register page isn't hardcoded but is rendered by parsing the UI data structure given by Ory Kratos. The visible portions correspond to the fields we've set in our ./kratos/email.schema.json schema file, but it includes
hidden fields (i.e a CSRF token to prevent CSRF). This project includes unstyled parsing code for the UI data structure.
</li>
<li>
I sign up with an email and password
</li>
<li>
Our leptos server will intercept the form data and then pass it on to the ory kratos service.
</li>
<li>
Ory Kratos validates those inputs given the validation criteria ./kratos/email.schema.json schema file
</li>
<li>
Ory Kratos then verifies me by sending me an email.
</li>
<li>
In this example we catch the email with an instance of mailcrab (an email server for testing purposes we run in our docker compose)
</li>
<li>
I look inside the email, I see a code and a link where I will input the code.
</li>
<li>
I click through and input the code, and I am verified.
</li>
<li>
When I go to the login page, it's rendered based on the same method as the registration page. I.e Kratos sends a UI data structure which is parsed into the UI we show the user.
</li>
<li>
I use my password and email on the login page to login.
</li>
<li>
Again, Our leptos server acts as the inbetween between the client and the Ory Kratos service. There were some pecularities between the CSRF token being set in the headers (which Ory Kratos updates with every step in the flow), SSR, and having the client communicate directly with Ory Kratos which lead me to use this approach where our server is the intermediary between the client and Ory Kratos.
</li>
<li>
Ory Kratos is session based, so after it recieves valid login credentials it creates a session and returns the session token. The session token is passed via cookies with every future request. All this does is establish the identity of the caller, to perform authentication we need a way to establish permissions given an individuals identity and how that relates to the content on the website. We use Ory Ketos for this. Kratos is Identification, Ketos is Authorization.
</li>
</ul>

When given bad input in a field, Ory Kratos issues a new render UI data structure with error messages and we rerender the login page.

## With regards to Ory Oathkeeper.

Ory Oathkeeper is a reverse proxy that sits between your server and the client, it takes the session token, looks to see what is being requested in the request and then checks the configuration files of your Ory Services to see if such a thing is allowed. It will communicate with the Ory services on your behalf and then pass on the authorized request to the appropriate location or reject it otherwise.
<br>
In this example we instead use an extractor to extract the session cookie and verify it with our kratos service and then in a series of middleware check permissions on server functions.
<br>
The idea here is that during deployment you'd have a virtual private server and you'd serve your leptos server behind Nginx, Nginx routes the calls to the Leptos Server and never to our Ory services. And the leptos server handles all the communication between the client and Ory services. This is simpler from an implementation perspective then including Ory Oathkeeper. Ory services presume all api calls they recieve are valid by default, so it's best not to expose them at all to any traffic from the outside world. And when building our leptos app we'll have a clear idea about when and how these services are being communicated with.

## How this project is tested

We use Gherkin feature files to describe the behavior of the application. We use [cucumber](https://docs.rs/cucumber/latest/cucumber/) as our test harness and match the feature files mostly to [fantoccinni](https://docs.rs/fantoccini/latest/fantoccini/) code to drive our chromedriver that we run alongside our services in our docker compose. In a production environment we'd have a seperate docker compose that didn't include mailcrab or chromedriver and we wouldn't test in our VPS but during our github workflow.
<br>
The `ids` crate includes a list of static strings that we'll use in our fantoccini lookups and our frontend to make our testing as smooth as possible. There are other ways to do this, such as find by text, which would find the "Sign Up" text and click it etc. But for the purposes of testing the functionality of our integrations, not merely our expectations with regards to their presentation,and while making the most of Gherkin as a way to actually drive developer velocity/confidence; we're going to be using ids and static URL's which will be consistent across our tests, frontend, server etc. All of which will be in the ids crate.

## How to use mkcert to get a locally signed certificate (and why)
We need to use https because we are sending cookies with the `Secure;` flag, cookies with the Secure flag can't be used in Chrome
unless delivered over https. Since we're using chromedriver for e2e testing let's use mkcert to create a cert that will allow 
https://docker.internal.host:3000/ to be a valid url.

## How this project's git workflow works

## Thoughts, Feedback, Criticism, Comments?
Send me any of the above, I'm @sjud on leptos discord. I'm always looking to improve and make these examples more helpful for the community. So please let me know how I can do that. Thanks!