@register
Feature: Register

    As a user
    I want to register
    So that I can login and see secret data.

    Background:
        Given I am on the homepage


    @test-navigate-see-register
    Scenario: From the homepage, navigate to the registration page and see the registration form.
        When I click register
        Then I see the registration form

    @test-register-valid
    Scenario: From the registration page, enter valid data and submit form, and redirect to the verify email page.
        Given I am on the registration page
        And I see the registration form
        When I enter valid credentials
        Then I am on the verify email page
    
    @test-register-valid-verify-email
    Scenario: From the registration page, enter valid data and submit form, and redirect to the verify email page.
        Given I am on the registration page
        And I see the registration form
        When I enter valid credentials
        And I check my email for the verification link and code
        And I copy the code onto the verification link page
        Then I am on the homepage