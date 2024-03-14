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